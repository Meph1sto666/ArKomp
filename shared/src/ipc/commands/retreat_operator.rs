use crate::ipc::commands::{ExecCommand, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct RetreatOperatorCommand {
    name: String,
}

impl ExecCommand for RetreatOperatorCommand {
    fn execute(&self, ctx: &mut crate::ipc::command_context::CommandContext) -> Response {
        match ctx.operators().write().unwrap().remove(&self.name) {
            Some(_) => Response::Success(
                json!({
                    "operator_id": self.name,
                })
                .to_string(),
            ),
            None => Response::Error(
                json!({
                    "operator_id": self.name,
                    "reason": "Operator not loaded"
                })
                .to_string(),
            ),
        }
    }
}
