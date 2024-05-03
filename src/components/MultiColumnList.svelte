<script>
    export let items;
</script>

<ul id="topic-list">
    {#each items as item}
        <li style="text-align: left">{item}</li>
    {/each}
</ul>

<style>
    #topic-list {
        display: flex;
        flex-wrap: wrap;
        justify-content: space-between;
    }

    #topic-list li {
        /* display: flex; */
        word-wrap: break-word;
        list-style-type: disc;
        min-width: 50%;
    }
</style>