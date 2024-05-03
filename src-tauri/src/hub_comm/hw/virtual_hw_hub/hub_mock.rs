use std::thread;
use std::time::Duration;

use error_stack::{Report, Result};
use rand::prelude::*;
use serialport::{SerialPort, TTYPort};
use std::collections::HashSet;
use std::io::ErrorKind;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, JoinHandle};

use crate::hub_comm::hw::hw_hub_manager::get_epoch_ms;
use crate::hub_comm::hw::internal::api_types::{
    hub_frame_pos, ResponseStatus, TermButtonState, TermEvent,
};
use crate::hub_comm::hw::internal::byte_handler::ByteHandler;
use crate::hub_comm::hw::internal::hub_protocol_io_handler::{format_bytes_hex, stuff_bytes};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn run_hub_mock() -> Result<(Box<dyn SerialPort>, JoinHandle<()>), String> {
    let (host_handle, device_tty) = TTYPort::pair().expect("Unable to create ptty pair");
    let string = device_tty.name().expect("Mock HUB. Not for prod");
    let device_handle = serialport::new(string, 0)
        .open()
        .expect("Mock HUB. Not for prod");

    let mut hub_mock = HubMock::new(Box::new(host_handle));

    hub_mock.run_event_generation();

    let handle = thread::spawn(move || {
        hub_mock.hub_mock_routine();
    });

    Ok((device_handle, handle))
}

#[derive(Debug)]
pub struct HubMock {
    port_handle: Box<dyn SerialPort>,
    terminals: Vec<u8>,
    events: Arc<Mutex<Vec<TermEvent>>>,
    byte_handler: ByteHandler,
    base_timestamp: u32,
}

impl HubMock {
    fn new(port_handle: Box<dyn SerialPort>) -> Self {
        Self {
            port_handle,
            events: Arc::new(Mutex::new(vec![])),
            terminals: generate_random_numbers(),
            byte_handler: ByteHandler::default(),
            base_timestamp: u32::default(),
        }
    }

    pub fn hub_mock_routine(&mut self) {
        loop {
            log::trace!("New reading attempt:");
            // Read data from the virtual port
            let mut buffer = [0; 1024];
            let bytes_read = match self.port_handle.read(&mut buffer) {
                Ok(val) => val,
                Err(err) => {
                    if err.kind() == ErrorKind::TimedOut {
                        thread::sleep(Duration::from_millis(50));
                        continue;
                    } else {
                        log::error!("Error in hub_mock_routine: {}", Report::new(err));
                        break;
                    }
                }
            };

            log::debug!("Request: {}", format_bytes_hex(&buffer[..bytes_read]));
            let frame = buffer[..bytes_read].to_vec();

            let response_frame = self.process_request_frame(frame);
            let stuffed = stuff_bytes(&response_frame);

            log::debug!("Responding with: {}", format_bytes_hex(&stuffed));
            let _bytes_written = self.port_handle.write(&stuffed).expect("Mock HUB. Not for prod");
        }
    }

    fn process_request_frame(&mut self, raw_frame: Vec<u8>) -> Vec<u8> {
        for byte in raw_frame {
            self.byte_handler.handle_byte(byte);
        }

        let input_frame = self.byte_handler.get_current_frame();

        if input_frame.len() < 4 {
            return vec![0x03, 0x00, 0x90, 0x00];
        }

        let version = input_frame[hub_frame_pos::PROTOCOL_VERSION];
        let tid = input_frame[hub_frame_pos::TID];
        let cmd = input_frame[hub_frame_pos::COMMAND_OR_STATUS];
        let _len = input_frame[hub_frame_pos::PAYLOAD_LEN];
        let payload = input_frame[hub_frame_pos::PAYLOAD..].to_vec();

        let result = self.process_cmd(cmd, payload);

        match result {
            Ok(response_payload) => {
                let mut response_frame = vec![version, tid, 0x00, response_payload.len() as u8];
                response_frame.append(&mut response_payload.clone());
                response_frame
            }
            Err(err) => {
                vec![version, tid, err.current_context().clone() as u8, 0x00]
            }
        }
    }

    fn process_cmd(&mut self, cmd: u8, payload: Vec<u8>) -> Result<Vec<u8>, ResponseStatus> {
        let response_payload = match cmd {
            0x80 => {
                // SetTimestamp
                self.base_timestamp = u32::from_le_bytes(
                    payload
                        .try_into()
                        .map_err(|_| Report::new(ResponseStatus::GenericError))?,
                );
                vec![]
            }
            0x81 => {
                // GetTimestamp
                self.base_timestamp.to_le_bytes().to_vec()
            }
            0x82 => {
                // SetHubRadioChannel
                self.terminals = generate_random_numbers();
                vec![]
            }
            0x83 => {
                // SetTermRadioChannel
                vec![]
            }
            0x90 => {
                // PingDevice
                let id = payload[0];
                return if self.terminals.contains(&id) {
                    Ok(vec![])
                } else {
                    Err(Report::new(ResponseStatus::TerminalNotResponding))
                };
            }
            0x91 => {
                // SetLightColor
                vec![]
            }
            0x92 => {
                // SetFeedbackLed
                vec![]
            }
            0xA0 => {
                // ReadEventQueue
                let events = self.read_event_queue();
                log::debug!("Events: {:?}", events);
                events
            }
            _ => panic!("Invalid command value {}", cmd),
        };
        Ok(response_payload)
    }

    pub fn run_event_generation(&mut self) {
        log::debug!("Start event generation");
        let events = self.events.clone(); // Clone the shared events Arc<Mutex<Vec<TermEvent>>>
        let mut terminals = self.terminals.clone(); // Clone the terminals vector

        // Spawn a new thread to generate events
        thread::spawn(move || {
            loop {
                let len = { events.lock().expect("Mutex is poisoned").len() };

                if len > 5 {
                    sleep(Duration::from_millis(100));
                    continue;
                }

                log::debug!("Current events num: {}. Adding new", len);

                terminals.shuffle(&mut thread_rng());
                terminals.iter().for_each(|id| {
                    let timestamp = get_epoch_ms().expect("Mock HUB. Not for prod");
                    let state = if timestamp % 2 == 0 {
                        TermButtonState::Pressed
                    } else {
                        TermButtonState::Released
                    };
                    let term_event = TermEvent {
                        term_id: *id,
                        timestamp,
                        state,
                    };

                    sleep(Duration::from_millis(10));

                    // Lock the mutex and push the event into the shared vector
                    let mut guard = events.lock().expect("Mock HUB. Not for prod");

                    if guard.len() > 5 {
                        return;
                    }
                    guard.push(term_event);
                });
            }
        });
    }

    pub fn read_event_queue(&mut self) -> Vec<u8> {
        let mut events = self.events.lock().expect("Mock HUB. Not for prod");

        log::debug!("Events registered by HUB: {:#?}", events);
        let mut response = vec![];

        events.iter().for_each(|event| {
            response.push(event.term_id);
            response.extend(&event.timestamp.to_le_bytes());
            response.push(if event.state.to_bool() { 0x01 } else { 0x00 });
        });

        events.clear();

        response
    }
}

fn generate_random_numbers() -> Vec<u8> {
    let mut rng = thread_rng();
    let random_count = rng.gen_range(2..=10);
    let mut set = HashSet::new();
    let mut numbers = Vec::new();

    numbers.push(random_count as u8);

    while set.len() < random_count as usize {
        let num = rng.gen_range(1..=10) as u8;
        set.insert(num);
    }

    numbers.extend(set.into_iter());
    numbers
}
