<script>
    import {currentFinalResultsStore} from "../../../../lib/stores.js";
    import Table from "../../../../components/generic/Table.svelte";

    currentFinalResultsStore.subscribe(value => {
        console.log(value);
    });

    let stats = $currentFinalResultsStore;


</script>

<!--
let gameStatsMock = {
    first: {
        name: "HalavicH",
        score: 500,
        state: PlayerState.Idle,
        icon: DFL_PLAYER_ICON,
    },
    second: {
        name: "Valadis",
        score: 400,
        state: PlayerState.Idle,
        icon: DFL_PLAYER_ICON,
    },
    third: {
        name: "Baadtrip",
        score: 200,
        state: PlayerState.Idle,
        icon: DFL_PLAYER_ICON,
    },
    theRest: [
        {
            name: "Button",
            score: -100,
            state: PlayerState.Dead,
            icon: DFL_PLAYER_ICON,
        }
    ]
}
-->
<div>
    <h2>You finally done that :D</h2>
    <p>Top places:</p>
    <div class="common-stats">
        <Table headers={["Place", "Icon", "Name", "Score"]}>
            <tr>
                <td>1</td>
                <td>
                    <img class="icon" src={stats.first.icon} alt="">
                </td>
                <td>{stats.first.name}</td>
                <td>{stats.first.score}</td>
            </tr>
            <tr>
                <td>2</td>
                <td>
                    <img class="icon" src={stats.second.icon} alt="">
                </td>
                <td>{stats.second.name}</td>
                <td>{stats.second.score}</td>
            </tr>
            <tr>
                <td>3</td>
                <td>
                    <img class="icon" src={stats.third.icon} alt="">
                </td>
                <td>{stats.third.name}</td>
                <td>{stats.third.score}</td>
            </tr>
            </Table>
    </div>

    <p>Other players:</p>
    <div class="common-stats">
        <Table headers={["Icon", "Name", "Score"]}>
            {#each stats.theRest as player}
                <tr>
                    <td>
                        <img class="icon" src={player.icon} alt="">
                    </td>
                    <td>{player.name}</td>
                    <td>{player.score}</td>
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
