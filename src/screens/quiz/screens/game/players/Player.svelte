<script>
    import {PlayerState} from '../../../../../lib/stores.js';
    import {writable} from 'svelte/store';
    import {notify} from "../../../../../lib/notifications.js";
    import {callBackend, TauriApiCommand} from "../../../../../lib/commands.js";

    export let player;

    // Function to get the player state class based on the player's state
    function getPlayerStateClass(playerState) {
        switch (playerState) {
            case PlayerState.Idle:
                return '';
            case PlayerState.QuestionChooser:
                return 'question-chooser';
            case PlayerState.Target:
                return 'target-player';
            case PlayerState.Answering:
                return 'answering';
            case PlayerState.Inactive:
            case PlayerState.Dead:
                return 'inactive';
            case PlayerState.AnsweredCorrectly:
                return 'correct-answer';
            case PlayerState.AnsweredWrong:
                return 'wrong-answer';
            default:
                return 'unknown-state';
        }
    }

    let isEditingScore = writable(false);
    let currentScore = player.score;
    let oldValue = currentScore;

    function evaluateExpression(str) {
        // Validate the input string
        const validPattern = /^[0-9+\- ]+$/;
        if (!validPattern.test(str)) {
            throw new Error("Invalid input: The string can only contain numbers, +, -, and spaces.");
        }

        // Evaluate the expression
        try {
            // Use the Function constructor to safely evaluate the expression
            return new Function('return ' + str)();
        } catch (e) {
            throw new Error("Error evaluating the expression");
        }
    }

    // Function to handle score update
    function handleScoreUpdate() {
        notify.info(`New score : ${currentScore}`)
        try {
        console.log("test");
            let score = evaluateExpression(currentScore)
            if (score === Number.parseInt(oldValue)) {
                return;
            }
            oldValue = score;
            disableEditing();

            callBackend(TauriApiCommand.EDIT_PLAYER_SCORE, {
                playerId: player.id, score
            })
        } catch (e) {
            handleCancellation()
        }
    }

    // Function to enable editing mode
    function enableEditing() {
        isEditingScore.set(true);
    }

    // Function to disable editing mode and reset score
    function disableEditing() {
        currentScore = oldValue;
        isEditingScore.set(false);
    }

    function handleCancellation() {
        disableEditing();
    }
</script>

<div class="badge {getPlayerStateClass(player.state)}">
    <div class="icon">
        <img src={player.iconPath} alt=""/>
    </div>
    <div class="details">
        <p class="name">{player.name}</p>
        <p class="score" on:dblclick={enableEditing}>
            {#if $isEditingScore}
                Score: <input type="text" bind:value={currentScore} on:blur={handleCancellation} on:keydown={(e) => e.key === 'Enter' && handleScoreUpdate()} class="score-input"/>
            {:else}
                Score: {player.score}
            {/if}
        </p>
    </div>
</div>

<style>
    .badge {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
        background-color: var(--items-block-color);
        border: 1px solid var(--items-block-border-color);
        border-radius: 10px;
        padding: 5px;
        margin: 5px;
        min-width: 10%;
        max-width: 15%;
        height: auto;
        transition: box-shadow 0.5s;
    }

    /* Statuses */
    .badge.inactive {
        color: #000000;
        filter: grayscale(100%);
        border: 1px solid #818181;
    }

    .badge.game-over {
        color: #000000;
        filter: grayscale(100%);
        border: 1px solid #818181;
    }

    .badge.answering {
        color: #ffffff;
        background: rgba(45, 62, 163, 0.5);
        border: 1px solid #3844b3;
        box-shadow: 0 0 0.5em #243fdb;
    }

    .badge.question-chooser {
        color: #ffffff;
        background: rgba(56, 143, 146, 0.8);
        border: 1px solid #18afb9;
        box-shadow: 0 0 0.5em #2dd1dd;
    }

    .badge.target-player {
        color: #ffffff;
        background: rgba(108, 36, 190, 0.836);
        border: 1px solid #3a0086;
        box-shadow: 0 0 0.9em #59049e;
    }

    .badge.wrong-answer {
        color: #ffffff;
        background: rgba(190, 36, 36, 0.836);
        border: 1px solid #cc0d0d;
        box-shadow: 0 0 0.9em #ff0000;
    }

    .badge.correct-answer {
        color: #ffffff;
        background: rgba(82, 170, 101, 0.6);
        border: 1px solid #52aa65;
        box-shadow: 0 0 0.5em #58db24;
    }

    .badge.unknown-state {
        color: #ffffff;
        background: rgba(0, 0, 0, 0.5);
        border: 1px solid #000000;
        box-shadow: 0 0 0.5em #000000;
    }

    .icon {
        display: flex;
        justify-content: center;
        box-sizing: border-box;
        max-width: 100%;
        max-height: 75%;
        width: auto;
        height: auto;
        object-fit: contain;
        align-self: center;
    }

    .icon img {
        max-width: 100%;
        height: auto;
    }

    .details {
    }

    .name {
        text-align: center;
        padding: 5px;
        margin: 0;
        border-top-left-radius: 10px;
        border-top-right-radius: 10px;
        font-weight: bolder;
        font-size: 20px;
        background-color: var(--modal-table-labels-color);
    }

    .score {
        display: flex;
        flex-direction: row;
        align-items: center;
        text-align: center;
        padding: 0 10px;
        margin: 0;
        border-bottom-left-radius: 10px;
        border-bottom-right-radius: 10px;
        background-color: var(--accent-color);
        font-size: 20px;
    }

    .score input {
        width: 100%;
        font-size: 20px;
        padding: 0;
        margin: 0;
        border: none;
        text-align: center;
        background-color: inherit;
    }

    .score input:focus {
        outline: none;
    }
</style>
