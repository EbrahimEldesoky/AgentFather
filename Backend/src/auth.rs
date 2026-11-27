use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json, Redirect},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub name: String,
    pub exp: usize,
}

fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "agentfather-jwt-super-secret-2026".to_string())
}

fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("agentfather-salt-2026:{}", password).as_bytes());
    hex::encode(hasher.finalize())
}

pub fn create_token(user_id: &str, email: &str, name: &str) -> Result<String, String> {
    let exp = (Utc::now() + Duration::days(7)).timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        name: name.to_string(),
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
    .map_err(|e| e.to_string())
}

pub fn extract_claims(headers: &HeaderMap) -> Option<Claims> {
    let auth = headers.get("authorization")?.to_str().ok()?;
    let token = auth.strip_prefix("Bearer ")?;
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &Validation::default(),
    )
    .ok()?;
    Some(data.claims)
}

#[derive(Deserialize)]
pub struct SignupReq {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginReq {
    pub email: String,
    pub password: String,
}

pub async fn signup_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SignupReq>,
) -> impl IntoResponse {
    let email = req.email.trim().to_lowercase();
    let name = req.name.trim().to_string();
    if email.is_empty() || req.password.len() < 6 || name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Name, valid email, and password (min 6 chars) are required"})),
        );
    }

    let id = Uuid::new_v4().to_string();
    let pw_hash = hash_password(&req.password);
    let now = Utc::now().to_rfc3339();

    let res = {
        let conn = state.db.conn.lock().unwrap();
        let exists: Option<String> = conn
            .query_row("SELECT id FROM users WHERE LOWER(email) = ?1 OR LOWER(name) = ?1", [&email], |r| r.get(0))
            .optional()
            .unwrap_or(None);
        if exists.is_some() {
            Err("Email already registered")
        } else {
            conn.execute(
                "INSERT INTO users (id, name, email, password_hash, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
                (&id, &name, &email, &pw_hash, &now),
            )
            .map(|_| ())
            .map_err(|_| "Database insert failed")
        }
    };

    if let Err(e) = res {
        return (StatusCode::CONFLICT, Json(json!({"error": e})));
    }

    let token = match create_token(&id, &email, &name) {
        Ok(t) => t,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))),
    };

    (
        StatusCode::CREATED,
        Json(json!({
            "token": token,
            "user": { "id": id, "name": name, "email": email, "gmail_connected": false }
        })),
    )
}

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginReq>,
) -> impl IntoResponse {
    let email = req.email.trim().to_lowercase();
    let pw_hash = hash_password(&req.password);

    let user = {
        let conn = state.db.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, name, email, password_hash, google_access_token, whatsapp_number FROM users WHERE LOWER(email) = ?1 OR LOWER(name) = ?1",
            [&email],
            |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, Option<String>>(3)?,
                    r.get::<_, Option<String>>(4)?,
                    r.get::<_, Option<String>>(5)?,
                ))
            },
        )
        .optional()
        .unwrap_or(None)
    };

    match user {
        Some((id, name, email, Some(stored_hash), g_tok, wa)) if stored_hash == pw_hash => {
            let token = create_token(&id, &email, &name).unwrap_or_default();
            (
                StatusCode::OK,
                Json(json!({
                    "token": token,
                    "user": {
                        "id": id,
                        "name": name,
                        "email": email,
                        "gmail_connected": g_tok.is_some(),
                        "whatsapp_number": wa
                    }
                })),
            )
        }
        _ => (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid email or password"})),
        ),
    }
}

pub async fn logout_handler() -> impl IntoResponse {
    Json(json!({"ok": true}))
}

pub async fn me_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let Some(claims) = extract_claims(&headers) else {
        return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"})));
    };

    let row = {
        let conn = state.db.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, name, email, google_access_token, whatsapp_number FROM users WHERE id = ?1",
            [&claims.sub],
            |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, Option<String>>(3)?,
                    r.get::<_, Option<String>>(4)?,
                ))
            },
        )
        .optional()
        .unwrap_or(None)
    };

    match row {
        Some((id, name, email, g_tok, wa)) => (
            StatusCode::OK,
            Json(json!({
                "user": {
                    "id": id,
                    "name": name,
                    "email": email,
                    "gmail_connected": g_tok.is_some(),
                    "whatsapp_number": wa
                }
            })),
        ),
        None => (StatusCode::UNAUTHORIZED, Json(json!({"error": "User not found"}))),
    }
}

pub async fn google_auth_status_handler() -> impl IntoResponse {
    let configured = std::env::var("GOOGLE_CLIENT_ID").map(|s| !s.is_empty()).unwrap_or(false)
        && std::env::var("GOOGLE_CLIENT_SECRET").map(|s| !s.is_empty()).unwrap_or(false);
    Json(json!({ "configured": configured }))
}

pub async fn google_auth_start_handler(headers: HeaderMap) -> impl IntoResponse {
    let client_id = std::env::var("GOOGLE_CLIENT_ID").unwrap_or_default();
    if client_id.is_empty() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "error": "Google OAuth is not configured yet. Set GOOGLE_CLIENT_ID and GOOGLE_CLIENT_SECRET in Backend environment."
            })),
        )
            .into_response();
    }

    let host = headers
        .get("host")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("localhost:8080");
    let scheme = if host.starts_with("localhost") || host.starts_with("127.0.0.1") {
        "http"
    } else {
        "https"
    };
    let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI")
        .unwrap_or_else(|_| format!("{scheme}://{host}/api/auth/google/callback"));

    let scope = "openid email profile https://www.googleapis.com/auth/gmail.readonly https://www.googleapis.com/auth/gmail.send";
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=offline&prompt=consent",
        url_encode(&client_id),
        url_encode(&redirect_uri),
        url_encode(scope)
    );

    Redirect::temporary(&auth_url).into_response()
}

#[derive(Deserialize)]
pub struct GoogleCallbackQuery {
    pub code: Option<String>,
    pub error: Option<String>,
}

pub async fn google_callback_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(q): Query<GoogleCallbackQuery>,
) -> impl IntoResponse {
    if let Some(err) = q.error {
        return Redirect::temporary(&format!("/?auth_error={}", url_encode(&err))).into_response();
    }
    let Some(code) = q.code else {
        return Redirect::temporary("/?auth_error=missing_code").into_response();
    };

    let client_id = std::env::var("GOOGLE_CLIENT_ID").unwrap_or_default();
    let client_secret = std::env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default();
    let host = headers
        .get("host")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("localhost:8080");
    let scheme = if host.starts_with("localhost") || host.starts_with("127.0.0.1") {
        "http"
    } else {
        "https"
    };
    let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI")
        .unwrap_or_else(|_| format!("{scheme}://{host}/api/auth/google/callback"));

    let client = reqwest::Client::new();
    let token_res = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", code.as_str()),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("redirect_uri", redirect_uri.as_str()),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .await;

    let Ok(resp) = token_res else {
        return Redirect::temporary("/?auth_error=token_exchange_failed").into_response();
    };
    let Ok(tok_json) = resp.json::<serde_json::Value>().await else {
        return Redirect::temporary("/?auth_error=token_parse_failed").into_response();
    };

    let access_token = tok_json["access_token"].as_str().unwrap_or_default().to_string();
    let refresh_token = tok_json["refresh_token"].as_str().map(|s| s.to_string());
    if access_token.is_empty() {
        return Redirect::temporary("/?auth_error=no_access_token").into_response();
    }

    let profile_res = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(&access_token)
        .send()
        .await;

    let Ok(prof_resp) = profile_res else {
        return Redirect::temporary("/?auth_error=userinfo_failed").into_response();
    };
    let Ok(prof) = prof_resp.json::<serde_json::Value>().await else {
        return Redirect::temporary("/?auth_error=userinfo_parse_failed").into_response();
    };

    let email = prof["email"].as_str().unwrap_or_default().to_lowercase();
    let name = prof["name"].as_str().unwrap_or("Agent Father User").to_string();
    let google_id = prof["id"].as_str().unwrap_or_default().to_string();
    let now = Utc::now().to_rfc3339();

    let user_id = {
        let conn = state.db.conn.lock().unwrap();
        let existing: Option<String> = conn
            .query_row("SELECT id FROM users WHERE LOWER(email) = ?1 OR LOWER(name) = ?1", [&email], |r| r.get(0))
            .optional()
            .unwrap_or(None);

        if let Some(uid) = existing {
            let _ = conn.execute(
                "UPDATE users SET google_id = ?1, google_access_token = ?2, google_refresh_token = COALESCE(?3, google_refresh_token) WHERE id = ?4",
                (&google_id, &access_token, &refresh_token, &uid),
            );
            uid
        } else {
            let uid = Uuid::new_v4().to_string();
            let _ = conn.execute(
                "INSERT INTO users (id, name, email, google_id, google_access_token, google_refresh_token, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (&uid, &name, &email, &google_id, &access_token, &refresh_token, &now),
            );
            uid
        }
    };

    let jwt = create_token(&user_id, &email, &name).unwrap_or_default();
    Redirect::temporary(&format!("/?token={}", url_encode(&jwt))).into_response()
}

pub async fn gmail_messages_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let Some(claims) = extract_claims(&headers) else {
        return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"})));
    };

    let g_token: Option<String> = {
        let conn = state.db.conn.lock().unwrap();
        conn.query_row(
            "SELECT google_access_token FROM users WHERE id = ?1",
            [&claims.sub],
            |r| r.get(0),
        )
        .optional()
        .unwrap_or(None)
        .flatten()
    };

    let Some(token) = g_token else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Gmail not connected. Sign in with Google to connect Gmail."})),
        );
    };

    let client = reqwest::Client::new();
    let list_res = client
        .get("https://gmail.googleapis.com/gmail/v1/users/me/messages?maxResults=5")
        .bearer_auth(&token)
        .send()
        .await;

    match list_res {
        Ok(r) if r.status().is_success() => {
            let val: serde_json::Value = r.json().await.unwrap_or_else(|_| json!({}));
            (StatusCode::OK, Json(val))
        }
        Ok(r) => (
            StatusCode::BAD_GATEWAY,
            Json(json!({"error": format!("Gmail API returned {}", r.status())})),
        ),
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            Json(json!({"error": format!("Gmail request failed: {e}")})),
        ),
    }
}

fn url_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}
