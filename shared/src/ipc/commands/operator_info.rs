use crate::ipc::commands::{ExecCommand, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct OperatorInfoCommand {
    name: String,
}

impl ExecCommand for OperatorInfoCommand {
    fn execute(&self, ctx: &mut crate::ipc::command_context::CommandContext) -> Response {
        if let Some(o) = ctx.operators().write().unwrap().get(&self.name) {
            let animations = o.infos();
            Response::Success(animations)
        } else {
            Response::Error(
                json!({
                    "operator_id": self.name,
                    "reason": "Operator not loaded"
                })
                .to_string(),
            )
        }
    }
}
