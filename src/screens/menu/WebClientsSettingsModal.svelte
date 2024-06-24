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
    import QRCode from 'qrcode';
    import DropDown from "../../components/generic/DropDown.svelte";
    import {callBackend, TauriApiCommand} from "../../lib/commands.js";

    // Provided by 'modals'
    export let isOpen;

    $: players = $currentPlayersStore.map(p => {
        if (p.iconPath === "default") {
            p.iconPath = DFL_PLAYER_ICON;
        }
        return p;
    });
    let config = $currentHubConfigStore;
    let joinQrCode;
    let hubPort;
    currentHubConfigStore.subscribe(async value => {
            console.log(value);
            joinQrCode = await QRCode.toDataURL(value.hubPort);
            hubPort = value.hubPort;
        }
    )
    $: portOptions = config.availablePorts.map((portName) => {
        return {
            value: portName,
            title: portName
        };
    });

    async function handleSetup(selected) {
        await callBackend(TauriApiCommand.DISCOVER_HUB, {path: selected});
    }
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
                    <div>Using network interface:</div>
                </td>
                <td>
                    <DropDown defaultValue={config.hubPort} options={portOptions} handleSelection={handleSetup}/>
                </td>
            </tr>
            <tr>
                <td>
                    <div>Scan to join the game</div>
                </td>
                <td>
                    <img class="qr-code" src={joinQrCode} alt="">
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

    .qr-code {
        display: block;
        margin: 0 auto;
        width: 7em;
        height: 7em;
        border-radius: 0.5em;
        border: solid 0.1em var(--items-block-border-color);
    }
</style>
