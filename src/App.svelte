<script>
    import Menu from "./screens/Menu.svelte";
    import {notify} from "./lib/notifications"
    import {setupEventListener} from "./lib/misc"
    import {Views} from "./screens/views.js";
    import Quiz from "./screens/Quiz.svelte";
    import {currentView} from "./lib/stores"
    import {invoke} from "@tauri-apps/api/tauri";
    import {TauriApiCommand} from "./lib/commands.js";

    invoke(TauriApiCommand.INIT_WINDOW_HANDLE).then(() => {
        console.log("Window handle stored successfully");
    })

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
</script>

<main class="container">
    {#if $currentView === Views.MENU}
        <Menu/>
    {:else if $currentView === Views.QUIZ}
        <Quiz/>
    {/if}
</main>

<style>
</style>
