<script>
    import {TauriApiCommand, callBackend} from "../../../../../lib/commands.js";
    import {doWithSound, getSelectQuestionSound} from "../../../../../lib/sound.js";

    export let topicName;
    export let question;

    function handleQuestionClick(topicName, index) {
        doWithSound(() => {
            console.log(`Pressed on question ${topicName}:${index}`);
            callBackend(TauriApiCommand.SELECT_QUESTION, {
                topic: topicName,
                price: question.price
            }).then();
        }, getSelectQuestionSound());
    }
</script>

{#if !question.used}
    <td class="round-td-price" on:click={() => handleQuestionClick(topicName, question.index)}>{question.price}</td>
{:else}
    <td class="round-td-price used">{question.price}</td>
{/if}

<style>
    td {
        border: 1px solid gray;
        padding: 0.3em;
        background-color: var(--items-block-color);
        border-radius: 15px;
        min-width: 8%;
        width: auto;
        max-width: 15%;
        text-align: center;
        font-weight: bold;
        font-size: xx-large;
        cursor: pointer;
        transition: 0.5s;
    }

    td:hover {
        color: #ff8a3c;
        filter: drop-shadow(0 0 0.2em #db9224);
    }

    td.used {
        filter: none;
        color: var(--used-question-color);
        transition: 0.5s;
        background-color: var(--used-question-background-color);
        cursor: default;
    }
</style>