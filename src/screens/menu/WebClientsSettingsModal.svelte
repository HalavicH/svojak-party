<script>
    import BaseModal from "../../components/abstract/BaseModal.svelte";
    import Button from "../../components/generic/Button.svelte";
    import {closeModal} from "svelte-modals";
    import ItemsBlock from "../../components/generic/ItemsBlock.svelte";
    import HubStatus from "../../components/HubStatus.svelte";
    import ActionsBlock from "../../components/generic/ActionsBlock.svelte";
    import Table from "../../components/generic/Table.svelte";
    import {DFL_PLAYER_ICON} from "../../lib/misc.js"
    import {currentHubConfigStore, currentPlayersStore} from "../../lib/stores.js";

    // Provided by 'modals'
    export let isOpen;

    $: players = $currentPlayersStore.map(p => {
        if (p.iconPath === "default") {
            p.iconPath = DFL_PLAYER_ICON;
        }
        return p;
    });
    let config = $currentHubConfigStore;
</script>

<BaseModal {isOpen}>
    <h2>Play over LAN settings</h2>
    <ItemsBlock title="Setup HUB connection:">
        <table class="grid">
            <tbody>
            <tr>
                <td>
                    <div>Hub status:</div>
                </td>
                <td>
                    <HubStatus hubStatus={config.hubStatus}/>
                </td>
            </tr>
            <tr>
                <td>
                    <div>Hub address:</div>
                </td>
                <td>
                    <div class="io-data-field">{config.hubPort}</div>
                </td>
            </tr>
            </tbody>
        </table>
    </ItemsBlock>
    <ItemsBlock title="Connected players:">
        <Table headers={[ "Icon", "Name", "Ready"]}>
            {#each players as player}
                <tr>
                    <td>
                        <img class="icon" src="{player.iconPath}" alt="">
                    </td>
                    <td>
                        {player.name}
                    </td>
                    <td>
                        <input type="checkbox" checked={player.ready}>
                    </td>
                </tr>
            {/each}
        </Table>
    </ItemsBlock>
    <ActionsBlock>
        <Button text="Ok" onClick={closeModal}/>
    </ActionsBlock>
</BaseModal>

<style>

    .io-data-field {
        -webkit-user-select: auto; /* Safari */
        -moz-user-select: auto; /* Firefox */
        -ms-user-select: auto; /* IE10+/Edge */
        user-select: auto; /* Standard */
        cursor: text;
        box-sizing: border-box;

        font-size: 0.8em;

        padding: 0.1em 0.5em;
        margin: 0 0.5em;
        border-radius: 10px;
        background-color: var(--primary-button-color);

        font-family: monospace;
    }

    h2 {
        margin-top: 0;
    }

    .icon {
        width: 2em;
        height: 2em;
    }
</style>
