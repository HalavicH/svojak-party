<script>
    import BaseModal from "../../components/abstract/BaseModal.svelte";
    import Button from "../../components/generic/Button.svelte";
    import {closeModal} from "svelte-modals";
    import ItemsBlock from "../../components/generic/ItemsBlock.svelte";
    import HubStatus from "../../components/HubStatus.svelte";
    import ActionsBlock from "../../components/generic/ActionsBlock.svelte";
    import Table from "../../components/generic/Table.svelte";
    import {TauriApiCommand, HubStatusOptions, hubManagerError2Msg, callBackend} from "../../lib/commands"
    import DropDown from "../../components/generic/DropDown.svelte";
    import Row from "../../components/generic/Row.svelte";
    import ConfigButton from "../../components/ConfigButton.svelte";
    import Input from "../../components/generic/TextInput.svelte";
    import {notify} from "../../lib/notifications"
    import {gameContext, gamePlayers} from "../../lib/stores.js";
    import {DFL_PLAYER_ICON} from "../../lib/misc.js"

    // Provided by 'modals'
    export let isOpen;

    let players;
    // let config;
    let hubStatus;
    let serialPorts = [];
    let radioChannel = null;

    let hubPortUsed;
    $: console.log(`Modal HwClientsSettingsModal is ${isOpen}`);
    $: console.log(`Hub port used: ${hubPortUsed}`);

    // Watch for changes in isOpen and trigger the API call if it becomes true
    $: if (isOpen) {
        let config = $gameContext;
        hubPortUsed = config.hubPort;
        players = config.players.map(p => {
            if (p.iconPath === "default") {
                p.iconPath = DFL_PLAYER_ICON;
            }
            return p;
        });
        hubStatus = config.hubStatus;
        radioChannel = config.radioChannel;
        serialPorts = config.availablePorts.map((portName) => {
            return {
                value: portName,
                title: portName
            };
        });
    }

    async function saveSettings() {
        console.log("Saved!");
        closeModal();
        await callBackend(TauriApiCommand.SAVE_PLAYERS, {players});
    }

    async function setRadioChannel() {
        if (!radioChannel) {
            notify.info(`Please type radio channel`);
            return;
        }

        callBackend(TauriApiCommand.SET_HW_HUB_RADIO_CHANNEL, {channelId: radioChannel})
            .catch(error => {
                notify.failure(hubManagerError2Msg(error));
            });
    }

    async function discoverHub(portName) {
        notify.info(`Discovering hub: ${portName}`)
        console.log(`Discovering hub: ${portName}`);
        callBackend(TauriApiCommand.DISCOVER_HUB, {path: portName}).then();
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
                    <DropDown defaultValue={hubPortUsed} options={serialPorts} handleSelection={discoverHub}/>
                </td>
            </tr>
            </tbody>
        </table>
    </ItemsBlock>
    <ItemsBlock title="Setup terminals & users">
        <Row jc="space-between">
            <div>Provide radio channel num:</div>
            <Row>
                <Input value={radioChannel} placeholder="1-127" style="width: 4em;"
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
                        {player.id}
                    </td>
                    <td>
                        <img class="icon" src="{player.iconPath}" alt="">
                    </td>
                    <td>
                        {player.name}
                    </td>
                    <td>
                        <input type="checkbox" checked={player.isUsed}>
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

    .icon {
        width: 2em;
        height: 2em;
    }
</style>
