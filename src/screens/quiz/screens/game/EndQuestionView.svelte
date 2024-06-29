<script>
    import Button from "../../../../components/generic/Button.svelte";
    import {callBackend, TauriApiCommand} from "../../../../lib/commands.js";
    import {currentQuestionStore} from "../../../../lib/stores.js";
    import VSpacing from "../../../../components/generic/VSpacing.svelte";
    import {onMount} from "svelte";
    import Scenario from "./question/Scenario.svelte";

    // answer: [
    //     {
    //         mediaType: QuestionMediaType.Text,
    //         content: "Front ui should send requests via `invoke` and listen response through `listen`",
    //     },
    //     {
    //         mediaType: QuestionMediaType.Image,
    //         content: "/bc-logo.png",
    //     }
    // ],
    $: answer = $currentQuestionStore.answer;

    async function finishQuestion() {
        await callBackend(TauriApiCommand.FINISH_QUESTION);
    }

    onMount(() => {
        const image = document.querySelector('.image');
        if (image) {
            const naturalWidth = image.naturalWidth;
            const naturalHeight = image.naturalHeight;
            const maxWidth = window.innerWidth;
            const maxHeight = window.innerHeight;

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

<!--<Centered greedy={true}>-->
<div class="tall-column">
    <h1>Question finished! Answer was:</h1>
    <VSpacing size="1em"/>
    <div class="answer">
        {#each answer as {mediaType, content}}
            <!--{#if mediaType === QuestionMediaType.Text}-->
            <!--    <div class="question-text">{content}</div>-->
            <!--{:else if mediaType === QuestionMediaType.Image}-->
            <!--    <img class="image" src={content} alt="Answer image"/>-->
            <!--{/if}-->
            <Scenario scenario={{mediaType, content}} isFullScreen={false}/>

            <!--{#if scenario.mediaType === QuestionMediaType.Image}-->
            <!--    <img class="image" src={content} alt="Image"/>-->
            <!--{:else if scenario.mediaType === QuestionMediaType.Video}-->
            <!--    <video controls class="video">-->
            <!--        <source src={content} type="video/mp4" />-->
            <!--        Your browser does not support the video tag.-->
            <!--    </video>-->
            <!--{:else if scenario.mediaType === QuestionMediaType.Voice}-->
            <!--    <audio controls>-->
            <!--        <source src={content} type="audio/mpeg"/>-->
            <!--        Your browser does not support the audio tag.-->
            <!--    </audio>-->
            <!--{:else if scenario.mediaType === QuestionMediaType.Marker}-->
            <!--    <div class="question-text">-->
            <!--        Marker: {content}-->
            <!--    </div>-->
            <!--{:else}-->
            <!--    <div class="question-text">-->
            <!--        {content}-->
            <!--    </div>-->
            <!--{/if}-->
        {/each}
    </div>
    <VSpacing size="1em"/>

    <!--{#if answer}-->
    <!--    <div class="answer">Answer was: {$currentQuestionStore.answer}</div>-->
    <!--{/if}-->
    <!--    <div>Are you ready do the next one?</div>-->
    <!--    <VSpacing size="1em"/>-->
    <Button text="Next question" onClick={finishQuestion}/>

</div>
<!--</Centered>-->

<style>
    .tall-column {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
        flex: 1;
    }

    .answer {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
        font-weight: bold;
        margin: 0.5em;
        border: 1px solid black;
        border-radius: 1em;
        padding: 0.5em 1em;
        background-color: rgba(133, 0, 200, 0.44);
    }

    .image {
        max-width: 100%;
        max-height: 100%;
        width: auto;
        height: auto;
        transform: scale(var(--scale, 1));
        transform-origin: center;
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