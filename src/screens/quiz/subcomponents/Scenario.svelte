<script>
    import {QuestionMediaType} from "../../../lib/stores.js";
    import {convertFileSrc} from "@tauri-apps/api/tauri";

    export let scenario;
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
</script>

<div class="slide">
    {#if scenario.mediaType === QuestionMediaType.Image}
        <img src={scenario.content} alt="Image"/>
    {:else if scenario.mediaType === QuestionMediaType.Video}
        <video controls>
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