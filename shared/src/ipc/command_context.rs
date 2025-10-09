use std::{collections::HashMap, sync::Arc};

use crate::{operator::Operator, plugin::PluginRegistry};

#[derive(Default)]
pub struct CommandContext {
    operators: Arc<std::sync::RwLock<HashMap<String, Box<dyn Operator>>>>,
    plugin_registry: Arc<std::sync::RwLock<PluginRegistry>>,
}

impl CommandContext {
    pub fn new(
        operators: Arc<std::sync::RwLock<HashMap<String, Box<dyn Operator>>>>,
        plugin_registry: Arc<std::sync::RwLock<PluginRegistry>>,
    ) -> Self {
        Self {
            operators,
            plugin_registry,
        }
    }

    pub fn operators(&mut self) -> &Arc<std::sync::RwLock<HashMap<String, Box<dyn Operator>>>> {
        &mut self.operators
    }

    pub fn plugin_registry(&mut self) -> &Arc<std::sync::RwLock<PluginRegistry>> {
        &self.plugin_registry
    }
}
