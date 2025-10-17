use crate::ipc::commands::{ExecCommand, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RetreatOperatorCommand {
    name: String,
    position: (i32, i32),
}

impl ExecCommand for RetreatOperatorCommand {
    fn execute(&self, ctx: &mut crate::ipc::command_context::CommandContext) -> Response {
        if let Some(_) = ctx.operators().write().unwrap().remove(&self.name) {
            Response::Success(format!(
                "retreated operator {} at {:?}",
                self.name, self.position
            ))
        } else {
            Response::Error(format!("operator {} is not loaded", self.name))
        }
    }
}
