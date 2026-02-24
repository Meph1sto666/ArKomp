use crate::{
    ipc::events::Event,
    ipc::{
        command_context::CommandContext,
        commands::{ExecCommand, Response},
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleEventCommand {
    #[serde(flatten)]
    event: Event,
}

impl ExecCommand for ScheduleEventCommand {
    fn execute(&self, ctx: &mut CommandContext) -> Response {
        match ctx.operators().write().unwrap().get_mut(self.event.to()) {
            Some(op) => match op.event_handler(self.event.clone()) {
                Ok(response) => response,
                Err(e) => Response::Error(e.to_string()),
            },
            None => Response::Error(
                json!({
                    "to": self.event.to(),
                    "reason": "Operator not loaded"
                })
                .to_string(),
            ),
        }
    }
}
