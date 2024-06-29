use crate::core::game_pack::game_pack_entites::*;
use crate::core::game_pack::pack_content_entities::{Atom, PackContent, QuestionMediaType};
use crate::core::game_pack::pack_content_loader::load_pack_content;
use error_stack::{bail, IntoReport, Report, report, Result, ResultExt};
use serde::Serialize;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use tauri::api::path::home_dir;
use unic_normal::StrNormalForm;
use urlencoding::decode;
use uuid::Uuid;
use zip::ZipArchive;

#[derive(Debug, Clone, Serialize, thiserror::Error)]
pub enum GamePackLoadingError {
    #[error("Invalid path to media element: {0}")]
    InvalidPathToMediaElement(String),
    #[error("Invalid path to pack: {0}")]
    InvalidPackFileExtension(String),
    #[error("Corrupted pack: {0}")]
    InvalidPathToPack(String),
    #[error("Corrupted pack: {0}")]
    CorruptedPack(String),
    #[error("Unknown version: {0}")]
    UnknownVersion(String),
    #[error("Internal error")]
    InternalError,
}

/// Accepts path to pack, returns Result with GamePack or GamePackLoadingError
pub fn load_game_pack(game_archive_path: &str) -> Result<GamePack, GamePackLoadingError> {
    validate_pack_path(game_archive_path)?;

    // let temp_dir = create_temp_directory()
    //     .change_context(GamePackLoadingError::InternalError)
    //     .attach_printable("Can't create temp directory")?;
    let home = home_dir().expect("Expected home directory");
    let temp_dir_path = home.join("svoyak").join("siq_temp");
    //  Create  directory if it doesn't exist
    if !temp_dir_path.exists() {
        fs::create_dir_all(&temp_dir_path).expect("Failed to create temp directory");
    }

    let tmp_dir_path_str = temp_dir_path
        .to_str()
        .ok_or(GamePackLoadingError::InternalError)?;

    unarchive_zip(game_archive_path, tmp_dir_path_str)?;

    let locations = PackLocationData {
        base_dir: Some(temp_dir_path.clone()),
        content_file_path: temp_dir_path.join(PACKAGE_CONTENT_FILE_NAME),
        audio_path: temp_dir_path.join(PACKAGE_AUDIO_DIR_NAME),
        images_path: temp_dir_path.join(PACKAGE_IMAGES_DIR_NAME),
        video_path: temp_dir_path.join(PACKAGE_VIDEO_DIR_NAME),
    };

    normalize_pack_entities_filenames(&locations)?;

    let err_message = format!("Can't load pack {game_archive_path}");
    let mut game_package = load_pack_content(&locations)
        .change_context(GamePackLoadingError::CorruptedPack(err_message.clone()))
        .attach_printable(err_message)?;

    // simplify_pack_assets_paths(&locations, &mut game_package)?;
    Ok(GamePack {
        location: locations,
        content: game_package,
    })
}

impl QuestionMediaType {
    fn get_media_dir<'a>(&'a self, locations: &'a PackLocationData) -> Option<&Path> {
        match self {
            QuestionMediaType::Voice => Some(&locations.audio_path),
            QuestionMediaType::Video => Some(&locations.video_path),
            QuestionMediaType::Image => Some(&locations.images_path),
            _ => None,
        }
    }
}

fn simplify_pack_assets_paths(locations: &PackLocationData, pack_content: &mut PackContent) -> error_stack::Result<(), GamePackLoadingError> {
    // generate uuid for each filename, rename the file and update the path in the pack content
    pack_content.rounds.iter_mut().try_for_each(|r| {
        r.topics.iter_mut().try_for_each(|(_, theme)| {
            theme.questions.iter_mut().try_for_each(|(_, q)| {
                q.scenario.iter_mut().try_for_each(|a| {
                    try_rename(locations, a)
                })
            })
        })
    })
}

fn try_rename(locations: &PackLocationData, a: &mut Atom) -> error_stack::Result<(), GamePackLoadingError> {
    let media_dir = a.atom_type.get_media_dir(locations);
    let Some(media_dir) = media_dir else {
        return Ok(());
    };

    let file_name = Uuid::new_v4().to_string()
        + "."
        + Path::new(&a.content)
        .extension()
        .expect("Expected file extension")
        .to_str()
        .unwrap();
    let new_path = media_dir.join(file_name);
    log::debug!(
            "Renaming file: {} -> {}",
            a.content,
            new_path.to_str().unwrap()
            );
    fs::rename(&a.content, &new_path)
        .into_report()
        .attach_printable_lazy(|| {
            format!(
                "Failed to rename file: {} -> {}",
                a.content,
                new_path.to_str().unwrap()
            )
        })
        .change_context(GamePackLoadingError::InternalError)?;
    a.content = new_path.to_str().unwrap().to_owned();
    Ok(())
}

fn normalize_pack_entities_filenames(
    locations: &PackLocationData,
) -> Result<(), GamePackLoadingError> {
    let image_dir_path = locations
        .images_path
        .to_str()
        .ok_or(Report::new(GamePackLoadingError::InternalError))?;
    normalize_filenames_in_dir(image_dir_path)?;

    let video_dir_path = locations
        .video_path
        .to_str()
        .ok_or(Report::new(GamePackLoadingError::InternalError))?;
    normalize_filenames_in_dir(video_dir_path)?;

    let audio_dir_path = locations
        .audio_path
        .to_str()
        .ok_or(Report::new(GamePackLoadingError::InternalError))?;
    normalize_filenames_in_dir(audio_dir_path)?;
    Ok(())
}

fn normalize_filenames_in_dir(dir_path: &str) -> Result<(), GamePackLoadingError> {
    log::info!("Normalizing names for {}", dir_path);

    let Ok(files) = fs::read_dir(dir_path) else {
        log::info!("It seems that dir {dir_path} doesn't exist. Nothing to normalize");
        return Ok(());
    };

    for file in files {
        let file_entry = file
            .into_report()
            .change_context(GamePackLoadingError::InternalError)?;
        let file_path = file_entry.path();

        // Get the filename from the file path
        let filename = file_path
            .file_name()
            .ok_or(Report::new(GamePackLoadingError::InternalError))?
            .to_string_lossy()
            .to_string();
        log::debug!("Normalizing name for:  {}", filename);

        // Decode the partially encoded URI
        let decoded_filename = decode(&filename)
            .into_report()
            .change_context(GamePackLoadingError::InternalError)?;
        log::debug!("Decoded name:          {}", decoded_filename);

        // Normalize the UTF-8 string
        let normalized_filename = decoded_filename.nfkd().collect::<String>();
        log::debug!("UTF-8 normalized name: {}", normalized_filename);

        // Encode the fully normalized filename
        let encoded_filename = urlencoding::encode(&normalized_filename);
        log::debug!("Encoded name:          {}", encoded_filename);

        // Create the new file path with the encoded filename
        let new_file_path = Path::new(dir_path).join(encoded_filename.as_ref());
        log::debug!(
            "New file path: {}",
            new_file_path
                .to_str()
                .ok_or(GamePackLoadingError::InternalError)?
        );

        // Rename the file
        fs::rename(&file_path, &new_file_path)
            .into_report()
            .change_context(GamePackLoadingError::InternalError)?;
    }

    Ok(())
}

fn validate_pack_path(game_archive_path: &str) -> Result<(), GamePackLoadingError> {
    let file_exists = std::path::Path::new(game_archive_path).exists();
    if !file_exists {
        let err_msg = format!("No pack found at: {}", game_archive_path);
        log::error!("{}", err_msg);
        return Err(Report::new(GamePackLoadingError::InvalidPathToPack(
            game_archive_path.to_string(),
        ))
            .attach_printable(err_msg));
    }

    if !game_archive_path.ends_with(".siq") {
        let file_name = game_archive_path.split('/').last().unwrap_or_default();
        let err_msg = format!(
            "Provided file doesn't have '.siq' file extension. Yot file: {}",
            file_name
        );
        log::error!("{}", err_msg);
        return Err(Report::new(GamePackLoadingError::InvalidPackFileExtension(
            game_archive_path.to_string(),
        ))
            .attach_printable(err_msg));
    }

    Ok(())
}

fn unarchive_zip(archive_path: &str, directory_path: &str) -> Result<(), GamePackLoadingError> {
    let file = fs::File::open(archive_path)
        .into_report()
        .change_context(GamePackLoadingError::InternalError)
        .attach_printable("Failed to open pack archive file")?;

    let mut archive = ZipArchive::new(file)
        .into_report()
        .attach_printable(format!("Failed to read archive {archive_path:?}"))
        .change_context(GamePackLoadingError::InternalError)?;

    archive
        .extract(directory_path)
        .into_report()
        .attach_printable("Failed to unpack archive")
        .change_context(GamePackLoadingError::InternalError)?;

    Ok(())
}
