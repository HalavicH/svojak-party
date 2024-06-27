import {currentScreen} from "../lib/stores.js";
import {notify} from "../lib/notifications.js";

export const Views = {
    MENU: "MENU",
    QUIZ: "QUIZ",
}

export async function goToMainMenu() {
    console.log("Going to main menu");
    navTo(Views.MENU);
}

export function navTo(view) {
    currentScreen.set(view);
    console.log(`Transitioned to: ${view}`)
    notify.info(`Transitioned to: ${view}`);
}

