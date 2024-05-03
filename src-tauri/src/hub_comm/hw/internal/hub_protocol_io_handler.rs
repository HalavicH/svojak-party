use std::default::Default;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::hub_comm::hw::internal::api_types::ProtocolVersion::Version;
use crate::hub_comm::hw::internal::api_types::{
    hub_frame_pos, HubResponse, HwHubIoError, HwHubRequest, ResponseStatus,
};
use crate::hub_comm::hw::internal::byte_handler::{ByteHandler, START_BYTE, STOP_BYTE};
use error_stack::{IntoReport, Report, Result, ResultExt};
use serialport::SerialPort;

const HUB_REQUEST_PROCESSING_TIMEOUT_MS: u64 = 50;

#[derive(Debug)]
pub struct HwHubCommunicationHandler {
    fsm_byte_handler: Arc<Mutex<ByteHandler>>,
    port_handle: Arc<Mutex<Box<dyn SerialPort>>>,
    #[allow(dead_code)]
    hub_mock_handle: Option<JoinHandle<()>>,
}

impl HwHubCommunicationHandler {
    pub fn new(port_handle: Box<dyn SerialPort>, hub_mock_handle: Option<JoinHandle<()>>) -> Self {
        Self {
            port_handle: Arc::new(Mutex::new(port_handle)),
            fsm_byte_handler: Arc::new(Mutex::new(ByteHandler::default())),
            hub_mock_handle,
        }
    }

    pub fn send_raw_frame(&self, request_frame: Vec<u8>) -> Result<Vec<u8>, HwHubIoError> {
        log::debug!(
            "Request frame: {:?}",
            format_bytes_hex(request_frame.as_slice())
        );
        {
            let mut port_handle = self
                .port_handle
                .lock()
                .map_err(|_| Report::new(HwHubIoError::InternalError))?;

            port_handle
                .write_all(&request_frame)
                .into_report()
                .change_context(HwHubIoError::SerialPortError)?;
        }

        let response_frame = self.read_raw_response_frame()?;
        log::debug!(
            "Response frame: {:?}",
            format_bytes_hex(response_frame.as_slice())
        );
        Ok(response_frame)
    }

    pub fn send_command(&self, request: HwHubRequest) -> Result<HubResponse, HwHubIoError> {
        let frame = assemble_frame(request.cmd(), request.payload());
        let stuffed_frame = stuff_bytes(&frame);

        {
            let mut port_handle = self
                .port_handle
                .lock()
                .map_err(|_| Report::new(HwHubIoError::InternalError))?;

            port_handle
                .write_all(&stuffed_frame)
                .into_report()
                .change_context(HwHubIoError::SerialPortError)?;
        }

        let response_frame = self.read_response_frame()?;
        log::debug!(
            "Response frame: {:?}",
            format_bytes_hex(response_frame.as_slice())
        );

        let id = response_frame[hub_frame_pos::TID];
        let status = ResponseStatus::from(response_frame[hub_frame_pos::COMMAND_OR_STATUS]);
        let payload = response_frame[hub_frame_pos::PAYLOAD..].to_vec();
        let response = HubResponse::new(id, status, payload);
        log::trace!("Hub response: {:#?}", response);
        Ok(response)
    }

    fn read_raw_response_frame(&self) -> Result<Vec<u8>, HwHubIoError> {
        let port_handle_ptr = Arc::clone(&self.port_handle);
        let mut port_handle = port_handle_ptr.lock()
            .expect("Mutex is poisoned");
        let mut buffer = [0; 1024];

        // Give HUB some time to perform operation
        thread::sleep(Duration::from_millis(HUB_REQUEST_PROCESSING_TIMEOUT_MS));
        let bytes_read = port_handle
            .read(&mut buffer)
            .into_report()
            .change_context(HwHubIoError::NoResponseFromHub)
            .attach_printable("Probably timeout")?;

        let response = buffer[..bytes_read].to_vec();
        Ok(response)
    }

    fn read_response_frame(&self) -> Result<Vec<u8>, HwHubIoError> {
        let byte_handler_ptr = Arc::clone(&self.fsm_byte_handler);
        let port_handle_ptr = Arc::clone(&self.port_handle);
        let mut port_handle = port_handle_ptr.lock().expect("Mutex is poisoned");
        let mut byte_handler = byte_handler_ptr.lock().expect("Mutex is poisoned");
        let mut byte: [u8; 1] = [0];

        byte_handler.reset();

        // Give HUB some time to perform operation
        thread::sleep(Duration::from_millis(HUB_REQUEST_PROCESSING_TIMEOUT_MS));

        while byte[0] != START_BYTE {
            log::trace!("Byte: {}", byte[0]);
            port_handle
                .read_exact(&mut byte)
                .into_report().change_context(HwHubIoError::NoResponseFromHub)
                .attach_printable("Probably timeout")?;
        }
        // Handle start byte
        byte_handler.handle_byte(byte[0]);

        loop {
            port_handle.read_exact(&mut byte)
                .into_report().change_context(HwHubIoError::NoResponseFromHub)
                .attach_printable("Probably timeout")?;
            byte_handler.handle_byte(byte[0]);

            if byte[0] == STOP_BYTE {
                log::trace!("Finished frame reading");
                break;
            }
        }

        Ok(byte_handler.get_current_frame())
    }
}

pub fn format_bytes_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn stuff_bytes(frame: &Vec<u8>) -> Vec<u8> {
    let mut stuffed = vec![START_BYTE];
    for byte in frame {
        match *byte {
            0xC0..=0xCF => {
                stuffed.push(0xC1);
                stuffed.push(*byte & 0x0F);
            }
            _ => stuffed.push(*byte),
        }
    }
    stuffed.push(STOP_BYTE);
    log::trace!("Frame after bit stuffing: {:?}", format_bytes_hex(&stuffed));

    stuffed
}

pub fn assemble_frame(cmd: u8, mut payload: Vec<u8>) -> Vec<u8> {
    let payload_len = payload.len() as u8;
    let tid = 0;
    let mut frame = vec![Version.to_value(), tid, cmd, payload_len];
    frame.append(&mut payload);
    log::trace!("Assembled frame: {:?}", format_bytes_hex(&frame));
    frame
}

#[cfg(test)]
mod tests {
    use crate::hub_comm::hw::internal::api_types::ProtocolVersion::Version;
    use crate::hub_comm::hw::internal::hub_protocol_io_handler::{assemble_frame, stuff_bytes};

    #[test]
    fn test_frame_assembly() {
        let expected = vec![Version.to_value(), 0x00, 0x90, 0x03, 0x01, 0x02, 0x03];
        let frame = assemble_frame(0x90, vec![0x01, 0x02, 0x03]);
        assert_eq!(frame, expected);
    }

    #[test]
    fn test_byte_stuffing_when_no_stuffing_occurs() {
        let input = vec![0x03, 0x00, 0x90, 0x03, 0x01, 0x02, 0x03];
        let goal = vec![0xC0, 0x03, 0x00, 0x90, 0x03, 0x01, 0x02, 0x03, 0xCF];
        let result = stuff_bytes(&input);
        assert_eq!(result, goal);
    }

    #[test]
    fn test_byte_stuffing() {
        let input = vec![0x03, 0x00, 0x90, 0x03, 0xC0, 0xC1, 0xCF];
        let expect = vec![
            0xC0, 0x03, 0x00, 0x90, 0x03, 0xC1, 0x00, 0xC1, 0x01, 0xC1, 0x0F, 0xCF,
        ];
        let result = stuff_bytes(&input);
        assert_eq!(result, expect);
    }
}
