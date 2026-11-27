use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json},
};
use chrono::Utc;
use rusqlite::OptionalExtension;
use serde::Deserialize;
use serde_json::json;
use std::{path::PathBuf, sync::Arc};
use uuid::Uuid;

use crate::{auth::extract_claims, AppState};

#[derive(Deserialize)]
pub struct ChatReq {
    pub message: String,
    #[allow(dead_code)]
    pub session_id: Option<String>,
    pub sub_agent_id: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateSubAgentReq {
    pub name: String,
    pub role: String,
    pub system_prompt: String,
}

pub async fn list_subagents_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let Some(claims) = extract_claims(&headers) else {
        return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Authentication required"})));
    };
    let conn = state.db.conn.lock().unwrap();
    let mut stmt = match conn.prepare(
        "SELECT id, name, role, system_prompt, created_at FROM sub_agents WHERE user_id = ?1 ORDER BY created_at DESC",
    ) {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    };
    let items = stmt
        .query_map([&claims.sub], |row| {
            Ok(json!({
                "id": row.get::<_, String>(0)?,
                "name": row.get::<_, String>(1)?,
                "role": row.get::<_, String>(2)?,
                "system_prompt": row.get::<_, String>(3)?,
                "created_at": row.get::<_, String>(4)?,
            }))
        })
        .map(|it| it.filter_map(|r| r.ok()).collect::<Vec<_>>())
        .unwrap_or_default();
    (StatusCode::OK, Json(json!({ "sub_agents": items })))
}

pub async fn create_subagent_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<CreateSubAgentReq>,
) -> impl IntoResponse {
    let Some(claims) = extract_claims(&headers) else {
        return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Authentication required"})));
    };
    let name = req.name.trim().to_string();
    let role = req.role.trim().to_string();
    let prompt = req.system_prompt.trim().to_string();
    if name.is_empty() || role.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "Name and role are required"})));
    }
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    {
        let conn = state.db.conn.lock().unwrap();
        if let Err(e) = conn.execute(
            "INSERT INTO sub_agents (id, user_id, name, role, system_prompt, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (&id, &claims.sub, &name, &role, &prompt, &now),
        ) {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})));
        }
    }
    (StatusCode::OK, Json(json!({
        "sub_agent": {
            "id": id,
            "name": name,
            "role": role,
            "system_prompt": prompt,
            "created_at": now,
        }
    })))
}

pub async fn delete_subagent_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Some(claims) = extract_claims(&headers) else {
        return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Authentication required"})));
    };
    let conn = state.db.conn.lock().unwrap();
    let _ = conn.execute(
        "DELETE FROM sub_agents WHERE id = ?1 AND user_id = ?2",
        (&id, &claims.sub),
    );
    (StatusCode::OK, Json(json!({ "ok": true })))
}

fn bridge_token() -> String {
    if let Ok(tok) = std::fs::read_to_string("/app/hermes_data/whatsapp/session/.bridge-token") {
        let t = tok.trim().to_string();
        if !t.is_empty() {
            return t;
        }
    }
    std::env::var("HERMES_WHATSAPP_BRIDGE_TOKEN").unwrap_or_default()
}

fn bridge_url() -> String {
    std::env::var("WHATSAPP_BRIDGE_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".to_string())
}

fn audio_host_dir() -> PathBuf {
    PathBuf::from(
        std::env::var("AUDIO_DIR").unwrap_or_else(|_| "/app/hermes_data/audio".to_string()),
    )
}

pub async fn chat_history_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let claims = match extract_claims(&headers) {
        Some(c) => c,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Authentication required"})),
            );
        }
    };

    let conn = state.db.conn.lock().unwrap();
    let mut stmt = match conn.prepare(
        "SELECT role, content, created_at FROM chat_messages WHERE user_id = ?1 ORDER BY created_at ASC LIMIT 100",
    ) {
        Ok(s) => s,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("DB error: {}", e)})),
            );
        }
    };

    let rows = stmt
        .query_map([&claims.sub], |row| {
            let role: String = row.get(0)?;
            let text: String = row.get(1)?;
            let created_at: String = row.get(2)?;
            Ok(json!({
                "role": if role == "assistant" { "agent" } else { "user" },
                "text": text,
                "created_at": created_at,
            }))
        })
        .map(|iter| iter.filter_map(|r| r.ok()).collect::<Vec<_>>())
        .unwrap_or_default();

    (StatusCode::OK, Json(json!({ "messages": rows })))
}

pub async fn chat_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<ChatReq>,
) -> impl IntoResponse {
    let claims = match extract_claims(&headers) {
        Some(c) => c,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Authentication required"})),
            );
        }
    };

    if req.message.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Message cannot be empty"})),
        );
    }

    let user_id = claims.sub;
    let now = Utc::now().to_rfc3339();

    let (user_name, user_wa, sub_agent_ctx): (String, Option<String>, Option<(String, String, String, String)>) = {
        let conn = state.db.conn.lock().unwrap();
        let _ = conn.execute(
            "INSERT INTO chat_messages (id, user_id, role, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            (&Uuid::new_v4().to_string(), &user_id, "user", &req.message, &now),
        );
        let u = conn
            .query_row(
                "SELECT name, whatsapp_number FROM users WHERE id = ?1",
                [&user_id],
                |r| Ok((r.get::<_, String>(0)?, r.get::<_, Option<String>>(1)?)),
            )
            .optional()
            .unwrap_or(None)
            .unwrap_or_else(|| ("User".to_string(), None));

        let sa = if let Some(ref sa_id) = req.sub_agent_id {
            conn.query_row(
                "SELECT id, name, role, system_prompt FROM sub_agents WHERE id = ?1 AND user_id = ?2",
                (sa_id, &user_id),
                |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?, r.get::<_, String>(3)?)),
            )
            .optional()
            .unwrap_or(None)
        } else {
            None
        };
        (u.0, u.1, sa)
    };

    let wa_info = match &user_wa {
        Some(num) if !num.is_empty() => format!("+{} (digits: {})", num, num),
        _ => "NOT LINKED".to_string(),
    };

    let sub_agent_line = match &sub_agent_ctx {
        Some((_, sa_name, sa_role, sa_prompt)) => format!(
            "Active Sub-Agent Persona: You are acting as '{}' (Role: {}). Custom Instructions: {}. ",
            sa_name, sa_role, sa_prompt
        ),
        None => String::new(),
    };

    let system_context = format!(
        "{}Web Dashboard Context: You are speaking with user '{}' via the Web Dashboard. Their linked WhatsApp number is: {}. \
        If the user asks to send a WhatsApp text message (to themselves or a specific number), include `[[wa_send:<digits>]]<exact message>` on its own line (use their linked digits if they say 'to me' or 'on WhatsApp', or tell them to link WhatsApp in Integrations if NOT LINKED). \
        If the user asks for a voice note / audio record, call `text_to_speech` and return `[[audio_as_voice]]` followed by `MEDIA:<path>`.",
        sub_agent_line, user_name, wa_info
    );

    let hermes_url = std::env::var("HERMES_API_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8642/v1/chat/completions".to_string());
    let hermes_key = std::env::var("HERMES_API_KEY")
        .unwrap_or_else(|_| "agentfather-hermes-secret-2026".to_string());

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(45))
        .build()
        .unwrap();

    let session_key = match &sub_agent_ctx {
        Some((sa_id, _, _, _)) => format!("web-subagent:{}:{}", user_id, sa_id),
        None => format!("web-user:{}", user_id),
    };
    let hermes_res = client
        .post(&hermes_url)
        .bearer_auth(&hermes_key)
        .header("X-Hermes-Session-Key", &session_key)
        .json(&json!({
            "model": "hermes-agent",
            "user": user_id,
            "messages": [
                {"role": "system", "content": system_context},
                {"role": "user", "content": req.message}
            ]
        }))
        .send()
        .await;

    let reply_text = match hermes_res {
        Ok(r) if r.status().is_success() => {
            let v: serde_json::Value = r.json().await.unwrap_or_else(|_| json!({}));
            v["choices"][0]["message"]["content"]
                .as_str()
                .map(|s| s.to_string())
        }
        _ => None,
    };

    let raw_reply = match reply_text {
        Some(t) if !t.is_empty() => t,
        _ => {
            let body = json!({
                "systemInstruction": {
                    "parts": [{"text": "You are Agent Father (الوكيل الأب), an independent personal AI executive assistant. Be concise, decisive, and match the user's language (Arabic, French, or English). Never mention Hermes or Nous Research."}]
                },
                "contents": [{
                    "role": "user",
                    "parts": [{"text": req.message}]
                }]
            });
            match state.key_pool.generate_content("gemini-3.5-flash-lite", &body).await {
                Ok(v) => v["candidates"][0]["content"]["parts"][0]["text"]
                    .as_str()
                    .unwrap_or("No response generated.")
                    .to_string(),
                Err((status, err)) => {
                    return (status, Json(json!({"error": err})));
                }
            }
        }
    };

    // Post-process [[wa_send:<digits>]]<msg> and MEDIA:<path>
    let mut final_lines: Vec<String> = Vec::new();
    let mut audio_url: Option<String> = None;
    let wants_whatsapp = {
        let lower = req.message.to_lowercase();
        lower.contains("whatsapp")
            || lower.contains("whats")
            || req.message.contains("واتس")
            || req.message.contains("واتساب")
    };

    for line in raw_reply.lines() {
        let trimmed = line.trim();
        if trimmed == "[[audio_as_voice]]" {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("[[wa_send:") {
            if let Some((digits_raw, msg_body)) = rest.split_once("]]") {
                let digits: String = digits_raw.chars().filter(|c| c.is_ascii_digit()).collect();
                let target = if !digits.is_empty() {
                    Some(digits)
                } else {
                    user_wa.clone()
                };
                if let Some(num) = target {
                    let chat_id = format!("{}@s.whatsapp.net", num);
                    let res = client
                        .post(format!("{}/send", bridge_url()))
                        .bearer_auth(bridge_token())
                        .json(&json!({ "chatId": chat_id, "message": msg_body.trim() }))
                        .send()
                        .await;
                    if res.map(|r| r.status().is_success()).unwrap_or(false) {
                        final_lines.push(format!("✓ Sent to WhatsApp (+{})", num));
                    } else {
                        final_lines.push("✗ Failed to send WhatsApp message.".to_string());
                    }
                } else {
                    final_lines.push("Please link your WhatsApp number in Integrations first.".to_string());
                }
                continue;
            }
        }
        if let Some(media_path) = trimmed.strip_prefix("MEDIA:") {
            let media_path = media_path.trim();
            if let Some(rel) = media_path.strip_prefix("/opt/data/") {
                let src_host = PathBuf::from("/app/hermes_data").join(rel);
                if src_host.exists() {
                    let ext = src_host
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("mp3");
                    let web_name = format!("web_{}.{}", Uuid::new_v4(), ext);
                    let dst_host = audio_host_dir().join(&web_name);
                    if std::fs::copy(&src_host, &dst_host).is_ok() {
                        audio_url = Some(format!("/media-audio/{}", web_name));
                    }

                    // If user asked to send to WhatsApp and has linked number, also send PTT to WhatsApp
                    if wants_whatsapp {
                        if let Some(ref num) = user_wa {
                            let ogg_name = format!("wa_{}.ogg", Uuid::new_v4());
                            let ogg_host = audio_host_dir().join(&ogg_name);
                            let container_ogg = format!("/opt/data/audio/{}", ogg_name);
                            let converted = tokio::process::Command::new("ffmpeg")
                                .args([
                                    "-y",
                                    "-i",
                                    src_host.to_str().unwrap_or(""),
                                    "-c:a",
                                    "libopus",
                                    "-b:a",
                                    "32k",
                                    ogg_host.to_str().unwrap_or(""),
                                ])
                                .output()
                                .await
                                .map(|o| o.status.success())
                                .unwrap_or(false);

                            let send_path = if converted {
                                container_ogg
                            } else {
                                media_path.to_string()
                            };
                            let chat_id = format!("{}@s.whatsapp.net", num);
                            let wa_res = client
                                .post(format!("{}/send-media", bridge_url()))
                                .bearer_auth(bridge_token())
                                .json(&json!({
                                    "chatId": chat_id,
                                    "filePath": send_path,
                                    "mediaType": "audio"
                                }))
                                .send()
                                .await;
                            if wa_res.map(|r| r.status().is_success()).unwrap_or(false) {
                                final_lines.push(format!("✓ Voice note sent to WhatsApp (+{})", num));
                            }
                        }
                    }
                }
            }
            continue;
        }
        final_lines.push(line.to_string());
    }

    let mut final_reply = final_lines.join("\n").trim().to_string();
    if final_reply.is_empty() && audio_url.is_some() {
        final_reply = "🎙️ Voice note ready:".to_string();
    }

    {
        let conn = state.db.conn.lock().unwrap();
        let _ = conn.execute(
            "INSERT INTO chat_messages (id, user_id, role, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            (&Uuid::new_v4().to_string(), &user_id, "assistant", &final_reply, &Utc::now().to_rfc3339()),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "reply": final_reply,
            "audio_url": audio_url
        })),
    )
}
