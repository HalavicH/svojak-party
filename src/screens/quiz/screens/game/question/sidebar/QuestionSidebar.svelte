<script>
    import {currentGameStateStore, currentRoundStore, GameState} from "../../../../../../lib/stores.js";
    import {TauriApiCommand, callBackend} from "../../../../../../lib/commands.js";
    import Button from "../../../../../../components/generic/Button.svelte";
    import VSpacing from "../../../../../../components/generic/VSpacing.svelte";
    import AllowAnswerButton from "./AllowAnswerButton.svelte";
    import CorrectAnswerButton from "./CorrectAnswerButton.svelte";
    import WrongAnswerButton from "./WrongAnswerButton.svelte";
    import QuestionMetaData from "./QuestionMetaData.svelte";

    $: currentRound = $currentRoundStore;
    $: state = $currentGameStateStore.gameState;

    // $: clickAllowed = currentQuestionStore.questionState === "ANSWERING";
    // $: noPlayersToAnswerLeft = currentQuestionStore.playersToAnswer.length === 0;
    $: clickAllowed = state === GameState.AnswerAttemptReceived;
    $: noPlayersToAnswerLeft = state === GameState.WaitingForAnswerRequests;

    async function allowAnswer() {
        await callBackend(TauriApiCommand.ALLOW_ANSWER);
    }

    async function correctAnswer() {
        await callBackend(TauriApiCommand.ANSWER_QUESTION, {answeredCorrectly: true});
    }

    async function wrongAnswer() {
        await callBackend(TauriApiCommand.ANSWER_QUESTION, {answeredCorrectly: false});
    }

    async function stopAskingAndShowAnswer() {
        await callBackend(TauriApiCommand.STOP_ASKING_AND_SHOW_ANSWER);
    }
</script>

<div class="sidebar">
    <VSpacing size="0.5em"/>
    <div class="title-bar">
        <span class="round-label">{currentRound.roundName}</span>
    </div>
    <VSpacing size="0.5em"/>
    <QuestionMetaData/>
    <VSpacing size="0.5em"/>
    <div class="controls">
        <AllowAnswerButton onClick={allowAnswer}/>
        <VSpacing size="0.5em"/>
        <CorrectAnswerButton onClick={correctAnswer} active={clickAllowed}/>
        <VSpacing size="0.5em"/>
        <WrongAnswerButton onClick={wrongAnswer} active={clickAllowed}/>
        <VSpacing size="0.5em"/>
        <Button text="Show answer and end question" onClick={stopAskingAndShowAnswer} active={clickAllowed && noPlayersToAnswerLeft}/>
    </div>
</div>

<style>
    /* Question */
    .sidebar {
        /*position: absolute;*/
        /*top: 1em;*/
        /*right: 1em;*/
        /*margin: 0.5em;*/
        display: flex;
        flex-direction: column;
        justify-content: flex-end;

        width: 25vw;
        border-radius: 0.5em;
    }

    .title-bar {
        padding: 0.5em;

        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;

        border: solid 1px #dcc680;
        border-radius: 15px;
        color: black;
        background-color: #fdf2d046;
    }

    .round-label {
        flex: 1;
        text-align: center;

        font-size: 1em;
        font-weight: bold;
        color: var(--text-color);
    }

    .controls {
        display: flex;
        flex-direction: column;
        padding: 0.5em;
        background-color: var(--items-block-color);
        border: 1px solid var(--items-block-border-color);
        border-radius: inherit;
    }

    #exit-dialog-text {
        text-align: center;
    }

    #exit-dialog-answer {
        display: flex;
        justify-content: space-around;
    }

    #exit-dialog-yes {
        background-color: var(--primary-button-color);
    }

    #exit-dialog-no {
        background-color: var(--secondary-button-color);
    }
</style>