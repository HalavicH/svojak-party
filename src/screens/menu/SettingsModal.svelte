<script>
    import BaseModal from "../../components/utils/BaseModal.svelte";
    import Button from "../../components/Button.svelte";
    import HSpacing from "../../components/utils/HSpacing.svelte";
    import VSpacing from "../../components/utils/VSpacing.svelte";
    import {closeModal, openModal} from "svelte-modals";
    import ItemsBlock from "../../components/ItemsBlock.svelte";
    import Row from "../../components/Row.svelte";
    import WebClientsSettingsModal from "./WebClientsSettingsModal.svelte";

    export let isOpen;

    function openPhysicalClientsSettings() {
        closeModal();
        // openModal(PhysicalClientsSettingsModal);
    }

    function openWebClientsSettings() {
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

    .row {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }
</style>
