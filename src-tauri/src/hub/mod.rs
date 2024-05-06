pub mod hub_api;
pub mod hw {
    pub mod hw_hub_api;
    pub mod hw_hub_manager;
    pub mod virtual_hw_hub;
    pub mod internal {
        pub mod api_types;
        pub mod byte_handler;
        pub mod hw_hub_device;
    }
}
pub mod web {
    pub mod web_hub_api;
    pub mod web_hub_manager;
    pub mod web_server {
        pub mod internal_api;
        pub mod player_api;
        pub mod server;
    }
}
