<script>
    export let jc = "center";
</script>

<div class="row" style="justify-content: {jc}">
    <slot></slot>
</div>

<style>
    .row {
        display: flex;
        flex-direction: row;
        /*justify-content: center;*/
        align-items: center;
    }
</style>