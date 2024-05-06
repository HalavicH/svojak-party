pub mod api {
    pub mod dto;
    pub mod events;
    pub mod mapper;

    pub mod controller {
        pub mod gameplay;
        pub mod startup;
    }
}

pub mod core {
    pub mod app_context;
    pub mod game_context;
    pub mod game_entities;
    pub mod player_listener;
    pub mod term_event_processing;
}

pub mod game_pack {
    pub mod game_pack_entites;
    pub mod game_pack_loader;
    mod pack_content_dto;
    pub mod pack_content_entities;
    pub mod pack_content_loader;
}

pub mod hub;
pub mod types;
