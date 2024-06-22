<script>
    import Menu from "./screens/Menu.svelte";
    import {notify} from "./lib/notifications"
    import {Views} from "./screens/views.js";
    import Quiz from "./screens/Quiz.svelte";
    import {currentView} from "./lib/stores"
    import {initEventListeners, setupEventListener} from "./lib/events.js";
    import DebugButton from "./screens/DebugButton.svelte";
    import DebugState from "./screens/DebugState.svelte";
    import {convertFileSrc} from "@tauri-apps/api/tauri";
    import {homeDir, resolve} from "@tauri-apps/api/path";
    import {tempdir} from "@tauri-apps/api/os";

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
        resolve("/Users/oleksandrkholiavko/RustroverProjects/svojak-hw/public/bc-logo.png")
            .then(path => {
                console.log("Resolved to: ", path)
                return convertFileSrc(path);
            })
            .then(url => {
                src = url
            })
    });

    tempdir().then(path => console.log("Tmp:", path));

    initEventListeners();
</script>

<main class="container">
<!--    <img src="file:///Users/oleksandrkholiavko/RustroverProjects/svojak-hw/public/bc-logo.png" alt="">-->
<!--    <img src={src} alt="">-->
    {#if $currentView === Views.MENU}
        <Menu/>
    {:else if $currentView === Views.QUIZ}
        <Quiz/>
    {/if}
    <DebugButton/>
    <DebugState/>
</main>

<style>
</style>
