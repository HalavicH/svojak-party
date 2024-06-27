<script>
    import {currentRoundStatsStore} from "../../../../lib/stores.js";
    import Table from "../../../../components/generic/Table.svelte";
    import {DFL_PLAYER_ICON} from "../../../../lib/misc.js"
    import NextRoundButton from "./NextRoundButton.svelte";
    import {getNewLevelSound} from "../../../../lib/sound.js";

    currentRoundStatsStore.subscribe(value => {
        console.log(value);
    });
    getNewLevelSound().play().then();

    let stats = $currentRoundStatsStore;

    function getRoundDurationTime() {
        if (stats.roundTimeSec === undefined) {
            return "0s";
        }

        if (stats.roundTimeSec < 60) {
            return stats.roundTimeSec + "s";
        }
        return Math.floor(stats.roundTimeSec / 60) + ":" + (stats.roundTimeSec % 60).toString().padStart(2, '0');
    }

    // Round time in minutes:seconds
    let roundDurationTime = getRoundDurationTime();
</script>

<!--
#[derive(Debug, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct RoundStatsDto {
    pub roundName: String,
    pub questionsPlayed: i32,
    pub normalQuestionsPlayed: i32,
    pub pigInPokeQuestionPlayed: i32,
    pub totalCorrectAnswers: i32,
    pub totalWrongAnswers: i32,
    pub totalTries: i32,
    pub roundTime: i32,
    pub players: Vec<PlayerEndRoundStatsDto>,
}
-->
<div class="full-screen">
    <h2>Round '{stats.roundName}' finished!</h2>
    <div class="common-stats">
        <Table headers={[]}>
            <tr>
                <td>Questions played:</td>
                <td>{stats.questionsPlayed}</td>
            </tr>
            <tr>
                <td>Normal questions:</td>
                <td>{stats.normalQuestionsPlayed}</td>
            </tr>
            <tr>
                <td>Pig in poke questions:</td>
                <td>{stats.pigInPokeQuestionPlayed}</td>
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
                <td><img class="icon" src={player.playerIconPath === "default" ? DFL_PLAYER_ICON : player.playerIconPath} alt=""/></td>
                <td>{player.name}</td>
                <td>{player.score}</td>
                <td>{player.totalAnswers}</td>
                <td>{player.answeredCorrectly}</td>
                <td>{player.answeredWrong}</td>
            </tr>
        {/each}
    </Table>
    </div>
    <NextRoundButton/>
</div>

<style>
    .full-screen {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: 100%;
    }
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