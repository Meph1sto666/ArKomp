pub mod commands;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Success(String),
    Error(String),
}
