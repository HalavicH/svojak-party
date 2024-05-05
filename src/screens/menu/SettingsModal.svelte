<script>
    import BaseModal from "../../components/abstract/BaseModal.svelte";
    import Button from "../../components/generic/Button.svelte";
    import HSpacing from "../../components/generic/HSpacing.svelte";
    import {closeModal, openModal} from "svelte-modals";
    import ItemsBlock from "../../components/generic/ItemsBlock.svelte";
    import Row from "../../components/generic/Row.svelte";
    import WebClientsSettingsModal from "./WebClientsSettingsModal.svelte";
    import {invoke} from "@tauri-apps/api/tauri";
    import {HubType, TauriApiCommand} from "../../lib/commands"
    import HwClientsSettingsModal from "./HwClientsSettingsModal.svelte";

    // Provided by 'modals'
    export let isOpen;

    async function openPhysicalClientsSettings() {
        invoke(TauriApiCommand.SET_HUB_TYPE, {hubType: HubType.HwHub}).then();
        closeModal();
        openModal(HwClientsSettingsModal);
    }

    function openWebClientsSettings() {
        invoke(TauriApiCommand.SET_HUB_TYPE, {hubType: HubType.WebHub}).then();
        closeModal();
        openModal(WebClientsSettingsModal);
    }
</script>

<BaseModal {isOpen}>
    <h2>HUB Settings</h2>
    <ItemsBlock title="Choose HUB variant">
        <Row>
            <div>
                <p>Play using special devices</p>
                <Button text="Play using controllers" onClick={openPhysicalClientsSettings}/>
            </div>
            <HSpacing size="5em"/>
            <div>
                <p>Play using yor smartphone</p>
                <Button text="Play over LAN" onClick={openWebClientsSettings}/>
            </div>
        </Row>
    </ItemsBlock>
</BaseModal>

<style>
    h2 {
        margin-top: 0;
    }

    p {
        margin: 5px 0px;
    }
</style>
