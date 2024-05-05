use crate::api::dto::{AppContextDto, QuestionDataDto, QuestionSceneDto, RoundDto, TopicDto};
use crate::api::dto::{PackInfoDto, PlayerDto, QuestionDto};
use crate::core::app_context::{app, app_mut, AppContext};
use crate::core::game_entities::Player;
use crate::game_pack::pack_content_entities::{PackContent, Question, Round};
use crate::hub_comm::common::hub_api::HubManager;
use std::collections::HashMap;

use crate::hub_comm::hw::hw_hub_manager::discover_serial_ports;

/// Takes whole game context and maps to config which contains only required elements
pub fn get_app_context_dto() -> AppContextDto {
    let context = app();
    let guard = context.get_unlocked_hub();
    map_app_context(&context, &guard)
}

pub fn map_app_context(context: &AppContext, hub: &Box<dyn HubManager>) -> AppContextDto {
    AppContextDto {
        availablePorts: discover_serial_ports(),
        hubPort: hub.get_hub_address(),
        radioChannel: hub.radio_channel(),
        hubStatus: hub.get_hub_status(),
        players: map_players_to_player_dto(context.players.values().collect()),
    }
}

/// Takes whole game context and maps to config which contains only required elements
pub fn update_players(players: &[Player]) {
    let mut context = app_mut();

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

pub fn map_players_to_player_dto(players: Vec<&Player>) -> Vec<PlayerDto> {
    players
        .iter()
        .map(|&player| PlayerDto {
            id: player.term_id as i32,
            iconPath: player.icon.clone(),
            name: player.name.clone(),
            score: player.stats.score,
            state: player.state.clone(),
            isUsed: player.is_used,
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
/// use svojak_app::api::mapper::map_round_to_dto;
/// use svojak_app::game_pack::pack_content_entities::{Round, Theme};
///
/// // Assume proper implementations for RoundType and Theme structs.
///
/// let round = Round {
///     name: "1".to_string(),
///     round_type: "normal".to_string(),
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
