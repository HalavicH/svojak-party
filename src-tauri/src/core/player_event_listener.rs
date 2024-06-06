use crate::core::game_entities::{GameplayError, Player};
use crate::hub::hub_api::{HubManager, PlayerEvent, TermButtonState};
use error_stack::Report;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::{Duration, Instant};

const EVT_POLLING_INTERVAL_MS: u64 = 1000;

pub fn start_event_listener(
    hub: Arc<RwLock<Box<dyn HubManager>>>,
    events_v: Arc<RwLock<Vec<PlayerEvent>>>,
) -> JoinHandle<()> {
    log::info!("Starting event listener");

    thread::spawn(move || {
        listen_hub_events(hub, events_v);
    })
}

fn listen_hub_events(
    hub: Arc<RwLock<Box<dyn HubManager>>>,
    events_lock: Arc<RwLock<Vec<PlayerEvent>>>,
) {
    loop {
        sleep(Duration::from_millis(EVT_POLLING_INTERVAL_MS));
        log::debug!("### New event listener iteration ###");
        let hub_guard = hub.read().expect("Mutex is poisoned");
        let events = hub_guard.read_event_queue().unwrap_or_else(|error| {
            log::error!("Can't get events. Err {:?}", error);
            vec![]
        });

        if events.is_empty() {
            log::debug!("No player events occurred");
            continue;
        }

        events.iter().for_each(|e| {
            hub_guard
                .set_term_feedback_led(e.term_id, &e.state)
                .unwrap_or_else(|error| {
                    log::error!("Can't set term_feedback let. Err {:?}", error);
                });

            log::debug!("New player event received: {:#?}. Pushing to the events", e);
        });

        let mut events_vec = events_lock
            .write()
            .expect("Expected to lock Rwlock to be aquired successfully");
        events_vec.extend(events);
    }
}

struct FastestClickRequest {
    start_time: Instant,
    timeout: Duration,
}

impl FastestClickRequest {
    pub fn new(start_time: Instant, timeout: Duration) -> Self {
        Self {
            start_time,
            timeout,
        }
    }
    pub(crate) fn is_timed_out(&self) -> bool {
        self.start_time.elapsed() >= self.timeout
    }
}

pub fn receive_fastest_click_from_hub(
    receiver: &Arc<Mutex<Box<Receiver<PlayerEvent>>>>,
    allow_answer_timestamp: u32,
    players: &HashMap<u8, Player>,
) -> error_stack::Result<u8, GameplayError> {
    let req = FastestClickRequest::new(Instant::now(), Duration::from_secs(10));

    loop {
        if req.is_timed_out() {
            log::error!("Timeout waiting for fastest click from hub");
            Err(GameplayError::AnswerRequestTimeout)?;
        }

        let receiver_guard = receiver.lock().expect("Mutex poisoned");
        let events = receive_events(&receiver_guard)?;
        let filtered = filter_irrelevant_events(allow_answer_timestamp, events, players);
        if filtered.is_empty() {
            log::debug!("No events after filtering. Waiting for the next iteration");
            sleep(Duration::from_secs(1));
            continue;
        }

        let sorted = sort_by_timestamp(filtered);
        if let Some(value) = sorted.first() {
            return Ok(value.term_id);
        }
    }
}

fn sort_by_timestamp(filtered: Vec<PlayerEvent>) -> Vec<PlayerEvent> {
    let mut sorted = filtered;
    sorted.sort_by(|e1, e2| e1.timestamp.cmp(&e2.timestamp));
    sorted
}

fn filter_irrelevant_events(
    allow_answer_timestamp: u32,
    events: Vec<PlayerEvent>,
    players: &HashMap<u8, Player>,
) -> Vec<PlayerEvent> {
    events
        .iter()
        .filter(|&e| {
            if e.timestamp < allow_answer_timestamp {
                log::debug!("Answer too early. Skipping: {:?}", e);
                return false;
            }
            log::debug!("After answer allowed. Event: {:?}", e);
            true
        })
        .filter(|&e| {
            if e.state != TermButtonState::Pressed {
                log::debug!("Release event. Skipping: {:?}", e);
                return false;
            }
            log::debug!("Press event - relevant. Event: {:?}", e);
            true
        })
        .filter(|&e| {
            let Some(player) = players.get(&e.term_id) else {
                log::debug!("Unknown terminal id {} event. Skipping: {:?}", e.term_id, e);
                return false;
            };
            log::debug!("Player: {} has pressed the button", e.term_id);

            if !player.allowed_to_click() {
                log::debug!(
                    "Player {} is not allowed to click. Skipping event: {:?}",
                    e.term_id,
                    e
                );
                false;
            }
            true
        })
        .cloned()
        .collect()
}

fn receive_events(
    receiver: &Receiver<PlayerEvent>,
) -> error_stack::Result<Vec<PlayerEvent>, GameplayError> {
    let mut events: Vec<PlayerEvent> = Vec::new();
    loop {
        match receiver.try_recv() {
            Ok(received_event) => {
                events.push(received_event);
            }
            Err(mpsc::TryRecvError::Empty) => {
                log::debug!("Got {} events for now.", events.len());
                break;
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                log::error!("Failed to get events: mpsc::TryRecvError::Disconnected error");
                // Channel has been disconnected, so we break the loop
                let report = Report::new(GameplayError::BrokenHubConnection)
                    .attach_printable("Failed to get events");
                return Err(report);
            }
        }
    }

    Ok(events)
}
