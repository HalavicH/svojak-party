<script>
    import {TauriApiCommand, callBackend} from "../../../../lib/commands.js";
    import Button from "../../../../components/generic/Button.svelte";

    async function nextRound() {
        await callBackend(TauriApiCommand.INIT_NEXT_ROUND);
    }
</script>

<div class="next">
    <Button text="Next round" onClick={nextRound} style="background-color: #69238c"/>
</div>

<style>
    .next {
        /*display: flex;*/
        /*flex-wrap: nowrap;*/
        /*justify-content: center;*/

        border-radius: 10px;
        background-color: var(--modal-table-background-color);

        /*height: 30vh;*/
        /*margin: 0.5em;*/
    }

</style>