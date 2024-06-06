<script>
    import BaseModal from "../components/abstract/BaseModal.svelte";
    import ItemsBlock from "../components/generic/ItemsBlock.svelte";
    import Row from "../components/generic/Row.svelte";
    import {TauriApiCommand, callBackend} from "../lib/commands.js";
    import {notify} from "../lib/notifications.js";
    import {currentPlayersStore, currentPackInfoStore, navTo, GameState} from "../lib/stores.js";
    import {Views} from "./views.js";
    import SecondaryButton from "../components/generic/SecondaryButton.svelte";
    import {isRunningInTauri} from "../lib/misc.js";
    import {currentGameStateStore} from "../lib/stores.js";

    // Provided by 'modals'
    export let isOpen;
    const VIRTUAL_HUB_PORT = "Demo HUB port";

    let gameStates = Object.values(GameState);

    async function setDemoHub() {
        await callBackend(TauriApiCommand.DISCOVER_HUB, {path: VIRTUAL_HUB_PORT});
        notify.info(`Opened: ${VIRTUAL_HUB_PORT}`)
    }

    async function openKefLoh() {
        await callBackend(TauriApiCommand.INIT_GAME_PACK, {path: '/Users/oleksandrkholiavko/Documents/Кеф лох.siq'});
        notify.info(`Opened: ${VIRTUAL_HUB_PORT}`);
    }

    async function resetGame() {
        await callBackend(TauriApiCommand.DBG_RESET_GAME);
        notify.info(`Game reset`);
    }

    async function startTheGame() {
        await goToQuiz();
        await callBackend(TauriApiCommand.START_NEW_GAME);
        notify.info(`Game started`);
    }

    async function goToMenu() {
        navTo(Views.MENU);
    }

    async function goToQuiz() {
        navTo(Views.QUIZ);
    }

    async function setState(state) {
        notify.info(`Set state to: '${state}'`);
        if (isRunningInTauri()) {
            await callBackend(TauriApiCommand.DBG_SET_GAME_STATE, {name: state});
        } else {
            currentGameStateStore.set({gameState: state});
        }
    }
</script>

<BaseModal {isOpen}>
    <h2>Debug Mode</h2>
    <ItemsBlock title="Backend Actions">
        <table>
            <tbody>
            <tr>
                <td>
                    <SecondaryButton text="Reset Game" onClick={resetGame}/>
                </td>
                <td><p>Set's state to SetupAndLoading. Clears players and hub</p></td>
            </tr>
            <tr>
                <td>
                    <SecondaryButton text="Init Demo Hub" onClick={setDemoHub}/>
                </td>
                <td>
                    <p>Players cnt: {$currentPlayersStore.length}</p>
                </td>
            </tr>
            <tr>
                <td>
                    <SecondaryButton text="Open 'Кеф Лох'" onClick={openKefLoh}/>
                </td>
                <td>
                    <p>Pack loaded: {$currentPackInfoStore.packName}</p>
                </td>
            </tr>
            <tr>
                <td>
                    <SecondaryButton text="Start the Game" onClick={startTheGame}/>
                </td>
                <td>
                    <p>Starts new game with current config</p>
                </td>
            </tr>
            </tbody>
        </table>
    </ItemsBlock>
    <ItemsBlock title="Go to:">
        <Row>
            <SecondaryButton text="Menu" onClick={goToMenu}/>
            <SecondaryButton text="Quiz" onClick={goToQuiz}/>
        </Row>
    </ItemsBlock>
    <ItemsBlock title="Set state:">
        <Row>
            <div class="container">
                {#each gameStates as state}
                    <div class="item">
                        <SecondaryButton text="{state}" onClick={()=>{setState(state)}}/>
                    </div>
                {/each}
            </div>
        </Row>
    </ItemsBlock>
</BaseModal>

<style>
    .container {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        /*width: 60%;*/
    }

    .item {
        flex: 0 0 auto;
        /* Adjust width and height as needed */
        /*width: 100px;*/
        /*height: 100px;*/
        /* Add margin or padding if desired */
        margin: 5px;
        /* Additional styling as needed */
        /*background-color: #f0f0f0;*/
        /*border: 1px solid #ccc;*/
    }

    p {
        text-align: left;
    }

    tr {
        /*display: flex;*/
    }

    td {
        /*border: 1px solid #0f0f0f;*/
        padding: 5px;
    }

    h2 {
        margin-top: 0;
    }

    p {
        margin: 5px 0px;
    }
</style>
