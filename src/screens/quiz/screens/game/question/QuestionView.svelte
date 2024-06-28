<script>

    import QuestionViewport from "./QuestionViewport.svelte";
    import QuestionSidebar from "./sidebar/QuestionSidebar.svelte";
    import Row from "../../../../../components/generic/Row.svelte";
</script>

<div class="row">
    <QuestionViewport/>
    <QuestionSidebar/>
</div>

<style>
    .row {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        flex: 1;
        align-items: center;
    }
</style>