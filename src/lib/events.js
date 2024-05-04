import {setupEventListener} from "./misc.js";
import {listen} from "@tauri-apps/api/event";
import {gameConfig} from "./stores.js";
import {notify} from "./notifications.js";

export const TauriEvents = {
    GameConfig: 'GameConfig',
}

listenAndStoreEvent(TauriEvents.GameConfig, gameConfig);

function listenAndStoreEvent(eventType, storage) {
    listen(eventType, event => {
        logEvent(eventType, event);
        const payload = event.payload;
        storage.set(payload);
    }).then(() => {
        console.log(`Start listening for event of type: ${eventType}`);
    });
}

function logEvent(type, event) {
    console.log(`New event: '${type}'. content: ${event}`);
    notify.info(`Event: ${type}`);
}

// setupEventListener('web-users', (event) => {
//     const users = event.payload;
//     console.error(`Web users: ${users}`);
// });

// listen(TauriEvents.GameConfig, (event) => {
//     logEvent(TauriEvents.GameConfig, event);
//     const config = event.payload;
//     gameConfig.set(config);
// }).then();