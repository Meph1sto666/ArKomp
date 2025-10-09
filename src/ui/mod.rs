use eframe::{
    NativeOptions,
    egui::{CentralPanel, Color32, Frame, ViewportBuilder},
};
use shared::{operator::Operator, plugin::PluginRegistry};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tracing::error;

#[derive(Debug)]
pub struct AppState {
    socket_server: super::ipc_handler::WebSocketServer,
    _server_handle: tokio::task::JoinHandle<()>,
    operators: Arc<RwLock<HashMap<String, Box<dyn Operator>>>>,
}

impl AppState {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let plug_reg = Arc::new(std::sync::RwLock::new(PluginRegistry::default()));
        let op_reg = Arc::new(std::sync::RwLock::new(HashMap::default()));
        let web_socket_server = super::ipc_handler::WebSocketServer::new(&plug_reg, &op_reg);
        let op_reg_copy = op_reg.clone();
        let server_handle = tokio::spawn(async move {
            let web_socket_server =
                super::ipc_handler::WebSocketServer::new(&plug_reg, &op_reg_copy);
            if let Err(e) = web_socket_server.run("127.0.0.1:2887").await {
                error!("WebSocket server error: {}", e);
            }
        });

        Self {
            socket_server: web_socket_server,
            _server_handle: server_handle,
            operators: op_reg,
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
            let mut operators_guard = self.operators.write().unwrap();
            for (_, op) in operators_guard.iter_mut() {
                op.render(ctx, ui);
                op.update_animation(ctx);
            }
        });
        ctx.request_repaint();
    }

    fn clear_color(&self, _visuals: &eframe::egui::Visuals) -> [f32; 4] {
        Color32::TRANSPARENT.to_normalized_gamma_f32()
    }
}

pub fn init() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Arkomp view master",
        NativeOptions {
            viewport: ViewportBuilder::default()
                .with_transparent(true)
                .with_always_on_top()
                .with_decorations(false)
                .with_fullscreen(true)
                .with_has_shadow(false)
                .with_mouse_passthrough(true)
                .with_taskbar(false)
                .with_window_level(eframe::egui::WindowLevel::AlwaysOnTop),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(AppState::new(cc)))),
    )
}
