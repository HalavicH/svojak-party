<script>
    import BaseModal from "../../components/utils/BaseModal.svelte";
    import Button from "../../components/Button.svelte";
    import HSpacing from "../../components/utils/HSpacing.svelte";
    import VSpacing from "../../components/utils/VSpacing.svelte";
    import ItemsBlock from "../../components/ItemsBlock.svelte";
    import Row from "../../components/Row.svelte";
    import DropDown from "../../components/DropDown.svelte";
    import MultiColumnList from "../../components/MultiColumnList.svelte";
    import {closeModal, openModal} from 'svelte-modals'
    import SettingsModal from "./SettingsModal.svelte";
    import WarningBar from "../../components/WarningBar.svelte";
    import {open} from "@tauri-apps/api/dialog";
    import {notify} from "../../lib/notifications"
    import {invoke} from "@tauri-apps/api/tauri";
    import {TauriApiCommand} from "../../lib/commands.js";
    import PackErrorModal from "./PackErrorModal.svelte";

    export let isOpen;

    export let packInfo = {
        packName: 'Zlyj reper Zenyk',
        packAuthor: 'Zlyj reper Zenyk',
        packRounds: 3,
        packTopics: 3,
        packQuestions: 69,
        packTopicList: [
            'Beer',
            'Pone',
            'Music',
            'Movies',
            'Fallout',
            'Beer',
            'Pone',
            'Music',
            'Movies',
            'Fallout',
            'Beer',
            'Pone',
            'Music',
            'Movies',
            'Fallout',
        ],
    };

    // Static
    let gameDurationOptions = [
        {value: 10, title: "10min"},
        {value: 15, title: "15min"},
        {value: 20, title: "20min"}
    ];

    let users = [
        {id: 1, name: "Button"},
        // {id: 2, name: "HalavicH"},
        // {id: 3, name: "Baadtrip"},
    ];

    function openSettings() {
        closeModal();
        openModal(SettingsModal);
    }
</script>

<BaseModal {isOpen}>
    <h2>Pack: {packInfo.packName}</h2>
    <ItemsBlock title="Pack info:">
        <div class="sub-title">Author: {packInfo.packAuthor}</div>
        <VSpacing size="0.5em"/>
        <Row jc={"space-around"}>
            <div>Rounds: {packInfo.packRounds}</div>
            <div>Topics: {packInfo.packTopics}</div>
            <div>Questions: {packInfo.packQuestions}</div>
        </Row>

        <VSpacing size="1em"/>

        <div class="sub-title">Topic list:</div>
        <MultiColumnList items={packInfo.packTopicList}/>
    </ItemsBlock>

    <ItemsBlock title="Gameplay settings:">
        <Row>
            <label for="round-duration">Select round duration:</label>
            <HSpacing size="1em"/>
            <DropDown options={gameDurationOptions}/>
        </Row>
    </ItemsBlock>

    <VSpacing size="1em"/>
    {#if users.length < 2}
        <WarningBar text="It's required to have at least 2 players to start"/>
        <div class="action-block">
            <Button text="Open settings" onClick={openSettings}/>
        </div>
    {:else}
        <div class="action-block">
            <Button text="Start the game" onClick={() => {console.log("Start pressed");}}/>
        </div>
    {/if}

</BaseModal>

<style>
    h2 {
        margin-top: 0;
    }

    p {
        margin: 5px 0px;
    }

    .sub-title {
        color: #adadad;
        font-style: italic;
    }
</style>
