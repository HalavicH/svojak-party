<script>
    import PlayersView from "./quiz/screens/game/players/PlayersView.svelte";
    import RoundView from "./quiz/screens/game/round/RoundView.svelte";
    import {currentGameStateStore, GameState} from "../lib/stores.js";
    import QuestionView from "./quiz/screens/game/question/QuestionView.svelte";
    import Row from "../components/generic/Row.svelte";
    import ModalPlaceholder from "../components/abstract/ModalPlaceholder.svelte";
    import MenuButton from "./quiz/MenuButton.svelte";
    import PickFirstQuestionChooserView from "./quiz/screens/game/PickFirstQuestionChooserView.svelte";
    import Centered from "../components/generic/Centered.svelte";
    import EndQuestionView from "./quiz/screens/game/EndQuestionView.svelte";
    import RoundStatsScreen from "./quiz/screens/stats/RoundStatsView.svelte";
    import GameFinishedScreen from "./quiz/screens/eog/GameFinishedScreen.svelte";

    $: gameState = $currentGameStateStore.gameState;
</script>

<div class="container">
    <ModalPlaceholder/>
    <MenuButton/>
    {#if gameState === GameState.ShowRoundStats}
        <RoundStatsScreen/>
    {:else if gameState === GameState.EndTheGame}
        <GameFinishedScreen/>
    {:else }
        <div class="main-view">
            {#if gameState === GameState.SetupAndLoading}
                <Centered greedy={true}>
                    <div>Loading...</div>
                </Centered>
            {:else if gameState === GameState.PickFirstQuestionChooser}
                <PickFirstQuestionChooserView/>
            {:else if gameState === GameState.ChooseQuestion}
                <RoundView/>
            {:else if gameState === GameState.DisplayQuestion
            || gameState === GameState.WaitingForAnswerRequests
            || gameState === GameState.AnswerAttemptReceived
            }
                <QuestionView/>
            {:else if gameState === GameState.EndQuestion}
                <EndQuestionView/>
            {:else}
                <Row>
                    <div>Unhandled state: {gameState}</div>
                </Row>
            {/if}
        </div>
        <PlayersView/>
    {/if}
</div>

<style>
    .main-view {
        padding-top: 0.3em; /* Fixes weird shift to top */
        display: flex;
        flex-direction: column;
        /*flex: 1;*/
        height: 66vh;
    }

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