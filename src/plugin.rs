use libloading::{Library, Symbol};
use shared::operator::Operator;
use std::path::Path;

pub struct Plugin {
    _lib: Library,
    symbol: fn(Option<&str>) -> Box<dyn Operator>,
}

impl Plugin {
    pub fn new(path: &Path, entry: &[u8]) -> Result<Self, libloading::Error> {
        unsafe {
            let lib: Library = Library::new(path)?;
            let symbol: Symbol<fn(Option<&str>) -> Box<dyn Operator>> = lib.get(entry)?;
            let symbol: fn(Option<&str>) -> Box<dyn Operator> = *symbol;

            Ok(Plugin { _lib: lib, symbol })
        }
    }

    pub fn symbol(&self) -> for<'a> fn(Option<&'a str>) -> Box<(dyn Operator + 'static)> {
        self.symbol
    }
}
