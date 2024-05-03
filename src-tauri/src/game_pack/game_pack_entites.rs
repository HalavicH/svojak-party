use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;

use crate::game_pack::pack_content_entities::PackContent;

pub static PACKAGE_AUDIO_DIR_NAME: &str = "Audio";
pub static PACKAGE_CONTENT_FILE_NAME: &str = "content.xml";
pub static PACKAGE_IMAGES_DIR_NAME: &str = "Images";
pub static PACKAGE_VIDEO_DIR_NAME: &str = "Video";

pub struct PackLoaderError {}

#[derive(Default, Debug, Clone)]
pub struct PackLocationData {
    // !warning: if you lose this pointer, temp directory will be deleted
    pub base_dir: Option<Arc<TempDir>>,
    pub content_file_path: PathBuf,
    pub video_path: PathBuf,
    pub images_path: PathBuf,
    pub audio_path: PathBuf,
}

#[derive(Default, Debug, Clone)]
pub struct GamePack {
    pub location: PackLocationData,
    pub content: PackContent,
}
