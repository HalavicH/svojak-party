import {listen} from "@tauri-apps/api/event";
import {gameContext, gamePackInfo} from "./stores.js";
import {notify} from "./notifications.js";
import {invoke} from "@tauri-apps/api/tauri";
import {TauriApiCommand} from "./commands.js";

export const TauriEvents = {
    GameConfig: 'GameConfig',
    PackInfo: 'PackInfo',
}

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
    console.log(`|> New event: '${type}'. content: `, event, '<|');
    notify.info(`Event: ${type}`);
}

export function initEventListeners() {
    listenAndStoreEvent(TauriEvents.GameConfig, gameContext);
    listenAndStoreEvent(TauriEvents.PackInfo, gamePackInfo);

    console.log("################################################");
    console.log("##### ALL EVENT LISTENERS HAS BEEN LOADED ######");
    console.log("################################################");

    // After all event listeners are initialized we can switch on event emitters
    invoke(TauriApiCommand.INIT_WINDOW_HANDLE).then(() => {
        console.log("Window handle stored successfully");
    })
}
