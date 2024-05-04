import {writable} from "svelte/store";
import {Views} from "../screens/views.js";
import {setupEventListener} from "./misc.js";
import {listen} from "@tauri-apps/api/event";

// Views
export const currentView = writable(Views.MENU);

export function navTo(view) {
    currentView.set(view);
}

export const gameConfig = writable({
    hub_port: "",
    available_ports: [],
    radio_channel: -1,
    players: [],
});

export const gamePlayers = writable([
    {
        termId: 0,
        icon: "",
        name: "",
        isUsed: false,
    }
]);

