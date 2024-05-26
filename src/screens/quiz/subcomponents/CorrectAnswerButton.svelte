<script>
    import {doWithSound, getCorrectAnswerSound} from "../../../lib/sound.js";
    export let onClick = () => {};
    export let active;

    function handleClick() {
        doWithSound(onClick, getCorrectAnswerSound());
    }
</script>

<button type="button" on:click={handleClick} class:inactive={!active}>Correct answer</button>

<style>
    button {
        border-radius: 8px;
        border: 1px solid transparent;
        padding: 0.6em 1.2em;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        color: #e8e8e8;
        background-color: rgb(49, 134, 0);
        transition: border-color 0.25s;
        box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
        cursor: pointer;
    }

    button:hover {
        border-color: #0e4700;
        filter: drop-shadow(0 0 0.2em #4fdb24);
    }

    button:active {
        border-color: #002b06;
        background-color: rgb(0, 88, 0);
    }

    .inactive {
        filter: grayscale(100%);
        pointer-events: none;
    }
</style>