use futures::{SinkExt, StreamExt as _};
use shared::{
    events::Event,
    ipc::{Response, commands::Command},
    operator::Operator,
    plugin::PluginRegistry,
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock, mpsc::Sender},
};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error};

#[derive(Debug, Clone)]
pub struct WebSocketServer {
    plugin_registry: Arc<RwLock<PluginRegistry>>,
    operator_registry: Arc<RwLock<HashMap<String, Box<dyn Operator>>>>,
}

impl WebSocketServer {
    pub fn new(
        plugin_registry: &Arc<std::sync::RwLock<PluginRegistry>>,
        operator_registry: &Arc<RwLock<HashMap<String, Box<dyn Operator>>>>,
    ) -> Self {
        Self {
            plugin_registry: plugin_registry.clone(),
            operator_registry: operator_registry.clone(),
        }
    }

    pub async fn run(&self, address: &str) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(address).await?;
        tracing::info!("WebSocket server running on ws://{}", address);
        let (operator_tx, service_rx) = std::sync::mpsc::channel::<Event>();

        let op_reg = self.operator_registry.clone();
        tokio::spawn(async move {
            while let Ok(event) = service_rx.recv() {
                handle_event(event, op_reg.clone());
            }
        });

        while let Ok((stream, _)) = listener.accept().await {
            let server = self.clone();
            let op_tx = operator_tx.clone();
            tokio::spawn(async move {
                if let Err(e) = server.handle_connection(stream, op_tx).await {
                    error!("Connection handler error: {}", e);
                }
            });
        }
        Ok(())
    }

    async fn handle_connection(
        &self,
        stream: tokio::net::TcpStream,
        operator_tx: Sender<Event>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ws_stream = tokio_tungstenite::accept_async(stream).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        while let Some(message) = ws_receiver.next().await {
            self.process_message(message, &mut ws_sender, operator_tx.clone())
                .await?;
        }
        Ok(())
    }

    async fn process_message(
        &self,
        message: Result<Message, tokio_tungstenite::tungstenite::Error>,
        ws_sender: &mut futures_util::stream::SplitSink<
            tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
            Message,
        >,
        operator_tx: Sender<Event>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match message {
            Ok(Message::Text(command_json)) => {
                let response = self.execute_command(&command_json, operator_tx).await;
                self.send_response(ws_sender, response).await?;
            }
            Ok(Message::Close(_)) => {
                error!("Connection closed by client");
                return Err("Connection closed by client".into());
            }
            Err(e) => {
                error!("Error receiving message: {}", e);
                return Err(e.into());
            }
            _ => {}
        }
        Ok(())
    }

    async fn send_response(
        &self,
        ws_sender: &mut futures_util::stream::SplitSink<
            tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
            Message,
        >,
        response: Response,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response_json = serde_json::to_string(&response).unwrap_or_else(|_| {
            debug!("serialisation failed: {response:?}");
            "{\"error\":\"Serialisation failed\"}".to_string()
        });

        ws_sender
            .send(Message::Text(response_json.into()))
            .await
            .map_err(|e| {
                error!("Error sending response: {}", e);
                e
            })?;
        Ok(())
    }

    async fn execute_command(&self, command_json: &str, operator_tx: Sender<Event>) -> Response {
        match Command::execute_from_json(
            command_json,
            &mut shared::ipc::command_context::CommandContext::new(
                self.operator_registry.clone(),
                self.plugin_registry.clone(),
                operator_tx,
            ),
        ) {
            Ok(response) => response,
            Err(e) => Response::Error(format!("Command execution failed: {:?}", e)),
        }
    }
}

fn handle_event(event: Event, op_registry: Arc<RwLock<HashMap<String, Box<dyn Operator>>>>) {
    if let Some(op) = op_registry.write().unwrap().get_mut(event.operator_id()) {
        op.event_handler(event)
    } else {
        debug!(
            "Failed to designate event {:?}; Operator not in registry",
            event
        )
    };
}
