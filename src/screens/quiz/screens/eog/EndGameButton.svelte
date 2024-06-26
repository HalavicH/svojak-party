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
    <Button text="Go to main menu" onClick={processGameEnd} style="background-color: #15828e"/>
</div>

<style>
    .next {
        /*display: flex;*/
        /*flex-wrap: nowrap;*/
        /*justify-content: center;*/

        border-radius: 10px;
        background-color: var(--modal-table-background-color);

        margin: 0.5em;
    }

</style>