#![allow(dead_code)]

use std::default::Default;

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use thiserror::Error;

use crate::core::game_entities::{HubStatus, Player};
use crate::hub_comm::common::hub_api::HubManager;
use crate::hub_comm::hw::internal::api_types::{
    HwHubIoError, HwHubRequest, ResponseStatus, TermButtonState, TermEvent,
};
use crate::hub_comm::hw::internal::hub_protocol_io_handler::HwHubCommunicationHandler;
use crate::hub_comm::hw::virtual_hw_hub::{setup_virtual_hub_connection, VIRTUAL_HUB_PORT};
use error_stack::{IntoReport, Report, Result, ResultExt};
use rgb::RGB8;
use serde::Serialize;
use serialport::SerialPort;

const HUB_CMD_TIMEOUT: Duration = Duration::from_millis(100);
const MAX_TERMINAL_CNT: u8 = 10;

#[derive(Debug, Clone, Serialize, Error)]
pub enum HubManagerError {
    #[error("Api not supported for this type of HUB")]
    ApiNotSupported,
    #[error("Hub not initialized")]
    NotInitializedError,
    #[error("Serial port error")]
    SerialPortError,
    #[error("HTTP communication error")]
    HttpCommunicationError,
    #[error("No response from hub")]
    NoResponseFromHub,
    #[error("No response from terminal")]
    NoResponseFromTerminal,
    #[error("Internal error")]
    InternalError,
}

#[derive(Debug)]
pub struct HwHubManager {
    port_name: String,
    hub_io_handler: Option<HwHubCommunicationHandler>,
    baudrate: u32,
    radio_channel: i32,
    base_timestamp: u32,
}

impl Default for HwHubManager {
    fn default() -> Self {
        Self {
            port_name: String::default(),
            radio_channel: 0,
            baudrate: 200_000,
            base_timestamp: 0,
            hub_io_handler: None,
        }
    }
}

impl HwHubManager {
    fn setup_physical_serial_connection(
        &mut self,
        port: &str,
    ) -> Result<Box<dyn SerialPort>, HubManagerError> {
        log::info!("Try to discover hub at port: {port}");
        self.port_name = port.to_owned();

        let mut serial_port = serialport::new(port, self.baudrate)
            .open()
            .into_report()
            .change_context(HubManagerError::SerialPortError)
            .attach_printable(format!("Can't open port {port}"))?;

        serial_port
            .set_timeout(HUB_CMD_TIMEOUT)
            .into_report()
            .change_context(HubManagerError::InternalError)?;
        Ok(serial_port)
    }

    fn hub_io_to_hub_mgr_error(e: Report<HwHubIoError>) -> Report<HubManagerError> {
        match e.current_context() {
            HwHubIoError::NoResponseFromHub => e.change_context(HubManagerError::NoResponseFromHub),
            _ => e.change_context(HubManagerError::InternalError),
        }
    }

    fn init_timestamp(&mut self) -> Result<(), HubManagerError> {
        self.base_timestamp = get_epoch_ms()?;
        Ok(())
    }

    fn get_hub_handle_or_err(&self) -> Result<&HwHubCommunicationHandler, HubManagerError> {
        let connection = self
            .hub_io_handler
            .as_ref()
            .ok_or(HubManagerError::NotInitializedError)?;
        Ok(connection)
    }
}

impl HubManager for HwHubManager {
    fn discover_players(&mut self) -> Result<Vec<Player>, HubManagerError> {
        let mut players = vec![];

        for term_id in 1..MAX_TERMINAL_CNT {
            if self.ping_terminal(term_id).is_ok() {
                log::debug!("Terminal #{} is alive", term_id);
                let mut player = Player::default();
                player.term_id = term_id;
                players.push(player);
            }
        }

        Ok(players)
    }
    /// ### get hub timestamp
    /// #### response payload
    /// `[tid] [status] [response length] [response payload (timestamp)]`
    fn get_hub_timestamp(&self) -> Result<u32, HubManagerError> {
        log::info!("Reading current HUB base timestamp");
        let handle = self.get_hub_handle_or_err()?;

        let response = handle
            .send_command(HwHubRequest::GetTimestamp)
            .map_err(Self::hub_io_to_hub_mgr_error)?;

        if response.status != ResponseStatus::Ok {
            return Err(Report::new(HubManagerError::InternalError));
        }

        let bytes: [u8; 4] = response
            .payload
            .try_into()
            .map_err(|_| Report::new(HubManagerError::NoResponseFromHub))?;
        let timestamp = u32::from_le_bytes(bytes);

        log::info!("Got HUB timestamp: {}", timestamp);

        Ok(timestamp)
    }
    fn set_hub_timestamp(&self, timestamp: u32) -> Result<(), HubManagerError> {
        log::info!("Setting timestamp of 0x{:X?}", timestamp);
        let handle = self.get_hub_handle_or_err()?;

        let request = HwHubRequest::SetTimestamp(timestamp);
        let response = handle
            .send_command(request)
            .map_err(Self::hub_io_to_hub_mgr_error)?;

        map_status_to_result(response.status)
    }
    fn set_term_light_color(&self, term_id: u8, color: RGB8) -> Result<(), HubManagerError> {
        log::info!("Setting terminal #{} light color to: {:?}", term_id, color);
        let handle = self.get_hub_handle_or_err()?;

        let request = HwHubRequest::SetLightColor(term_id, color);
        let response = handle
            .send_command(request)
            .map_err(Self::hub_io_to_hub_mgr_error)?;

        map_status_to_result(response.status)
    }

    fn set_term_feedback_led(
        &self,
        term_id: u8,
        state: &TermButtonState,
    ) -> Result<(), HubManagerError> {
        log::info!(
            "Setting terminal #{} feedback light to: {:?}",
            term_id,
            state
        );
        let handle = self.get_hub_handle_or_err()?;

        let request = HwHubRequest::SetFeedbackLed(term_id, state.to_bool());
        let response = handle
            .send_command(request)
            .map_err(Self::hub_io_to_hub_mgr_error)?;

        map_status_to_result(response.status)
    }

    fn read_event_queue(&self) -> Result<Vec<TermEvent>, HubManagerError> {
        log::info!("Reading event queue");
        let handle = self.get_hub_handle_or_err()?;

        let request = HwHubRequest::ReadEventQueue;
        let response = handle
            .send_command(request)
            .map_err(Self::hub_io_to_hub_mgr_error)?;

        map_status_to_result(response.status)?;

        let mut events = vec![];
        let event_size = 6;
        for chunk in response.payload.chunks_exact(event_size) {
            log::trace!("Chunk {:?}", chunk);

            let term_id = chunk[0];
            let bytes = chunk[1..5].try_into().into_report()
                .change_context(HubManagerError::InternalError)?;
            let timestamp = u32::from_le_bytes(bytes);
            let state_byte = chunk[5];
            let state = TermButtonState::try_from(state_byte)
                .into_report()
                .change_context(HubManagerError::InternalError)
                .attach_printable(format!(
                    "Can't parse TermButtonState for terminal {}",
                    term_id
                ))?;

            let event = TermEvent {
                term_id,
                timestamp,
                state,
            };

            events.push(event);
        }

        Ok(events)
    }

    fn get_hub_address(&self) -> String {
        self.port_name.clone()
    }

    fn radio_channel(&self) -> i32 {
        self.radio_channel
    }

    fn hub_io_handler(&self) -> Result<&HwHubCommunicationHandler, HubManagerError> {
        Ok(self
            .hub_io_handler
            .as_ref()
            .ok_or(HubManagerError::NotInitializedError)?)
    }

    fn probe(&mut self, port: &str) -> Result<HubStatus, HubManagerError> {
        if let Some(hub) = &self.hub_io_handler {
            log::info!("Previous HUB io handle found: {:?}. Erasing", hub);
            self.hub_io_handler = None;
        }

        self.port_name = port.to_owned();
        self.setup_hub_connection(port)?;

        self.init_timestamp()?;
        self.set_hub_timestamp(self.base_timestamp)?;
        Ok(HubStatus::Detected)
    }

    fn setup_hub_connection(&mut self, port: &str) -> Result<(), HubManagerError> {
        if port == VIRTUAL_HUB_PORT {
            log::info!("Virtual hub selected. Let's have fun");
            let (serial_port, hub_mock_handle) = setup_virtual_hub_connection()?;
            self.hub_io_handler = Some(HwHubCommunicationHandler::new(
                serial_port,
                Some(hub_mock_handle),
            ));
        } else {
            let serial_port = self.setup_physical_serial_connection(port)?;
            self.hub_io_handler = Some(HwHubCommunicationHandler::new(serial_port, None));
        }
        Ok(())
    }

    fn set_hub_radio_channel(&self, channel_num: u8) -> Result<(), HubManagerError> {
        log::info!("Setting hub radio channel to: {}", channel_num);
        let handle = self.get_hub_handle_or_err()?;

        let request = HwHubRequest::SetHubRadioChannel(channel_num);
        let response = handle
            .send_command(request)
            .map_err(Self::hub_io_to_hub_mgr_error)?;

        map_status_to_result(response.status)
    }

    fn set_term_radio_channel(&self, term_id: u8, channel_num: u8) -> Result<(), HubManagerError> {
        log::info!(
            "Setting terminal radio channel to: {} for {}",
            channel_num,
            term_id
        );
        let handle = self.get_hub_handle_or_err()?;

        let request = HwHubRequest::SetTermRadioChannel(term_id, channel_num);
        let response = handle
            .send_command(request)
            .map_err(Self::hub_io_to_hub_mgr_error)?;

        map_status_to_result(response.status)
    }

    fn ping_terminal(&self, term_id: u8) -> Result<(), HubManagerError> {
        log::info!("Pinging terminal with id: #{}", term_id);
        let handle = self.get_hub_handle_or_err()?;

        let request = HwHubRequest::PingDevice(term_id);
        let response = handle
            .send_command(request)
            .map_err(Self::hub_io_to_hub_mgr_error)?;

        map_status_to_result(response.status)
    }
}

fn map_status_to_result(status: ResponseStatus) -> Result<(), HubManagerError> {
    match status {
        ResponseStatus::Ok => Ok(()),
        ResponseStatus::TerminalNotResponding => {
            Err(Report::new(HubManagerError::NoResponseFromTerminal))
        }
        _ => Err(Report::new(HubManagerError::InternalError)),
    }
}

/// Queries OS for all available serial ports
pub fn discover_serial_ports() -> Vec<String> {
    let ports = serialport::available_ports()
        .expect("Couldn't discover ports");
    let mut ports_vec = vec![VIRTUAL_HUB_PORT.to_owned()];

    log::info!("Serial ports: {:?}", ports);

    for p in ports {
        log::info!("{}", p.port_name);

        ports_vec.push(p.port_name.clone());
    }

    ports_vec
}

pub fn get_epoch_ms() -> Result<u32, HubManagerError> {
    let now = SystemTime::now();
    let since_the_epoch = now
        .duration_since(UNIX_EPOCH)
        .into_report()
        .attach_printable("Can't get unix time")
        .change_context(HubManagerError::InternalError)?;

    let milliseconds_since_base: u32 = since_the_epoch
        .as_secs()
        .checked_mul(1000)
        .and_then(|ms| {
            let stripped_ms = ms & 0xFFFFFFFF;
            stripped_ms.checked_add(u64::from(since_the_epoch.subsec_nanos()) / 1_000_000)
        })
        .and_then(|ms| ms.try_into().ok())
        .ok_or(HubManagerError::InternalError)
        .into_report()
        .attach_printable("Can't process UNIX time to timestamp")?;

    Ok(milliseconds_since_base)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_get_epoch_ms() {
        // Get the expected result manually
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Test");

        let _expected_milliseconds_since_base: u32 = since_the_epoch
            .as_secs()
            .checked_mul(1000)
            .and_then(|ms| {
                let stripped_ms = ms & 0xFFFFFFFF;
                stripped_ms.checked_add(u64::from(since_the_epoch.subsec_nanos()) / 1_000_000)
            })
            .and_then(|ms| ms.try_into().ok())
            .expect("Test");

        // Call the actual function
        let result = get_epoch_ms();

        // Check the result
        assert!(result.is_ok());
        let _execution_offset = 100;
        let _timestamp = result.expect("Test");
        // assert!(timestamp > expected_milliseconds_since_base &&
        //     timestamp < (expected_milliseconds_since_base + execution_offset));
    }

    #[test]
    fn test_hub_timestamp_init() {
        let mut hub = HwHubManager::default();
        assert_eq!(hub.base_timestamp, 0);

        hub.init_timestamp().expect("Test");
        assert_eq!(hub.base_timestamp, get_epoch_ms().expect("Test"));
    }

    #[test]
    fn test_event_time_offset() {
        let execution_offset = 50;
        let mut hub = HwHubManager::default();
        hub.init_timestamp().expect("Test");
        let terminal_timestamp = get_epoch_ms().expect("Test");
        assert!(
            terminal_timestamp > hub.base_timestamp
                && terminal_timestamp < (hub.base_timestamp + execution_offset)
        );

        sleep(Duration::from_secs(1));
        let terminal_timestamp = get_epoch_ms().expect("Test");

        assert!(
            terminal_timestamp > hub.base_timestamp
                && terminal_timestamp < (hub.base_timestamp + 1000 + execution_offset)
        );
    }
}
