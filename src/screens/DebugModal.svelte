<script>
    import BaseModal from "../components/abstract/BaseModal.svelte";
    import ItemsBlock from "../components/generic/ItemsBlock.svelte";
    import Row from "../components/generic/Row.svelte";
    import {callBackend, TauriApiCommand} from "../lib/commands.js";
    import {notify} from "../lib/notifications.js";
    import {
        currentGameStateStore,
        currentHubConfigStore,
        currentPackInfoStore,
        currentPlayersStore,
        currentRoundStore,
        currentQuestionStore,
        GameState,
        navTo
    } from "../lib/stores.js";
    import {Views} from "./views.js";
    import SecondaryButton from "../components/generic/SecondaryButton.svelte";
    import {isRunningInTauri} from "../lib/misc.js";

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

    function renderStoreContent(store) {
        // Pretty print json with 4 spaces per tab
        return JSON.stringify(store, null, 4);
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
    <ItemsBlock title="Stores content">
    <!-- JSON view -->
        <p>Game state store</p>
        <div class="json-view"><pre>{renderStoreContent($currentGameStateStore)}</pre></div>

        <p>Question store</p>
        <div class="json-view"><pre>{renderStoreContent($currentQuestionStore)}</pre></div>

        <p>Players store</p>
        <div class="json-view"><pre>{renderStoreContent($currentPlayersStore)}</pre></div>

        <p>Hub config store</p>
        <div class="json-view"><pre>{renderStoreContent($currentHubConfigStore)}</pre></div>

        <p>Pack info store</p>
        <div class="json-view"><pre>{renderStoreContent($currentPackInfoStore)}</pre></div>

        <p>Round store</p>
        <div class="json-view"><pre>{renderStoreContent($currentRoundStore)}</pre></div>
    </ItemsBlock>
</BaseModal>

<style>
    .container {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        /*width: 60%;*/
    }

    .json-view {
        display: flex;
        flex-direction: column;
        flex-wrap: wrap;
        font-family: monospace;
        font-size: 1em;
        text-align: left;
        padding-left: 5px;
        border: 1px solid gray;
        border-radius: 5px;
    }

    .item {
        flex: 0 0 auto;
        margin: 5px;
    }

    p {
        text-align: left;
    }

    td {
        padding: 5px;
    }

    h2 {
        margin-top: 0;
    }

    p {
        margin: 5px 0px;
    }
</style>
