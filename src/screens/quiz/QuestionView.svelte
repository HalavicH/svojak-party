<script>

    import QuestionViewport from "./subcomponents/QuestionViewport.svelte";
    import QuestionSidebar from "./subcomponents/QuestionSidebar.svelte";
    import Row from "../../components/generic/Row.svelte";
</script>

<div class="row">
    <QuestionViewport/>
    <QuestionSidebar/>
</div>

<style>
    .row {
        display: flex;
        flex-direction: row;
        /*justify-content: center;*/
        flex: 1;
        align-items: center;
    }
</style>