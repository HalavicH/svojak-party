<script>
    import Row from "./Row.svelte";
    import HSpacing from "./HSpacing.svelte";

    export let text;
</script>

<div class="reset">
    <div class="warning">
        <Row>
            <img src="../../../public/warning-sign-svgrepo-com.svg" alt="">
            <HSpacing size="0.5em"></HSpacing>
            <span>{text}</span>
        </Row>
    </div>
</div>

<style>
    .reset {
        display: flex;
        justify-content: center;
    }

    .warning {
        border-radius: 1em;
        border: 1px solid var(--items-block-border-color);
        padding: 0.5em;
        margin: 0.5em;
        background: #c06a00;
        color: #0f0f0f;
        filter: drop-shadow(0 0 0.15em #ff8000);
    }

    span {
        float: left;
    }
    img {
        width: 1.5em;
        height: 1.5em;
    }

</style>