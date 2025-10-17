use std::path::PathBuf;
use eframe::egui;
use serde::{Deserialize, Serialize};

pub trait OperatorSkin: std::fmt::Debug {
    fn new(operator_id: &str, skin_id: Option<&str>) -> Result<Self, Error>
    where
        Self: Sized;
    fn operator_id(&self) -> String;
    fn id(&self) -> Option<String>;
    fn ensure_textures_loaded(&mut self, ctx: &egui::Context);
}

pub enum SkeletonFile {
    Binary(PathBuf),
    Json(PathBuf),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    AtlasFileNotFound(String),
    SkeletonFileNotFound(String),
    TextureFileNotFound(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AtlasFileNotFound(e) => write!(f, "Atlas error: {}", e),
            Error::SkeletonFileNotFound(e) => write!(f, "Skeleton error: {}", e),
            Error::TextureFileNotFound(e) => write!(f, "Texture error: {}", e),
        }
    }
}
