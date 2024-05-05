<script>
    import BaseModal from "../../components/abstract/BaseModal.svelte";
    import Button from "../../components/generic/Button.svelte";
    import HSpacing from "../../components/generic/HSpacing.svelte";
    import VSpacing from "../../components/generic/VSpacing.svelte";
    import ItemsBlock from "../../components/generic/ItemsBlock.svelte";
    import Row from "../../components/generic/Row.svelte";
    import DropDown from "../../components/generic/DropDown.svelte";
    import MultiColumnList from "../../components/generic/MultiColumnList.svelte";
    import {closeModal, openModal} from 'svelte-modals'
    import SettingsModal from "./SettingsModal.svelte";
    import WarningBar from "../../components/generic/WarningBar.svelte";
    import {gameContext, gamePackInfo, navTo} from "../../lib/stores.js";
    import {Views} from "../views.js";
    import {TauriApiCommand, callBackend} from "../../lib/commands.js";

    export let isOpen;

    export let packInfo = $gamePackInfo;

    // Static
    let gameDurationOptions = [
        {value: 10, title: "10min"},
        {value: 15, title: "15min"},
        {value: 20, title: "20min"}
    ];

    let players = $gameContext.players;

    function openSettings() {
        closeModal();
        openModal(SettingsModal);
    }

    function startTheGame() {
        console.log("Start pressed");
        callBackend(TauriApiCommand.START_THE_GAME)
        navTo(Views.QUIZ);
    }

    async function setRoundDuration(selected) {
        callBackend(TauriApiCommand.SAVE_ROUND_DURATION, {round_minutes: selected})
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
            <DropDown selectedValue={gameContext.roundDurationMin} options={gameDurationOptions} handleSelection={setRoundDuration}/>
        </Row>
    </ItemsBlock>

    <div>Players ready: {players.length}</div>
    <VSpacing size="1em"/>
    {#if players.length < 2}
        <WarningBar text="It's required to have at least 2 players to start"/>
        <div class="action-block">
            <Button text="Open settings" onClick={openSettings}/>
        </div>
    {:else}
        <div class="action-block">
            <Button text="Start the game" onClick={startTheGame}/>
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
