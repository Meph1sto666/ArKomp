use crate::{
    ipc::commands::{Command, Response},
    plugin::PluginRegistry,
};
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnloadPluginCommand {
    name: String,
}

impl Command for UnloadPluginCommand {
    fn execute(&self, plugin_registry: &mut PluginRegistry) -> Response {
        match plugin_registry.deregister_plugin(&self.name) {
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
                Response::Error(format!("Failed to unload plugin: {}", e))
            }
        }
    }
}
