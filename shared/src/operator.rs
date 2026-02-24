use crate::ipc::Response;

pub trait Operator: std::fmt::Debug + Send + Sync {
    fn render(&mut self, ctx: &eframe::egui::Context, ui: &mut eframe::egui::Ui);
    fn id(&self) -> String;
    fn start_animation(&mut self, anim: &str) -> Result<(), Error>;
    fn update_animation(&mut self, ctx: &eframe::egui::Context);
    fn load_textures(&mut self, ctx: &eframe::egui::Context);
    fn event_handler(&mut self, event: crate::ipc::events::Event) -> Result<Response, Error>;
    fn infos(&self) -> String;
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    SkinError(crate::skin::Error),
    InvalidOperatorId(String),
    Other(String),
}

impl From<crate::skin::Error> for Error {
    fn from(value: crate::skin::Error) -> Self {
        Self::SkinError(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SkinError(error) => write!(f, "{}", error),
            Error::InvalidOperatorId(error) => write!(f, "{}", error),
            Error::Other(error) => write!(f, "{}", error),
        }
    }
}
