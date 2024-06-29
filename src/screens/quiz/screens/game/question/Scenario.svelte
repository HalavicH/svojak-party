<script>
    import {QuestionMediaType} from "../../../../../lib/stores.js";
    import {convertFileSrc} from "@tauri-apps/api/tauri";
    import {onMount} from "svelte";

    export let scenario;
    export let isFullScreen = true;
    export let scaleContent = true;

    let content;
    switch (scenario.mediaType) {
        case QuestionMediaType.Image:
        case QuestionMediaType.Video:
        case QuestionMediaType.Voice:
            content = convertFileSrc(scenario.content);
            break;
        default:
            content = scenario.content;
    }
    console.log("Scenario content: ", content);

    onMount(() => {
        const image = document.querySelector('.image');
        if (image) {
            const naturalWidth = image.naturalWidth;
            const naturalHeight = image.naturalHeight;
            // const maxWidth = window.innerWidth;
            // const maxHeight = window.innerHeight;
            // We need to check width / height of the parent element
            const maxWidth = image.parentElement.clientWidth;
            const maxHeight = image.parentElement.clientHeight;

            let scale = 1;
            if (naturalWidth > 0 && naturalHeight > 0) {
                const widthScale = maxWidth / naturalWidth;
                const heightScale = maxHeight / naturalHeight;
                scale = Math.min(widthScale, heightScale, 2); // Ensure scale is at most 2x
            }

            image.style.setProperty('--scale', scale);
        }
    });
</script>

<div class:full-screen-slide={isFullScreen}>
    {#if scenario.mediaType === QuestionMediaType.Image}
        <img class="image" src={content} alt="Image" class:scale-content={scaleContent}/>
    {:else if scenario.mediaType === QuestionMediaType.Video}
        <video controls class="video">
            <source src={content} type="video/mp4"/>
            Your browser does not support the video tag.
        </video>
    {:else if scenario.mediaType === QuestionMediaType.Voice}
        <audio controls>
            <source src={content} type="audio/mpeg"/>
            Your browser does not support the audio tag.
        </audio>
    {:else if scenario.mediaType === QuestionMediaType.Marker}
        <div class="question-text">
            Marker: {content}
        </div>
    {:else}
        <div class="question-text">
            {content}
        </div>
    {/if}
</div>

<style>
    .full-screen-slide {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: 100%;
        width: 100%;
    }

    .image {
        max-width: 100%;
        max-height: 100%;
        width: auto;
        height: auto;
        transform-origin: center;
    }

    .scale-content {
        transform: scale(var(--scale, 1));
    }

    .question-text {
        font-size: 1.5em;
        text-align: center;
    }

    .video {
        max-width: 100%;
        max-height: 100%;
    }
</style>

