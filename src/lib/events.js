import {listen} from "@tauri-apps/api/event";
import {notify} from "./notifications.js";
import {callBackend, TauriApiCommand} from "./commands.js";
import {isRunningInTauri} from "./misc.js";
import {onDestroy, onMount} from "svelte";
import {
    currentGameStateStore, currentFinalResultsStore,
    currentHubConfigStore,
    currentPackInfoStore,
    currentPlayersStore,
    currentQuestionStore, currentRoundStatsStore,
    currentRoundStore
} from "./stores.js";

export const TauriEvents = {
    HubConFig: "HubConFig",
    Players: "Players",
    PackInfo: "PackInfo",
    Round: "Round",
    Question: "Question",
    GameState: "GameState",
    RoundStats: "RoundStats",
    FinalResults: "FinalResults",
}

export function initEventListeners() {
    listenAndStoreEvent(TauriEvents.HubConFig, currentHubConfigStore);
    listenAndStoreEvent(TauriEvents.Players, currentPlayersStore);
    listenAndStoreEvent(TauriEvents.PackInfo, currentPackInfoStore);
    listenAndStoreEvent(TauriEvents.Round, currentRoundStore);
    listenAndStoreEvent(TauriEvents.Question, currentQuestionStore);
    listenAndStoreEvent(TauriEvents.GameState, currentGameStateStore);
    listenAndStoreEvent(TauriEvents.RoundStats, currentRoundStatsStore);
    listenAndStoreEvent(TauriEvents.FinalResults, currentFinalResultsStore);

    console.log("################################################");
    console.log("##### ALL EVENT LISTENERS HAS BEEN LOADED ######");
    console.log("################################################");

    // After all event listeners are initialized we can switch on event emitters
    callBackend(TauriApiCommand.INIT_WINDOW_HANDLE).then(() => {
        console.log("Window handle stored successfully");
    })
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

    console.log(`Setting up event listener for: ${eventType}`)

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
