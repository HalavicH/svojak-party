<script>
    import Menu from "./screens/Menu.svelte";
    import {notify} from "./lib/notifications"
    import {Views} from "./screens/views.js";
    import Quiz from "./screens/Quiz.svelte";
    import {currentView} from "./lib/stores"
    import {initEventListeners, setupEventListener} from "./lib/events.js";
    import Button from "./components/generic/Button.svelte";
    import DebugButton from "./screens/DebugButton.svelte";

    setupEventListener('message', (event) => {
        const message = event.payload;
        notify.info(message);
        console.info(`Message: ${message}`);
    });

    setupEventListener('error', (event) => {
        const message = event.payload;
        notify.failure(message);
        console.error(`Rust error: ${message}`);
    });

    initEventListeners();
</script>

<main class="container">
    {#if $currentView === Views.MENU}
        <Menu/>
    {:else if $currentView === Views.QUIZ}
        <Quiz/>
    {/if}
    <DebugButton/>
</main>

<style>
</style>
