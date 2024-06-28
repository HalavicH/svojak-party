const volumeFactor = 1;
let clickSound = new Audio('/sounds/click.mp3');
clickSound.volume = 0.3 * volumeFactor;

let collectSound = new Audio('/sounds/collect.mp3');
collectSound.volume = 0.5 * volumeFactor;

let countdownSound = new Audio('/sounds/countdown.mp3');
countdownSound.volume = 0.2 * volumeFactor;

let swipeSound = new Audio('/sounds/movement-swipe-whoosh.mp3');
swipeSound.volume = 0.1 * volumeFactor;

let newLevelSound = new Audio('/sounds/new-level.mp3');
newLevelSound.volume = 0.3 * volumeFactor;

let ohNoSound = new Audio('/sounds/oh-no.mp3');
ohNoSound.volume = 0.15 * volumeFactor;

let pickSound = new Audio('/sounds/pick.mp3');
pickSound.volume = 0.2 * volumeFactor;

let selectSound = new Audio('/sounds/select.mp3');
selectSound.volume = 0.2 * volumeFactor;

export function getClickSound() {
    return clickSound;
}

export function getAllowAnswerSound() {
    return countdownSound;
}

export function getCorrectAnswerSound() {
    return collectSound;
}

export function getWrongAnswerSound() {
    return ohNoSound;
}

export function getWhooshSound() {
    return swipeSound;
}

export function getSelectQuestionSound() {
    return selectSound;
}

export function getNewLevelSound() {
    return newLevelSound;
}


export function doWithSound(action, audioSound) {
    Promise.all([playClickSound(audioSound), action()])
        .then(() => {
            // Both sound playback and onClick function execution are complete
        })
        .catch(error => {
            console.error('Error:', error);
        });
}

function playClickSound(clickSound) {
    return new Promise((resolve, reject) => {
        clickSound.play();
        clickSound.onended = resolve;
        clickSound.onerror = reject;
    });
}

export async function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

export async function stopSoundWithFadeOut(soundToStop) {
    let startVolume = soundToStop.volume;
    let step = soundToStop.volume / 10;
    for (let i = 0; i < 10; i++) {
        soundToStop.volume -= step;
        await sleep(100);
    }
    soundToStop.pause();
    soundToStop.currentTime = 0;
    soundToStop.volume = startVolume;
}

