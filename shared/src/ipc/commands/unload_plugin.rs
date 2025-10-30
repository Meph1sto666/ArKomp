use crate::ipc::{
    command_context::CommandContext,
    commands::{ExecCommand, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnloadPluginCommand {
    name: String,
}

impl ExecCommand for UnloadPluginCommand {
    fn execute(&self, ctx: &mut CommandContext) -> Response {
        match ctx
            .plugin_registry()
            .write()
            .unwrap()
            .deregister_plugin(&self.name)
        {
            Ok(plugin) => {
                debug!("Unloaded plugin: {} / {}", self.name, plugin.name());
                Response::Success(format!(
                    "Unloaded plugin: {} / {}",
                    self.name,
                    plugin.name()
                ))
            }
            Err(e) => {
                debug!("Failed to unload plugin: {}", e);
                Response::Error(
                    json!({
                        "plugin": self.name,
                        "reason": "Operator not loaded"
                    })
                    .to_string(),
                )
            }
        }
    }
}
