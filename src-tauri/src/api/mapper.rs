use crate::api::dto::{ConfigDto, QuestionDataDto, QuestionSceneDto, RoundDto, TopicDto};
use crate::api::dto::{PackInfoDto, PlayerGameDto, QuestionDto};
use crate::core::game_entities::{game, Player};
use crate::game_pack::pack_content_entities::{PackContent, Question, Round};
use std::collections::HashMap;


use crate::hub_comm::hw::hw_hub_manager::discover_serial_ports;

use super::dto::PlayerSetupDto;

/// Takes whole game context and maps to config which contains only required elements
pub fn get_config_dto() -> ConfigDto {
    let context = game();
    let hub_guard = context.get_unlocked_hub();
    let players = context.players.values().cloned().collect();
    ConfigDto {
        available_ports: discover_serial_ports(),
        hub_port: hub_guard.get_hub_address(),
        radio_channel: hub_guard.radio_channel(),
        players: map_players_to_players_setup_dto(&players),
    }
}

pub fn map_players_to_players_setup_dto(players: &Vec<Player>) -> Vec<PlayerSetupDto> {
        players.iter()
        .map(|p| PlayerSetupDto {
            icon: p.icon.clone(),
            isUsed: p.is_used,
            name: p.name.clone(),
            termId: p.term_id,
        })
        .collect()
}

/// Takes whole game context and maps to config which contains only required elements
pub fn update_players(players: &Vec<Player>) {
    let mut context = game();

    context.players = players.iter().fold(HashMap::new(), |mut map, player| {
        map.insert(player.term_id, player.clone());
        map
    });
}

pub fn map_package_to_pack_info_dto(package: &PackContent) -> PackInfoDto {
    let author = match package.info.authors.first() {
        Some(author) => author.name.clone(),
        None => String::new(),
    };

    let num_rounds = package.rounds.len() as i32;
    let num_topics = package
        .rounds
        .iter()
        .map(|round| round.themes.len())
        .sum::<usize>() as i32;
    let num_questions = package
        .rounds
        .iter()
        .flat_map(|round| round.themes.iter())
        .map(|(_, theme)| theme.questions.len())
        .sum::<usize>() as i32;

    let topic_list: Vec<String> = package
        .rounds
        .iter()
        .flat_map(|round| round.themes.values().map(|theme| theme.name.clone()))
        .collect();

    PackInfoDto {
        packName: package.name.clone(),
        packAuthor: author,
        packRounds: num_rounds,
        packTopics: num_topics,
        packQuestions: num_questions,
        packTopicList: topic_list,
    }
}

pub fn map_players_to_player_game_dto(players: &HashMap<u8, Player>) -> Vec<PlayerGameDto> {
    players
        .values()
        .map(|player| PlayerGameDto {
            id: player.term_id as i32,
            playerIconPath: player.icon.clone(),
            playerName: player.name.clone(),
            score: player.stats.score,
            state: player.state.clone(),
        })
        .collect()
}

/// Converts a `Round` struct to a `RoundDto` struct.
///
/// # Arguments
///
/// * `round` - A reference to the `Round` struct to be converted.
///
/// # Returns
///
/// A `RoundDto` struct representing the converted `Round` struct.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use serde::Serialize;
/// use svoyak_tauri_app::api::mapper::map_round_to_dto;
/// use svoyak_tauri_app::game_pack::pack_content_entities::{Round, RoundType, Theme};
///
/// // Assume proper implementations for RoundType and Theme structs.
///
/// let round = Round {
///     name: "1".to_string(),
///     round_type: RoundType::Normal,
///     themes: HashMap::new(),
///     question_count: 30,
///     questions_left: 27,
///     normal_question_count: 29,
///     pip_question_count: 1
/// };
///
/// let round_dto = map_round_to_dto(&round);
/// ```
pub fn map_round_to_dto(round: &Round) -> RoundDto {
    let round_topics: Vec<TopicDto> = round
        .themes
        .values()
        .map(|theme| {
            log::info!("{theme:#?}");
            let mut game_questions: Vec<Question> =
                theme.questions.values().cloned().collect::<Vec<Question>>();
            game_questions.sort_by(|q1, q2| q1.price.cmp(&q2.price));

            let mut questions = Vec::new();
            game_questions.iter().enumerate().for_each(|(i, q)| {
                questions.push(QuestionDto {
                    index: i,
                    price: q.price,
                });
            });

            TopicDto {
                topicName: theme.name.clone(),
                questions,
            }
        })
        .collect();

    RoundDto {
        roundName: round.name.clone(),
        roundType: round.round_type.clone(),
        roundTopics: round_topics,
    }
}

pub fn map_question_to_question_dto(
    topic: String,
    question: Question,
    q_num: i32,
) -> QuestionDataDto {
    QuestionDataDto {
        number: q_num,
        category: topic,
        price: question.price,
        questionType: question.question_type,
        scenario: question
            .scenario
            .iter()
            .map(|a| QuestionSceneDto {
                content: a.content.clone(),
                mediaType: a.atom_type.clone(),
            })
            .collect(),
        answer: question.right_answer.clone(),
    }
}
