<script>
    import PlayersView from "./quiz/PlayersView.svelte";
    import RoundView from "./quiz/RoundView.svelte";
    import {currentGameStateStore, GameState} from "../lib/stores.js";
    import QuestionView from "./quiz/QuestionView.svelte";
    import Row from "../components/generic/Row.svelte";
    import ModalPlaceholder from "../components/abstract/ModalPlaceholder.svelte";
    import MenuButton from "./quiz/subcomponents/MenuButton.svelte";
    import PickFirstQuestionChooser from "./quiz/PickFirstQuestionChooser.svelte";
    import Centered from "../components/generic/Centered.svelte";
    import Button from "../components/generic/Button.svelte";
    import EndQuestionScreen from "./quiz/subcomponents/EndQuestionScreen.svelte";

    $: gameState = $currentGameStateStore.gameState;
</script>

<div class="container">
    <ModalPlaceholder/>
    <MenuButton/>
    <div class="main-view">
        {#if gameState === GameState.SetupAndLoading}
            <Centered greedy={true}>
                <div>Loading...</div>
            </Centered>
        {:else if gameState === GameState.PickFirstQuestionChooser}
            <PickFirstQuestionChooser/>
        {:else if gameState === GameState.ChooseQuestion}
            <RoundView/>
        {:else if gameState === GameState.DisplayQuestion
                || gameState === GameState.WaitingForAnswerRequests
                || gameState === GameState.AnswerAttemptReceived
        }
            <QuestionView/>
        {:else if gameState === GameState.EndQuestion}
            <EndQuestionScreen/>
        {:else}
            <Row>
                <div>Unhandled state: {gameState}</div>
            </Row>
        {/if}
    </div>
    <PlayersView/>
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