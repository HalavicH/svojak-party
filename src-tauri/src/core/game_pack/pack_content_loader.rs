use std::path::Path;
#[allow(dead_code, unused, unused_imports)]
use std::{collections::HashMap, error::Error, fmt, fs, io};
use std::io::BufRead;
use error_stack::{IntoReport, Result, ResultExt};
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::Deserialize;
use serde_xml_rs::from_str;
use unic_normal::StrNormalForm;
use urlencoding::encode;

use crate::core::game_pack::game_pack_entites::PackLocationData;
use crate::core::game_pack::game_pack_loader::GamePackLoadingError;
use crate::core::game_pack::pack_content_dto::*;
use crate::core::game_pack::pack_content_dto_v4::{AtomDtoV4, AtomTypeDtoV4, PackageDtoV4, QuestionDtoV4, RoundDtoV4, ThemeDtoV4};
use crate::core::game_pack::pack_content_entities::*;
use crate::host_api::dto::QuestionType;

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

    let package_by_version = parse_package(package_content_file_str)?;
    match package_by_version {
        PackageByVersion::V4(package) => {
            let mut mapped_content = PackContent::from(&package);
            expand_and_validate_package_paths(&mut mapped_content, pack_location_data)?;
            Ok(mapped_content)
        }
        PackageByVersion::V5(package) => todo!("V5 package version is not supported yet")
    }
}

fn parse_package_version<R: BufRead>(reader: R) -> Option<String> {
    let mut reader = Reader::from_reader(reader);
    reader.trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == quick_xml::name::QName(b"package") => {
                for attribute in e.attributes() {
                    match attribute {
                        Ok(attr) if attr.key == quick_xml::name::QName(b"version") => {
                            return attr.unescape_value().ok().map(|v| v.to_string());
                        }
                        _ => (),
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("Error at position {}: {:?}", reader.buffer_position(), e);
                break;
            }
            _ => (),
        }
        buf.clear();
    }
    None
}

fn expand_and_validate_package_paths(
    pack: &mut PackContent,
    locations: &PackLocationData,
) -> Result<(), GamePackLoadingError> {
    let mut result = Ok(());

    pack.rounds.iter_mut().for_each(|r| {
        r.topics.iter_mut().for_each(|(_, theme)| {
            theme.questions.iter_mut().for_each(|(_, q)| {
                q.scenario.iter_mut().for_each(|a| {
                    log::debug!("Atom {:?} before mapping: {}", a.atom_type, a.content);
                    match a.atom_type {
                        QuestionMediaType::Text => {}
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

#[derive(Debug, Deserialize)]
pub struct PackageDtoV5 {}

enum PackageByVersion {
    V4(PackageDtoV4),
    V5(PackageDtoV5),
}

enum PackageVersion {
    V4,
    V5,
}

fn get_package_version(package_xml: &str) -> Result<PackageVersion, GamePackLoadingError> {
    let reader = package_xml.as_bytes();
    let version = parse_package_version(reader)
        .ok_or(GamePackLoadingError::CorruptedPack("Can't parse package version".to_string()))
        .into_report()
        .attach_printable("Can't parse package version. Check package content file")?;
    match version.as_str() {
        "4" => Ok(PackageVersion::V4),
        "5" => Ok(PackageVersion::V5),
        _ => Err(GamePackLoadingError::UnknownVersion(version)).into_report(),
    }
}

fn parse_package(file_path: &str) -> Result<PackageByVersion, GamePackLoadingError> {
    let package_xml = fs::read_to_string(file_path)
        .into_report()
        .attach_printable_lazy(|| format!("Can't open package content file: '{file_path}'"))
        .change_context(GamePackLoadingError::CorruptedPack("Can't open package content file".to_string()))?;

    let version = get_package_version(&package_xml)?;

    match version {
        PackageVersion::V4 => {
            let package_dto = from_str(&package_xml)
                .into_report()
                .attach_printable_lazy(|| format!("Can't parse pack content XML file: '{file_path}'"))
                .change_context(GamePackLoadingError::CorruptedPack("Can't parse pack content XML file".to_string()))?;

            Ok(PackageByVersion::V4(package_dto))
        }
        PackageVersion::V5 => {
            let package_dto: PackageDtoV5 = from_str(&package_xml)
                .into_report()
                .attach_printable_lazy(|| format!("Can't parse pack content XML file: '{file_path}'"))
                .change_context(GamePackLoadingError::CorruptedPack("Can't parse pack content XML file".to_string()))?;

            Ok(PackageByVersion::V5(package_dto))
        }
    }
}
