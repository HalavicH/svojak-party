<script>
    import {getAllowAnswerSound} from "../../../../../../lib/sound.js";
    import {currentGameStateStore, GameState} from "../../../../../../lib/stores.js";
    import {get} from "svelte/store";

    export let onClick = async () => {
    };
    export let active;

    async function sleep(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }

    async function playWithSoundUntilState() {
        const sound = getAllowAnswerSound();
        sound.play().then();
        let counter = 10;
        while (get(currentGameStateStore).gameState !== GameState.AnswerAttemptReceived && counter > 0) {
            await sleep(1000);
            counter--;
        }

        let startVolume = sound.volume;
        let step = sound.volume / 10;
        for (let i = 0; i < 10; i++) {
            sound.volume -= step;
            await sleep(100);
        }
        sound.pause();
        sound.currentTime = 0;
        sound.volume = startVolume;
    }

    function handleClick() {
        onClick().then();
        playWithSoundUntilState().then();
    }
</script>

<button type="button" on:click={handleClick} class:inactive={!active}>Allow answer</button>

<style>
    button {
        border-radius: 8px;
        border: 1px solid transparent;
        padding: 0.6em 1.2em;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        color: #e8e8e8;
        background-color: rgb(105, 0, 134);
        transition: border-color 0.25s;
        box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
        cursor: pointer;
    }

    button:hover {
        border-color: #2b0047;
        filter: drop-shadow(0 0 0.2em #6724db);
    }

    .inactive {
        filter: grayscale(100%);
        pointer-events: none;
    }
</style>
