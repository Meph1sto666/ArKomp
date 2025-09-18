use eframe::egui;
use rusty_spine::controller::SkeletonController;

pub trait OperatorSkin: std::fmt::Debug {
    fn new(
        operator_id: &str,
        skin_id: &str,
        atlas_path: String,
        skeleton_file: SkeletonFile,
        texture: crate::texture::SpineTexture,
    ) -> Self
    where
        Self: Sized;
    fn operator_id(&self) -> String;
    fn id(&self) -> String;
    fn controller_mut(&mut self) -> &mut SkeletonController;
    fn ensure_textures_loaded(&mut self, ctx: &egui::Context);
}

pub enum SkeletonFile {
    Binary(String),
    Json(String),
}
