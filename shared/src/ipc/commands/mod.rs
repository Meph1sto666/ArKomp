use crate::{
    ipc::{
        Response, commands::load_plugin::LoadPluginCommand,
        commands::unload_plugin::UnloadPluginCommand,
    },
    plugin::PluginRegistry,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex},
};
use tracing::debug;
mod load_plugin;
mod unload_plugin;

pub trait Command: std::fmt::Debug + Send + Sync {
    fn execute(&self, plugin_registry: &mut PluginRegistry) -> Response;
}

pub type CommandFactory =
    Box<dyn Fn(serde_json::Value) -> Result<Box<dyn Command>, Error> + Send + Sync>;

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    SerdeError(String),
    CommandRegistrationFailed(String),
    CommandDeregistrationFailed(String),
    CommandDoesNotExist(String),
    MutexPoisoned(String),
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value.to_string())
    }
}

pub struct CommandRegistry {
    commands: Arc<Mutex<HashMap<String, CommandFactory>>>,
}

impl Debug for CommandRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.commands.lock() {
            Ok(commands) => f
                .debug_struct("CommandRegistry")
                .field(
                    "commands",
                    &Vec::from_iter(commands.iter().map(|(k, _v)| format!("{}", k))),
                )
                .finish(),
            Err(poisoned) => f
                .debug_struct("CommandRegistry")
                .field("commands", &format!("Mutex poisoned: {}", poisoned))
                .finish(),
        }
    }
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn new_with_defaults() -> Result<Self, Error> {
        let mut registry: CommandRegistry = Self {
            commands: Arc::new(Mutex::new(HashMap::new())),
        };
        registry.register_built_in_commands()?;
        Ok(registry)
    }

    pub fn register_built_in_commands(&mut self) -> Result<(), Error> {
        self.register_command::<LoadPluginCommand>("load_plugin".into())?;
        self.register_command::<UnloadPluginCommand>("unload_plugin".into())?;
        Ok(())
    }

    pub fn register_command<C>(&mut self, name: String) -> Result<(), Error>
    where
        C: Command + for<'de> Deserialize<'de> + 'static,
    {
        debug!("Registering command [{}]", name);
        let factory = Box::new(move |args: serde_json::Value| {
            let command: C = match serde_json::from_value(args) {
                Ok(cmd) => cmd,
                Err(e) => {
                    debug!("Failed to build command from message: {}", e);
                    Err(Error::CommandDeregistrationFailed(e.to_string()))?
                }
            };
            Ok(Box::new(command) as Box<dyn Command>)
        });
        self.commands
            .lock()
            .map_err(|e| {
                debug!("{}", e);
                Error::MutexPoisoned(e.to_string())
            })?
            .insert(name.clone(), factory);
        debug!(
            "Successfully registered command [{}] -> [{}]",
            name,
            std::any::type_name::<C>()
        );
        Ok(())
    }

    pub fn execute_command(
        &self,
        name: &str,
        args: serde_json::Value,
        plugin_registry: &Arc<std::sync::Mutex<PluginRegistry>>,
    ) -> Result<Response, Error> {
        let mg = self.commands.lock().map_err(|e| {
            debug!("{}", e);
            Error::MutexPoisoned(e.to_string())
        })?;
        let factory = mg.get(name).ok_or_else(|| {
            debug!("Command [{name}] does not exist");
            Error::CommandDoesNotExist(name.to_string())
        })?;

        let command: Box<dyn Command> = factory(args)?;
        let mut plugin_mg = plugin_registry.lock().map_err(|e| {
            debug!("{}", e);
            Error::MutexPoisoned(e.to_string())
        })?;
        Ok(command.execute(&mut plugin_mg))
    }
}
