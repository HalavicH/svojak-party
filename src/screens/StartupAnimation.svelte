<script>
    import {onMount} from "svelte";

    export let duration = 2; // duration of the unblurring animation in seconds

    let overlayVisible = true;

    onMount(() => {
        setTimeout(() => {
            overlayVisible = false;
        }, 1);
    });
</script>


<div class:overlay-visible={overlayVisible} class="overlay" style="--blur-duration: {duration}s">
    <div class="center-circle"></div>
</div>

<style>
    .overlay {
        pointer-events: none;

        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0);
        transition: background-color var(--blur-duration) ease-in-out;
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
    }

    .center-circle {
        width: 0;
        height: 0;
        background-color: transparent;
        border-radius: 50%;
        transition: width var(--blur-duration) ease-in-out, height var(--blur-duration) ease-in-out, background-color var(--blur-duration) ease-in-out;
    }

    .overlay-visible .center-circle {
        width: 150%;
        height: 150%;
        background-color: rgba(255, 255, 255, 0);
    }

    .overlay-visible {
        backdrop-filter: blur(0px);
        background-color: var(--background-color);
    }
</style>
