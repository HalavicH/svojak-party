<script>
    export let greedy = false;
</script>

<div class="centered" class:greedy={greedy}>
    <slot></slot>
</div>

<style>
    .centered {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    .greedy {
        flex: 1;
    }
</style>