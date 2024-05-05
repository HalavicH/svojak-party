<!-- Table.svelte -->
<script>
    export let headers;
</script>

<table class="dark-table">
    <tbody>
    <tr class="dark-table-labels">
        {#each headers as header}
            <td class="header">
                {header}
            </td>
        {/each}
    </tr>
    <slot></slot> <!-- Slot to receive rows -->
    </tbody>
</table>


<style>
    .dark-table {
        background-color: var(--modal-table-background-color);
        border: solid 1px var(--modal-table-border-color);
        margin-top: 5px;
        padding: 5px;
        border-radius: 10px;
    }

    .dark-table tr {
        text-align: center;
    }

    .dark-table-labels {
        background-color: var(--modal-table-labels-color);
    }

    .dark-table-labels td {
        border-radius: 10px;
        align-items: center;
    }

    .player-image {
        width: 32px;
        height: 32px;
        border-radius: 50%;
        cursor: pointer;
    }


</style>