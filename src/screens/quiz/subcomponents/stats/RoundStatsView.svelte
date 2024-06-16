<script>
    import {currentRoundStatsStore} from "../../../../lib/stores.js";
    import Table from "../../../../components/generic/Table.svelte";

    currentRoundStatsStore.subscribe(value => {
        console.log(value);
    });

    let stats = $currentRoundStatsStore;

    function getRoundDurationTime() {
        if (stats.roundTimeSec === undefined) {
            return "0s";
        }

        if (stats.roundTimeSec < 60) {
            return stats.roundTimeSec + "s";
        }
        // return Math.floor(stats.roundTimeSec / 60) + ":" + stats.roundTimeSec % 60;
        // With leading zero
        return Math.floor(stats.roundTimeSec / 60) + ":" + (stats.roundTimeSec % 60).toString().padStart(2, '0');
    }

    // Round time in minutes:seconds
    let roundDurationTime = getRoundDurationTime();
</script>

<!--
roundStatsMock = {
    roundName: "Злий Репер Зеник",
    questionsPlayed: 4,
    normalQuestionNum: 3,
    pigInPokeQuestionNum: 1,
    totalCorrectAnswers: 4,
    totalWrongAnswers: 3,
    totalTries: 7,
    roundTimeSec: 666,
    players: [
        {
            id: 1,
            name: "HalavicH",
            score: 500,
            playerIconPath: DFL_PLAYER_ICON,
            totalAnswers: 3,
            answeredCorrectly: 2,
            answeredWrong: 1,
        },
        {
            id: 2,
            name: "Button",
            score: -100,
            playerIconPath: DFL_PLAYER_ICON,
            totalAnswers: 2,
            answeredCorrectly: 1,
            answeredWrong: 1,
        },
        {
            id: 3,
            name: "Baadtrip",
            score: 200,
            playerIconPath: DFL_PLAYER_ICON,
            totalAnswers: 1,
            answeredCorrectly: 1,
            answeredWrong: 0,
        },
        {
            id: 4,
            name: "Valadis",
            score: 400,
            playerIconPath: DFL_PLAYER_ICON,
            totalAnswers: 1,
            answeredCorrectly: 0,
            answeredWrong: 1,
        }
    ]
}
-->

<div>
    <h2>Round '{stats.roundName}' finished!</h2>
    <div class="common-stats">
        <Table headers={[]}>
            <tr>
                <td>Questions played:</td>
                <td>{stats.questionsPlayed}</td>
            </tr>
            <tr>
                <td>Normal questions:</td>
                <td>{stats.normalQuestionNum}</td>
            </tr>
            <tr>
                <td>Pig in poke questions:</td>
                <td>{stats.pigInPokeQuestionNum}</td>
            </tr>
            <tr>
                <td>Total correct answers:</td>
                <td>{stats.totalCorrectAnswers}</td>
            </tr>
            <tr>
                <td>Total wrong answers:</td>
                <td>{stats.totalWrongAnswers}</td>
            </tr>
            <tr>
                <td>Total tries:</td>
                <td>{stats.totalTries}</td>
            </tr>
            <tr>
                <td>Round time:</td>
                <td>{roundDurationTime}</td>
            </tr>
        </Table>
    <Table headers={["", "Player", "Score", "Total answers", "Correct answers", "Wrong answers"]}>
        {#each stats.players as player}
            <tr>
                <td><img class="icon" src={player.playerIconPath} alt=""/></td>
                <td>{player.name}</td>
                <td>{player.score}</td>
                <td>{player.totalAnswers}</td>
                <td>{player.answeredCorrectly}</td>
                <td>{player.answeredWrong}</td>
            </tr>
        {/each}
    </Table>
    </div>
</div>

<style>
    .icon {
        width: 2em;
        height: 2em;
    }

    .common-stats {
        display: flex;
        flex-direction: column;
        justify-content: center;
        padding: 0.5em;
        margin: 1em 2em;
        border-radius: 1em;
        background-color: #3a3a3a;
    }
</style>