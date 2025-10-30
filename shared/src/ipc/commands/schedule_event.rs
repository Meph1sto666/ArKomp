use crate::{
    events::Event,
    ipc::{
        command_context::CommandContext,
        commands::{ExecCommand, Response},
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleEventCommand {
    event: Event,
}

impl ExecCommand for ScheduleEventCommand {
    fn execute(&self, ctx: &mut CommandContext) -> Response {
        match ctx
            .operators()
            .write()
            .unwrap()
            .get_mut(self.event.operator_id())
        {
            Some(op) => {
                return match op.event_handler(self.event.clone()) {
                    Ok(response) => response,
                    Err(e) => Response::Error(e.to_string()),
                };
            }
            None => Response::Error(
                json!({
                    "operator_id": self.event.operator_id(),
                    "reason": "Operator not loaded"
                })
                .to_string(),
            ),
        }
    }
}
