<script>
    import {currentQuestionStore} from "../../../lib/stores.js";

    $: question = $currentQuestionStore;
</script>

<div class="viewport">
    <pre style="text-align: start">
        {JSON.stringify(question, null, 2)}
    </pre>
    <!--    <div class="slider"></div>-->
    <!--    <div class="navigation">-->
    <!--        <button id="prev-button" class="prev-button">&lt;</button>-->
    <!--        <button id="next-button" class="next-button">&gt;</button>-->
    <!--    </div>-->
</div>

<style>
    .viewport {
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
        flex: 1;

        margin: 0.2em;
        height: 100%;
        background-color: rgba(181, 229, 172, 0.16);
        border: solid 3px var(--items-block-border-color);
        border-radius: inherit;
    }

    .question-text {
        display: flex;
        justify-content: center;
        flex-wrap: wrap;
        flex: 1;
        text-align: center;

        padding: 1em;
        border-radius: 0.3em;

        box-sizing: border-box;

        font-weight: bold;
        font-size: 400%;
        line-height: 120%;
        font-family: sans-serif;

        background-color: var(--modal-table-background-color);
        color: var(--text-color);
    }

    .slider {
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .slider img,
    .slider video {
        object-fit: contain;
        max-width: 100%;
        max-height: 100%;
        width: auto;
        height: auto;
    }

    .navigation {
        position: absolute;
        top: 60%;
        transform: translateY(-50%);
        z-index: 1;
    }

    .prev-button,
    .next-button {
        font-size: 24px;
        padding: 10px 20px;
        background-color: rgba(255, 255, 255, 0.5);
    }

    .prev-button {
        margin-right: 10px;
    }

    .next-button {
        margin-left: 10px;
    }
</style>