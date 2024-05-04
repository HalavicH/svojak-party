<script>
    export let options;
    export let handleSelection;

    console.log("Drop-down-menu:")
    function handleChange(event) {
        const selectedValue = event.target.value;
        handleSelection(selectedValue);
    }
</script>

<select name="size" on:change={handleChange}>
    {#each options as opt}
        <option value={opt.value}>{opt.title}</option>
    {/each}
</select>

<style>
    select {
        --modal-select-color: #262828;
        appearance: none;
        -webkit-appearance: none;
        -moz-appearance: none;
        padding: 0.2em 0.3em;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        color: var(--text-color);
        background-color: var(--modal-select-color);
        border-radius: 8px;
        border: solid 1px #3a3a3a6e;
        transition: border-color 0.25s;
        box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    }
</style>