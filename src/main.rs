use socketioxide::SocketIo;
use socketioxide::extract::{Data, SocketRef};
use axum::Router;
use log::info;
use serde_derive::{Deserialize, Serialize};
use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use http::Method;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let (websocket_layer, io) = SocketIo::builder().build_layer();

    io.ns("/", on_send_message);

    let cors_layer = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
    .allow_origin(Any)
    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
    .expose_headers([AUTHORIZATION]);

    let app = Router::new()
        .layer(websocket_layer)
        .layer(cors_layer);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8088")
        .await
        .unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn on_send_message(socket: SocketRef) {
    socket.on(
        "send-message",
        |socket: SocketRef,
         Data::<ChatRequest>(request)| async move {
            info!("Join chat");
            socket.join("chat");

            let response = ChatResponse {
                message: request.message,
            };
            socket
                .within("chat")
                .emit("message-res", &response)
                .await
                .unwrap();
        },
    );
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ChatResponse {
    message: String,
}
