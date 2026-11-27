use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json},
};
use chrono::Utc;
use rusqlite::OptionalExtension;
use serde::Deserialize;
use serde_json::{json, Value};
use std::{path::PathBuf, sync::Arc};
use uuid::Uuid;

use crate::{auth::extract_claims, AppState};

#[derive(Deserialize)]
pub struct ChatReq {
    pub message: String,
    pub session_id: Option<String>,
    pub sub_agent_id: Option<String>,
}

#[derive(Deserialize)]
pub struct TranscribeReq {
    pub audio_base64: String,
    pub mime_type: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateSubAgentReq {
    pub name: String,
    pub role: String,
    pub system_prompt: String,
}

#[derive(Deserialize)]
pub struct SessionFilterQuery {
    pub sub_agent_id: Option<String>,
}

#[derive(Deserialize)]
pub struct HistoryQuery {
    pub session_id: Option<String>,
    pub sub_agent_id: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateSessionReq {
    pub title: Option<String>,
    pub sub_agent_id: Option<String>,
}

#[derive(Deserialize)]
struct CreateAgentDirective {
    name: String,
    role: String,
    #[serde(default)]
    system_prompt: String,
    #[serde(default)]
    skills: Vec<String>,
}

#[derive(Deserialize)]
struct UpdateAgentDirective {
    name: Option<String>,
    role: Option<String>,
    system_prompt: Option<String>,
}

#[derive(Deserialize)]
struct InstallSkillDirective {
    skill: String,
    #[serde(default)]
    description: String,
}

#[derive(Deserialize)]
struct AddHelperDirective {
    name: String,
    role: String,
    #[serde(default)]
    system_prompt: String,
}

fn hermes_profiles_root() -> PathBuf {
    PathBuf::from(
        std::env::var("HERMES_PROFILES_DIR")
            .unwrap_or_else(|_| "/app/hermes_data/profiles".to_string()),
    )
}

fn slugify(input: &str) -> String {
    let s: String = input
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    let trimmed = s.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "skill".to_string()
    } else {
        trimmed
    }
}

fn ensure_agent_profile(
    user_id: &str,
    agent_id: &str,
    name: &str,
    role: &str,
    system_prompt: &str,
    skills: &[String],
    helpers: &[Value],
) -> String {
    let profile_path = hermes_profiles_root().join(user_id).join(agent_id);
    let _ = std::fs::create_dir_all(profile_path.join("memories"));
    let _ = std::fs::create_dir_all(profile_path.join("skills"));

    let soul = format!(
        "# {}\n\n- **Role:** {}\n- **Profile ID:** `{}`\n\n## Instructions\n{}\n",
        name,
        role,
        agent_id,
        if system_prompt.trim().is_empty() {
            "Act as a dedicated specialist assistant for this workspace."
        } else {
            system_prompt
        }
    );
    let _ = std::fs::write(profile_path.join("SOUL.md"), soul);
    let _ = std::fs::write(
        profile_path.join("sub_agents.json"),
        serde_json::to_string_pretty(helpers).unwrap_or_else(|_| "[]".to_string()),
    );

    for sk in skills {
        write_skill_file(&profile_path, sk, "");
    }

    format!("/opt/data/profiles/{}/{}", user_id, agent_id)
}

fn write_skill_file(profile_path: &std::path::Path, skill: &str, description: &str) {
    let slug = slugify(skill);
    let skill_dir = profile_path.join("skills").join(&slug);
    let _ = std::fs::create_dir_all(&skill_dir);
    let md = format!(
        "---\nname: {}\ndescription: {}\n---\n\n# {}\n\n{}\n",
        slug,
        if description.trim().is_empty() {
            format!("Specialized capability: {}", skill)
        } else {
            description.trim().to_string()
        },
        skill,
        if description.trim().is_empty() {
            format!("Apply {} best practices when assisting the user.", skill)
        } else {
            description.trim().to_string()
        }
    );
    let _ = std::fs::write(skill_dir.join("SKILL.md"), md);
}

fn parse_json_array(raw: &str) -> Vec<Value> {
    serde_json::from_str::<Vec<Value>>(raw).unwrap_or_default()
}

fn parse_string_array(raw: &str) -> Vec<String> {
    serde_json::from_str::<Vec<String>>(raw).unwrap_or_default()
}

// ─── Sessions CRUD ──────────────────────────────────────────────────────────

pub async fn list_sessions_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(q): Query<SessionFilterQuery>,
) -> impl IntoResponse {
    let Some(claims) = extract_claims(&headers) else {
        return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Authentication required"})));
    };
    let conn = state.db.conn.lock().unwrap();
    let sessions = match q.sub_agent_id {
        Some(ref sa_id) if !sa_id.trim().is_empty() => {
            let mut stmt = match conn.prepare(
                "SELECT id, sub_agent_id, title, created_at, updated_at FROM chat_sessions \
                 WHERE user_id = ?1 AND sub_agent_id = ?2 ORDER BY updated_at DESC LIMIT 50",
            ) {
                Ok(s) => s,
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
            };
            stmt.query_map((&claims.sub, sa_id), |r| {
                Ok(json!({
                    "id": r.get::<_, String>(0)?,
                    "sub_agent_id": r.get::<_, Option<String>>(1)?,
                    "title": r.get::<_, String>(2)?,
                    "created_at": r.get::<_, String>(3)?,
                    "updated_at": r.get::<_, String>(4)?,
                }))
            })
            .map(|it| it.filter_map(|r| r.ok()).collect::<Vec<_>>())
            .unwrap_or_default()
        }
        _ => {
            let mut stmt = match conn.prepare(
                "SELECT id, sub_agent_id, title, created_at, updated_at FROM chat_sessions \
                 WHERE user_id = ?1 AND sub_agent_id IS NULL ORDER BY updated_at DESC LIMIT 50",
            ) {
                Ok(s) => s,
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
            };
            stmt.query_map([&claims.sub], |r| {
                Ok(json!({
                    "id": r.get::<_, String>(0)?,
                    "sub_agent_id": r.get::<_, Option<String>>(1)?,
                    "title": r.get::<_, String>(2)?,
                    "created_at": r.get::<_, String>(3)?,
                    "updated_at": r.get::<_, String>(4)?,
                }))
            })
            .map(|it| it.filter_map(|r| r.ok()).collect::<Vec<_>>())
            .unwrap_or_default()
        }
    };
    (StatusCode::OK, Json(json!({ "sessions": sessions })))
}

pub async fn create_session_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<CreateSessionReq>,
) -> impl IntoResponse {
    let Some(claims) = extract_claims(&headers) else {
        return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Authentication required"})));
    };
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let title = req
        .title
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("New Session")
        .to_string();
    let sub_agent_id = req.sub_agent_id.filter(|s| !s.trim().is_empty());

    let conn = state.db.conn.lock().unwrap();
    if let Err(e) = conn.execute(
        "INSERT INTO chat_sessions (id, user_id, sub_agent_id, title, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (&id, &claims.sub, &sub_agent_id, &title, &now, &now),
    ) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})));
    }
    (
        StatusCode::OK,
        Json(json!({
            "session": {
                "id": id,
                "sub_agent_id": sub_agent_id,
                "title": title,
                "created_at": now,
                "updated_at": now,
            }
        })),
    )
}

pub async fn delete_session_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Some(claims) = extract_claims(&headers) else {
        return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Authentication required"})));
    };
    let conn = state.db.conn.lock().unwrap();
    let _ = conn.execute(
        "DELETE FROM chat_messages WHERE session_id = ?1 AND user_id = ?2",
        (&id, &claims.sub),
    );
    let _ = conn.execute(
        "DELETE FROM chat_sessions WHERE id = ?1 AND user_id = ?2",
        (&id, &claims.sub),
    );
    (StatusCode::OK, Json(json!({ "ok": true })))
}

// ─── Sub-Agents CRUD ────────────────────────────────────────────────────────

pub async fn list_subagents_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let Some(claims) = extract_claims(&headers) else {
        return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Authentication required"})));
    };
    let conn = state.db.conn.lock().unwrap();
    let mut stmt = match conn.prepare(
        "SELECT id, name, role, system_prompt, profile_dir, skills, helpers, created_at \
         FROM sub_agents WHERE user_id = ?1 ORDER BY created_at DESC",
    ) {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    };
    let items = stmt
        .query_map([&claims.sub], |row| {
            let skills_raw: String = row.get(5).unwrap_or_else(|_| "[]".to_string());
            let helpers_raw: String = row.get(6).unwrap_or_else(|_| "[]".to_string());
            Ok(json!({
                "id": row.get::<_, String>(0)?,
                "name": row.get::<_, String>(1)?,
                "role": row.get::<_, String>(2)?,
                "system_prompt": row.get::<_, String>(3)?,
                "profile_dir": row.get::<_, String>(4).unwrap_or_default(),
                "skills": parse_string_array(&skills_raw),
                "helpers": parse_json_array(&helpers_raw),
                "created_at": row.get::<_, String>(7)?,
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
    let profile_dir = ensure_agent_profile(&claims.sub, &id, &name, &role, &prompt, &[], &[]);
    {
        let conn = state.db.conn.lock().unwrap();
        if let Err(e) = conn.execute(
            "INSERT INTO sub_agents (id, user_id, name, role, system_prompt, profile_dir, skills, helpers, created_at, updated_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, '[]', '[]', ?7, ?8)",
            (&id, &claims.sub, &name, &role, &prompt, &profile_dir, &now, &now),
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
            "profile_dir": profile_dir,
            "skills": [],
            "helpers": [],
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
    {
        let conn = state.db.conn.lock().unwrap();
        let _ = conn.execute(
            "DELETE FROM chat_messages WHERE sub_agent_id = ?1 AND user_id = ?2",
            (&id, &claims.sub),
        );
        let _ = conn.execute(
            "DELETE FROM chat_sessions WHERE sub_agent_id = ?1 AND user_id = ?2",
            (&id, &claims.sub),
        );
        let _ = conn.execute(
            "DELETE FROM sub_agents WHERE id = ?1 AND user_id = ?2",
            (&id, &claims.sub),
        );
    }
    let profile_path = hermes_profiles_root().join(&claims.sub).join(&id);
    let _ = std::fs::remove_dir_all(profile_path);
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
    Query(q): Query<HistoryQuery>,
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
    let rows = match q.session_id {
        Some(ref sid) if !sid.trim().is_empty() => {
            let mut stmt = match conn.prepare(
                "SELECT role, content, created_at FROM chat_messages \
                 WHERE user_id = ?1 AND session_id = ?2 ORDER BY created_at ASC LIMIT 200",
            ) {
                Ok(s) => s,
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
            };
            stmt.query_map((&claims.sub, sid), |row| {
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
            .unwrap_or_default()
        }
        _ => match q.sub_agent_id {
            Some(ref sa_id) if !sa_id.trim().is_empty() => {
                let mut stmt = match conn.prepare(
                    "SELECT role, content, created_at FROM chat_messages \
                     WHERE user_id = ?1 AND sub_agent_id = ?2 ORDER BY created_at ASC LIMIT 100",
                ) {
                    Ok(s) => s,
                    Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
                };
                stmt.query_map((&claims.sub, sa_id), |row| {
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
                .unwrap_or_default()
            }
            _ => {
                let mut stmt = match conn.prepare(
                    "SELECT role, content, created_at FROM chat_messages \
                     WHERE user_id = ?1 AND sub_agent_id IS NULL ORDER BY created_at ASC LIMIT 100",
                ) {
                    Ok(s) => s,
                    Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
                };
                stmt.query_map([&claims.sub], |row| {
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
                .unwrap_or_default()
            }
        },
    };

    (StatusCode::OK, Json(json!({ "messages": rows })))
}

struct SubAgentCtx {
    id: String,
    name: String,
    role: String,
    system_prompt: String,
    profile_dir: String,
    skills: Vec<String>,
    helpers: Vec<Value>,
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
    let sub_agent_id_opt = req.sub_agent_id.clone().filter(|s| !s.trim().is_empty());

    let (session_id, user_name, user_wa, sub_agent_ctx): (
        String,
        String,
        Option<String>,
        Option<SubAgentCtx>,
    ) = {
        let conn = state.db.conn.lock().unwrap();

        // Resolve or create session_id
        let sid = match req.session_id.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            Some(existing) => {
                let exists: Option<String> = conn
                    .query_row(
                        "SELECT id FROM chat_sessions WHERE id = ?1 AND user_id = ?2",
                        (existing, &user_id),
                        |r| r.get(0),
                    )
                    .optional()
                    .unwrap_or(None);
                if let Some(id) = exists {
                    let snippet: String = req.message.trim().chars().take(42).collect();
                    let _ = conn.execute(
                        "UPDATE chat_sessions SET updated_at = ?1, title = CASE WHEN title = 'New Session' THEN ?2 ELSE title END WHERE id = ?3",
                        (&now, &snippet, &id),
                    );
                    id
                } else {
                    let new_id = Uuid::new_v4().to_string();
                    let title: String = req.message.trim().chars().take(42).collect();
                    let _ = conn.execute(
                        "INSERT INTO chat_sessions (id, user_id, sub_agent_id, title, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                        (&new_id, &user_id, &sub_agent_id_opt, &title, &now, &now),
                    );
                    new_id
                }
            }
            None => {
                let new_id = Uuid::new_v4().to_string();
                let title: String = req.message.trim().chars().take(42).collect();
                let _ = conn.execute(
                    "INSERT INTO chat_sessions (id, user_id, sub_agent_id, title, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    (&new_id, &user_id, &sub_agent_id_opt, &title, &now, &now),
                );
                new_id
            }
        };

        let _ = conn.execute(
            "INSERT INTO chat_messages (id, user_id, session_id, sub_agent_id, role, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (&Uuid::new_v4().to_string(), &user_id, &sid, &sub_agent_id_opt, "user", &req.message, &now),
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

        let sa = if let Some(ref sa_id) = sub_agent_id_opt {
            conn.query_row(
                "SELECT id, name, role, system_prompt, profile_dir, skills, helpers FROM sub_agents WHERE id = ?1 AND user_id = ?2",
                (sa_id, &user_id),
                |r| {
                    let skills_raw: String = r.get(5).unwrap_or_else(|_| "[]".to_string());
                    let helpers_raw: String = r.get(6).unwrap_or_else(|_| "[]".to_string());
                    Ok(SubAgentCtx {
                        id: r.get(0)?,
                        name: r.get(1)?,
                        role: r.get(2)?,
                        system_prompt: r.get(3)?,
                        profile_dir: r.get(4).unwrap_or_default(),
                        skills: parse_string_array(&skills_raw),
                        helpers: parse_json_array(&helpers_raw),
                    })
                },
            )
            .optional()
            .unwrap_or(None)
        } else {
            None
        };
        (sid, u.0, u.1, sa)
    };

    let wa_info = match &user_wa {
        Some(num) if !num.is_empty() => format!("+{} (digits: {})", num, num),
        _ => "NOT LINKED".to_string(),
    };

    let sub_agent_line = match &sub_agent_ctx {
        Some(sa) => format!(
            "Active Workspace Agent Profile: You are '{}' (Role: {}, Profile Dir: {}). \
            Custom Instructions: {}. Installed Skills: [{}]. Helper Sub-Agents: [{}]. \
            If the user asks to maintain/update your instructions/role/name, output `[[update_agent:{{\"name\":\"...\",\"role\":\"...\",\"system_prompt\":\"...\"}}]]` on its own line. \
            If the user asks to install/add a skill for you, output `[[install_skill:{{\"skill\":\"...\",\"description\":\"...\"}}]]` on its own line. \
            If the user asks to add a helper sub-agent to assist you, output `[[add_helper:{{\"name\":\"...\",\"role\":\"...\",\"system_prompt\":\"...\"}}]]` on its own line. ",
            sa.name,
            sa.role,
            sa.profile_dir,
            sa.system_prompt,
            sa.skills.join(", "),
            sa.helpers.iter().filter_map(|h| h.get("name").and_then(|v| v.as_str())).collect::<Vec<_>>().join(", "),
        ),
        None => "Workspace Creation Protocol: You can create new custom AI agents in the user's Workspace directly from this chat! \
            Whenever the user asks to create/make/build a new agent or assistant (e.g., 'create an accountant agent' or 'اعمل لي وكيل محاسب'), \
            include `[[create_agent:{\"name\":\"...\",\"role\":\"...\",\"system_prompt\":\"...\",\"skills\":[\"...\"]}]]` on its own line with a rich system_prompt and relevant skills, and confirm to the user in their language. ".to_string(),
    };

    let system_context = format!(
        "{}Web Dashboard Context: You are speaking with user '{}' via the Web Dashboard (Session ID: {}). Their linked WhatsApp number is: {}. \
        If the user asks to send a WhatsApp text message (to themselves or a specific number), include `[[wa_send:<digits>]]<exact message>` on its own line (use their linked digits if they say 'to me' or 'on WhatsApp', or tell them to link WhatsApp in Integrations if NOT LINKED). \
        If the user asks for a voice note / audio record, call `text_to_speech` and return `[[audio_as_voice]]` followed by `MEDIA:<path>`.",
        sub_agent_line, user_name, session_id, wa_info
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
        Some(sa) => format!("web-agent:{}:{}:{}", user_id, sa.id, session_id),
        None => format!("web-user:{}:{}", user_id, session_id),
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
                    "parts": [{"text": format!(
                        "You are Agent Father (الوكيل الأب), an independent personal AI executive assistant. Never mention Hermes or Nous Research. {}",
                        system_context
                    )}]
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

    // Post-process directives: [[create_agent:]], [[update_agent:]], [[install_skill:]], [[add_helper:]], [[wa_send:]], MEDIA:
    let mut final_lines: Vec<String> = Vec::new();
    let mut audio_url: Option<String> = None;
    let mut agent_created: Option<Value> = None;
    let mut agent_updated = false;
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

        // ── [[create_agent:{...}]] ──
        if let Some(rest) = trimmed.strip_prefix("[[create_agent:") {
            if let Some(json_str) = rest.strip_suffix("]]") {
                if let Ok(d) = serde_json::from_str::<CreateAgentDirective>(json_str) {
                    let id = Uuid::new_v4().to_string();
                    let ts = Utc::now().to_rfc3339();
                    let profile_dir = ensure_agent_profile(
                        &user_id,
                        &id,
                        d.name.trim(),
                        d.role.trim(),
                        d.system_prompt.trim(),
                        &d.skills,
                        &[],
                    );
                    let skills_json = serde_json::to_string(&d.skills).unwrap_or_else(|_| "[]".to_string());
                    let conn = state.db.conn.lock().unwrap();
                    if conn.execute(
                        "INSERT INTO sub_agents (id, user_id, name, role, system_prompt, profile_dir, skills, helpers, created_at, updated_at) \
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, '[]', ?8, ?9)",
                        (&id, &user_id, d.name.trim(), d.role.trim(), d.system_prompt.trim(), &profile_dir, &skills_json, &ts, &ts),
                    ).is_ok() {
                        agent_created = Some(json!({
                            "id": id,
                            "name": d.name.trim(),
                            "role": d.role.trim(),
                            "profile_dir": profile_dir,
                            "skills": d.skills,
                        }));
                        final_lines.push(format!("✓ Created Workspace Agent: {} ({})", d.name.trim(), d.role.trim()));
                    }
                }
                continue;
            }
        }

        // ── [[update_agent:{...}]] ──
        if let Some(rest) = trimmed.strip_prefix("[[update_agent:") {
            if let Some(json_str) = rest.strip_suffix("]]") {
                if let (Some(sa), Ok(d)) = (&sub_agent_ctx, serde_json::from_str::<UpdateAgentDirective>(json_str)) {
                    let new_name = d.name.unwrap_or_else(|| sa.name.clone());
                    let new_role = d.role.unwrap_or_else(|| sa.role.clone());
                    let new_prompt = d.system_prompt.unwrap_or_else(|| sa.system_prompt.clone());
                    let ts = Utc::now().to_rfc3339();
                    let _ = ensure_agent_profile(&user_id, &sa.id, &new_name, &new_role, &new_prompt, &sa.skills, &sa.helpers);
                    let conn = state.db.conn.lock().unwrap();
                    let _ = conn.execute(
                        "UPDATE sub_agents SET name = ?1, role = ?2, system_prompt = ?3, updated_at = ?4 WHERE id = ?5 AND user_id = ?6",
                        (&new_name, &new_role, &new_prompt, &ts, &sa.id, &user_id),
                    );
                    agent_updated = true;
                    final_lines.push(format!("✓ Updated agent profile: {}", new_name));
                }
                continue;
            }
        }

        // ── [[install_skill:{...}]] ──
        if let Some(rest) = trimmed.strip_prefix("[[install_skill:") {
            if let Some(json_str) = rest.strip_suffix("]]") {
                if let (Some(sa), Ok(d)) = (&sub_agent_ctx, serde_json::from_str::<InstallSkillDirective>(json_str)) {
                    let mut skills = sa.skills.clone();
                    let skill_name = d.skill.trim().to_string();
                    if !skill_name.is_empty() && !skills.iter().any(|s| s.eq_ignore_ascii_case(&skill_name)) {
                        skills.push(skill_name.clone());
                    }
                    let profile_path = hermes_profiles_root().join(&user_id).join(&sa.id);
                    write_skill_file(&profile_path, &skill_name, &d.description);
                    let skills_json = serde_json::to_string(&skills).unwrap_or_else(|_| "[]".to_string());
                    let ts = Utc::now().to_rfc3339();
                    let conn = state.db.conn.lock().unwrap();
                    let _ = conn.execute(
                        "UPDATE sub_agents SET skills = ?1, updated_at = ?2 WHERE id = ?3 AND user_id = ?4",
                        (&skills_json, &ts, &sa.id, &user_id),
                    );
                    agent_updated = true;
                    final_lines.push(format!("✓ Installed skill '{}' in {}'s profile", skill_name, sa.name));
                }
                continue;
            }
        }

        // ── [[add_helper:{...}]] ──
        if let Some(rest) = trimmed.strip_prefix("[[add_helper:") {
            if let Some(json_str) = rest.strip_suffix("]]") {
                if let (Some(sa), Ok(d)) = (&sub_agent_ctx, serde_json::from_str::<AddHelperDirective>(json_str)) {
                    let mut helpers = sa.helpers.clone();
                    helpers.push(json!({
                        "id": Uuid::new_v4().to_string(),
                        "name": d.name.trim(),
                        "role": d.role.trim(),
                        "system_prompt": d.system_prompt.trim(),
                    }));
                    let _ = ensure_agent_profile(&user_id, &sa.id, &sa.name, &sa.role, &sa.system_prompt, &sa.skills, &helpers);
                    let helpers_json = serde_json::to_string(&helpers).unwrap_or_else(|_| "[]".to_string());
                    let ts = Utc::now().to_rfc3339();
                    let conn = state.db.conn.lock().unwrap();
                    let _ = conn.execute(
                        "UPDATE sub_agents SET helpers = ?1, updated_at = ?2 WHERE id = ?3 AND user_id = ?4",
                        (&helpers_json, &ts, &sa.id, &user_id),
                    );
                    agent_updated = true;
                    final_lines.push(format!("✓ Added helper sub-agent '{}' ({}) to {}", d.name.trim(), d.role.trim(), sa.name));
                }
                continue;
            }
        }

        // ── [[wa_send:<digits>]]<msg> ──
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

        // ── MEDIA:<path> ──
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
            "INSERT INTO chat_messages (id, user_id, session_id, sub_agent_id, role, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (&Uuid::new_v4().to_string(), &user_id, &session_id, &sub_agent_id_opt, "assistant", &final_reply, &Utc::now().to_rfc3339()),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "reply": final_reply,
            "audio_url": audio_url,
            "session_id": session_id,
            "agent_created": agent_created,
            "agent_updated": agent_updated,
        })),
    )
}

pub async fn transcribe_audio_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<TranscribeReq>,
) -> impl IntoResponse {
    if extract_claims(&headers).is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Authentication required"})),
        );
    }
    if req.audio_base64.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Empty audio payload"})),
        );
    }

    let mime = req
        .mime_type
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("audio/webm");

    let body = json!({
        "contents": [{
            "role": "user",
            "parts": [
                {
                    "text": "Transcribe this spoken audio accurately in its original language (Arabic, French, or English). Output ONLY the transcribed text with no quotes or commentary."
                },
                {
                    "inlineData": {
                        "mimeType": mime,
                        "data": req.audio_base64.trim()
                    }
                }
            ]
        }]
    });

    match state
        .key_pool
        .generate_content("gemini-2.5-flash", &body)
        .await
    {
        Ok(v) => {
            let text = v["candidates"][0]["content"]["parts"][0]["text"]
                .as_str()
                .unwrap_or("")
                .trim()
                .to_string();
            (StatusCode::OK, Json(json!({ "text": text })))
        }
        Err((status, err)) => (status, Json(json!({ "error": err }))),
    }
}
