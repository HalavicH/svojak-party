<script>
    import {currentGameStateStore, currentQuestionStore, GameState} from "../../../../../../lib/stores.js";

    $: question = $currentQuestionStore;

    function calcAnswerStatus(gameState) {
        switch (gameState) {
            case GameState.AnswerAttemptReceived:
                return "in progress";
            case GameState.WaitingForAnswerRequests:
                return "allowed";
            default:
                return "forbidden";
        }
    }
</script>

<div class="metadata-box">
    <div class="metadata">Question: {question.number}</div>
    <div class="metadata">Category: {question.category}</div>
    <div class="metadata">Price: {question.price}</div>
    <div class="metadata">Answer {calcAnswerStatus($currentGameStateStore.gameState)}</div>
</div>

<style>
    .metadata-box {
        display: flex;
        flex-direction: column;
        /*flex: 1;*/
        /*justify-content: space-between;*/
        text-align: center;

        /*margin-top: 10px;*/
        padding: 0.5em;

        border: solid 1px var(--items-block-border-color);
        border-radius: inherit;
        background-color: var(--items-block-color);
    }

    .metadata {
        margin: 0.1em 0;

        padding: 0.2rem;
        border-radius: 10px;

        /*max-height: 1.8em;*/
        font-size: 1em;
        background-color: var(--accent-color);
        /*background-color: #4d4d4d;*/
    }
</style>
