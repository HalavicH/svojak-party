<script>
    import Row from "./Row.svelte";
    import DropDown from "./DropDown.svelte";
    import HSpacing from "./utils/HSpacing.svelte";

    export let title;
</script>

<div class="items-block">
    <p class="items-block-title">{title}</p>
    <slot/>
</div>


<style>
    .items-block {
        --title-text-color: #9cb7b9;
        --items-block-color: #2f2f2f;
        --items-block-border-color: #000000;

        background-color: var(--items-block-color);
        border: 1px solid var(--items-block-border-color);
        border-radius: inherit;
        padding: 5px 5px 5px 10px;
        margin: 5px;
        display: flex;
        flex-direction: column;
    }

    .items-block-title {
        display: flex;
        flex-direction: row;
        justify-content: center;

        margin-top: 5px;

        color: var(--title-text-color);
        font-style: italic;
        font-size: 1.1em;
    }
</style>