use crate::core::game_entities::{HubStatus, Player};
use crate::hub_comm::hw::hw_hub_manager::HubManagerError;
use crate::hub_comm::hw::internal::api_types::{TermButtonState, TermEvent};
use crate::hub_comm::hw::internal::hub_protocol_io_handler::HwHubCommunicationHandler;
use error_stack::{Report, Result};
use rgb::RGB8;
use std::default::Default;
use std::fmt::Debug;
use serde::Deserialize;

#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize)]
pub enum HubType {
    #[default]
    HwHub,
    WebHub,
}

pub trait HubManager: Debug + Send + Sync {
    // Common
    fn get_hub_address(&self) -> String;
    fn probe(&mut self, port: &str) -> Result<HubStatus, HubManagerError>;
    fn discover_players(&mut self) -> Result<Vec<Player>, HubManagerError>;
    fn get_hub_timestamp(&self) -> Result<u32, HubManagerError>;
    fn set_hub_timestamp(&self, timestamp: u32) -> Result<(), HubManagerError>;
    fn set_term_light_color(&self, term_id: u8, color: RGB8) -> Result<(), HubManagerError>;
    fn set_term_feedback_led(
        &self,
        term_id: u8,
        state: &TermButtonState,
    ) -> Result<(), HubManagerError>;
    fn read_event_queue(&self) -> Result<Vec<TermEvent>, HubManagerError>;

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
    fn set_hub_radio_channel(&self, _channel_num: u8) -> Result<(), HubManagerError> {
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
