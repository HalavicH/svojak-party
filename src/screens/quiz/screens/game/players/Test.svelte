<script>
    let name = '';
</script>

<input bind:value={name} placeholder="enter your name" />
<p>Hello {name || 'stranger'}!</p>

<style>
    input {
        pointer-events: auto;
    }

</style>