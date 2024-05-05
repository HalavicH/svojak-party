<script>
    export let title;
</script>

<div>
    <p>{title}</p>
    <slot/>
</div>


<style>
    div {
        background-color: var(--items-block-color);
        border: 1px solid var(--items-block-border-color);
        border-radius: inherit;
        padding: 5px 5px 5px 10px;
        margin: 5px;
        display: flex;
        flex-direction: column;
    }

    p {
        display: flex;
        flex-direction: row;
        justify-content: center;

        margin: 5px 0.5em 0.5em;

        color: var(--title-text-color);
        font-style: italic;
        font-size: 1.1em;
        font-weight: bolder;
    }
</style>