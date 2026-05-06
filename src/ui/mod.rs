use eframe::{
    NativeOptions,
    egui::{CentralPanel, Color32, Frame, ViewportBuilder},
};
use shared::{ipc::events::Event, operator::Operator, plugin::PluginRegistry};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};
use tracing::error;

#[derive(Debug)]
pub struct AppState {
    _server_handle: tokio::task::JoinHandle<()>,
    operators: Arc<RwLock<HashMap<String, Box<dyn Operator>>>>,
    fps: u8,
    ui_rx: tokio::sync::mpsc::Receiver<Event>,
    mouse_passthrough: bool,
    debug_overlay: bool,
    _server_tx: tokio::sync::mpsc::Sender<Event>,
}
impl AppState {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let plug_reg = Arc::new(std::sync::RwLock::new(PluginRegistry::default()));
        let op_reg = Arc::new(std::sync::RwLock::new(HashMap::default()));

        let (ui_tx, ui_rx) = tokio::sync::mpsc::channel::<Event>(10);

        let plug_reg_clone = plug_reg.clone();
        let op_reg_clone = op_reg.clone();

        // create the socket and run it
        let (s_tx, mut s_rx) = tokio::sync::mpsc::channel::<Event>(10);
        let server_handle = tokio::spawn(async move {
            let server =
                super::ipc_handler::WebSocketServer::new(&plug_reg_clone, &op_reg_clone, &ui_tx);

            if let Err(e) = server.run("127.0.0.1:2887", &mut s_rx).await {
                error!("WebSocket server error: {}", e);
            }
        });

        Self {
            _server_handle: server_handle,
            operators: op_reg,
            fps: 15,
            ui_rx,
            mouse_passthrough: true,
            debug_overlay: true,
            _server_tx: s_tx.clone(),
        }
    }

    fn event_handler(&mut self, ctx: &eframe::egui::Context) {
        while let Ok(e) = self.ui_rx.try_recv() {
            match e {
                Event::SetMousePassthrough(val) => {
                    self.mouse_passthrough = val;
                    ctx.send_viewport_cmd(eframe::egui::ViewportCommand::MousePassthrough(val));
                }
                Event::ShowDebugOverlay(val) => self.mouse_passthrough = val,
                _ => todo!(),
            }
        }
    }
}

impl eframe::App for AppState {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx();
        self.event_handler(ctx);
        CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
            // ui.input(|input| {
            //     if let Err(e) = self.server_tx.try_send(Event::OnMouseMove {
            //         to: "all".into(),
            //         position: input.pointer.hover_pos().map(|f| (f.x, f.y)),
            //     }) {
            //         debug!("{}", e.to_string());
            //     }
            // });

            let mut operators_guard = self.operators.write().unwrap();

            for (_, op) in operators_guard.iter_mut() {
                op.render(ctx, ui);
                op.update_animation(ui);
            }
        });
        ctx.request_repaint_after(Duration::from_secs_f32(1.0 / self.fps as f32));
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
