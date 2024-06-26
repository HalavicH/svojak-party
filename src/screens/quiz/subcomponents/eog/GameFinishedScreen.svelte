<script>
    import {currentFinalResultsStore, EndGameReason} from "../../../../lib/stores.js";
    import Table from "../../../../components/generic/Table.svelte";
    import {toPlayerImage} from "../../../../lib/misc.js";

    currentFinalResultsStore.subscribe(value => {
        console.log(value);
    });

    let stats = $currentFinalResultsStore;
    let endGameText;
    switch (stats.endGameReason) {
        case EndGameReason.NoPlayersLeft:
            endGameText = "It seems no one survived! Well, game pack author will be happy 😈";
            break;
        case EndGameReason.AllRoundsPlayed:
            endGameText = "All rounds played! Finally you're not obligated to play anymore! 🤝";
            break;
        case EndGameReason.OnePlayerLeft:
            endGameText = "Only one survived! Congratulations.. I guess ??";
            break;
    }

</script>

<div>
    <h2>{endGameText}</h2>
    <p>Top places:</p>
    <div class="common-stats">
        <Table headers={["Place", "Icon", "Name", "Score"]}>
            <tr>
                <td>1</td>
                <td>
                    <img class="icon" src={toPlayerImage(stats.first.icon)} alt="">
                </td>
                <td>{stats.first.name}</td>
                <td>{stats.first.score}</td>
            </tr>
            <tr>
                <td>2</td>
                <td>
                    <img class="icon" src={toPlayerImage(stats.second.icon)} alt="">
                </td>
                <td>{stats.second.name}</td>
                <td>{stats.second.score}</td>
            </tr>
            {#if stats.third !== undefined && stats.third !== null}
                <tr>
                    <td>3</td>
                    <td>
                        <img class="icon" src={toPlayerImage(stats.third.icon)} alt="">
                    </td>
                    <td>{stats.third.name}</td>
                    <td>{stats.third.score}</td>
                </tr>
            {/if}
        </Table>
    </div>

    {#if stats.theRest.length > 0}
        <p>Other players:</p>
        <div class="common-stats">
            <Table headers={["Icon", "Name", "Score"]}>
                {#each stats.theRest as player}
                    <tr>
                        <td>
                            <img class="icon" src={toPlayerImage(player.icon)} alt="">
                        </td>
                        <td>{player.name}</td>
                        <td>{player.score}</td>
                    </tr>
                {/each}
            </Table>
        </div>
    {/if}
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
