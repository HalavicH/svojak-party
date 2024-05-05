export const DFL_PLAYER_ICON = "/bc-logo.png";

export function isRunningInTauri() {
    return window.tauri !== undefined;
}

// Check if the application is running within Tauri
if (isRunningInTauri()) {
    console.log('Running within Tauri');
} else {
    console.log('Running standalone');
}

