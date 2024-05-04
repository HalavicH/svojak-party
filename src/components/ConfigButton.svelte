<script>
    import Button from "./Button.svelte";

    export let id = "";
    export let text = "Button";
    export let onClick = () => {};
    let style = `
        border-radius: 5px;
        padding: 0.2em 0.6em;
        color: #2a2a2a;
        background-color: rgba(0, 255, 146, 0.78);
        font-size: 0.8em;
    `;
</script>

<Button {id} {text} {onClick} {style}/>