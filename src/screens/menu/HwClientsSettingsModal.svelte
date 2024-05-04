<script>
    import BaseModal from "../../components/utils/BaseModal.svelte";
    import Button from "../../components/Button.svelte";
    import {closeModal} from "svelte-modals";
    import ItemsBlock from "../../components/ItemsBlock.svelte";
    import HubStatus from "../../components/HubStatus.svelte";
    import ActionsBlock from "../../components/ActionsBlock.svelte";
    import Table from "../../components/Table.svelte";
    import {invoke} from "@tauri-apps/api/tauri";
    import {TauriApiCommand, HubStatusOptions, hubManagerError2Msg} from "../../lib/commands"
    import {onDestroy, onMount} from "svelte";
    import DropDown from "../../components/DropDown.svelte";
    import Row from "../../components/Row.svelte";
    import {DFL_PLAYER_ICON} from "../../lib/misc";
    import ConfigButton from "../../components/ConfigButton.svelte";
    import Input from "../../components/TextInput.svelte";
    import {notify} from "../../lib/notifications"

    // Provided by 'modals'
    export let isOpen;

    let players = [];
    let config;
    let hubPort;
    let hubStatus = HubStatusOptions.NoDevice;
    let serialPorts = [];
    let radioChannel = null;

    const emptyOption = {
        title: "Select serial port",
        value: ""
    };

    $: console.log(`Modal HwClientsSettingsModal is ${isOpen}`);

    // Watch for changes in isOpen and trigger the API call if it becomes true
    $: if (isOpen) {
        fetchConfiguration().then();
    }

    // Function to fetch configuration
    async function fetchConfiguration() {
        try {
            config = await invoke(TauriApiCommand.FETCH_CONFIGURATION);
            console.log(config);
            hubPort = config.hub_port;
            let portsFromOs = config.available_ports.map((portName) => {
                return {
                    value: portName,
                    title: portName
                };
            });
            serialPorts = [emptyOption, ...portsFromOs];
        } catch (error) {
            console.error('Error fetching configuration:', error);
        }
    }

    async function saveSettings() {
        console.log("Saved!");
        closeModal();
        await invoke(TauriApiCommand.SAVE_PLAYERS, {players});
    }

    async function setRadioChannel() {
        if (!radioChannel) {
            notify.info(`Please type radio channel`);
            return;
        }

        invoke(TauriApiCommand.SET_HUB_RADIO_CHANNEL, {channelId: radioChannel})
            .catch(error => {
                notify.failure(hubManagerError2Msg(error));
            });
    }

    async function openSerialPort(portName) {
        console.log(`Discovering hub: ${portName}`);
        hubStatus = await invoke(TauriApiCommand.DISCOVER_HUB, {path: portName});
        console.log(`Hub status: ${hubStatus}`)
    }

    function captureInput(text) {
        if (text === null || text === "") {
            console.log("Radio channel is empty");
            return;
        }

        let chNum = Number.parseInt(text);
        if (chNum) {
            notify.info(`RC is ${chNum}`);
            radioChannel = chNum;
        } else {
            notify.failure(`Invalid channel: '${text}'`);
            radioChannel = null;
        }
    }

    // Listen for players
    let interval;
    onMount(() => {
        // Start the interval on mount
        interval = setInterval(async () => {
            if (!(hubStatus === HubStatusOptions.Detected && isOpen === true)) {
                return;
            }

            console.log("Setup modal is opened. Polling for players");
            try {
                let newPlayers = await invoke(TauriApiCommand.DISCOVER_PLAYERS, {path: hubPort});
                players = newPlayers.map(player => {
                    player.name = `Player ${player.termId}`;
                    player.icon = DFL_PLAYER_ICON;
                    return player;
                });
                console.log("Players: ", players);
            } catch (error) {
                console.error('Error fetching players:', error);
            }
        }, 1000);
    });

    onDestroy(() => {
        // Clean up by stopping the interval on destroy
        clearInterval(interval);
    });
</script>

<BaseModal {isOpen}>
    <h2>Settings</h2>
    <ItemsBlock title="Setup HUB Serial connection:">
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
                    <div>Select serial device:</div>
                </td>
                <td>
                    <DropDown options={serialPorts} handleSelection={openSerialPort}/>
                </td>
            </tr>
            </tbody>
        </table>
    </ItemsBlock>
    <ItemsBlock title="Setup terminals & users">
        <Row jc="space-between">
            <div>Provide radio channel num:</div>
            <Row>
                <Input placeholder="1-127" style="width: 4em;"
                       onInput={captureInput}
                />
                <!--                       onReturnPressed={(text) => {notify.info(`Return Text${text}`)}}-->
                <ConfigButton text="Set channel" onClick={setRadioChannel}/>
            </Row>
        </Row>
        <Table headers={[ "Id", "Icon", "Name", "Ready"]}>
            {#each players as player}
                <tr>
                    <td>
                        {player.termId}
                    </td>
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
    h2 {
        margin-top: 0;
    }

    .player-icon {
        width: 2em;
        height: 2em;
    }
</style>
