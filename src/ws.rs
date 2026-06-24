use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::Response,
};
use dashmap::DashMap;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

const CHANNEL_CAPACITY: usize = 128;

/// Mensaje de chat serializado como JSON en el WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub room: String,
    pub user: String,
    pub text: String,
    pub timestamp: i64,
    #[serde(rename = "type")]
    pub msg_type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Chat,
    Join,
    Leave,
    System,
}

impl ChatMessage {
    pub fn chat(room: &str, user: &str, text: &str) -> Self {
        ChatMessage {
            id: Uuid::new_v4().to_string(),
            room: room.to_string(),
            user: user.to_string(),
            text: text.to_string(),
            timestamp: chrono::Utc::now().timestamp_millis(),
            msg_type: MessageType::Chat,
        }
    }

    pub fn system(room: &str, text: &str) -> Self {
        ChatMessage {
            id: Uuid::new_v4().to_string(),
            room: room.to_string(),
            user: "sistema".to_string(),
            text: text.to_string(),
            timestamp: chrono::Utc::now().timestamp_millis(),
            msg_type: MessageType::System,
        }
    }

    pub fn join(room: &str, user: &str) -> Self {
        ChatMessage {
            id: Uuid::new_v4().to_string(),
            room: room.to_string(),
            user: user.to_string(),
            text: format!("{user} se unió a la sala"),
            timestamp: chrono::Utc::now().timestamp_millis(),
            msg_type: MessageType::Join,
        }
    }

    pub fn leave(room: &str, user: &str) -> Self {
        ChatMessage {
            id: Uuid::new_v4().to_string(),
            room: room.to_string(),
            user: user.to_string(),
            text: format!("{user} abandonó la sala"),
            timestamp: chrono::Utc::now().timestamp_millis(),
            msg_type: MessageType::Leave,
        }
    }
}

/// Hub de salas de chat. Cada sala tiene su propio canal broadcast.
#[derive(Clone)]
pub struct ChatHub {
    rooms: Arc<DashMap<String, broadcast::Sender<ChatMessage>>>,
}

impl ChatHub {
    pub fn new() -> Self {
        ChatHub {
            rooms: Arc::new(DashMap::new()),
        }
    }

    /// Obtiene o crea el canal broadcast de una sala.
    pub fn get_or_create_room(&self, room: &str) -> broadcast::Sender<ChatMessage> {
        if let Some(tx) = self.rooms.get(room) {
            return tx.clone();
        }
        let (tx, _) = broadcast::channel(CHANNEL_CAPACITY);
        self.rooms.insert(room.to_string(), tx.clone());
        tracing::info!(room, "nueva sala de chat creada");
        tx
    }

    /// Envía un mensaje a una sala sin pasar por WebSocket (útil para notificaciones del server).
    pub fn send_system(&self, room: &str, text: &str) {
        let tx = self.get_or_create_room(room);
        let _ = tx.send(ChatMessage::system(room, text));
    }

    /// Lista de salas activas (con al menos un subscriptor).
    pub fn active_rooms(&self) -> Vec<String> {
        self.rooms
            .iter()
            .filter(|e| e.value().receiver_count() > 0)
            .map(|e| e.key().clone())
            .collect()
    }
}

impl Default for ChatHub {
    fn default() -> Self {
        Self::new()
    }
}

/// Handler de upgrade WebSocket para una sala de chat.
///
/// Ruta: `GET /ws/chat/:room?user=nombre`
pub async fn ws_chat_handler<S>(
    ws: WebSocketUpgrade,
    Path(room): Path<String>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
    State(hub): State<Arc<ChatHub>>,
) -> Response
where
    S: Send + Sync + 'static,
{
    let user = params
        .get("user")
        .cloned()
        .unwrap_or_else(|| format!("usuario_{}", &Uuid::new_v4().to_string()[..6]));

    ws.on_upgrade(move |socket| handle_socket(socket, room, user, hub))
}

async fn handle_socket(socket: WebSocket, room: String, user: String, hub: Arc<ChatHub>) {
    let tx = hub.get_or_create_room(&room);
    let mut rx = tx.subscribe();

    let (mut sender, mut receiver) = socket.split();

    // Notifica a la sala que alguien entró
    let _ = tx.send(ChatMessage::join(&room, &user));

    // Task: reenvía mensajes del broadcast al cliente WebSocket
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let json = match serde_json::to_string(&msg) {
                Ok(j) => j,
                Err(_) => continue,
            };
            if sender.send(Message::Text(json.into())).await.is_err() {
                break;
            }
        }
    });

    // Task: recibe mensajes del cliente y los publica al broadcast
    let tx_clone = tx.clone();
    let room_clone = room.clone();
    let user_clone = user.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    // El cliente envía solo el texto; el server construye el mensaje completo
                    let text = text.trim().to_string();
                    if text.is_empty() || text.len() > 2000 {
                        continue;
                    }
                    let chat_msg = ChatMessage::chat(&room_clone, &user_clone, &text);
                    let _ = tx_clone.send(chat_msg);
                }
                Message::Close(_) => break,
                Message::Ping(p) => {
                    // axum maneja pong automáticamente, pero registramos el ping
                    let _ = p;
                }
                _ => {}
            }
        }
    });

    // Si cualquiera de los dos tasks termina, cancela el otro
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }

    // Notifica que el usuario salió
    let _ = tx.send(ChatMessage::leave(&room, &user));
    tracing::debug!(room, user, "conexión WebSocket cerrada");
}
