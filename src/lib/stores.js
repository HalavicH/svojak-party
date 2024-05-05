import {writable} from "svelte/store";
import {Views} from "../screens/views.js";

// Views
export const currentView = writable(Views.MENU);

export function navTo(view) {
    currentView.set(view);
}

export const gameContext = writable({
    hub_port: "",
    available_ports: [],
    radio_channel: -1,
    players: [
        {
            termId: 0,
            icon: "",
            name: "",
            isUsed: false,
        }
    ],
});

export const gamePlayers = writable([]);
export const gamePackInfo = writable({
    packName: 'Zlyj reper Zenyk',
    packAuthor: 'Zlyj reper Zenyk',
    packRounds: 3,
    packTopics: 3,
    packQuestions: 69,
    packTopicList: [
        'Beer',
        'Pone',
        'Music',
        'Movies',
        'Fallout',
        'Beer',
        'Pone',
        'Music',
        'Movies',
        'Fallout',
        'Beer',
        'Pone',
        'Music',
        'Movies',
        'Fallout',
    ],
});

export const currentRound = writable({
    roundName: 'roundName',
    roundType: 'roundType',
    roundTopics: [
        {
            topicName: 'First topic',
            questions: [
                {
                    index: 0,
                    price: 100,
                    used: false,
                },
                {
                    index: 1,
                    price: 200,
                    used: false,
                },
                {
                    index: 2,
                    price: 300,
                    used: false,
                }
            ],
        },
        {
            topicName: 'Second topic',
            questions: [
                {
                    index: 0,
                    price: 100,
                    used: false,
                },
                {
                    index: 1,
                    price: 200,
                    used: false,
                },
                {
                    index: 2,
                    price: 300,
                    used: false,
                },
                {
                    index: 3,
                    price: 400,
                    used: false,
                }
            ],
        }
    ],
});


console.log("################################################");
console.log("########## ALL STORES HAS BEEN LOADED ##########");
console.log("################################################");
