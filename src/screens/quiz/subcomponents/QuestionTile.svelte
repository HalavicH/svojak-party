<script>
    export let topicName;
    export let question;

    function handleQuestionClick(topicName, index) {
        console.log(`Pressed on question ${topicName}:${index}`);
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
        width: 10%;
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
        color: #868686;
        transition: 0.5s;
    }
</style>