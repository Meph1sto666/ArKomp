// shared/plugin/mod.rs
pub mod types;
use libloading::{Library, Symbol};
use std::{
    any::Any,
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
};
use tracing::debug;

pub trait Plugin: fmt::Debug + Send + Sync + Any {
    fn name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

pub fn cast_plugin_to<P: Plugin>(plugin: &dyn Plugin) -> Result<&P, Error> {
    Ok(plugin.as_any().downcast_ref::<P>().ok_or_else(|| {
        debug!(
            "Failed attempted downcast of {:?} into {}",
            plugin,
            std::any::type_name::<P>()
        );
        Error::UnsupportedCast(format!(
            "Failed attempted downcast of {:?} into {}",
            plugin,
            std::any::type_name::<P>()
        ))
    })?)
}

#[derive(Debug)]
pub(crate) struct PluginLibrary {
    pub(crate) library: Library,
    path: PathBuf,
}

impl PluginLibrary {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let library: Library = unsafe { Library::new(path) }?;

        Ok(Self {
            library,
            path: path.to_path_buf(),
        })
    }

    pub fn load_symbol<T>(&self, sym_name: &[u8]) -> Result<Symbol<'_, T>, Error> {
        let sym: Symbol<T> = unsafe { self.library.get(sym_name) }?;
        Ok(sym)
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    PluginNotRegistered(String),
    PluginFileNotFound(libloading::Error),
    SymbolNotFound(libloading::Error),
    UnsupportedCast(String),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::PluginNotRegistered(e) => write!(f, "Plugin is not in registry: {}", e),
            Error::PluginFileNotFound(e) => write!(f, "Plugin file not found: {}", e),
            Error::SymbolNotFound(e) => write!(f, "Symbol not found: {}", e),
            Error::UnsupportedCast(e) => write!(f, "Cast is not supported: {}", e),
            Error::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<libloading::Error> for Error {
    fn from(value: libloading::Error) -> Self {
        match value {
            libloading::Error::DlOpenUnknown | libloading::Error::DlOpen { .. } => {
                Self::PluginFileNotFound(value)
            }
            libloading::Error::DlSymUnknown | libloading::Error::DlSym { .. } => {
                Self::SymbolNotFound(value)
            }
            _ => Self::Other(value.to_string()),
        }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Other(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Other(value.to_string())
    }
}

#[derive(Debug, Default)]
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn register_plugin(&mut self, name: String, plugin: Box<dyn Plugin>) {
        self.plugins.insert(name, plugin);
    }

    pub fn get_plugin(&self, name: &str) -> Result<&dyn Plugin, Error> {
        self.plugins.get(name).map(|p| p.as_ref()).ok_or_else(|| {
            debug!("Plugin {} is not registered", name);
            Error::PluginNotRegistered(format!("Plugin {} is not registered", name))
        })
    }

    pub fn deregister_plugin(&mut self, name: &str) -> Result<Box<dyn Plugin>, Error> {
        self.plugins.remove(name).ok_or_else(|| {
            debug!("Plugin {} is not registered", name);
            Error::PluginNotRegistered(format!("Plugin {} is not registered", name))
        })
    }

    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    pub fn plugin_list(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }
}
