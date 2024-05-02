<script>
    import BaseModal from "../../components/utils/BaseModal.svelte";
    import Button from "../../components/Button.svelte";
    import HSpacing from "../../components/utils/HSpacing.svelte";
    import VSpacing from "../../components/utils/VSpacing.svelte";
    import ItemsBlock from "../../components/ItemsBlock.svelte";
    import Row from "../../components/Row.svelte";
    import DropDown from "../../components/DropDown.svelte";

    export let isOpen;
</script>

<BaseModal {isOpen}>
    <h2>Pack: Zlyj reper Zenyk</h2>
    <div class="items-block">
        <p class="items-block-title" id="pack-info-title">Pack info:</p>
        <div id="pack-author-form">Author: Zlyj reper Zenyk</div>
        <ul id="pack-content-brief">
            <li id="pack-round-num">Rounds: 3</li>
            <li id="pack-topics-num">Topics: 20</li>
            <li id="pack-question-num">Questions: 66</li>
        </ul>
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
    </div>

    <ItemsBlock title="Gameplay settings:">
        <Row>
            <label for="round-duration">Select round duration:</label>
            <HSpacing/>
            <DropDown options={[
                {value: 10, title: "10min"},
                {value: 15, title: "15min"},
                {value: 20, title: "20min"}
            ]}/>
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
