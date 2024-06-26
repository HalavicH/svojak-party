<script>
    import {currentGameStateStore} from "../../../lib/stores.js";
</script>

<div class="debug-button">
    <span class="stuff">{$currentGameStateStore.gameState}</span>
</div>


<style>
    .debug-button {
        position: absolute;
        top: 1em;
        left: 70%;
        transform: translateX(-50%);
    }

    .stuff {
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

</style>