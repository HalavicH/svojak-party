<script>
    import {doWithSound, getClickSound} from "../../lib/sound.js";

    export let id = "";
    export let text = "Button";
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
        border: 1px solid transparent;
        padding: 0.6em 1.2em;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        color: var(--text-color);
        background-color: var(--primary-button-color);
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