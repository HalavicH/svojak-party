<script>
    export let id = "";
    export let text = "Button";
    export let onClick = () => {};
</script>

<button id={id} type="button" on:click={onClick}>{text}</button>

<style>
    button:hover {
        filter: drop-shadow(0 0 2em #747bff);
    }
</style>