import {writable} from "svelte/store";
import {Views} from "../screens/views.js";

export const currentView = writable(Views.MENU);

export function navTo(view) {
    currentView.set(view);
}