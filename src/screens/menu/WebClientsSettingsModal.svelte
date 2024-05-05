<script>
    import BaseModal from "../../components/abstract/BaseModal.svelte";
    import Button from "../../components/generic/Button.svelte";
    import {closeModal} from "svelte-modals";
    import ItemsBlock from "../../components/generic/ItemsBlock.svelte";
    import HubStatus from "../../components/HubStatus.svelte";
    import ActionsBlock from "../../components/generic/ActionsBlock.svelte";
    import Table from "../../components/generic/Table.svelte";
    import {invoke} from "@tauri-apps/api/tauri";
    import {TauriApiCommand, HubStatusOptions} from "../../lib/commands"
    import {onDestroy, onMount} from "svelte";

    // Provided by 'modals'
    export let isOpen;
    export let title;
    export let message;

    async function saveSettings() {
        console.log("Saved!");
        closeModal();
        await invoke(TauriApiCommand.SAVE_PLAYERS, {players});
    }

    let players = [];
    let config;
    let hubPort;
    let hubStatus = HubStatusOptions.NoDevice;

    // Function to fetch configuration
    async function fetchConfiguration() {
        try {
            config = await invoke(TauriApiCommand.FETCH_CONFIGURATION);
            console.log(config);
            hubPort = config.hub_port;
        } catch (error) {
            console.error('Error fetching configuration:', error);
        }
    }

    // Watch for changes in isOpen and trigger the API call if it becomes true
    $: if (isOpen) {
        fetchConfiguration().then(async () => {
            console.log("Discovering hub: " + hubPort);
            hubStatus = await invoke(TauriApiCommand.DISCOVER_HUB, {path: hubPort});
            console.log(`Hub status: ${hubStatus}`)
        });
    }

    $: console.log(`Modal WebClientsSettingsModal is ${isOpen}`);

    let interval;
    onMount(() => {
        // Start the interval on mount
        interval = setInterval(async () => {
            if (hubStatus === HubStatusOptions.Detected && isOpen === true) {
                console.log("Setup modal is opened. Polling for players");
                try {
                    players = await invoke(TauriApiCommand.DISCOVER_PLAYERS, { path: hubPort });
                    console.log(players);
                } catch (error) {
                    console.error('Error fetching players:', error);
                }
            }
        }, 1000);
    });

    onDestroy(() => {
        // Clean up by stopping the interval on destroy
        clearInterval(interval);
    });
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
                    <HubStatus {hubStatus}/>
                </td>
            </tr>
            <tr>
                <td>
                    <div>Hub address:</div>
                </td>
                <td>
                    <div class="io-data-field">{hubPort}</div>
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
                        <img class="player-icon" src="{player.icon}" alt="">
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
        <Button text="Don't save & Close" onClick={closeModal}/>
        <Button text="Save & Close" onClick={saveSettings}/>
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

    .player-icon {
        width: 2em;
        height: 2em;
    }
</style>
