<script>
    import {closeModal} from 'svelte-modals';
    import {doWithSound, getWhooshSound} from "../../lib/sound.js";

    // provided by Modals
    export let isOpen;

    function handleCloseModal() {
        doWithSound(closeModal, getWhooshSound());
    }
</script>

{#if isOpen}
    <div role="dialog" class="modal">
        <div class="contents">
            <div class="close-panel">
                <span class="close" on:click={handleCloseModal}>&times;</span>
            </div>
            <slot/>
        </div>
    </div>
{/if}

<style>
    body {
        overflow: hidden; /* Disable scrolling on the main page */
    }

    .modal {
        z-index: 99999;
        position: fixed;
        top: 0;
        bottom: 0;
        right: 0;
        left: 0;
        display: flex;
        justify-content: center;
        align-items: center;
        overflow: auto; /* Allow scrolling if the modal content overflows */

        /* allow click-through to backdrop */
        pointer-events: none;
    }

    .contents {
        --modal-background-color: #383838;
        min-width: 30%;
        max-width: 90%;
        max-height: 90%; /* Ensure the modal content doesn't exceed the viewport height */
        border-radius: 6px;
        padding: 16px;
        background: var(--modal-background-color);
        display: flex;
        flex-direction: column;
        justify-content: center;
        pointer-events: auto;
        overflow: auto; /* Allow scrolling within the modal content if it exceeds the max-height */
    }

    .close-panel {
        position: relative;
        top: 0;
        display: flex;
        justify-content: end;
    }

    .close {
        color: #888;
        font-size: 28px;
        font-weight: bold;
        cursor: pointer;
    }

    .close:hover,
    .close:focus {
        color: #000;
        text-decoration: none;
        cursor: pointer;
    }

    h2 {
        text-align: center;
        font-size: 24px;
    }

    p {
        text-align: center;
        margin-top: 16px;
    }
</style>
