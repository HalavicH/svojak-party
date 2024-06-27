<script>
    import MenuButton from "./quiz/MenuButton.svelte";
    import {navTo, Views} from "./views.js";
    import {currentGameStateStore, currentScreen, GameState} from "../lib/stores.js";


    let onScreen;

    currentScreen.subscribe((scr) => {onScreen = scr})
    async function onKeyDown(e) {
        if (e.key !== 'Escape') {
            console.log(`Key is ${e.key}`)
            // play default key
            return;
        }

        if ($currentGameStateStore.gameState === GameState.SetupAndLoading) {
            console.log("Still on loading phase. Won't navigate");
            return;
        }

        console.log("Navigation is allowed")
        if (onScreen === Views.QUIZ) {
            navTo(Views.MENU);
        } else {
            navTo(Views.QUIZ);
        }
    }
</script>

<div>
    {#if onScreen === Views.QUIZ}
        <MenuButton/>
    {/if}
</div>
<svelte:window on:keydown|preventDefault={onKeyDown} />