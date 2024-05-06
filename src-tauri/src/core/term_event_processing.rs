use std::collections::HashMap;
use crate::hub_comm::common::hub_api::HubManager;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, mpsc, Mutex, RwLock, RwLockReadGuard};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::{Duration, Instant};
use error_stack::Report;
use crate::core::game_entities::{GameplayError, Player};
use crate::hub_comm::hw::hw_hub_manager::HubManagerError;
use crate::hub_comm::hw::internal::api_types::TermButtonState::Pressed;

use crate::hub_comm::hw::internal::api_types::TermEvent;

const EVT_POLLING_INTERVAL_MS: u64 = 1000;

pub fn start_event_listener(
    hub: Arc<RwLock<Box<dyn HubManager>>>,
    sender: Sender<TermEvent>,
) -> JoinHandle<()> {
    log::info!("Starting event listener");

    thread::spawn(move || {
        listen_hub_events(hub, sender);
    })
}

fn listen_hub_events(hub: Arc<RwLock<Box<dyn HubManager>>>, sender: Sender<TermEvent>) {
    loop {
        log::debug!("############# NEW ITERATION ###############");
        sleep(Duration::from_millis(EVT_POLLING_INTERVAL_MS));
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
            process_term_event(&hub_guard, e, &sender);
        });
    }
}

fn process_term_event(
    hub_guard: &RwLockReadGuard<Box<dyn HubManager>>,
    e: &TermEvent,
    sender: &Sender<TermEvent>,
) {
    hub_guard
        .set_term_feedback_led(e.term_id, &e.state)
        .unwrap_or_else(|error| {
            log::error!("Can't set term_feedback let. Err {:?}", error);
        });

    sender
        .send((*e).clone())
        .map_err(|e| {
            log::error!("Can't send the event: {}", e);
        })
        .unwrap_or_default();
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

pub fn get_fastest_click_from_hub(receiver: &Arc<Mutex<Box<Receiver<TermEvent>>>>, allow_answer_timestamp: u32, players: &HashMap<u8, Player>) -> error_stack::Result<u8, GameplayError> {
    let req = FastestClickRequest::new(Instant::now(), Duration::from_secs(10));

    loop {
        if req.is_timed_out() {
            log::error!("Timeout waiting for fastest click from hub");
            Err(GameplayError::AnswerRequestTimeout)?;
        }

        let receiver_guard = receiver.lock().expect("Mutex poisoned");
        let events = get_events(&receiver_guard)?;
        let filtered = filter_irrelevant_events(allow_answer_timestamp, events, players);
        if filtered.len() == 0 {
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

fn sort_by_timestamp(filtered: Vec<TermEvent>) -> Vec<TermEvent> {
    let mut sorted = filtered;
    sorted.sort_by(|e1, e2| e1.timestamp.cmp(&e2.timestamp));
    sorted
}

fn filter_irrelevant_events(allow_answer_timestamp: u32, events: Vec<TermEvent>, players: &HashMap<u8, Player>) -> Vec<TermEvent> {
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
            if e.state != Pressed {
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

fn get_events(
    receiver: &Receiver<TermEvent>,
) -> error_stack::Result<Vec<TermEvent>, GameplayError> {
    let mut events: Vec<TermEvent> = Vec::new();
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
