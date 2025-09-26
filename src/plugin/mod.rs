pub mod operator_plugin;
use libloading::{Library, Symbol};
use std::path::Path;

pub trait Plugin: std::fmt::Debug {
    fn new(path: &Path) -> Result<Box<Self>, Error>;
    fn get_id(&self) -> &String;
    fn get_lib(&self) -> &Library;
    fn load_lib(path: &Path) -> Result<Library, Error> {
        Ok(unsafe { Library::new(path) }?)
    }
    fn load_symbol<T>(&'_ self, sym_name: &[u8]) -> Result<Symbol<'_, T>, Error> {
        let sym: Symbol<T> = unsafe { self.get_lib().get(sym_name) }?;
        Ok(sym)
    }
}

#[derive(Debug)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum Error {
    PluginNotFound(libloading::Error),
    SymbolNotFound(libloading::Error),
    Other(libloading::Error),
}

impl From<libloading::Error> for Error {
    fn from(value: libloading::Error) -> Self {
        match value {
            libloading::Error::DlOpenUnknown | libloading::Error::DlOpen { .. } => {
                Self::PluginNotFound(value)
            }
            libloading::Error::DlSymUnknown | libloading::Error::DlSym { .. } => {
                Self::SymbolNotFound(value)
            }
            _ => Self::Other(value),
        }
    }
}
