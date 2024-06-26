<script>
    import Button from "../../../../components/generic/Button.svelte";
    import Centered from "../../../../components/generic/Centered.svelte";
    import {callBackend, TauriApiCommand} from "../../../../lib/commands.js";
    import {currentQuestionStore} from "../../../../lib/stores.js";
    import VSpacing from "../../../../components/generic/VSpacing.svelte";

    $: answer = $currentQuestionStore.answer;

    async function finishQuestion() {
        await callBackend(TauriApiCommand.FINISH_QUESTION);
    }
</script>

<Centered greedy={true}>
    <div>Question finished!</div>
    {#if answer}
        <div class="answer">Answer was: {$currentQuestionStore.answer}</div>
    {/if}
    <div>Are you ready do the next one?</div>
    <VSpacing size="1em"/>
    <Button text="Next question" onClick={finishQuestion}/>
</Centered>

<style>
    .answer {
        font-weight: bold;
        margin: 0.5em;
        border: 1px solid black;
        border-radius: 1em;
        padding: 0.5em 1em;
        background-color: rgba(133, 0, 200, 0.44);
    }
</style>