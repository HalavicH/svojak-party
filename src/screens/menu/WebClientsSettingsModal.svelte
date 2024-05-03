<script>
    import BaseModal from "../../components/utils/BaseModal.svelte";
    import Button from "../../components/Button.svelte";
    import HSpacing from "../../components/utils/HSpacing.svelte";
    import VSpacing from "../../components/utils/VSpacing.svelte";
    import {closeModal, openModal} from "svelte-modals";
    import ItemsBlock from "../../components/ItemsBlock.svelte";
    import HubStatus from "../../components/HubStatus.svelte";
    import ActionsBlock from "../../components/ActionsBlock.svelte";
    import Table from "../../components/Table.svelte";

    export let isOpen;

    let hubStatus = "Undefined";

    function saveSettings() {
        console.log("Saved!");
        closeModal();
    }

    let headers = [
        "Icon",
        "Name",
        "Ready"
    ];

    let players = [
        {id: 1, name: "Button", icon: "../../public/bc-logo.png", ready: true},
        {id: 2, name: "HalavicH", icon: "../../public/bc-logo.png", ready: true},
    ]
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
                    <div id="hub-ip-field" class="io-data-field"></div>
                </td>
            </tr>
            </tbody>
        </table>
    </ItemsBlock>
    <ItemsBlock title="Connected players:">
        <Table {headers}>
            {#each players as player}
                <tr>
                    <!--{#each row as columnContent}-->
                    <!--    <td>-->
                    <!--        {@html columnContent}-->
                    <!--    </td>-->
                    <!--{/each}-->
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
