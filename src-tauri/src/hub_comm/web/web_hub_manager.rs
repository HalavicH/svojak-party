use std::str::FromStr;
use std::{env, thread};
use std::thread::{JoinHandle, sleep};
use std::time::Duration;
use rgb::RGB8;

use error_stack::{IntoReport, Report, Result, ResultExt};
use reqwest::Url;
use tokio::runtime::Runtime;
use std::net::{Ipv4Addr};
use network_interface::{Addr, NetworkInterface};
use network_interface::NetworkInterfaceConfig;

use crate::hub_comm::common::hub_api::HubManager;
use crate::hub_comm::hw::hw_hub_manager::HubManagerError;
use crate::hub_comm::hw::internal::api_types::{TermButtonState, TermEvent};
use crate::core::game_entities::{HubStatus, Player};
use crate::hub_comm::web::web_server::internal_api::{TermFeedbackState, TermLightColorDto, TimestampDto};
use crate::hub_comm::web::web_server::internal_api::INTERNAL_API::*;
use crate::hub_comm::web::web_server::server;
use crate::hub_comm::web::web_server::server::PlayerIdentityDto;

const RETRY_INTERVAL_MS: u64 = 100;

#[derive(Debug)]
pub struct WebHubManager {
    pub base_url: Url,
    pub port: String,
    pub server_handle: Option<JoinHandle<()>>,
    pub client: reqwest::Client,
    pub rt: Runtime,
}

impl Default for WebHubManager {
    fn default() -> Self {
        let port = env::var("ROCKET_PORT").unwrap_or("8888".to_string());
        let endpoint = format!("http://127.0.0.1:{}/", port);
        log::info!("###################################");
        log::info!("Configuring manager to call port {}", port);
        log::info!("###################################");

        let manager = Self {
            port,
            base_url: Url::from_str(&endpoint).expect("Bad base url"),
            server_handle: None,
            client: Default::default(),
            rt: Runtime::new().expect("No runtime - no game :D"),
        };
        manager
    }
}

impl WebHubManager {
    fn start_hub_server(&mut self) {
        let handle = thread::spawn(move || {
            server::main();
        });
        self.server_handle = Some(handle);
    }
}

impl Drop for WebHubManager {
    fn drop(&mut self) {
        log::info!("--> Trying to drop WebHubManager <--");

        if let Some(handle) = self.server_handle.take() {
            let result = self.rt.block_on(async {
                self.client
                    .get(self.base_url.join(SHUTDOWN).expect("Bad URL join"))
                    .send().await
            }).into_report().change_context(HubManagerError::HttpCommunicationError);

            match result {
                Ok(_) => {
                    handle.join().expect("Can't join thread");
                }
                Err(err) => {
                    log::error!("Ну.. Прес F. Хз що робити. Err {:?}", err);
                }
            }

        }
    }
}

#[allow(dead_code, unused_variables)]
impl HubManager for WebHubManager {
    fn get_hub_address(&self) -> String {
        get_ipv4_interfaces_ip(&self.port).join("\n")
    }

    fn probe(&mut self, _port: &str) -> Result<HubStatus, HubManagerError> {
        if self.server_handle.is_some() {
            log::debug!("Web HUB already initialized. Nothing to do");
            self.get_hub_timestamp()?;
            return Ok(HubStatus::Detected)
        }

        self.start_hub_server();
        for i in 0..50 {
            sleep(Duration::from_millis(RETRY_INTERVAL_MS));
            match self.get_hub_timestamp() {
                Ok(_) => return Ok(HubStatus::Detected),
                Err(err) => {
                    log::warn!("Can't reach web hub for {i} try. Err: {:?}", err);
                }
            }
        }

        log::error!("Web HUB can't be reached.");
        Err(Report::new(HubManagerError::HttpCommunicationError))
    }

    fn discover_players(&mut self) -> Result<Vec<Player>, HubManagerError> {
        let players: Vec<PlayerIdentityDto> = self.rt.block_on(async {
            self.client
                .get(self.base_url.join(GET_PLAYERS).expect("Bad URL join"))
                .send().await?
                .json().await
        }).into_report().change_context(HubManagerError::HttpCommunicationError)?;

        let players = players.iter()
            .map(|p| {
                let mut player = Player::default();
                player.term_id = p.id;
                player.name = p.name.clone();
                player
            })
            .collect();

        log::debug!("Received players: {:?}", players);
        Ok(players)
    }

    fn get_hub_timestamp(&self) -> Result<u32, HubManagerError> {
        let timestamp: TimestampDto = self.rt.block_on(async {
            self.client
                .get(self.base_url.join(GET_TIMESTAMP).expect("Bad URL join"))
                .send().await?
                .json().await
        }).into_report().change_context(HubManagerError::HttpCommunicationError)?;

        log::debug!("Received players: {:?}", timestamp.timestamp);
        Ok(timestamp.timestamp)
    }

    fn set_hub_timestamp(&self, timestamp: u32) -> Result<(), HubManagerError> {
        log::debug!("Setting timestamp of: {:?}", timestamp);

        self.rt.block_on(async {
            let dto = TimestampDto { timestamp };
            self.client
                .post(self.base_url.join(SET_TIMESTAMP).expect("Bad URL join"))
                .json(&dto)
                .send().await
        }).into_report().change_context(HubManagerError::HttpCommunicationError)?;
        Ok(())
    }

    fn set_term_light_color(&self, term_id: u8, color: RGB8) -> Result<(), HubManagerError> {
        log::debug!("Setting term {} color to {}", term_id, color);

        self.rt.block_on(async {
            let dto = TermLightColorDto {
                id: term_id,
                color: color.into(),
            };
            self.client
                .post(self.base_url.join(SET_TERM_COLOR).expect("Bad URL join"))
                .json(&dto)
                .send().await
        }).into_report().change_context(HubManagerError::HttpCommunicationError)?;
        Ok(())
    }

    fn set_term_feedback_led(&self, term_id: u8, state: &TermButtonState,
    ) -> Result<(), HubManagerError> {
        log::debug!("Setting feedback light for {} to {:?}", term_id, state);

        self.rt.block_on(async {
            let dto = TermFeedbackState {
                id: term_id,
                state: state.to_bool(),
            };
            self.client
                .post(self.base_url.join(SET_FEEDBACK_STATE).expect("Bad URL join"))
                .json(&dto)
                .send().await
        }).into_report().change_context(HubManagerError::HttpCommunicationError)?;
        Ok(())
    }

    fn read_event_queue(&self) -> Result<Vec<TermEvent>, HubManagerError> {
        let events: Vec<TermEvent> = self.rt.block_on(async {
            self.client
                .get(self.base_url.join(TAKE_EVENT_QUEUE).expect("Bad URL join"))
                .send().await?
                .json().await
        }).into_report().change_context(HubManagerError::HttpCommunicationError)?;

        log::debug!("Received events: {:?}", events);
        Ok(events)
    }
}

fn get_ipv4_interfaces_ip(port: &String) -> Vec<String> {
    let network_interfaces = NetworkInterface::show().unwrap();
    let localhost = Ipv4Addr::from_str("127.0.0.1").unwrap();
    let mut ips: Vec<String> = vec![];

    for itf in network_interfaces.iter() {
        itf.addr.iter()
            .for_each(|a|{
                match a {
                    Addr::V4(ip) => {
                        if localhost != ip.ip {
                            println!("{:#?}", ip.ip);
                            ips.push(format!("Interface: {} --> {}:{}", itf.name, ip.ip.to_string(), port));
                        }
                    }
                    Addr::V6(_) => {}
                }
            });
    }

    return ips;
}
