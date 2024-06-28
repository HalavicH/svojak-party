<script>
    import Menu from "./screens/Menu.svelte";
    import {notify, setAllowNotifications} from "./lib/notifications"
    import {Views} from "./screens/views.js";
    import Quiz from "./screens/Quiz.svelte";
    import {currentScreen, isDebugMode} from "./lib/stores"
    import {initEventListeners, setupEventListener} from "./lib/events.js";
    import DebugButton from "./screens/quiz/debug/DebugButton.svelte";
    import DebugState from "./screens/quiz/debug/DebugState.svelte";
    import ThemeSwitcher from "./screens/ThemeSwitcher.svelte";
    import {callBackend, TauriApiCommand} from "./lib/commands.js";
    import {onMount} from "svelte";
    import Navigator from "./screens/Navigator.svelte";
    import BlurAnimation from "./screens/StartupAnimation.svelte";

    let devMode = true;

    const handleContextMenu = (event) => {
        if (!devMode) {
            event.preventDefault();
        }
    };

    const handleSelectStart = (event) => {
        if (!devMode) {
            event.preventDefault();
        }
    };

    callBackend(TauriApiCommand.IS_DEBUG_MODE).then(isDebug => {
        isDebug = true;
        console.log(isDebug ? "Debug mode is ON" : "Debug mode is OFF");
        isDebugMode.set(isDebug);
        setAllowNotifications(isDebug);
    });

    onMount(() => {
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

        initEventListeners().then(() => {
            callBackend(TauriApiCommand.REQUEST_CONTEXT_UPDATE);
        });

        const unsubscribe = isDebugMode.subscribe((value) => {
            devMode = true;
            document.body.classList.toggle('no-select', !value);
            document.body.classList.toggle('no-caret', !value)
        });

        document.addEventListener('contextmenu', handleContextMenu);
        document.addEventListener('selectstart', handleSelectStart);

        return () => {
            unsubscribe();
            document.removeEventListener('contextmenu', handleContextMenu);
            document.removeEventListener('selectstart', handleSelectStart);
        };
    });
</script>

<BlurAnimation/>
<main class="app-container">
    <Navigator/>
    {#if $currentScreen === Views.MENU}
        <ThemeSwitcher/>
        <Menu/>
    {:else if $currentScreen === Views.QUIZ}
        <Quiz/>
    {/if}
    {#if $isDebugMode}
        <DebugButton/>
        <DebugState/>
    {/if}
</main>

<style>
    .app-container {
        margin: 0;
        /*padding-top: 10vh;*/
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
        height: 100%;
    }

    :global(body.no-select) {
        user-select: none;
    }

    :global(body.no-caret) {
        cursor: default;
    }
</style>
