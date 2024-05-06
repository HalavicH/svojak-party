<script>
    import {doWithSound, getClickSound} from "../../lib/sound.js";

    export let id = "";
    export let text = "Secondary Button";
    export let style = "";
    export let onClick = () => {};

    function handleClick() {
        doWithSound(onClick, getClickSound());
    }
</script>

<button id={id} type="button" {style} on:click={handleClick}>{text}</button>

<style>
    button {
        border-radius: 8px;
        border: 1px solid white;
        padding: 0.3em 0.6em;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        color: #000000;
        background-color: rgba(134, 134, 134, 0.6);
        transition: border-color 0.25s;
        box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
        cursor: pointer;
    }

    button:hover {
        border-color: #396cd8;
        filter: drop-shadow(0 0 0.3em #24c8db);
    }

    button:active {
        border-color: #396cd8;
        background-color: var(--primary-button-active-color);
    }
</style>