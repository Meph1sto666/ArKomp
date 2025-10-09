use crate::{
    ipc::{
        command_context::CommandContext,
        commands::{ExecCommand, Response},
    },
    plugin::types::operator_plugin::OperatorPlugin,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadPluginCommand {
    name: String,
    path: PathBuf,
}

impl ExecCommand for LoadPluginCommand {
    fn execute(&self, ctx: &mut CommandContext) -> Response {
        match OperatorPlugin::new(self.path.as_path(), self.name.to_owned()) {
            Ok(plugin) => {
                ctx.plugin_registry()
                    .write()
                    .unwrap()
                    .register_plugin(self.name.clone(), Box::new(plugin));
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
