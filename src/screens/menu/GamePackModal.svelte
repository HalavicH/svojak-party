<script>
    import BaseModal from "../../components/utils/BaseModal.svelte";
    import Button from "../../components/Button.svelte";
    import HSpacing from "../../components/utils/HSpacing.svelte";
    import VSpacing from "../../components/utils/VSpacing.svelte";
    import ItemsBlock from "../../components/ItemsBlock.svelte";
    import Row from "../../components/Row.svelte";
    import DropDown from "../../components/DropDown.svelte";

    export let isOpen;

    // Static
    let gameDurationOptions = [
        {value: 10, title: "10min"},
        {value: 15, title: "15min"},
        {value: 20, title: "20min"}
    ];
</script>

<BaseModal {isOpen}>
    <h2>Pack: Zlyj reper Zenyk</h2>
    <ItemsBlock title="Pack info:">
        <label>Author: Zlyj reper Zenyk</label>
        <VSpacing size="0.5em"/>
        <Row jc={"space-around"}>
            <div id="pack-round-num">Rounds: 3</div>
            <div id="pack-topics-num">Topics: 20</div>
            <div id="pack-question-num">Questions: 66</div>
        </Row>
        <VSpacing size="0.5em"/>
        <div id="topic-list-title">Topic list:</div>
        <ul id="topic-list">
            <li>Beer</li>
            <li>Pone</li>
            <li>Music</li>
            <li>Movies</li>
            <li>Fallout equestria</li>
            <li>Beer</li>
            <li>Pone</li>
            <li>Music</li>
            <li>Movies</li>
            <li>Fallout equestria</li>
            <li>Beer</li>
            <li>Pone</li>
            <li>Music</li>
            <li>Movies</li>
            <li>Fallout equestria</li>
        </ul>
    </ItemsBlock>

    <ItemsBlock title="Gameplay settings:">
        <Row>
            <label for="round-duration">Select round duration:</label>
            <HSpacing/>
            <DropDown options={gameDurationOptions}/>
        </Row>
    </ItemsBlock>

    <div class="action-block">
        <button id="start-the-game" type="button">Start the game</button>
    </div>
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
