use crate::{ipc::events::Event, operator::Operator, plugin::PluginRegistry};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc::Sender;

pub struct CommandContext {
    operators: Arc<std::sync::RwLock<HashMap<String, Box<dyn Operator>>>>,
    plugin_registry: Arc<std::sync::RwLock<PluginRegistry>>,
    operator_tx: Sender<Event>,
    ui_tx: Sender<Event>,
}

impl CommandContext {
    pub fn new(
        operators: Arc<std::sync::RwLock<HashMap<String, Box<dyn Operator>>>>,
        plugin_registry: Arc<std::sync::RwLock<PluginRegistry>>,
        operator_tx: Sender<Event>,
        ui_tx: Sender<Event>,
    ) -> Self {
        Self {
            operators,
            plugin_registry,
            operator_tx,
            ui_tx,
        }
    }

    pub fn operators(&mut self) -> &Arc<std::sync::RwLock<HashMap<String, Box<dyn Operator>>>> {
        &self.operators
    }

    pub fn plugin_registry(&mut self) -> &Arc<std::sync::RwLock<PluginRegistry>> {
        &self.plugin_registry
    }

    pub fn sender(&self) -> tokio::sync::mpsc::Sender<Event> {
        self.operator_tx.clone()
    }
    pub fn ui_sender(&self) -> tokio::sync::mpsc::Sender<Event> {
        self.ui_tx.clone()
    }
}
