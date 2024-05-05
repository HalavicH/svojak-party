use crate::api::dto::{AppContextDto, PackErrorData};
use crate::api::mapper::{get_app_context_dto, map_package_to_pack_info_dto, update_players};
use crate::core::game_entities::{GameplayError, Player, PlayerState};
use error_stack::Report;
use tauri::command;

use crate::api::dto::PlayerSetupDto;
use crate::api::events::emit_pack_info;
use crate::core::app_context::{app, app_mut};

use crate::game_pack::game_pack_loader::{load_game_pack, GamePackLoadingError};

pub mod context;
pub mod hub;
pub mod hw_hub;
pub mod pack;
