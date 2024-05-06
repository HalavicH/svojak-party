<script>
    import {doWithSound, getWrongAnswerSound} from "../../../lib/sound.js";
    export let onClick = () => {};

    function handleClick() {
        doWithSound(onClick, getWrongAnswerSound());
    }
</script>

<button type="button" on:click={handleClick}>Wrong answer</button>

<style>
    button {
        border-radius: 8px;
        border: 1px solid transparent;
        padding: 0.6em 1.2em;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        color: #e8e8e8;
        background-color: rgb(148, 0, 0);
        transition: border-color 0.25s;
        box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
        cursor: pointer;
    }

    button:hover {
        border-color: #470000;
        filter: drop-shadow(0 0 0.2em #db2424);
    }

    button:active {
        border-color: #2b0000;
        background-color: rgb(88, 0, 0);
    }
</style>