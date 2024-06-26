use crate::core::game_entities::{GameplayError, Player, PlayerState};
use crate::host_api::dto::PlayerDto;
use crate::host_api::events::{emit_error, emit_hub_config};
use crate::hub::hub_api::{HubManager, HubType, PlayerEvent};
use crate::hub::hw::hw_hub_manager::HwHubManager;
use crate::hub::web::web_hub_manager::WebHubManager;
use crate::types::{ArcRwBox, Swap};
use error_stack::Report;
use std::ops::Deref;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::Duration;
use log::log;
use crate::core::game_controller::{game, game_mut};
use crate::player_server::entities::PsPlayer;
use crate::player_server::player_connection_listener::{run_player_discovery_loop};

lazy_static::lazy_static! {
    static ref PLAYER_SERVER: Arc<RwLock<PlayerServer >> = Arc::new(RwLock::new(PlayerServer::default()));
}

pub fn ps_mut() -> RwLockWriteGuard<'static, PlayerServer> {
    PLAYER_SERVER
        .write()
        .map_err(|e| format!("Mutex is poisoned: {e:#?}"))
        .expect("Mutex is poisoned")
}

pub fn ps() -> RwLockReadGuard<'static, PlayerServer> {
    PLAYER_SERVER
        .read()
        .map_err(|e| format!("Mutex is poisoned: {e:#?}"))
        .expect("Mutex is poisoned")
}

pub struct PlayerServer {
    hub: ArcRwBox<dyn HubManager>,
    hub_type: HubType,
    player_poling_thread_handle: Option<JoinHandle<()>>,
    event_poling_thread_handle: Option<JoinHandle<()>>,
    players: ArcRwBox<Vec<PsPlayer>>,
}

impl Default for PlayerServer {
    fn default() -> Self {
        Self {
            hub_type: HubType::default(),
            hub: ArcRwBox::new(RwLock::new(Box::<HwHubManager>::default())),
            player_poling_thread_handle: None,
            event_poling_thread_handle: None,
            players: Arc::new(Default::default()),
        }
    }
}

impl PlayerServer {
    pub fn request_context_update(&self) {
        emit_hub_config(self.hub().deref().into());
    }

    pub fn drop_hub(&mut self) {
        self.hub = Arc::new(RwLock::new(Box::<HwHubManager>::default()))
    }

    pub fn hub_lock(&self) -> Arc<RwLock<Box<dyn HubManager>>> {
        self.hub.clone()
    }

    pub fn hub(&self) -> RwLockReadGuard<Box<dyn HubManager>> {
        self.hub
            .read()
            .map_err(|e| {
                Report::new(GameplayError::InternalError)
                    .attach_printable(format!("Can't get HUB for read. {:?}", e))
            })
            .expect("Poisoned")
    }

    pub fn hub_mut(&self) -> RwLockWriteGuard<Box<dyn HubManager>> {
        self.hub
            .write()
            .map_err(|e| {
                Report::new(GameplayError::InternalError)
                    .attach_printable(format!("Can't get HUB for write. {:?}", e))
            })
            .expect("Poisoned")
    }
    pub fn set_hub_radio_channel(&self, channel_id: u8) {
        let mut hub_guard = self.hub_mut();

        match hub_guard.set_hw_hub_radio_channel(channel_id) {
            Ok(_) => {
                emit_hub_config(self.hub().deref().into());
            }
            Err(e) => {
                log::error!("{:#?}", e);
                emit_error(e.to_string())
            }
        };
    }
    pub fn select_hub_type(&mut self, hub_type: HubType) {
        log::info!("Strong count to hub: {}", Arc::strong_count(&self.hub));
        log::info!("Weak count to hub: {}", Arc::weak_count(&self.hub));

        if self.hub_type == hub_type {
            log::info!("Hub is already set to: {:?}. Nothing to do", hub_type);
            return;
        }

        match hub_type {
            HubType::HwHub => {
                log::info!("||| --> Selecting SERIAL hub <---");
                self.hub.swap(Box::<HwHubManager>::default());
            }
            HubType::WebHub => {
                log::info!("||| --> Selecting WEB hub <---");
                self.hub.swap(Box::<WebHubManager>::default());
            }
        }
        self.hub_type = hub_type;
        emit_hub_config(self.hub().deref().into());
    }

    pub fn discover_hub_and_players(&mut self, path: String) {
        log::debug!(
            "Requested HUB change. Removing players as outdated: {:#?}",
            self.players
        );

        self.players = Arc::new(Default::default());

        let result = self.hub_mut().probe(&path);
        match result {
            Ok(_) => {
                emit_hub_config(self.hub().deref().into());

                self.run_polling_for_players();
                self.listen_for_player_events();
            }
            Err(err) => log::error!("Can't initialize hub on port: {}. Error: {:?}", path, err),
        }
    }

    pub fn update_players(&mut self, players: &[PlayerDto]) {
        let player_entities: Vec<Player> = players
            .iter()
            .map(|player| Player {
                icon: player.iconPath.clone(),
                name: player.name.clone(),
                term_id: player.id as u8,
                is_used: player.isUsed,
                state: PlayerState::Idle,
                stats: Default::default(),
            })
            .collect();

        log::info!("Converted players: {:#?}", player_entities);

        // let players = players.iter().map(|p| (p.term_id, p.clone())).collect();
        // self.game_state.game_mut().set_players(players);
        // No emit_players required, as we just set them, but I'll do it anyway to maintain consistency
        // let vec = app.game_state.game_ctx_ref().players_ref_as_vec();
        // emit_players(vec.into_iter().map(|p| p.into()).collect());"
    }

    /// Players polling
    fn run_polling_for_players(&mut self) {
        if self.player_poling_thread_handle.is_some() {
            log::info!("Player polling thread already started");
            return;
        }

        let hub_arc = self.hub.clone();
        let players_arc = self.players.clone();
        log::info!("Starting player polling thread");
        let handle = thread::spawn(move || run_player_discovery_loop(hub_arc, players_arc));

        log::info!("Saving new thread handle");
        self.player_poling_thread_handle = Some(handle)
    }
    /// Event listener
    pub fn listen_for_player_events(&mut self) {
        if self.event_poling_thread_handle.is_some() {
            log::info!("Event listener already started");
            return;
        }
        log::info!("Starting event listener");

        let arc = { game().get_events_handle() };
        let hub_arc = self.hub.clone();
        let handle = thread::spawn(move || {
            listen_hub_events(hub_arc, arc);
        });
        log::info!("Saving new event listener thread handle");
        self.event_poling_thread_handle = Some(handle)
    }
}

const EVT_POLLING_INTERVAL_MS: u64 = 200;

fn listen_hub_events(
    hub: Arc<RwLock<Box<dyn HubManager>>>,
    events_arc: Arc<RwLock<Vec<PlayerEvent>>>,
) {
    loop {
        sleep(Duration::from_millis(EVT_POLLING_INTERVAL_MS));
        log::debug!("### New event listener iteration ###");
        let hub_guard = hub.read().expect("Mutex is poisoned");
        let Ok(events) = hub_guard.read_event_queue() else {
            log::error!("Can't read event queue. Skipping iteration");
            continue;
        };

        if events.is_empty() {
            log::debug!("No player events occurred");
            continue;
        }

        events.iter().for_each(|e| {
            if let Err(err) = hub_guard
                .set_term_feedback_led(e.term_id, &e.state) {
                log::error!("Can't set term feedback led for term_id: {}. Error: {:#?}",
                    e.term_id, err);
            };

            log::debug!("New player event received: {:#?}. Pushing to the events", e);
        });

        log::debug!("Pushing events to the game");
        // game().push_events(events);
        events_arc.write().expect("Expected to be able acquire write lock on events")
            .extend(events);
    }
    log::error!("Event listener thread is finished unexpectedly");
}
// self.hub_type = context.hub_type;
// self.hub = context.hub;
// self.player_poling_thread_handle = context.player_poling_thread_handle;
