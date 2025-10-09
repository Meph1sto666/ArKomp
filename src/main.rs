mod ipc_handler;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    shared::logging::init_logger().expect("failed to init logger");
    ui::init()?;
    Ok(())
}
