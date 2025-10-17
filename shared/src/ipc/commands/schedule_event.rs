use crate::{
    events::Event,
    ipc::{
        command_context::CommandContext,
        commands::{ExecCommand, Response},
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleEventCommand {
    event: Event,
}

impl ExecCommand for ScheduleEventCommand {
    fn execute(&self, ctx: &mut CommandContext) -> Response {
        ctx.sender().send(self.event.clone()).unwrap();
        Response::Success(format!(
            "Scheduled event {}",
            serde_json::to_string(&self.event).unwrap()
        ))
    }
}
