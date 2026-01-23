use axum::routing::{delete, get, post, put};
use axum::Router;
use dashmap::DashMap;
use dotenvy::dotenv;
use futures_util::{SinkExt, StreamExt};
use once_cell::sync::{Lazy, OnceCell};
use serde::Serialize;
use std::env;
use std::panic;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::signal;
use tokio::sync::broadcast;
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::accept_hdr_async;
use tower_http::cors::CorsLayer;
use tracing::info;
use tungstenite::handshake::server::{Request, Response};
use tungstenite::{Message, Utf8Bytes};

use crate::routes::clients::login;
use crate::routes::clients::register;
use crate::services::websocket::handle_message;
use crate::types::client::Client;
use crate::types::connection::ClientConnection;
use crate::types::connection::ClientsV2;
use crate::types::db::init_db;
use crate::types::messages::ClientMessage;
use crate::types::messages::ServerMessage;

mod routes;
mod services;
mod types;

pub static JWT_SECRET: OnceCell<String> = OnceCell::new();
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    dotenv().ok();
    JWT_SECRET
        .set(env::var("JWT_SECRET").expect("No JWT_SECRET env var."))
        .expect("Global var already set.");
    info!("Connecting to database");
    init_db().await;

    info!("Setting up server");
    let clients: ClientsV2 = Arc::new(DashMap::new());

    let (shutdown_tx, _) = broadcast::channel::<()>(1);

    let ws_shutdown_rx = shutdown_tx.subscribe();
    let rocket_shutdown_rx = shutdown_tx.subscribe();

    let ws_server = tokio::spawn(run_ws_server(clients.clone(), ws_shutdown_rx));
    let axum_server = tokio::spawn(run_axum(clients.clone(), rocket_shutdown_rx));

    info!("Setting up cron");
    info!("WebSocket + Axum + Cron running... Press Ctrl+C to stop");
    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    tracing::error!("\nReceived Ctrl+C - shutting down gracefully...");
    let _ = shutdown_tx.send(());
    let _ = tokio::join!(ws_server, axum_server);
    tracing::info!("All servers stopped cleanly");
}

async fn run_ws_server(clients: ClientsV2, mut shutdown_rx: broadcast::Receiver<()>) {
    let addr = env::var("WS_ADDR").expect("No WS_ADDR env var.");
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    info!(
        "WebSocket server running on: {}",
        listener.local_addr().unwrap()
    );

    loop {
        tokio::select! {
            accept_res = listener.accept() => {
                match accept_res {
                    Ok((stream, _)) => {
                        tokio::spawn(handle_connection(stream, clients.clone()));
                    }
                    Err(e) => {
                        tracing::error!("Accept error: {e}");
                        break;
                    }
                }
            }
            _ = shutdown_rx.recv() => {
                tracing::warn!("WebSocket server shutting down...");
                break;
            }
        }
    }

    tracing::info!("WebSocket server stopped");
}

pub async fn run_axum(clients: ClientsV2, mut shutdown_rx: broadcast::Receiver<()>) {
    let app = Router::new()
        .route("/api/clients/login", post(login::login))
        .route("/api/clients/register", post(register::register))
        .route(
            "/api/rooms",
            post(crate::routes::rooms::create_room::create_room)
                .get(crate::routes::rooms::get_rooms::get_rooms),
        )
        .route(
            "/api/rooms/{room_id}",
            put(crate::routes::rooms::update_room::update_room)
                .delete(crate::routes::rooms::delete_room::delete_room),
        )
        .route(
            "/api/rooms/{room_id}/questions",
            post(crate::routes::rooms::questions::create_question::create_question)
                .get(crate::routes::rooms::questions::get_questions::get_questions),
        )
        .route(
            "/api/rooms/{room_id}/questions/{question_id}",
            put(crate::routes::rooms::questions::update_question::update_question)
                .delete(crate::routes::rooms::questions::delete_question::delete_question),
        )
        .with_state(clients.clone())
        .layer(CorsLayer::permissive());
    info!("Axum routes ready and hot.");
    let addr = env::var("AXUM_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Cannot bind tcp listener");
    info!("Axum server listening on: {addr}");
    info!("Local address: {}", listener.local_addr().unwrap());
    let shutdown_signal = async move {
        let _ = shutdown_rx.recv().await;
        tracing::warn!("Axum server shutting down...");
    };

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await
        .map_err(|err| {
            tracing::error!("Axum launch failed: {err}");
        })
        .ok();
    boardcast(&clients, &ServerMessage::Exiting())
        .await
        .map_err(|err| {
            tracing::error!("{err}");
        })
        .ok();
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    info!("Axum server stopped.");
}

// Handling connection of an user, runs per connected user.
async fn handle_connection(stream: TcpStream, clients: ClientsV2) {
    let mut user_jwt = String::new();
    let mut room = String::new();
    // Opens a stream and gets an UUID
    let ws_stream = accept_hdr_async(stream, |req: &Request, res: Response| {
        if let Some(room_id) = req.headers().get("X-Room")
            && let Ok(room_id) = room_id.to_str()
        {
            room = room_id.to_string();
        }
        if let Some(auth) = req.headers().get("Authorization") && let Ok(auth) = auth.to_str(){
            user_jwt = auth.to_string()
        }
        Ok(res)
    })
    .await
    .expect("Error during WebSocket handshake");
    if user_jwt.is_empty() {
        let error_msg = Message::Text(
            serde_json::json!({
                "error": "Auth token is not provided."
            })
            .to_string()
            .into(),
        );
        let mut stream = ws_stream;
        let _ = stream.send(error_msg).await;
        let _ = stream.close(None).await;
        return;
    }
    let client: Client = match Client::from_jwt(&user_jwt).await {
        Some(c) => {
            if room.is_empty() {
                let error_msg = Message::Text(
                    serde_json::json!({
                        "error": "No room provided"
                    })
                    .to_string()
                    .into(),
                );
                let mut stream = ws_stream;
                let _ = stream.send(error_msg).await;
                let _ = stream.close(None).await;
                return;
            }
            c
        }
        None => {
            let error_msg = Message::Text(
                serde_json::json!({
                    "error": "Connection rejected: Invalid token."
                })
                .to_string()
                .into(),
            );
            let mut stream = ws_stream;
            let _ = stream.send(error_msg).await;
            let _ = stream.close(None).await;
            return;
        }
    };

    // Splits ws stream in to write and read
    let (mut write, mut read) = ws_stream.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    let pending = Arc::new(DashMap::new());

    let client_id = client.clone().id;
    clients
        .entry(room.clone())
        .or_insert_with(DashMap::new)
        .insert(
            client_id.clone(),
            ClientConnection {
                tx,
                pending: pending.clone(),
                _db_client: client.clone(),
            },
        );

    let write_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = write.send(msg).await {
                tracing::error!("Failed to send message to target, {e}");
                break;
            }
        }
    });

    // Await a message from clients loop
    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) => {
                if msg.is_text() {
                    let text = msg.to_text().unwrap();
                    match serde_json::from_str::<ClientMessage>(text) {
                        Ok(parsed) => {
                            handle_message(&clients, &room, &client, parsed).await;
                        }
                        Err(e) => tracing::error!("Error parsing message: {e}"),
                    }
                }
            }
            Err(_) => break,
        }
    }

    if let Some(room_clients) = clients.get(&room) {
        room_clients.remove(&client_id);
    }
    write_task.abort();
    info!("Connection closed");
}

// Boardcast to all connacted users
async fn boardcast<T: Serialize>(clients: &ClientsV2, msg: &T) -> Result<(), String> {
    let json: Utf8Bytes = serde_json::to_string(&msg).unwrap().into();
    for room in clients.iter() {
        for client in room.value().iter() {
            let _ = client.value().tx.send(Message::Text(json.clone()));
        }
    }
    Ok(())
}
