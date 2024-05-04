import {writable} from "svelte/store";
import {Views} from "../screens/views.js";
import {setupEventListener} from "./misc.js";
import {listen} from "@tauri-apps/api/event";

// Views
export const currentView = writable(Views.MENU);
export function navTo(view) {
    currentView.set(view);
}

// Config
export const gameConfig = writable({});
