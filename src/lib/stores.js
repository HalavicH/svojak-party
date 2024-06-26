import {writable} from "svelte/store";
import {Views} from "../screens/views.js";
import {DFL_PLAYER_ICON} from "./misc.js";
import {notify} from "./notifications.js";
import {HubStatusOptions} from "./commands.js";

// Views
export const currentScreen = writable(Views.MENU);


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
    ShowRoundStats: 'ShowRoundStats',
    StartNextRound: 'StartNextRound',
    EndTheGame: 'EndTheGame',
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
    // {
    //     id: 1,
    //     iconPath: DFL_PLAYER_ICON,
    //     name: "HalavicH",
    //     isUsed: true,
    //     score: 500,
    //     state: PlayerState.Idle
    // },
    // {
    //     termId: 2,
    //     iconPath: DFL_PLAYER_ICON,
    //     name: "Button",
    //     isUsed: true,
    //     score: -100,
    //     state: PlayerState.Dead
    // },
    // {
    //     termId: 3,
    //     iconPath: DFL_PLAYER_ICON,
    //     name: "Baadtrip",
    //     isUsed: true,
    //     score: 200,
    //     state: PlayerState.QuestionChooser
    // },
    // {
    //     termId: 4,
    //     iconPath: DFL_PLAYER_ICON,
    //     name: "Valadis",
    //     isUsed: true,
    //     score: 400,
    //     state: PlayerState.Inactive
    // },
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

export const QuestionType = {
    Normal: 'Normal',
    PigInPoke: 'PigInPoke',
    Auction: 'Auction',
}

export const QuestionMediaType = {
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
            content: "/bc-logo.png",
        }
    ],
    answer: [
        {
            mediaType: QuestionMediaType.Text,
            content: "Front ui should send requests via `invoke` and listen response through `listen`",
        },
        {
            mediaType: QuestionMediaType.Image,
            content: "/bc-logo.png",
        }
    ],
}

let roundStatsMock = {
    roundName: "Злий Репер Зеник",
    questionsPlayed: 4,
    normalQuestionsPlayed: 3,
    pigInPokeQuestionPlayed: 1,
    totalCorrectAnswers: 4,
    totalWrongAnswers: 3,
    totalTries: 7,
    roundTimeSec: 666,
    players: [
        {
            id: 1,
            name: "HalavicH",
            score: 500,
            playerIconPath: DFL_PLAYER_ICON,
            totalAnswers: 3,
            answeredCorrectly: 2,
            answeredWrong: 1,
        },
        {
            id: 2,
            name: "Button",
            score: -100,
            playerIconPath: DFL_PLAYER_ICON,
            totalAnswers: 2,
            answeredCorrectly: 1,
            answeredWrong: 1,
        },
        {
            id: 3,
            name: "Baadtrip",
            score: 200,
            playerIconPath: DFL_PLAYER_ICON,
            totalAnswers: 1,
            answeredCorrectly: 1,
            answeredWrong: 0,
        },
        {
            id: 4,
            name: "Valadis",
            score: 400,
            playerIconPath: DFL_PLAYER_ICON,
            totalAnswers: 1,
            answeredCorrectly: 0,
            answeredWrong: 1,
        }
    ]
};

// #[derive(Clone, Debug)]
// pub enum EndGameReason {
//     OnePlayerLeft,
//     NoPlayersLeft,
//     AllRoundsPlayed,
// }
export const EndGameReason = {
    OnePlayerLeft: 'OnePlayerLeft',
    NoPlayersLeft: 'NoPlayersLeft',
    AllRoundsPlayed: 'AllRoundsPlayed',
}

let endGameStatsMock = {
    endGameReason: EndGameReason.AllRoundsPlayed,
    first: {
        name: "HalavicH",
        score: 500,
        state: PlayerState.Idle,
        icon: DFL_PLAYER_ICON,
    },
    second: {
        name: "Valadis",
        score: 400,
        state: PlayerState.Idle,
        icon: DFL_PLAYER_ICON,
    },
    third: {
        name: "Baadtrip",
        score: 200,
        state: PlayerState.Idle,
        icon: DFL_PLAYER_ICON,
    },
    theRest: [
        {
            name: "Button",
            score: -100,
            state: PlayerState.Dead,
            icon: DFL_PLAYER_ICON,
        }
    ]
}

// Game stores
export const currentHubConfigStore = writable(mockHubConfig);
export const currentPlayersStore = writable(mockPlayers);
export const currentPackInfoStore = writable(mockPackInfo);
export const currentRoundStore = writable(mockRound);
export const currentQuestionStore = writable(mockQuestion);
export const currentGameStateStore = writable({gameState: GameState.SetupAndLoading});
export const currentRoundStatsStore = writable(roundStatsMock);
export const currentFinalResultsStore = writable(endGameStatsMock);
export const isDebugMode = writable(false);

console.log("################################################");
console.log("########## ALL STORES HAS BEEN LOADED ##########");
console.log("################################################");
