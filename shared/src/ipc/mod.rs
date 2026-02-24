pub mod command_context;
pub mod commands;
pub mod events;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SocketMessage {
    Response(Response),
    Event(events::Event),
}

impl From<Response> for SocketMessage {
    fn from(res: Response) -> Self {
        Self::Response(res)
    }
}

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
