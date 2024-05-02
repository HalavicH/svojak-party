<script>
    import { closeModal } from 'svelte-modals'

    // provided by Modals
    export let isOpen

    // export let title
    // export let message
</script>

{#if isOpen}
    <div role="dialog" class="modal">
        <div class="contents">
            <div class="close-panel">
                <span class="close" on:click={closeModal}>&times;</span>
            </div>
            <slot/>
<!--            <h2>{title}</h2>-->
<!--            <p>{message}</p>-->
<!--            <div class="actions">-->
<!--                <button on:click="{closeModal}">OK</button>-->
<!--            </div>-->
        </div>
    </div>
{/if}

<style>
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

        /* allow click-through to backdrop */
        pointer-events: none;
    }

    .contents {
        --modal-background-color: #383838;
        min-width: 30%;
        max-width: 90%;
        border-radius: 6px;
        padding: 16px;
        background: var(--modal-background-color);
        display: flex;
        flex-direction: column;
        justify-content: center;
        pointer-events: auto;
    }

    .close-panel {
        position: relative;
        top: 0;
        display: flex;
        justify-content: end;
    }

    .close {
        color: #888;
        float: right;
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

    .actions {
        margin-top: 32px;
        display: flex;
        justify-content: flex-end;
    }
</style>