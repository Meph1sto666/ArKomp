use crate::plugin::{Error, Plugin};
use libloading::Library;
use shared::operator::Operator;
use std::path::Path;

#[derive(Debug)]
pub struct OperatorPlugin {
    lib: Library,
    id: String,
}

impl Plugin for OperatorPlugin {
    fn new(path: &Path) -> Result<Box<Self>, Error> {
        Ok(Box::new(Self {
            id: (&path)
                .to_str()
                .expect("failed to convert path to string")
                .into(),
            lib: Self::load_lib(&path)?,
        }))
    }

    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_lib(&self) -> &Library {
        &self.lib
    }
}

impl OperatorPlugin {
    pub fn build(&self, skin: Option<&str>) -> Result<Box<dyn Operator>, Error> {
        Ok((self
            .load_symbol::<fn(Option<&str>) -> Box<dyn Operator>>(
                b"new",
            )?)(skin))
    }
}
