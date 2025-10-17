use crate::ipc::{
    Response,
    command_context::CommandContext,
    commands::{
        load_plugin::LoadPluginCommand, retreat_operator::RetreatOperatorCommand,
        schedule_event::ScheduleEventCommand, spawn_operator::SpawnOperatorCommand,
        unload_plugin::UnloadPluginCommand,
    },
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
mod load_plugin;
mod retreat_operator;
mod schedule_event;
mod spawn_operator;
mod unload_plugin;

pub trait ExecCommand: std::fmt::Debug + Send + Sync {
    fn execute(&self, ctx: &mut CommandContext) -> Response;
}

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    SerdeError(String),
    CommandRegistrationFailed(String),
    CommandDeregistrationFailed(String),
    CommandDoesNotExist(String),
    MutexPoisoned(String),
    ContextInvalid(String),
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "command")]
pub enum Command {
    LoadPlugin(LoadPluginCommand),
    UnloadPlugin(UnloadPluginCommand),
    SpawnOperator(SpawnOperatorCommand),
    ScheduleEvent(ScheduleEventCommand),
    RetreatOperator(RetreatOperatorCommand),
}

impl Command {
    pub fn execute(self, ctx: &mut CommandContext) -> Response {
        match self {
            Command::LoadPlugin(cmd) => cmd.execute(ctx),
            Command::UnloadPlugin(cmd) => cmd.execute(ctx),
            Command::SpawnOperator(cmd) => cmd.execute(ctx),
            Command::ScheduleEvent(cmd) => cmd.execute(ctx),
            Command::RetreatOperator(cmd) => cmd.execute(ctx),
        }
    }

    pub fn from_json(value: &str) -> Result<Self, Error> {
        serde_json::from_str(value).map_err(|e| Error::SerdeError(e.to_string()))
    }

    pub fn execute_from_json(value: &str, ctx: &mut CommandContext) -> Result<Response, Error> {
        let command = Command::from_json(value)?;
        Ok(command.execute(ctx))
    }
}
