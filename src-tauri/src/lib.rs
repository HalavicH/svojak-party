pub mod api {
    pub mod dto;
    pub mod mapper;

    pub mod controller {
        pub mod gameplay;
        pub mod startup;
    }
}

pub mod core {
    pub mod game_entities;
    pub mod game_logic;
}

pub mod game_pack {
    pub mod game_pack_entites;
    pub mod game_pack_loader;
    mod pack_content_dto;
    pub mod pack_content_entities;
    pub mod pack_content_loader;
}

pub mod hub_comm {
    pub mod common {
        pub mod hub_api;
    }
    pub mod hw {
        pub mod hw_hub;
        pub mod hw_hub_manager;
        pub mod virtual_hw_hub;
        pub mod internal {
            pub mod api_types;
            pub mod byte_handler;
            pub mod hub_protocol_io_handler;
        }
    }
    pub mod web {
        pub mod web_hub_manager;
        pub mod web_server {
            pub mod server;
            pub mod internal_api;
            pub mod player_api;
        }
    }
}
