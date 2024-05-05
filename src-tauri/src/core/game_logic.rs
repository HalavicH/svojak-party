use crate::hub_comm::common::hub_api::HubManager;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;

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
