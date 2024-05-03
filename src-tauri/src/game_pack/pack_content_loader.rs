use std::path::Path;
#[allow(dead_code, unused, unused_imports)]
use std::{collections::HashMap, error::Error, fmt, fs, io};

use error_stack::{IntoReport, Result, ResultExt};
use serde_xml_rs::from_str;
use unic_normal::StrNormalForm;
use urlencoding::encode;

use crate::api::dto::QuestionType;
use crate::game_pack::game_pack_entites::PackLocationData;
use crate::game_pack::game_pack_loader::GamePackLoadingError;
use crate::game_pack::pack_content_dto::*;
use crate::game_pack::pack_content_entities::*;

#[derive(Debug)]
pub struct ParsePackContentError;

impl fmt::Display for ParsePackContentError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to parse content: invalid pack content")
    }
}

impl Error for ParsePackContentError {}

pub fn load_pack_content(
    pack_location_data: &PackLocationData,
) -> Result<PackContent, GamePackLoadingError> {
    let package_content_file_str = pack_location_data
        .content_file_path
        .to_str()
        .ok_or(GamePackLoadingError::InvalidPathToPack(
            "Can't get content file path".to_string(),
        ))
        .into_report()
        .attach_printable("Can't get content file path. Check pack location data validity")?;

    let package: PackageDto = parse_package(package_content_file_str)
        .change_context(GamePackLoadingError::CorruptedPack(
            "Can't parse package".to_string(),
        ))
        .attach_printable_lazy(|| "Can't load pack content: parsing failed".to_string())?;

    let mut mapped_content = map_package(package);
    expand_and_validate_package_paths(&mut mapped_content, pack_location_data)?;
    Ok(mapped_content)
}

fn expand_and_validate_package_paths(
    pack: &mut PackContent,
    locations: &PackLocationData,
) -> Result<(), GamePackLoadingError> {
    let mut result = Ok(());

    pack.rounds.iter_mut().for_each(|r| {
        r.themes.iter_mut().for_each(|(_, theme)| {
            theme.questions.iter_mut().for_each(|(_, q)| {
                q.scenario.iter_mut().for_each(|a| {
                    log::debug!("Atom {:?} before mapping: {}", a.atom_type, a.content);
                    match a.atom_type {
                        QuestionMediaType::Say => {}
                        QuestionMediaType::Voice => {
                            a.content = locations
                                .audio_path
                                .join(to_url_filename(a))
                                .to_str()
                                .unwrap_or_default()
                                .to_owned();
                        }
                        QuestionMediaType::Video => {
                            a.content = locations
                                .video_path
                                .join(to_url_filename(a))
                                .to_str()
                                .unwrap_or_default()
                                .to_owned()
                        }
                        QuestionMediaType::Marker => {}
                        QuestionMediaType::Image => {
                            a.content = locations
                                .images_path
                                .join(to_url_filename(a))
                                .to_str()
                                .unwrap_or_default()
                                .to_owned()
                        }
                    }
                    log::debug!("Atom {:?} after mapping: {}", a.atom_type, a.content);
                    if is_atom_media(&a.atom_type) && !Path::new(&a.content).exists() {
                        let err_msg = format!(
                            "Atom corrupted! Round: {}, theme: {}, question: {}, atom {:?}",
                            r.name, theme.name, q.price, a
                        );
                        log::error!("{}", err_msg);
                        result = Err(GamePackLoadingError::CorruptedPack(err_msg.clone()))
                            .into_report()
                            .attach_printable(err_msg);
                    }
                })
            })
        })
    });

    result
}

fn is_atom_media(qmt: &QuestionMediaType) -> bool {
    *qmt == QuestionMediaType::Image
        || *qmt == QuestionMediaType::Voice
        || *qmt == QuestionMediaType::Video
}

fn to_url_filename(a: &mut Atom) -> String {
    let orig_path = &a.content[1..].to_owned();

    let normalized_filename = orig_path.nfkd().collect::<String>();

    encode(&normalized_filename).to_string()
}

fn parse_package(file_path: &str) -> Result<PackageDto, ParsePackContentError> {
    let package_xml = fs::read_to_string(file_path)
        .into_report()
        .attach_printable_lazy(|| format!("Can't open package content file: '{file_path}'"))
        .change_context(ParsePackContentError)?;

    let package_dto = from_str(&package_xml)
        .into_report()
        .attach_printable_lazy(|| format!("Can't parse pack content XML file: '{file_path}'"))
        .change_context(ParsePackContentError)?;

    Ok(package_dto)
}

fn map_package(dto: PackageDto) -> PackContent {
    PackContent {
        name: dto.name,
        version: dto.version,
        id: dto.id,
        restriction: dto.restriction,
        date: dto.date,
        difficulty: dto.difficulty,
        info: Info {
            authors: {
                dto.info
                    .authors
                    .iter()
                    .map(|a| Author {
                        name: a.name.clone(),
                    })
                    .collect::<Vec<Author>>()
            },
        },
        rounds: {
            dto.rounds
                .rounds_list
                .iter()
                .map(map_round)
                .collect::<Vec<Round>>()
        },
    }
}

fn map_atoms(a: &AtomDto) -> Atom {
    Atom {
        atom_type: {
            match a.r#type {
                AtomTypeDto::say => QuestionMediaType::Say,
                AtomTypeDto::voice => QuestionMediaType::Voice,
                AtomTypeDto::video => QuestionMediaType::Video,
                AtomTypeDto::marker => QuestionMediaType::Marker,
                AtomTypeDto::image => QuestionMediaType::Image,
            }
        },
        content: a.content.clone(),
    }
}

fn map_question(q: &QuestionDto) -> Question {
    Question {
        price: q.price,
        scenario: {
            q.scenario
                .atoms_list
                .iter()
                .map(map_atoms)
                .collect::<Vec<Atom>>()
        },
        right_answer: q.right.answer.clone(),
        // TODO: Set random pip
        question_type: QuestionType::Normal,
    }
}

fn map_theme(t: &ThemeDto) -> (String, Theme) {
    (
        t.name.clone(),
        Theme {
            name: t.name.clone(),
            questions: {
                t.questions
                    .questions_list
                    .iter()
                    .map(|q| (q.price, { map_question(q) }))
                    .collect::<HashMap<i32, Question>>()
            },
        },
    )
}

fn map_round(r: &RoundDto) -> Round {
    let mut round = Round {
        name: r.name.clone(),
        round_type: r.r#type.clone(),
        themes: {
            r.themes
                .themes_list
                .iter()
                .map(map_theme)
                .collect::<HashMap<String, Theme>>()
        },
        questions_left: -1,
        question_count: -1,
        normal_question_count: -1,
        pip_question_count: -1,
    };
    let vec = Vec::from_iter(round.themes.values());
    round.question_count = vec
        .iter()
        .map(|&theme| theme.questions.len() as i32)
        .sum::<i32>();

    round.questions_left = round.question_count;
    round.normal_question_count = round.question_count;
    round.pip_question_count = 0;
    round
}
