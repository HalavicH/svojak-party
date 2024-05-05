<script>
    import PlayersView from "./quiz/PlayersView.svelte";
    import RoundView from "./quiz/RoundView.svelte";
</script>

<div class="container">
<!--    <div>-->
<!--        This is a quiz screen. Work in progress-->
<!--    </div>-->
<!--    <a href="/">Home</a>-->
    <RoundView/>
    <PlayersView/>
</div>

<style>
    .container {
        height: 98vh;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        border: solid 1px gray;
        overflow: auto;

        background-color: var(--modal-table-background-color);
    }
</style>