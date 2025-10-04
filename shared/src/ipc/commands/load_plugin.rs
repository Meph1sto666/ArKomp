use crate::{
    ipc::commands::{Command, Response},
    plugin::{PluginRegistry, types::operator_plugin::OperatorPlugin},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadPluginCommand {
    name: String,
    path: PathBuf,
}

impl Command for LoadPluginCommand {
    fn execute(&self, plugin_registry: &mut PluginRegistry) -> Response {
        match OperatorPlugin::new(self.path.as_path(), self.name.to_owned()) {
            Ok(plugin) => {
                plugin_registry.register_plugin(self.name.clone(), Box::new(plugin));
                debug!("Loaded plugin: {}", self.name);
                Response::Success(format!("Loaded plugin: {}", self.name))
            }
            Err(e) => {
                debug!("Failed to load plugin: {:?}", e);
                Response::Error(format!("Failed to load plugin: {:?}", e))
            }
        }
    }
}
