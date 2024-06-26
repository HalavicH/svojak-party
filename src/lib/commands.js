import {invoke} from "@tauri-apps/api/tauri";
import {isRunningInTauri} from "./misc.js";

export async function callBackend(apiCommand, params) {
    if (!isRunningInTauri()) {
        console.warn(`No Tauri context!\nSkipping Tauri API call: '${apiCommand}' with payload: '${JSON.stringify(params)}'`);
        return;
    }
    console.log(`Calling: ${apiCommand} with params`, params)
    return await invoke(apiCommand, params)
}

export const TauriApiCommand = {
    // Window setup api
    INIT_WINDOW_HANDLE: 'init_window_handle',
    REQUEST_CONTEXT_UPDATE: 'request_context_update',
    IS_DEBUG_MODE: 'is_debug_mode',

    // Startup API
    SET_HUB_TYPE: 'set_hub_type',
    DISCOVER_HUB: 'discover_hub',
    SET_HW_HUB_RADIO_CHANNEL: 'set_hw_hub_radio_channel',
    SAVE_PLAYERS: 'save_players',
    INIT_GAME_PACK: 'init_game_pack',
    START_NEW_GAME: 'start_new_game',

    // Gameplay API
    SELECT_QUESTION: 'select_question',
    ALLOW_ANSWER: 'allow_answer',
    ANSWER_QUESTION: 'answer_question',
    STOP_ASKING_AND_SHOW_ANSWER: 'stop_asking_and_show_answer',
    FINISH_QUESTION: 'finish_question',
    INIT_NEXT_ROUND: 'init_next_round',
    SEND_PIP_VICTIM: 'send_pip_victim',
    GET_ACTIVE_PLAYER_ID: 'get_active_player_id',
    IS_ALLOW_ANSWER_REQUIRED: 'is_allow_answer_required',
    FETCH_ROUND_STATS: 'fetch_round_stats',
    FINISH_GAME: 'finish_game',
    RESET_GAME: 'reset_game',

    // Debug API
    DBG_SET_GAME_STATE: 'dbg_set_game_state',

    /////////// LEGACY API ////////////
    SETUP_HUB_CONNECTION: 'setup_hub_connection',
    SEND_RAW_REQUEST_FRAME: 'send_raw_request_frame',
    SEND_HUB_COMMAND: 'send_hub_command',
    EDIT_PLAYER_SCORE: 'edit_player_score'
};

export const HubType = {
    HwHub: 'HwHub',
    WebHub: 'WebHub',
}

export const HubStatusOptions = {
    Detected: 'Detected',
    NoDevice: 'NoDevice',
}

export const HubManagerError = {
    ApiNotSupported: 'ApiNotSupported',
    NotInitializedError: 'NotInitializedError',
    SerialPortError: 'SerialPortError',
    HttpCommunicationError: 'HttpCommunicationError',
    NoResponseFromHub: 'NoResponseFromHub',
    NoResponseFromTerminal: 'NoResponseFromTerminal',
    InternalError: 'InternalError',
};

export function hubManagerError2Msg(err) {
    if (HubManagerError.ApiNotSupported === err) return 'Api not supported for this type of HUB';
    if (HubManagerError.NotInitializedError === err) return 'Hub is not initialized';
    if (HubManagerError.SerialPortError === err) return 'Serial port error';
    if (HubManagerError.HttpCommunicationError === err) return 'HTTP communication error';
    if (HubManagerError.NoResponseFromHub === err) return 'No response from hub';
    if (HubManagerError.NoResponseFromTerminal === err) return 'No response from terminal';
    if (HubManagerError.InternalError === err) return 'Internal error';
    return err;
}
