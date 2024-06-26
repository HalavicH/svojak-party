<script>
    import {PlayerState} from '../../../../../lib/stores.js'

    export let player

    $: stateClass = getPlayerStateClass(player.state)

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
</script>

<div class="badge {getPlayerStateClass(player.state)}">
    <div class="icon">
        <img src={player.iconPath} alt=""/>
    </div>
    <div class="details">
        <p class="name">{player.name}</p>
        <p class="score">Score: {player.score}</p>
    </div>
</div>

<style>
    .badge {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
        /* box-sizing: border-box; */

        background-color: var(--items-block-color);
        border: 1px solid var(--items-block-border-color);
        border-radius: 10px;
        padding: 5px;
        margin: 5px;
        min-width: 10%;
        max-width: 15%;
        height: auto; /* Let the height adjust according to content */

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
        max-width: 100%; /* Ensure the image doesn't exceed the container's width */
        height: auto; /* Allow the height to adjust proportionally */
    }

    .details {
        /*display: flex;*/
        /*flex-direction: column;*/
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

    /* For future use */
    .player-details-score-value {
        text-align: center;
        padding: 5px;
        margin: 0;

        font-weight: bold;
        font-size: 20px;
    }

    .player-details-bid {
        display: flex;
        flex-direction: row;
        align-items: center;

        justify-content: center;
        padding: 0 10px;
        margin: 0;

        background-color: var(--secondary-button-border-color);

        font-size: 20px;
    }

    .player-details-bid-value {
        text-align: center;
        padding: 5px;
        margin: 0;

        font-weight: bold;
        font-size: 20px;
    }

</style>