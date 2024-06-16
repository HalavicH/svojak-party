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
        currentQuestionStore,
        currentRoundStore,
        GameState,
        navTo
    } from "../lib/stores.js";
    import {Views} from "./views.js";
    import SecondaryButton from "../components/generic/SecondaryButton.svelte";
    import {isRunningInTauri} from "../lib/misc.js";
    import HSpacing from "../components/generic/HSpacing.svelte";

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

    async function sleep(time) {
        await new Promise(r => setTimeout(r, time));
    }

    async function initAndLoadAndStart() {
        await openKefLoh();
        await setDemoHub();
        // Sleep for 1 second to let the hub load
        await sleep(2000);
        await startTheGame();
    }

    async function waitForState(desiredState) {
        let gameState;
        currentGameStateStore.subscribe((state) => {
            console.log("Current state: ", state);
            gameState = state.gameState;
        });
        let tries = 10;
        while (gameState !== desiredState && tries-- > 0) {
            notify.info("Current state: " + gameState + ". Waiting for: " + desiredState);
            await sleep(1000);
        }
    }

    async function processQuestionPlaySkip(question, topic) {
        // notify.info(`Playing question: ${question.price}`);
        console.log("Playing question: ", question.price);
        await waitForState(GameState.ChooseQuestion);
        // Select the question
        await callBackend(TauriApiCommand.SELECT_QUESTION, {
            topic: topic.topicName,
            price: question.price
        });
        console.log("Selected question: ", question.price)
        await waitForState(GameState.DisplayQuestion);
        // Allow players to answer
        await callBackend(TauriApiCommand.STOP_ASKING_AND_SHOW_ANSWER);
        await waitForState(GameState.EndQuestion);
        await callBackend(TauriApiCommand.FINISH_QUESTION);
        console.log("Finished question: ", question.price)
    }

    async function processQuestionPlayCorrect(question, topic) {
        // notify.info(`Playing question: ${question.price}`);
        console.log("Playing question: ", question.price);
        await waitForState(GameState.ChooseQuestion);
        // Select the question
        await callBackend(TauriApiCommand.SELECT_QUESTION, {
            topic: topic.topicName,
            price: question.price
        });
        console.log("Selected question: ", question.price)
        await waitForState(GameState.DisplayQuestion);
        // Allow players to answer
        await callBackend(TauriApiCommand.ALLOW_ANSWER);
        console.log("Allowed answer: ", question.price)
        // Wait for answers
        await waitForState(GameState.AnswerAttemptReceived);
        // Press 'correct answer'
        await callBackend(TauriApiCommand.ANSWER_QUESTION, {answeredCorrectly: true});
        console.log("Answered correctly: ", question.price)
        await waitForState(GameState.EndQuestion);
        await callBackend(TauriApiCommand.FINISH_QUESTION);
        console.log("Finished question: ", question.price)
    }

    async function playRound(playHandler) {
        // For each question in the round
        // 1. Select the question
        // 2. Allow players to answer
        // 3. Wait for answers
        // 4. Press 'correct answer'
        // 5. Press next question
        // 6. Repeat
        let round = $currentRoundStore;
        // notify.info(`Playing round: ${round.roundName}`);
        console.log("Playing round: ", round.roundName)
        for (const topic of round.roundTopics) {
            // notify.info(`Playing topic: ${topic.topicName}`);
            console.log("Playing topic: ", topic.topicName)
            for (const question of topic.questions) {
                await playHandler(question, topic);
            }
        }
        notify.info(`Finished round: ${round.roundName}`);
    }
</script>

<BaseModal {isOpen}>
    <h2>Debug Mode</h2>
    <ItemsBlock title="Startup Actions">
        <table>
            <tbody>
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
            <tr>
                <td>
                    <SecondaryButton text="Init + load + start" onClick={initAndLoadAndStart}/>
                </td>
                <td>
                    <p>Starts new game with current config</p>
                </td>
            </tr>
            </tbody>
        </table>
    </ItemsBlock>
    <ItemsBlock title="Gameplay actions">
        <table>
            <tbody>
            <tr>
                <td>
                    <SecondaryButton text="Play round (all correct)" onClick={() => playRound(processQuestionPlayCorrect)}/>
                </td>
                <td><p>Play the round until done (answer correctly)</p></td>
            </tr>
            <tr>
                <td>
                    <SecondaryButton text="Play round (all skip)" onClick={() => playRound(processQuestionPlaySkip)}/>
                </td>
                <td><p>Play the round until done (all skip)</p></td>
            </tr>
            </tbody>
        </table>
    </ItemsBlock>
    <ItemsBlock title="Misc">
        <table>
            <tbody>
            <tr>
                <td>
                    <SecondaryButton text="Reset Game" onClick={resetGame}/>
                </td>
                <td><p>Set's state to SetupAndLoading. Clears players and hub</p></td>
            </tr>
            </tbody>
        </table>
    </ItemsBlock>
    <ItemsBlock title="Go to:">
        <Row>
            <SecondaryButton text="Menu" onClick={goToMenu}/>
            <HSpacing size="1em"/>
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
        <div class="json-view">
            <pre>{renderStoreContent($currentGameStateStore)}</pre>
        </div>

        <p>Question store</p>
        <div class="json-view">
            <pre>{renderStoreContent($currentQuestionStore)}</pre>
        </div>

        <p>Players store</p>
        <div class="json-view">
            <pre>{renderStoreContent($currentPlayersStore)}</pre>
        </div>

        <p>Hub config store</p>
        <div class="json-view">
            <pre>{renderStoreContent($currentHubConfigStore)}</pre>
        </div>

        <p>Pack info store</p>
        <div class="json-view">
            <pre>{renderStoreContent($currentPackInfoStore)}</pre>
        </div>

        <p>Round store</p>
        <div class="json-view">
            <pre>{renderStoreContent($currentRoundStore)}</pre>
        </div>
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
