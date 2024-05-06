use std::sync::RwLockWriteGuard;

use crate::api::dto::{HubRequestDto, HubResponseDto};
use crate::core::app_context::app;
use crate::hub::hub_api::{HubManager, HubManagerError};
use tauri::command;

use crate::hub::hw::internal::api_types::{HwHubIoError, HwHubRequest};

/// Calls HUB to set specific radio channel
#[command]
pub fn set_hw_hub_radio_channel(channel_id: i32) {
    log::info!("Got channel id: {channel_id}");
    app().set_hub_radio_channel(channel_id as u8);
}

/// HUB Debug API
#[command]
pub fn dbg_setup_hub_connection(port_name: String) -> Result<(), HubManagerError> {
    log::info!("Trying to open HUB connection");
    let game_ctx = app();
    let mut hub = game_ctx.hub_mut();
    hub.setup_hub_connection(&port_name).map_err(|e| {
        log::error!("Operation failed: {:?}", e);
        e.current_context().clone()
    })
}

#[command]
pub fn dbg_send_raw_request_frame(request_frame: Vec<u8>) -> Result<Vec<u8>, HwHubIoError> {
    log::info!("Sending raw frame request to HUB");
    let guard = app();
    let hub_guard = guard.hub_mut();
    let Ok(handler) = hub_guard.hub_io_handler() else {
        return Err(HwHubIoError::NotInitializedError);
    };

    handler.send_raw_frame(request_frame).map_err(|e| {
        log::error!("Operation failed: {:?}", e);
        e.current_context().clone()
    })
}

#[command]
pub fn dbg_send_hub_command(request: HubRequestDto) -> Result<HubResponseDto, HubManagerError> {
    log::info!("Sending request to HUB.\n{:#?}", request);
    let guard = app();
    let mut hub_guard = guard.hub_mut();

    let request_enum = HwHubRequest::from_debug_request(request);
    let result = dbg_process_hub_command(&mut hub_guard, request_enum).map_err(|e| {
        log::error!("{:?}", e);
        e.current_context().clone()
    })?;

    let dto = HubResponseDto {
        request_frame: "Watch logs (DEBUG)".to_string(),
        response_frame: "Watch logs (DEBUG)".to_string(),
        generic_response_obj: "".to_string(),
        response_obj: result,
    };
    Ok(dto)
}

fn dbg_process_hub_command(
    hub_guard: &mut RwLockWriteGuard<Box<dyn HubManager>>,
    request_enum: HwHubRequest,
) -> error_stack::Result<String, HubManagerError> {
    match request_enum {
        HwHubRequest::SetTimestamp(timestamp) => {
            hub_guard.set_hub_timestamp(timestamp)?;
            Ok("".to_owned())
        }
        HwHubRequest::GetTimestamp => {
            let timestamp = hub_guard.calc_hub_timestamp()?;
            Ok(format!("Hub timestamp: {}", timestamp))
        }
        HwHubRequest::SetHubRadioChannel(channel_num) => {
            hub_guard.set_hw_hub_radio_channel(channel_num)?;
            Ok("Set hub radio channel successfully".to_owned())
        }
        HwHubRequest::SetTermRadioChannel(term_id, channel_num) => {
            hub_guard.set_term_radio_channel(term_id, channel_num)?;
            Ok(format!(
                "Set terminal {} radio channel successfully",
                term_id
            ))
        }
        HwHubRequest::PingDevice(term_id) => {
            hub_guard.ping_terminal(term_id)?;
            Ok(format!("Ping terminal {} successfully", term_id))
        }
        HwHubRequest::SetLightColor(term_id, color) => {
            hub_guard.set_term_light_color(term_id, color)?;
            Ok(format!("Set terminal {} light color successfully", term_id))
        }
        HwHubRequest::SetFeedbackLed(term_id, state) => {
            hub_guard.set_term_feedback_led(term_id, &state.into())?;
            Ok(format!(
                "Set terminal {} feedback LED to {} successfully",
                term_id, state
            ))
        }
        HwHubRequest::ReadEventQueue => {
            let events = hub_guard.read_event_queue()?;
            Ok(format!("Event queue: {:#?}", events))
        }
    }
}
