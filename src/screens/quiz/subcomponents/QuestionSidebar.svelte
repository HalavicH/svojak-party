<script>
    import {currentQuestionStore, currentRoundStore} from "../../../lib/stores.js";
    import Button from "../../../components/generic/Button.svelte";
    import {TauriApiCommand, callBackend} from "../../../lib/commands.js";
    import AllowAnswerButton from "./AllowAnswerButton.svelte";
    import CorrectAnswerButton from "./CorrectAnswerButton.svelte";
    import WrongAnswerButton from "./WrongAnswerButton.svelte";
    import {goToMainMenu} from "../../views.js";
    import VSpacing from "../../../components/generic/VSpacing.svelte";
    import MenuButton from "./MenuButton.svelte";
    import QuestionMetaData from "./QuestionMetaData.svelte";

    $: currentRound = $currentRoundStore;

    async function allowAnswer() {
        await callBackend(TauriApiCommand.ALLOW_ANSWER);
    }

    async function correctAnswer() {
        await callBackend(TauriApiCommand.ANSWER_QUESTION, {answeredCorrectly: true});
    }

    async function wrongAnswer() {
        await callBackend(TauriApiCommand.ANSWER_QUESTION, {answeredCorrectly: false});
    }

    async function showAnswerAndEndQuestion() {
        await callBackend(TauriApiCommand.FINISH_QUESTION_PREMATURELY_AND_SHOW_ANSWER);
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
        <CorrectAnswerButton onClick={correctAnswer}/>
        <VSpacing size="0.5em"/>
        <WrongAnswerButton onClick={wrongAnswer}/>
        <VSpacing size="0.5em"/>
        <Button text="Show answer and end question" onClick={showAnswerAndEndQuestion}/>
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

        width: 20%;
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
        background-color: rgba(100, 100, 100, 0.5);
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