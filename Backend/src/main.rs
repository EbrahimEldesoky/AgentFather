mod auth;
mod chat;
mod db;
mod key_pool;
mod whatsapp;

use axum::{
    routing::{delete, get, post, put},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

use db::Database;
use key_pool::KeyPool;

pub struct AppState {
    pub db: Database,
    pub key_pool: KeyPool,
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(json!({"status": "ok", "service": "Agent Father"}))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db_path = std::env::var("DATABASE_PATH")
        .unwrap_or_else(|_| "/app/data/agentfather.db".to_string());
    let db = Database::open(&db_path).expect("Failed to open SQLite database");
    let key_pool = KeyPool::new();
    let state = Arc::new(AppState { db, key_pool });

    let frontend_dist = std::env::var("FRONTEND_DIST")
        .unwrap_or_else(|_| "/app/frontend/dist".to_string());
    let assets_dir = std::env::var("ASSETS_DIR")
        .unwrap_or_else(|_| "/app/assets".to_string());

    let spa_service = ServeDir::new(&frontend_dist)
        .not_found_service(ServeFile::new(format!("{}/index.html", frontend_dist)));

    let app = Router::new()
        .route("/api/health", get(health_handler))
        // Auth
        .route("/api/auth/signup", post(auth::signup_handler))
        .route("/api/auth/login", post(auth::login_handler))
        .route("/api/auth/logout", post(auth::logout_handler))
        .route("/api/auth/me", get(auth::me_handler))
        .route("/api/auth/google/status", get(auth::google_auth_status_handler))
        .route("/api/auth/google", get(auth::google_auth_start_handler))
        .route("/api/auth/google/callback", get(auth::google_callback_handler))
        .route("/api/gmail/messages", get(auth::gmail_messages_handler))
        // WhatsApp OTP Verification
        .route("/api/whatsapp/request-otp", post(whatsapp::request_otp_handler))
        .route("/api/whatsapp/verify-otp", post(whatsapp::verify_otp_handler))
        // Agent Chat & Sessions
        .route("/api/chat", post(chat::chat_handler))
        .route("/api/chat/transcribe", post(chat::transcribe_audio_handler))
        .route("/api/chat/history", get(chat::chat_history_handler))
        .route("/api/chat/sessions", get(chat::list_sessions_handler))
        .route("/api/chat/sessions", post(chat::create_session_handler))
        .route("/api/chat/sessions/{id}", delete(chat::delete_session_handler))
        // Sub-Agents
        .route("/api/sub-agents", get(chat::list_subagents_handler))
        .route("/api/sub-agents", post(chat::create_subagent_handler))
        .route("/api/sub-agents/{id}", put(chat::update_subagent_handler))
        .route("/api/sub-agents/{id}", delete(chat::delete_subagent_handler))
        .route("/api/sub-agents/{id}/share", post(chat::share_subagent_handler))
        // Internal OpenAI-compatible rotating proxy & TTS for Hermes Agent
        .route("/internal/llm/v1/models", get(key_pool::openai_models_handler))
        .route(
            "/internal/llm/v1/chat/completions",
            post(key_pool::openai_chat_handler),
        )
        .route("/internal/tts", post(key_pool::internal_tts_handler))
        .route("/internal/keys/status", get(key_pool::key_pool_status_handler))
        // Static assets & SPA
        .nest_service("/brand-assets", ServeDir::new(&assets_dir))
        .nest_service("/media-audio", ServeDir::new("/app/hermes_data/audio"))
        .fallback_service(spa_service)
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "0.0.0.0:8090";
    tracing::info!("Agent Father Backend listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
