// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[allow(unused_imports)]
use rocket::futures::io::Window;
use svojak_app::api::controller::gameplay::*;
use svojak_app::api::controller::startup::hub::*;
use svojak_app::api::controller::startup::hw_hub::*;
use svojak_app::api::controller::startup::pack::*;
use svojak_app::api::controller::startup::settings::*;
use svojak_app::core::app_context::app;

fn main() {
    env_logger::init();

    // let result = GameContext::new(PackContent::default(), HashMap::default())
    //     .start()
    //     .unwrap()
    //     ;

    // return;
    log_ctx_content();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Window setup api
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
            // Debug API
            dbg_setup_hub_connection,
            dbg_send_raw_request_frame,
            dbg_send_hub_command,
            // Gameplay API
            select_question,
            allow_answer,
            answer_question,
            send_pip_victim,
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
    let context = app();
    log::info!("default context: {context:#?}");
}
