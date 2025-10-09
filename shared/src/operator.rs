use crate::skin::OperatorSkin;

pub trait Operator: std::fmt::Debug + Send + Sync {
    fn render(&mut self, ctx: &eframe::egui::Context, ui: &mut eframe::egui::Ui);
    fn id(&self) -> String;
    fn selected_skin(&'_ self) -> Box<dyn OperatorSkin>;
    fn start_animation(&mut self, anim: &str);
    fn update_animation(&mut self, ctx: &eframe::egui::Context);
    fn load_textures(&mut self, ctx: &eframe::egui::Context);
}
