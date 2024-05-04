<script>
    export let id = "";
    export let placeholder = "";
    export let style = "";
    export let value = '';

    export let onInput = (_) => {};

    function handleInput(event) {
        const { value } = event.target;
        onInput(value);
    }
</script>

<input type="text" {id} {style} {placeholder} bind:value on:input={handleInput}>

<style>
    input {
        padding: 0 0.3em;
        margin: 0 0.3em;
        border-radius: 0.5em;
        border: 1px solid transparent;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        color: var(--text-color);
        background-color: var(--primary-button-color);
        transition: border-color 0.25s, filter 0.5s;
        box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
        will-change: filter;
        cursor: pointer;
        outline: none;
        box-sizing: border-box;
    }
</style>