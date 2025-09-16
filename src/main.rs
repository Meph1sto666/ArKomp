mod plugin;

fn main() {
    shared::logging::init_logger().expect("failed to init logger");
}

#[test]
fn test_libload() {
    use libloading::{Library, Symbol};
    use shared::operator::Operator;
    use std::path::Path;
    use log::info;

    shared::logging::init_logger().expect("failed to init logger");
    let plugin_path = Path::new("plugins/libchar_421_crow.so");
    let lib = unsafe { Library::new(plugin_path).unwrap() };

    unsafe {
        let plugin: Symbol<fn() -> Box<dyn Operator>> = lib.get(b"new").unwrap();
        let my_plugin = plugin();
        info!("{:?}", my_plugin);
    }
}
