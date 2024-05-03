import {listen} from "@tauri-apps/api/event";
import {onDestroy, onMount} from "svelte";

export function setupEventListener(eventName, callback) {
    let unlisten;

    onMount(async () => {
        unlisten = await listen(eventName, callback);
    });

    onDestroy(() => {
        unlisten();
    });
}