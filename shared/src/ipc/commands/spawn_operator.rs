use crate::{
    ipc::commands::{ExecCommand, Response},
    plugin::types::operator_plugin::OperatorPlugin,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SpawnOperatorCommand {
    name: String,
    position: (i32, i32),
}

impl ExecCommand for SpawnOperatorCommand {
    fn execute(&self, ctx: &mut crate::ipc::command_context::CommandContext) -> Response {
        let build_result = {
            let binding = ctx.plugin_registry().read().unwrap();
            let plugin = crate::plugin::cast_plugin_to::<OperatorPlugin>(
                binding.get_plugin(&self.name).unwrap(),
            )
            .unwrap();
            plugin.build(Some(self.name.clone()))
        };

        match build_result {
            Ok(v) => {
                ctx.operators()
                    .write()
                    .unwrap()
                    .insert(self.name.clone(), v);
                Response::Success(format!("spawned operator {}", self.name))
            }
            Err(_) => Response::Error(format!("failed to spawn operator {}", self.name)),
        }
    }
}
