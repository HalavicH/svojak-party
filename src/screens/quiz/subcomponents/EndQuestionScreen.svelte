<script>
    import Button from "../../../components/generic/Button.svelte";
    import Centered from "../../../components/generic/Centered.svelte";
    import {callBackend, TauriApiCommand} from "../../../lib/commands.js";


    async function finishQuestion() {
        await callBackend(TauriApiCommand.FINISH_QUESTION);
    }
</script>

<Centered greedy={true}>
    <div>Question finished! Are you ready do the next one?</div>
    <Button text="Next question" onClick={finishQuestion}/>
</Centered>
