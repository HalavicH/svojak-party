<script>
import {goToMainMenu} from "../views.js";
</script>


<button type="button" on:click={goToMainMenu}>Menu</button>

<style>
    button {
        background-color: var(--primary-button-color);
        color: var(--primary-button-text-color);
        position: absolute;
        top: 0;
        left: 50%;
        transform: translateX(-50%);

        width: 5em;
        padding: 0.1em 0.3em;

        border: 1px solid transparent;
        border-radius: 0 0 1em 1em;
        transition: all 0.25s;
    }

    button:hover {
        transform: scale(150%) translateX(-30%) translateY(15%);
        border-color: var(--primary-button-border-color);
        filter: drop-shadow(0 0 0.2em #dbd824);
    }
</style>