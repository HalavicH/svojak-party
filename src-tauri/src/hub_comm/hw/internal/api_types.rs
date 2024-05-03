use crate::api::dto::HubRequestDto;
use rgb::RGB8;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Error)]
pub enum HwHubIoError {
    #[error("Hub not initialized")]
    NotInitializedError,
    #[error("Serial port error")]
    SerialPortError,
    #[error("No response from hub")]
    NoResponseFromHub,
    #[error("Corrupted response from hub")]
    CorruptedResponseFromHub,
    #[error("Internal error")]
    InternalError,
}

/// HUB REQUEST
pub enum HwHubRequest {
    SetTimestamp(u32),
    GetTimestamp,
    SetHubRadioChannel(u8),
    SetTermRadioChannel(u8, u8),
    PingDevice(u8),
    SetLightColor(u8, RGB8),
    SetFeedbackLed(u8, bool),
    ReadEventQueue,
}

impl HwHubRequest {
    pub fn from_debug_request(request: HubRequestDto) -> Self {
        let x = request.param2.to_ne_bytes();
        let rgb = RGB8::new(x[0], x[1], x[2]);
        let state = if request.param2 != 0 { true } else { false };
        match request.cmd.as_str() {
            "set_timestamp" => HwHubRequest::SetTimestamp(request.param1),
            "get_timestamp" => HwHubRequest::GetTimestamp,
            "set_hub_radio_channel" => HwHubRequest::SetHubRadioChannel(request.param1 as u8),
            "set_term_radio_channel" => {
                HwHubRequest::SetTermRadioChannel(request.param1 as u8, request.param2 as u8)
            }
            "ping_device" => HwHubRequest::PingDevice(request.param1 as u8),
            "set_light_color" => HwHubRequest::SetLightColor(request.param1 as u8, rgb),
            "set_feedback_led" => HwHubRequest::SetFeedbackLed(request.param1 as u8, state),
            "read_event_queue" => HwHubRequest::ReadEventQueue,
            _ => todo!("Unknown string"), // Handle unknown strings if necessary
        }
    }

    pub fn cmd(&self) -> u8 {
        match self {
            HwHubRequest::SetTimestamp(_) => 0x80,
            HwHubRequest::GetTimestamp => 0x81,
            HwHubRequest::SetHubRadioChannel(_) => 0x82,
            HwHubRequest::SetTermRadioChannel(_, _) => 0x82,
            HwHubRequest::PingDevice(_) => 0x90,
            HwHubRequest::SetLightColor(_, _) => 0x91,
            HwHubRequest::SetFeedbackLed(_, _) => 0x92,
            HwHubRequest::ReadEventQueue => 0xA0,
        }
    }

    pub fn payload(&self) -> Vec<u8> {
        match self {
            HwHubRequest::SetTimestamp(timestamp) => timestamp.to_le_bytes().to_vec(),
            HwHubRequest::GetTimestamp => vec![],
            HwHubRequest::SetHubRadioChannel(channel_num) => vec![*channel_num],
            HwHubRequest::SetTermRadioChannel(term_id, channel_num) => vec![*term_id, *channel_num],
            HwHubRequest::PingDevice(term_id) => vec![*term_id],
            HwHubRequest::SetLightColor(term_id, color) => {
                vec![*term_id, color.r, color.g, color.b]
            }
            HwHubRequest::SetFeedbackLed(term_id, state) => vec![*term_id, *state as u8],
            HwHubRequest::ReadEventQueue => vec![],
        }
    }
}

/// HUB RESPONSE
#[derive(Debug, Eq, PartialEq)]
pub struct HubResponse {
    pub id: u8,
    pub status: ResponseStatus,
    pub payload_len: u8,
    pub payload: Vec<u8>,
}

impl HubResponse {
    pub fn new(id: u8, status: ResponseStatus, payload: Vec<u8>) -> Self {
        Self {
            id,
            status,
            payload_len: payload.len() as u8,
            payload,
        }
    }
}

/// HUB RESPONSE STATUS
#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub enum ResponseStatus {
    Ok = 0x00,
    GenericError = 0x80,
    TerminalNotResponding = 0x90,
    UnknownError = 0xFF,
}

impl fmt::Display for ResponseStatus {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to perform hub operation:")
    }
}

impl Error for ResponseStatus {}

impl From<u8> for ResponseStatus {
    fn from(value: u8) -> Self {
        match value {
            0x00 => ResponseStatus::Ok,
            0x80 => ResponseStatus::GenericError,
            0x90 => ResponseStatus::TerminalNotResponding,
            _ => ResponseStatus::UnknownError,
        }
    }
}

/// HUB PROTOCOL VERSION
pub enum ProtocolVersion {
    Version = 0x03,
}

impl ProtocolVersion {
    pub fn to_value(&self) -> u8 {
        0x03
    }
}

/// HUB FRAME ELEMENTS POSITION
pub mod hub_frame_pos {
    pub const PROTOCOL_VERSION: usize = 0;
    pub const TID: usize = 1;
    pub const COMMAND_OR_STATUS: usize = 2;
    pub const PAYLOAD_LEN: usize = 3;
    pub const PAYLOAD: usize = 4;
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TermEvent {
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

#[derive(Debug, Clone, Serialize)]
pub enum MyParseError {
    FromU8(u8),
}

impl fmt::Display for MyParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Invalid TermButtonState value")
    }
}

impl Error for MyParseError {}

impl TryFrom<u8> for TermButtonState {
    type Error = MyParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TermButtonState::Released),
            1 => Ok(TermButtonState::Pressed),
            _ => Err(MyParseError::FromU8(value)),
        }
    }
}
