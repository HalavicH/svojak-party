export const TauriApiCommand = {
    // Startup API
    SET_HUB_TYPE: 'set_hub_type',
    FETCH_CONFIGURATION: 'fetch_configuration',
    DISCOVER_HUB: 'discover_hub',
    SET_HUB_RADIO_CHANNEL: 'set_hub_radio_channel',
    DISCOVER_PLAYERS: 'discover_players',
    SAVE_PLAYERS: 'save_players',
    GET_PACK_INFO: 'get_pack_info',
    SAVE_ROUND_DURATION: 'save_round_duration',
    START_THE_GAME: 'start_the_game',
    // Debug API
    SETUP_HUB_CONNECTION: 'setup_hub_connection',
    SEND_RAW_REQUEST_FRAME: 'send_raw_request_frame',
    SEND_HUB_COMMAND: 'send_hub_command',
    // Gameplay API
    FETCH_PLAYERS: 'fetch_players',
    FETCH_ROUND: 'fetch_round',
    GET_QUESTION_DATA: 'get_question_data',
    ALLOW_ANSWER: 'allow_answer',
    GET_FASTEST_CLICK: 'get_fastest_click',
    ANSWER_QUESTION: 'answer_question',
    HAS_NEXT_QUESTION: 'has_next_question',
    FINISH_QUESTION_PREMATURELY: 'finish_question_prematurely',
    INIT_NEXT_ROUND: 'init_next_round',
    SEND_PIP_VICTIM: 'send_pip_victim',
    GET_ACTIVE_PLAYER_ID: 'get_active_player_id',
    IS_ALLOW_ANSWER_REQUIRED: 'is_allow_answer_required',
    FETCH_ROUND_STATS: 'fetch_round_stats'
};
export const HubType = {
    HwHub: 'HwHub',
    WebHub: 'WebHub',
}


// Example usage:
// invoke(TauriApiCommand.SET_HUB_TYPE);
