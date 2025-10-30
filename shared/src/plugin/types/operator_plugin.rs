use crate::{
    events::Event,
    operator::Operator,
    plugin::{Error, Plugin, PluginLibrary},
};
use std::{ffi::CString, path::Path};
use tracing::{debug, error};

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

    pub fn build(
        &self,
        id: Option<String>,
        event_tx: tokio::sync::mpsc::Sender<Event>,
    ) -> Result<Box<dyn Operator>, Error> {
        let constructor: libloading::Symbol<
            fn(
                *const std::ffi::c_char,
                *const std::ffi::c_void,
            ) -> Result<Box<dyn Operator>, crate::operator::Error>,
        > = match self.library.load_symbol(b"new") {
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

        let op_id = id.map(|s| CString::new(s).unwrap());
        let id_ptr = op_id.as_ref().map_or(std::ptr::null(), |s| s.as_ptr());

        let event_tx_ptr: *const std::ffi::c_void =
            &event_tx as *const _ as *const std::ffi::c_void;
        Ok(constructor(id_ptr, event_tx_ptr).map_err(|e| {
            error!("{:?}", e);
            Error::Other(e.to_string())
        })?)
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
