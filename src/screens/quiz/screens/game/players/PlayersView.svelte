<script>
    import {currentPlayersStore} from "../../../../../lib/stores.js";
    import Player from "./Player.svelte";
    import {DFL_PLAYER_ICON} from "../../../../../lib/misc.js";

    $: players = $currentPlayersStore.map(p => {
        if (p.iconPath === "default") {
            p.iconPath = DFL_PLAYER_ICON;
        }
        return p;
    });
</script>

<div class="player-list">
    {#each players as player}
        <Player {player}/>
    {/each}
</div>

<style>
    .player-list {
        display: flex;
        flex-wrap: nowrap;
        justify-content: center;

        border-radius: 10px;
        background-color: var(--modal-table-background-color);

        height: 30vh;
        margin: 0.5em;
    }
</style>