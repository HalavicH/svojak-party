// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
#[allow(unused_imports)]
use rocket::futures::io::Window;
use svojak_app::core::game_controller::game;
use svojak_app::host_api::controller::debug_api::*;
use svojak_app::host_api::controller::gameplay_api::*;
use svojak_app::host_api::controller::startup::*;
use svojak_app::host_api::controller::startup::game_ctx::*;
use svojak_app::host_api::controller::startup::player_server::*;

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    println!("Logger env var is: {:#?}", env::var("RUST_LOG"));
    env_logger::init();

    log_ctx_content();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Window setup api
            is_debug_mode,
            init_window_handle,
            // Event requests
            request_context_update,
            // Startup API
            set_hub_type,
            discover_hub,
            set_hw_hub_radio_channel,
            save_players,
            init_game_pack,
            save_round_duration,
            init_game_pack,
            start_new_game,
            // Gameplay API
            select_question,
            allow_answer,
            answer_question,
            send_pip_victim,
            stop_asking_and_show_answer,
            finish_question,
            init_next_round,
            finish_game,
            reset_game,
            // Debug API
            dbg_setup_hub_connection,
            dbg_send_raw_request_frame,
            dbg_send_hub_command,
            dbg_set_game_state,
            dbg_reset_game,
        ])
        .run(tauri::generate_context!())
        .expect("Can't start Tauri app");

    /*
     *  Game loader usage example
     *
     * let game: GameInstance = load_game("path/to/game_package.siq");
     *
     * log::info!("{:#?}", game);
     *
     * Before using such modules should be included:
     *
     * use svoyak_tauri_app::game_process::game_processor::load_game;
     * use svoyak_tauri_app::game_process::game_info::GameInstance;
     *
     * Out example:
     *
     * GameInstance {
     *     information: GameInfo {
     *         pack_content_dir: TempDir {
     *             path: "/var/folders/s2/99s5p5054xz0kgpp1y4f0slm0000gn/T/.tmpha0THA",
     *         },
     *         pack_content_file_path: "/var/folders/s2/99s5p5054xz0kgpp1y4f0slm0000gn/T/.tmpha0THA/content.xml",
     *         pack_video_path: "/var/folders/s2/99s5p5054xz0kgpp1y4f0slm0000gn/T/.tmpha0THA/Video",
     *         pack_images_path: "/var/folders/s2/99s5p5054xz0kgpp1y4f0slm0000gn/T/.tmpha0THA/Images",
     *         pack_audio_path: "/var/folders/s2/99s5p5054xz0kgpp1y4f0slm0000gn/T/.tmpha0THA/Audio",
     *     },
     *     package: Package {
     *         ...
     *     }
     * }
     */
}

fn log_ctx_content() {
    let context = game();
    log::info!("default context: {context:#?}");
}
