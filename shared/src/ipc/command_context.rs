use crate::{events::Event, operator::Operator, plugin::PluginRegistry};
use std::sync::mpsc::Sender;
use std::{collections::HashMap, sync::Arc};
pub struct CommandContext {
    operators: Arc<std::sync::RwLock<HashMap<String, Box<dyn Operator>>>>,
    plugin_registry: Arc<std::sync::RwLock<PluginRegistry>>,
    operator_tx: Sender<Event>,
}

impl CommandContext {
    pub fn new(
        operators: Arc<std::sync::RwLock<HashMap<String, Box<dyn Operator>>>>,
        plugin_registry: Arc<std::sync::RwLock<PluginRegistry>>,
        operator_tx: Sender<Event>,
    ) -> Self {
        Self {
            operators,
            plugin_registry,
            operator_tx,
        }
    }

    pub fn operators(&mut self) -> &Arc<std::sync::RwLock<HashMap<String, Box<dyn Operator>>>> {
        &self.operators
    }

    pub fn plugin_registry(&mut self) -> &Arc<std::sync::RwLock<PluginRegistry>> {
        &self.plugin_registry
    }

    pub fn sender(&self) -> std::sync::mpsc::Sender<Event> {
        self.operator_tx.clone()
    }
}
