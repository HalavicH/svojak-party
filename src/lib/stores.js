import {writable} from "svelte/store";
import {Views} from "../screens/views.js";
import {DFL_PLAYER_ICON} from "./misc.js";
import {notify} from "./notifications.js";
import {HubStatusOptions} from "./commands.js";

// Views
export const currentView = writable(Views.MENU);

export function navTo(view) {
    currentView.set(view);
    notify.info(`Transitioned to: ${view}`);
}

// Players involved in game
export const PlayerState = {
    Idle: 'Idle',
    QuestionChooser: 'QuestionChooser',
    Target: 'Target',
    Answering: 'Answering',
    Inactive: 'Inactive',
    Dead: 'Dead',
    AnsweredCorrectly: 'AnsweredCorrectly',
    AnsweredWrong: 'AnsweredWrong',
}

// Game State
export const GameState = {
    SetupAndLoading: 'SetupAndLoading',
    PickFirstQuestionChooser: 'PickFirstQuestionChooser',
    ChooseQuestion: 'ChooseQuestion',
    DisplayQuestion: 'DisplayQuestion',
    WaitingForAnswerRequests: 'WaitingForAnswerRequests',
    AnswerAttemptReceived: 'AnswerAttemptReceived',
    EndQuestion: 'EndQuestion',
    CheckEndOfRound: 'CheckEndOfRound',
    CalcStatsAndStartNextRound: 'CalcStatsAndStartNextRound',
}


// Mock data
const mockHubConfig = {
    hubPort: "",
    availablePorts: [
        "Test 1",
        "Test 2",
        "Test 3",
        "Test 4",
    ],
    hubStatus: HubStatusOptions.Detected,
    radioChannel: -1,
    roundDurationMin: 10,
};


const mockPlayers = [
    {
        id: 1,
        iconPath: DFL_PLAYER_ICON,
        name: "HalavicH",
        isUsed: true,
        score: 500,
        state: PlayerState.Idle
    },
    {
        termId: 2,
        iconPath: DFL_PLAYER_ICON,
        name: "Button",
        isUsed: true,
        score: -100,
        state: PlayerState.Dead
    },
    {
        termId: 3,
        iconPath: DFL_PLAYER_ICON,
        name: "Baadtrip",
        isUsed: true,
        score: 200,
        state: PlayerState.QuestionChooser
    },
    {
        termId: 4,
        iconPath: DFL_PLAYER_ICON,
        name: "Valadis",
        isUsed: true,
        score: 400,
        state: PlayerState.Inactive
    },
];

const mockPackInfo = {
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
};

const mockRound = {
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
};

const QuestionType = {
    Normal: 'Normal',
    PigInPoke: 'PigInPoke',
    Auction: 'Auction',
}

const QuestionMediaType = {
    Text: 'Text',
    Voice: 'Voice',
    Video: 'Video',
    Marker: 'Marker',
    Image: 'Image',
}

const mockQuestion = {
    number: 1,
    category: "Beer",
    price: 100,
    questionType: QuestionType.Normal,
    scenario: [
        {
            mediaType: QuestionMediaType.Text,
            content: "Front ui should send requests via `invoke` and listen response through `listen`",
        },
        {
            mediaType: QuestionMediaType.Image,
            content: "/tauri.svg",
        }
    ],
    answer: String,
}

// Game stores
export const currentHubConfigStore = writable(mockHubConfig);
export const currentPlayersStore = writable(mockPlayers);
export const currentPackInfoStore = writable(mockPackInfo);
export const currentRoundStore = writable(mockRound);
export const currentQuestionStore = writable(mockQuestion);
export const currentGameStateStore = writable({gameState: GameState.SetupAndLoading});

console.log("################################################");
console.log("########## ALL STORES HAS BEEN LOADED ##########");
console.log("################################################");
