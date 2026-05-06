use crate::ipc::{
    command_context::CommandContext,
    commands::{ExecCommand, Response},
    events::Event,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct SetClickthroughCommand {
    clickthrough: bool,
}

impl ExecCommand for SetClickthroughCommand {
    fn execute(&self, ctx: &mut CommandContext) -> Response {
        match ctx
            .ui_sender()
            .try_send(Event::SetMousePassthrough(self.clickthrough))
        {
            Ok(_) => Response::Success("".to_string()),
            Err(e) => Response::Error(
                json!({
                    "reason": e.to_string()
                })
                .to_string(),
            ),
        }
    }
}
