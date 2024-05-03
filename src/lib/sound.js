let clickSound = new Audio('../../public/sounds/click.mp3');
clickSound.volume = 0.3;

let swipeSound = new Audio('../../public/sounds/movement-swipe-whoosh.mp3');
swipeSound.volume = 0.1; // Set volume

let selectionSound = new Audio('../../public/sounds/selection-sound.mp3');
selectionSound.volume = 0.1; // Set volume

export function getClickSound() {
    return clickSound;
}

export function getWhooshSound() {
    return swipeSound;
}

export function getSelectionSound() {
    return selectionSound;
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

