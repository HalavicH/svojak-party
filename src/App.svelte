<script>
    import Menu from "./screens/Menu.svelte";
    import {notify, setAllowNotifications} from "./lib/notifications"
    import {Views} from "./screens/views.js";
    import Quiz from "./screens/Quiz.svelte";
    import {currentScreen, isDebugMode} from "./lib/stores"
    import {initEventListeners, setupEventListener} from "./lib/events.js";
    import DebugButton from "./screens/quiz/debug/DebugButton.svelte";
    import DebugState from "./screens/quiz/debug/DebugState.svelte";
    import {convertFileSrc} from "@tauri-apps/api/tauri";
    import {appDataDir, homeDir, resolve} from "@tauri-apps/api/path";
    import {tempdir} from "@tauri-apps/api/os";
    import ThemeSwitcher from "./screens/ThemeSwitcher.svelte";
    import {callBackend, TauriApiCommand} from "./lib/commands.js";
    import {onMount} from "svelte";
    import Navigator from "./screens/Navigator.svelte";

    initEventListeners();

    callBackend(TauriApiCommand.IS_DEBUG_MODE).then(isDebug => {
        console.log(isDebug ? "Debug mode is ON" : "Debug mode is OFF");
        isDebugMode.set(isDebug);
        setAllowNotifications(isDebug);
    });

    let devMode;

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

    onMount(() => {
        callBackend(TauriApiCommand.REQUEST_CONTEXT_UPDATE).then();

        const unsubscribe = isDebugMode.subscribe((value) => {
            devMode = value;
            document.body.classList.toggle('no-select', !value);
            document.body.classList.toggle('no-caret', !value);
        });

        document.addEventListener('contextmenu', handleContextMenu);
        document.addEventListener('selectstart', handleSelectStart);

        return () => {
            unsubscribe();
            document.removeEventListener('contextmenu', handleContextMenu);
            document.removeEventListener('selectstart', handleSelectStart);
        };
    });

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

    let src;
    homeDir().then(home => {
        console.log("Home:", home);
        // resolve(home, "RustroverProjects/svojak-hw/public/bc-logo.png")
        // resolve("/Users/oleksandrkholiavko/RustroverProjects/svojak-hw/public/bc-logo.png")
        // resolve("/Users/oleksandrkholiavko/RustroverProjects/1765ff27-6ed1-4119-9edc-b0b97ec60d9f.jpg")
        // resolve("/Users/oleksandrkholiavko/1765ff27-6ed1-4119-9edc-b0b97ec60d9f.jpg")
        // resolve("/Users/oleksandrkholiavko/.svoyak/1765ff27-6ed1-4119-9edc-b0b97ec60d9f.jpg")
        resolve("/Users/oleksandrkholiavko/svo.yak/1765ff27-6ed1-4119-9edc-b0b97ec60d9f.jpg")
            // resolve("/Users/oleksandrkholiavko/.svoyak/siq_temp/Images/1765ff27-6ed1-4119-9edc-b0b97ec60d9f.jpg")
            .then(path => {
                console.log("Resolved to: ", path)
                return convertFileSrc(path);
            })
            .then(url => {
                console.log("URL: ", url)
                src = url
            })
    });

    tempdir().then(path => console.log("Tmp:", path));
    appDataDir().then(path => console.log("Cache:", path));
</script>

<main class="app-container">
    <Navigator/>
    <ThemeSwitcher/>
    {#if $currentScreen === Views.MENU}
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
