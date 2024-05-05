<script>
    import {currentRound, navTo} from "../../lib/stores.js";
    import Button from "../../components/generic/Button.svelte";
    import {Views} from "../views.js";
    import QuestionTile from "./subcomponents/QuestionTile.svelte";

    console.log(">>>> ", $currentRound);
    function goToMainMenu() {
        console.log("Going to main menu");
        navTo(Views.MENU);
    }

</script>

<div class="round-screen">
    <div class="title-bar">
        <p class="round-label">Round: {$currentRound.roundName}</p>
        <Button text="Menu" onClick={() => {goToMainMenu}}/>
    </div>
    <div class="round-table-box">
        <table class="round-table">
            <tbody>
            {#each $currentRound.roundTopics as topic}
                <tr>
                    <td class="round-topic">{topic.topicName}</td>
                    {#each topic.questions as question}
                        <QuestionTile topicName={topic.topicName} {question}/>
                    {/each}
                </tr>
            {/each}
            </tbody>
        </table>
    </div>
</div>

<style>
    .round-screen {
        display: flex;
        flex-direction: column;
        /*flex: 1;*/
        height: 66vh;
    }

    .title-bar {
        margin: 15px 15px 0;
        padding: 3px 10px;

        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;

        border: solid 1px #dcc680;
        border-radius: 15px;
        color: black;
        background-color: #fdf2d046;
    }

    .round-table-box {
        flex: 1;
        border: 0;
        margin: 10px;
        padding: 0;
    }

    .round-label {
        flex: 1;
        height: 27px;
        text-align: center;

        padding: 10px 0px 10px 0px;
        margin: 0;

        font-size: 35px;
        font-weight: bold;
        color: var(--text-color);
    }

    /* Table */
    .round-table {
        flex: 1;
        background-color: var(--modal-table-background-color);
    }

    tr,
    td {
        border: 1px solid gray;
        padding: 0.3em;
        background-color: var(--items-block-color);
        border-radius: 15px;
    }

    .round-topic {
        width: 35%;
        font-weight: bold;
        text-align: center;
        font-size: xx-large;
    }

</style>