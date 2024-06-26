<script>
    import Menu from "./screens/Menu.svelte";
    import {notify} from "./lib/notifications"
    import {Views} from "./screens/views.js";
    import Quiz from "./screens/Quiz.svelte";
    import {currentView} from "./lib/stores"
    import {initEventListeners, setupEventListener} from "./lib/events.js";
    import DebugButton from "./screens/quiz/debug/DebugButton.svelte";
    import DebugState from "./screens/quiz/debug/DebugState.svelte";
    import {convertFileSrc} from "@tauri-apps/api/tauri";
    import {appCacheDir, appDataDir, homeDir, resolve} from "@tauri-apps/api/path";
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
