import {writable} from "svelte/store";
import {Views} from "../screens/views.js";
import {DFL_PLAYER_ICON} from "./misc.js";

// Views
export const currentView = writable(Views.MENU);

export function navTo(view) {
    currentView.set(view);
}


// Players involved in game
export const PlayerState = {
    Idle: 'Idle',
    QuestionChooser: 'QuestionChooser',
    Target: 'Target',
    FirstResponse: 'FirstResponse',
    Inactive: 'Inactive',
    Dead: 'Dead',
    AnsweredCorrectly: 'AnsweredCorrectly',
    AnsweredWrong: 'AnsweredWrong',
}

export const gamePlayers = writable([
    {
        termId: 0,
        iconPath: DFL_PLAYER_ICON,
        name: "HalavicH",
        score: 500,
        state: PlayerState.Idle
    },
    {
        termId: 2,
        iconPath: DFL_PLAYER_ICON,
        name: "Button",
        score: -100,
        state: PlayerState.Dead
    },
    {
        termId: 3,
        iconPath: DFL_PLAYER_ICON,
        name: "Baadtrip",
        score: 200,
        state: PlayerState.QuestionChooser
    },
    {
        termId: 4,
        iconPath: DFL_PLAYER_ICON,
        name: "Valadis",
        score: 400,
        state: PlayerState.Inactive
    },
]);


export const gameContext = writable({
    hub_port: "",
    available_ports: [],
    radio_channel: -1,
    // These players are used only for setup
    players: [
        {
            termId: 1,
            icon: DFL_PLAYER_ICON,
            name: "HalavicH",
            score: 500,
            state: PlayerState.Idle
        },
        {
            termId: 2,
            icon: DFL_PLAYER_ICON,
            name: "Button",
            score: -100,
            state: PlayerState.Dead
        },
        {
            termId: 3,
            icon: DFL_PLAYER_ICON,
            name: "Baadtrip",
            score: 200,
            state: PlayerState.QuestionChooser
        },
        {
            termId: 4,
            icon: DFL_PLAYER_ICON,
            name: "Valadis",
            score: 400,
            state: PlayerState.Inactive
        },
    ],
});

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
    roundName: 'Злий Репер Зеник',
    roundType: 'roundType',
    roundTopics: [
        {
            topicName: 'ГМО',
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
            topicName: 'Металісти',
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
