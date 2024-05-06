import {listen} from "@tauri-apps/api/event";
import {currentRound, gameContext, gamePackInfo} from "./stores.js";
import {notify} from "./notifications.js";
import {callBackend, TauriApiCommand} from "./commands.js";
import {isRunningInTauri} from "./misc.js";
import {onDestroy, onMount} from "svelte";

export const TauriEvents = {
    GameConfig: 'GameConfig',
    PackInfo: 'PackInfo',
    Round: 'Round',
}

export function setupEventListener(eventType, callback) {
    if (!isRunningInTauri()) {
        console.warn(`No Tauri context!\nSkipped '${eventType}' event listener setup. Callback:\n\t${callback}\nwon't be executed`);
        return;
    }

    let unlisten;

    onMount(async () => {
        unlisten = await listen(eventType, callback);
    });

    onDestroy(() => {
        unlisten();
    });
}

function listenAndStoreEvent(eventType, storage) {
    if (!isRunningInTauri()) {
        console.warn(`No Tauri context!\nSkipped '${eventType}' event listener setup. Storage won't be updated`);
        return;
    }

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
    listenAndStoreEvent(TauriEvents.Round, currentRound);

    console.log("################################################");
    console.log("##### ALL EVENT LISTENERS HAS BEEN LOADED ######");
    console.log("################################################");

    // After all event listeners are initialized we can switch on event emitters
    callBackend(TauriApiCommand.INIT_WINDOW_HANDLE).then(() => {
        console.log("Window handle stored successfully");
    })
}
