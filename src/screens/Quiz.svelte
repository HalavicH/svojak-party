<script>
    import PlayersView from "./quiz/PlayersView.svelte";
    import RoundView from "./quiz/RoundView.svelte";
    import {currentGameStateStore, GameState} from "../lib/stores.js";
    import QuestionView from "./quiz/QuestionView.svelte";
    import Row from "../components/generic/Row.svelte";

    $: gameState = $currentGameStateStore.gameState;
</script>

<div class="container">
    {#if gameState === GameState.ChooseQuestion}
        <RoundView/>
    {:else if gameState === GameState.DisplayQuestion}
        <QuestionView/>
    {:else}
        <Row>
            <div>Unhandled state: {gameState}</div>
        </Row>
    {/if}
    <PlayersView/>
</div>

<style>
    .container {
        height: 98vh;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        border: solid 1px gray;
        overflow: auto;

        background-color: var(--modal-table-background-color);
    }
</style>