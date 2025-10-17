use tracing::debug;

use crate::{
    operator::Operator,
    plugin::{Error, Plugin, PluginLibrary},
};
use std::path::Path;

#[derive(Debug)]
pub struct OperatorPlugin {
    library: PluginLibrary,
    name: String,
}

impl OperatorPlugin {
    pub fn new(path: &Path, name: String) -> Result<Self, Error> {
        let library = match PluginLibrary::new(path) {
            Ok(v) => v,
            Err(e) => {
                debug!(
                    "Failed to load library {}@[{}]: {}",
                    name,
                    path.display(),
                    e
                );
                Err(e)?
            }
        };
        Ok(Self { library, name })
    }

    pub fn build(&self, id: Option<String>) -> Result<Box<dyn Operator>, Error> {
        let constructor: libloading::Symbol<fn(Option<String>) -> Box<dyn Operator>> =
            match self.library.load_symbol(b"new") {
                Ok(v) => v,
                Err(e) => {
                    debug!(
                        "Failed to load symbol 'new' in {}@[{}]",
                        self.name,
                        self.library.path().display()
                    );
                    Err(e)?
                }
            };
        Ok(constructor(id))
    }
}

impl Plugin for OperatorPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
