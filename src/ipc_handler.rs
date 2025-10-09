use futures::{SinkExt, StreamExt as _};
use shared::{
    ipc::{Response, commands::Command},
    operator::Operator,
    plugin::PluginRegistry,
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
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

        while let Ok((stream, _)) = listener.accept().await {
            let server = self.clone();

            tokio::spawn(async move {
                if let Err(e) = server.handle_connection(stream).await {
                    error!("Connection handler error: {}", e);
                }
            });
        }
        Ok(())
    }

    async fn handle_connection(
        &self,
        stream: tokio::net::TcpStream,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ws_stream = tokio_tungstenite::accept_async(stream).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        while let Some(message) = ws_receiver.next().await {
            self.process_message(message, &mut ws_sender).await?;
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
    ) -> Result<(), Box<dyn std::error::Error>> {
        match message {
            Ok(Message::Text(command_json)) => {
                let response = self.execute_command(&command_json).await;
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

    async fn execute_command(&self, command_json: &str) -> Response {
        match Command::execute_from_json(
            command_json,
            &mut shared::ipc::command_context::CommandContext::new(
                self.operator_registry.clone(),
                self.plugin_registry.clone(),
            ),
        ) {
            Ok(response) => response,
            Err(e) => Response::Error(format!("Command execution failed: {:?}", e)),
        }
    }
}
