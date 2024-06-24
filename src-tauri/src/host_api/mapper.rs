use crate::host_api::dto::{
    HubConfigDto, PlayerEndRoundStatsDto, QuestionDto, QuestionSceneDto, RoundDto, RoundStatsDto,
    TopicDto,
};
use crate::host_api::dto::{PackInfoDto, PlayerDto, QuestionBriefDto};
use crate::core::game_entities::Player;
use crate::game_pack::pack_content_entities::{Atom, PackContent, Question, Round, RoundStats};
use crate::hub::hub_api::HubManager;

/// Hub manager
impl From<&Box<dyn HubManager>> for HubConfigDto {
    fn from(hub: &Box<dyn HubManager>) -> Self {
        Self {
            hubPort: hub.hub_address(),
            availablePorts: hub.available_ports(),
            radioChannel: hub.radio_channel(),
            hubStatus: hub.hub_status(),
        }
    }
}

/// Player
impl From<&Player> for PlayerDto {
    fn from(player: &Player) -> Self {
        Self {
            id: player.term_id as i32,
            iconPath: player.icon.clone(),
            name: player.name.clone(),
            isUsed: player.is_used,
            state: player.state.clone(),
            score: player.stats.score,
        }
    }
}

/// Pack content
impl From<&PackContent> for PackInfoDto {
    fn from(package: &PackContent) -> Self {
        let author = match package.info.authors.first() {
            Some(author) => author.name.clone(),
            None => String::new(),
        };

        let num_rounds = package.rounds.len() as i32;
        let num_topics = package
            .rounds
            .iter()
            .map(|round| round.topics.len())
            .sum::<usize>() as i32;
        let num_questions = package
            .rounds
            .iter()
            .flat_map(|round| round.topics.iter())
            .map(|(_, theme)| theme.questions.len())
            .sum::<usize>() as i32;

        let topic_list: Vec<String> = package
            .rounds
            .iter()
            .flat_map(|round| round.topics.values().map(|theme| theme.name.clone()))
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
}

pub fn game_to_round_stats_dto(
    round: &Round,
    stats: &RoundStats,
    players: Vec<Player>,
) -> RoundStatsDto {
    RoundStatsDto {
        roundName: round.name.to_owned(),
        questionsPlayed: round.question_count,
        normalQuestionsPlayed: round.normal_question_count,
        pigInPokeQuestionPlayed: round.pip_question_count,
        totalCorrectAnswers: stats.total_correct_answers,
        totalWrongAnswers: stats.total_wrong_answers,
        totalTries: stats.total_tries,
        roundTimeSec: 666,
        players: players
            .iter()
            .map(|p| PlayerEndRoundStatsDto {
                id: p.term_id as i32,
                name: p.name.to_owned(),
                score: p.stats.score,
                playerIconPath: p.icon.to_owned(),
                totalAnswers: p.stats.total_tries,
                answeredCorrectly: p.stats.answered_correctly,
                answeredWrong: p.stats.answered_wrong,
            })
            .collect(),
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

// QuestionDto

impl From<&Round> for RoundDto {
    fn from(round: &Round) -> Self {
        let round_topics: Vec<TopicDto> = round
            .topics
            .values()
            .map(|theme| {
                let mut game_questions: Vec<Question> = theme.questions.values().cloned().collect();

                game_questions.sort_by(|q1, q2| q1.price.cmp(&q2.price));

                TopicDto {
                    topicName: theme.name.clone(),
                    questions: game_questions
                        .iter()
                        .enumerate()
                        .map(|(i, q)| QuestionBriefDto {
                            index: i,
                            price: q.price,
                        })
                        .collect(),
                }
            })
            .collect();

        RoundDto {
            roundName: round.name.clone(),
            roundType: round.round_type.clone(),
            roundTopics: round_topics,
        }
    }
}

impl From<&Question> for QuestionDto {
    fn from(question: &Question) -> Self {
        Self {
            number: -1,
            category: question.topic.clone(),
            price: question.price,
            questionType: question.question_type.clone(),
            scenario: question.scenario.iter().map(|a| a.into()).collect(),
            answer: question.correct_answer.clone(),
        }
    }
}

impl From<&Atom> for QuestionSceneDto {
    fn from(atom: &Atom) -> Self {
        QuestionSceneDto {
            content: atom.content.clone(),
            mediaType: atom.atom_type.clone(),
        }
    }
}
