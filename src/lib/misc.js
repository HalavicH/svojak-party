import {listen} from "@tauri-apps/api/event";
import {onDestroy, onMount} from "svelte";

export const DFL_PLAYER_ICON = "/bc-logo.png";

export function setupEventListener(eventName, callback) {
    let unlisten;

    onMount(async () => {
        unlisten = await listen(eventName, callback);
    });

    onDestroy(() => {
        unlisten();
    });
}