<script>
    import {currentPlayersStore} from "../../lib/stores.js";
    import Player from "./subcomponents/Player.svelte";

    $: players = $currentPlayersStore;
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
        margin: 0 10px 10px;
    }
</style>