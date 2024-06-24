<script>
    import {TauriApiCommand, callBackend} from "../../../../lib/commands.js";
    import Button from "../../../../components/generic/Button.svelte";
    import {goToMainMenu} from "../../../views.js";

    async function processGameEnd() {
        await callBackend(TauriApiCommand.FINISH_GAME);
        await goToMainMenu();
    }
</script>

<div class="next">
    <Button text="Go to main menu" onClick={processGameEnd} style="background-color: #8c4823"/>
</div>

<style>
    .next {
        /*display: flex;*/
        /*flex-wrap: nowrap;*/
        /*justify-content: center;*/

        border-radius: 10px;
        background-color: var(--modal-table-background-color);

        height: 30vh;
        margin: 0.5em;
    }

</style>