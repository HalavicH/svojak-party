use crate::hub::hub_api::{HubManager, PlayerEvent};
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;
use crate::types::LazyExpect;

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
        let events = hub_guard.read_event_queue()
            .expect("Expected to read event queue");

        if events.is_empty() {
            log::debug!("No player events occurred");
            continue;
        }

        events.iter().for_each(|e| {
            hub_guard
                .set_term_feedback_led(e.term_id, &e.state)
                .expect("Expected to set term feedback led");

            log::debug!("New player event received: {:#?}. Pushing to the events", e);
        });

        let mut events_vec = events_lock
            .write()
            .expect("Expected to lock Rwlock to be aquired successfully");
        events_vec.extend(events);
    }
}
