use futures::{SinkExt, StreamExt};
use serde_json::json;
use shared::{
    events::Event,
    ipc::{Response, commands::Command},
    operator::Operator,
    plugin::PluginRegistry,
};
use std::{collections::HashMap, sync::Arc};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tracing::{error, info};

type OperatorRegistry = Arc<std::sync::RwLock<HashMap<String, Box<dyn Operator>>>>;

#[derive(Debug, Clone)]
pub struct WebSocketServer {
    plugins: Arc<std::sync::RwLock<PluginRegistry>>,
    operators: OperatorRegistry,
    event_tx: tokio::sync::mpsc::Sender<Event>,
}

impl WebSocketServer {
    pub fn new(
        plugins: &Arc<std::sync::RwLock<PluginRegistry>>,
        operators: &OperatorRegistry,
    ) -> Self {
        let (event_tx, event_rx) = tokio::sync::mpsc::channel::<Event>(100);

        let operators_clone = operators.clone();
        tokio::spawn(Self::event_processor(event_rx, operators_clone));

        Self {
            plugins: plugins.clone(),
            operators: operators.clone(),
            event_tx,
        }
    }

    pub async fn run(&self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(addr).await?;
        info!("WebSocket server running on ws://{}", addr);

        while let Ok((stream, _)) = listener.accept().await {
            let server = self.clone();
            tokio::spawn(async move {
                if let Err(e) = server.handle_client(stream).await {
                    error!("Client error: {}", e);
                }
            });
        }
        Ok(())
    }

    async fn handle_client(&self, stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let ws = accept_async(stream).await?;
        let (mut writer, mut reader) = ws.split();
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        let write_handle = tokio::spawn(async move {
            while let Some(response) = rx.recv().await {
                if let Ok(json) = serde_json::to_string(&response) {
                    if writer.send(Message::Text(json.into())).await.is_err() {
                        break;
                    }
                }
            }
        });

        while let Some(Ok(Message::Text(cmd))) = reader.next().await {
            let response = self.process_command(&cmd).await;
            if tx.send(response).is_err() {
                break;
            }
        }

        drop(tx);
        let _ = write_handle.await;
        Ok(())
    }

    async fn event_processor(
        mut event_rx: tokio::sync::mpsc::Receiver<Event>,
        operators: OperatorRegistry,
    ) {
        while let Some(event) = event_rx.recv().await {
            if let Some(op) = operators.write().unwrap().get_mut(event.operator_id()) {
                if let Ok(_response) = op.event_handler(event) {                    
                    // let _ = event_tx.send(response);
                    // just ignore channel sent events responses for now...
                }
            }
        }
    }

    async fn process_command(&self, cmd: &str) -> Response {
        match Command::execute_from_json(
            cmd,
            &mut shared::ipc::command_context::CommandContext::new(
                self.operators.clone(),
                self.plugins.clone(),
                self.event_tx.clone(),
            ),
        ) {
            Ok(response) => response,
            Err(e) => Response::Error(
                json!({
                    "reason": e
                })
                .to_string(),
            ),
        }
    }
}
