<script>
    import {closeModal, openModal} from 'svelte-modals'
    import Button from "../components/generic/Button.svelte";
    import ModalPlaceholder from "../components/abstract/ModalPlaceholder.svelte";
    import SettingsModal from "./menu/SettingsModal.svelte";
    import GamePackModal from "./menu/GamePackModal.svelte";
    import {notify} from "../lib/notifications.js";
    import {callBackend, TauriApiCommand} from "../lib/commands.js";
    import PackErrorModal from "./menu/PackErrorModal.svelte";
    import {getPackFilePath} from "../lib/misc.js"
    import {currentGameStateStore, GameState, isDebugMode} from "../lib/stores.js";
    import {navTo, Views} from "./views.js";

    function openSetup() {
        openModal(SettingsModal)
    }

    function initGamePack(filePath) {
        callBackend(TauriApiCommand.INIT_GAME_PACK, {path: filePath})
            .then(() => {
                openModal(GamePackModal)
            })
            .catch((error) => {
                console.error("Promise rejection:", error);
                // Log the rejection payload or handle the error in any other way
                closeModal();
                // openPackErrorModel(error);
                openModal(PackErrorModal, {message: error});
            });
    }

    async function openGamePack() {
        let filePath = await getPackFilePath();

        if (filePath === null || filePath.length === 0) {
            notify.info("Canceled pack selection");
            return;
            // closeModal();
        } else {
            notify.info(`Selected game package path: ${filePath}`);
        }
        initGamePack(filePath);
    }

    $: state = $currentGameStateStore.gameState;

    async function endGame() {
        await callBackend(TauriApiCommand.RESET_GAME);
    }
</script>


<div class="menu-container">
    <ModalPlaceholder/>
    <h1>Welcome to Svojak!</h1>
    <p>Powered by BronuCon commuity</p>
    <img
            src="/bc-logo.png"
            class="logo bronucon"
            alt="BronuCon logo"
    />

    <div>
        {#if state === GameState.SetupAndLoading}
            <Button text="Check setup (HW & Players)" onClick={openSetup}/>
            <p>then</p>
            <Button text="Start new game" onClick={openGamePack}/>
        {:else}
            <p>Game in progress. Do you have fun?</p>
            <Button text="Return to game" onClick={() => navTo(Views.QUIZ)}/>
            <p>or</p>
            <Button text="End game and return to main menu" onClick={endGame}/>
        {/if}
        {#if $isDebugMode}
            <p>For staff</p>
            <Button text="Debug menu" onClick={openSetup}/>
        {/if}
    </div>
</div>

<style>
    .logo {
        height: 6em;
        padding: 1.5em;
        will-change: filter;
        transition: 0.75s;
    }

    h1 {
        text-align: center;
    }

    .menu-container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: 100%;
    }
</style>