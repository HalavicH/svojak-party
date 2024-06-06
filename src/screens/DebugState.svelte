<script>
    import {doWithSound, getClickSound} from "../lib/sound.js";
    import {closeModal, openModal} from "svelte-modals";
    import DebugModal from "./DebugModal.svelte";

    export let id = "";
    export let text = "Debug";
    export let style = "";
    export let onClick = openDebugMenu;

    function openDebugMenu() {
        closeModal();
        openModal(DebugModal)
        console.log("Pressed");
    }

    function handleClick() {
        doWithSound(onClick, getClickSound());
    }
</script>

<div class="debug-button">
    <button id={id} type="button" {style} on:click={handleClick}>{text}</button>
</div>


<style>
    .debug-button {
        position: absolute;
        bottom: 1em;
        left: 50%;
        transform: translateX(-50%);
    }

    button {
        border-radius: 8px;
        border: 1px solid #ffffff;
        padding: 0.1em 0.3em;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        color: #e8e8e8;
        background-color: rgba(48, 59, 24, 0.6);
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