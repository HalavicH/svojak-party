export const DFL_PLAYER_ICON = "/bc-logo.png";

let isTauri = typeof window.__TAURI__ !== 'undefined';

export function isRunningInTauri() {
    return isTauri;
}

// Check if the application is running within Tauri
if (isRunningInTauri()) {
    console.log('Running within Tauri');
} else {
    console.log('Running standalone');
}

