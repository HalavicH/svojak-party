import {navTo} from "../lib/stores.js";

export const Views = {
    MENU: "MENU",
    QUIZ: "QUIZ",
}

export async function goToMainMenu() {
    console.log("Going to main menu");
    navTo(Views.MENU);
}
