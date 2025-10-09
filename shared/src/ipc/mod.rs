pub mod commands;
pub mod command_context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Success(String),
    Error(String),
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(m) => write!(f, "{}", m),
            Self::Error(m) => write!(f, "{}", m),
        }
    }
}
