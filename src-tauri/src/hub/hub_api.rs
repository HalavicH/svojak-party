use crate::core::game_entities::{HubStatus, Player};
use crate::hub::hw::internal::hw_hub_device::HwHubCommunicationHandler;
use error_stack::{IntoReport, Report, Result, ResultExt};
use rgb::RGB8;
use rocket::serde::Serialize;
use serde::Deserialize;
use std::default::Default;
use std::fmt::Debug;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize)]
pub enum HubType {
    #[default]
    HwHub,
    WebHub,
}

#[derive(Debug, Clone, Serialize, Error)]
pub enum HubManagerError {
    #[error("Api not supported for this type of HUB")]
    ApiNotSupported,
    #[error("Hub is not initialized")]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlayerEvent {
    pub term_id: u8,
    pub timestamp: u32,
    pub state: TermButtonState,
}

/// Terminal button state enum
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TermButtonState {
    Pressed,
    Released,
}

impl From<bool> for TermButtonState {
    fn from(state: bool) -> Self {
        match state {
            true => TermButtonState::Pressed,
            false => TermButtonState::Released,
        }
    }
}

impl TermButtonState {
    pub fn to_bool(&self) -> bool {
        match self {
            TermButtonState::Pressed => true,
            TermButtonState::Released => false,
        }
    }
}

pub trait HubManager: Debug + Send + Sync {
    // Common
    fn hub_address(&self) -> String;
    fn probe(&mut self, port: &str) -> Result<(), HubManagerError>;
    fn hub_status(&self) -> HubStatus;
    fn discover_players(&mut self) -> Result<Vec<Player>, HubManagerError>;
    fn calc_hub_timestamp(&self) -> Result<u32, HubManagerError>;
    fn set_hub_timestamp(&self, timestamp: u32) -> Result<(), HubManagerError>;
    fn set_term_light_color(&self, term_id: u8, color: RGB8) -> Result<(), HubManagerError>;
    fn set_term_feedback_led(
        &self,
        term_id: u8,
        state: &TermButtonState,
    ) -> Result<(), HubManagerError>;
    fn read_event_queue(&self) -> Result<Vec<PlayerEvent>, HubManagerError>;
    fn available_ports(&self) -> Vec<String>;

    // HW-specific
    fn radio_channel(&self) -> i32 {
        i32::default()
    }
    fn hub_io_handler(&self) -> Result<&HwHubCommunicationHandler, HubManagerError> {
        Err(Report::new(HubManagerError::ApiNotSupported))
    }
    fn setup_hub_connection(&mut self, _port: &str) -> Result<(), HubManagerError> {
        Err(Report::new(HubManagerError::ApiNotSupported))
    }
    fn set_hw_hub_radio_channel(&mut self, _channel_num: u8) -> Result<(), HubManagerError> {
        Err(Report::new(HubManagerError::ApiNotSupported))
    }
    fn set_term_radio_channel(
        &self,
        _term_id: u8,
        _channel_num: u8,
    ) -> Result<(), HubManagerError> {
        Err(Report::new(HubManagerError::ApiNotSupported))
    }
    fn ping_terminal(&self, _term_id: u8) -> Result<(), HubManagerError> {
        Err(Report::new(HubManagerError::ApiNotSupported))
    }
}

/// Misc
/// TODO: Move to separate project
pub fn calc_current_epoch_ms() -> Result<u32, HubManagerError> {
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
        let result = calc_current_epoch_ms();

        // Check the result
        assert!(result.is_ok());
        let _execution_offset = 100;
        let _timestamp = result.expect("Test");
        // assert!(timestamp > expected_milliseconds_since_base &&
        //     timestamp < (expected_milliseconds_since_base + execution_offset));
    }
}
