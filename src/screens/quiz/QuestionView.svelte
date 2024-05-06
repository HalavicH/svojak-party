<script>

    import QuestionViewport from "./subcomponents/QuestionViewport.svelte";
    import QuestionSidebar from "./subcomponents/QuestionSidebar.svelte";
    import Row from "../../components/generic/Row.svelte";
</script>

<Row>
    <QuestionViewport/>
    <QuestionSidebar/>
</Row>

<style>
    /* Question screen */
    .question-screen {

        flex-direction: row;
        height: 66vh;

        margin: 10px;
    }
</style>