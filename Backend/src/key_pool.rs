use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Json, Response},
};
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    path::Path,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
use tokio::sync::RwLock;

use crate::AppState;

// ─── Gemini keys: loaded from GEMINI_KEYS env var (comma-separated) ────────
const DEFAULT_GEMINI_KEYS: &[&str] = &[];

// ─── Groq keys: loaded from GROQ_KEYS env var (comma-separated) ────────────
const DEFAULT_GROQ_KEYS: &[&str] = &[];

// ─── Groq TTS models: Arabic + English auto-detect ─────────────────────────
// Arabic text → canopylabs/orpheus-arabic-saudi
// English/mixed → canopylabs/orpheus-v1-english  (fallback: gemini tts)
#[allow(dead_code)]
const GROQ_TTS_MODEL_AR: &str = "canopylabs/orpheus-arabic-saudi";
#[allow(dead_code)]
const GROQ_TTS_MODEL_EN: &str = "canopylabs/orpheus-v1-english";

// ─── KeySlot ───────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct KeySlot {
    pub key: String,
    pub masked: String,
    pub cooldown_until: Option<Instant>,
    pub requests_ok: u64,
    pub quota_rotations: u64,
    pub last_status: Option<u16>,
}

fn make_slot(k: &str) -> KeySlot {
    let len = k.len();
    let masked = if len > 12 {
        format!("{}...{}", &k[..8], &k[len - 4..])
    } else {
        "****".to_string()
    };
    KeySlot {
        key: k.to_string(),
        masked,
        cooldown_until: None,
        requests_ok: 0,
        quota_rotations: 0,
        last_status: None,
    }
}

// ─── Generic key pool ──────────────────────────────────────────────────────
pub struct KeyPool {
    slots: RwLock<Vec<KeySlot>>,
    cursor: AtomicUsize,
    client: reqwest::Client,
    // separate Groq pool for TTS
    groq_slots: RwLock<Vec<KeySlot>>,
    #[allow(dead_code)]
    groq_cursor: AtomicUsize,
    thought_signatures: RwLock<std::collections::HashMap<String, String>>,
}

impl KeyPool {
    pub fn new() -> Self {
        // ── Gemini keys ──
        let gemini_raw: Vec<String> = std::env::var("GEMINI_KEYS")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(|s| {
                s.split(',')
                    .map(|k| k.trim().to_string())
                    .filter(|k| !k.is_empty())
                    .collect()
            })
            .unwrap_or_else(|| DEFAULT_GEMINI_KEYS.iter().map(|s| s.to_string()).collect());

        let slots: Vec<KeySlot> = gemini_raw.iter().map(|k| make_slot(k)).collect();

        // ── Groq keys ──
        let groq_raw: Vec<String> = std::env::var("GROQ_KEYS")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(|s| {
                s.split(',')
                    .map(|k| k.trim().to_string())
                    .filter(|k| !k.is_empty())
                    .collect()
            })
            .unwrap_or_else(|| DEFAULT_GROQ_KEYS.iter().map(|s| s.to_string()).collect());

        let groq_slots: Vec<KeySlot> = groq_raw.iter().map(|k| make_slot(k)).collect();

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .expect("reqwest client");

        Self {
            slots: RwLock::new(slots),
            cursor: AtomicUsize::new(0),
            client,
            groq_slots: RwLock::new(groq_slots),
            groq_cursor: AtomicUsize::new(0),
            thought_signatures: RwLock::new(std::collections::HashMap::new()),
        }
    }

    pub async fn store_thought_sig(&self, call_id: &str, sig: &str) {
        let mut map = self.thought_signatures.write().await;
        if map.len() > 2000 {
            map.clear();
        }
        map.insert(call_id.to_string(), sig.to_string());
    }

    pub async fn get_thought_sig(&self, call_id: &str) -> Option<String> {
        if call_id.is_empty() {
            return None;
        }
        self.thought_signatures.read().await.get(call_id).cloned()
    }

    // ── Gemini key picker ──
    async fn pick_key(&self) -> (usize, String) {
        let now = Instant::now();
        let slots = self.slots.read().await;
        let n = slots.len();
        let start = self.cursor.fetch_add(1, Ordering::Relaxed) % n;
        for offset in 0..n {
            let idx = (start + offset) % n;
            match slots[idx].cooldown_until {
                Some(until) if until > now => continue,
                _ => return (idx, slots[idx].key.clone()),
            }
        }
        // all cooling — pick soonest
        let mut best = start;
        for idx in 0..n {
            if slots[idx].cooldown_until < slots[best].cooldown_until {
                best = idx;
            }
        }
        (best, slots[best].key.clone())
    }

    async fn mark_ok(&self, idx: usize) {
        let mut slots = self.slots.write().await;
        if let Some(s) = slots.get_mut(idx) {
            s.cooldown_until = None;
            s.requests_ok += 1;
            s.last_status = Some(200);
        }
    }

    async fn mark_rotate(&self, idx: usize, status: u16, cooldown_secs: u64) {
        let mut slots = self.slots.write().await;
        if let Some(s) = slots.get_mut(idx) {
            s.cooldown_until = Some(Instant::now() + Duration::from_secs(cooldown_secs));
            s.quota_rotations += 1;
            s.last_status = Some(status);
        }
    }

    // ── Groq key picker ──
    #[allow(dead_code)]
    async fn pick_groq_key(&self) -> (usize, String) {
        let now = Instant::now();
        let slots = self.groq_slots.read().await;
        let n = slots.len();
        let start = self.groq_cursor.fetch_add(1, Ordering::Relaxed) % n;
        for offset in 0..n {
            let idx = (start + offset) % n;
            match slots[idx].cooldown_until {
                Some(until) if until > now => continue,
                _ => return (idx, slots[idx].key.clone()),
            }
        }
        let mut best = start;
        for idx in 0..n {
            if slots[idx].cooldown_until < slots[best].cooldown_until {
                best = idx;
            }
        }
        (best, slots[best].key.clone())
    }

    #[allow(dead_code)]
    async fn mark_groq_ok(&self, idx: usize) {
        let mut slots = self.groq_slots.write().await;
        if let Some(s) = slots.get_mut(idx) {
            s.cooldown_until = None;
            s.requests_ok += 1;
            s.last_status = Some(200);
        }
    }

    #[allow(dead_code)]
    async fn mark_groq_rotate(&self, idx: usize, status: u16, cooldown_secs: u64) {
        let mut slots = self.groq_slots.write().await;
        if let Some(s) = slots.get_mut(idx) {
            s.cooldown_until = Some(Instant::now() + Duration::from_secs(cooldown_secs));
            s.quota_rotations += 1;
            s.last_status = Some(status);
        }
    }

    // ── Status snapshot (Gemini keys) ──
    pub async fn status_snapshot(&self) -> Vec<Value> {
        let now = Instant::now();
        let slots = self.slots.read().await;
        slots
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let cooling = s.cooldown_until.map(|u| u > now).unwrap_or(false);
                json!({
                    "index": i + 1,
                    "key": s.masked,
                    "healthy": !cooling,
                    "requests_ok": s.requests_ok,
                    "quota_rotations": s.quota_rotations,
                    "last_status": s.last_status
                })
            })
            .collect()
    }

    // ── Groq TTS status snapshot ──
    pub async fn groq_status_snapshot(&self) -> Vec<Value> {
        let now = Instant::now();
        let slots = self.groq_slots.read().await;
        slots
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let cooling = s.cooldown_until.map(|u| u > now).unwrap_or(false);
                json!({
                    "index": i + 1,
                    "key": s.masked,
                    "healthy": !cooling,
                    "requests_ok": s.requests_ok,
                    "quota_rotations": s.quota_rotations,
                    "last_status": s.last_status
                })
            })
            .collect()
    }

    // ══════════════════════════════════════════════════════════════════════
    // Gemini generateContent (LLM only — no TTS)
    // ══════════════════════════════════════════════════════════════════════
    pub async fn generate_content(
        &self,
        model: &str,
        body: &Value,
    ) -> Result<Value, (StatusCode, String)> {
        let n = self.slots.read().await.len();
        let mut last_err = String::from("All Gemini API keys exhausted");

        for attempt in 0..(n * 2) {
            let (idx, key) = self.pick_key().await;
            let url = format!(
                "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
                model, key
            );

            match self.client.post(&url).json(body).send().await {
                Ok(resp) => {
                    let code = resp.status().as_u16();
                    if resp.status().is_success() {
                        match resp.json::<Value>().await {
                            Ok(v) => {
                                self.mark_ok(idx).await;
                                return Ok(v);
                            }
                            Err(e) => {
                                last_err = format!("JSON decode error: {e}");
                                self.mark_rotate(idx, code, 5).await;
                            }
                        }
                    } else {
                        let text = resp.text().await.unwrap_or_default();
                        last_err = format!("HTTP {code}: {text}");
                        let cd = if code == 429 || code == 403 { 60 } else { 8 };
                        tracing::warn!(
                            "Gemini key #{} rotated on HTTP {} (attempt {}): {}",
                            idx + 1,
                            code,
                            attempt + 1,
                            &text[..text.len().min(160)]
                        );
                        self.mark_rotate(idx, code, cd).await;
                    }
                }
                Err(e) => {
                    last_err = format!("Network error: {e}");
                    tracing::warn!("Gemini key #{} network error: {}", idx + 1, e);
                    self.mark_rotate(idx, 504, 5).await;
                }
            }
        }

        Err((StatusCode::BAD_GATEWAY, last_err))
    }

    // ══════════════════════════════════════════════════════════════════════
    // Groq TTS → returns MP3 bytes (fast, no ffmpeg needed)
    // Auto-detects Arabic vs English from text content.
    // Falls back to Gemini TTS if all Groq keys are exhausted.
    // ══════════════════════════════════════════════════════════════════════
    pub async fn generate_tts_ogg_bytes(&self, text: &str) -> Result<Vec<u8>, String> {
        let tmp = std::path::Path::new("/tmp")
            .join(format!("af_tts_{}.ogg", uuid::Uuid::new_v4()));
        self.generate_tts_ogg_gemini(text, &tmp).await?;
        let bytes = tokio::fs::read(&tmp)
            .await
            .map_err(|e| format!("Read OGG failed: {e}"))?;
        let _ = tokio::fs::remove_file(&tmp).await;
        Ok(bytes)
    }

    // ── Groq TTS attempt (all keys) ──
    #[allow(dead_code)]
    async fn try_groq_tts(&self, text: &str) -> Result<Vec<u8>, String> {
        let n = self.groq_slots.read().await.len();
        let mut last_err = String::from("All Groq TTS keys exhausted");

        // Pick model: Arabic if majority of chars are Arabic
        let arabic_char_count = text.chars().filter(|c| {
            let n = *c as u32;
            (0x0600..=0x06FF).contains(&n) || (0x0750..=0x077F).contains(&n)
        }).count();
        let model = if arabic_char_count > text.chars().count() / 3 {
            GROQ_TTS_MODEL_AR
        } else {
            GROQ_TTS_MODEL_EN
        };

        for attempt in 0..(n * 2) {
            let (idx, key) = self.pick_groq_key().await;
            let body = json!({
                "model": model,
                "input": text,
                "response_format": "mp3"
            });

            match self
                .client
                .post("https://api.groq.com/openai/v1/audio/speech")
                .header("Authorization", format!("Bearer {}", key))
                .json(&body)
                .send()
                .await
            {
                Ok(resp) => {
                    let code = resp.status().as_u16();
                    if resp.status().is_success() {
                        match resp.bytes().await {
                            Ok(b) if !b.is_empty() => {
                                self.mark_groq_ok(idx).await;
                                tracing::info!(
                                    "Groq TTS OK: model={} key=#{} {} bytes",
                                    model, idx + 1, b.len()
                                );
                                return Ok(b.to_vec());
                            }
                            Ok(_) => {
                                last_err = "Groq returned empty audio".to_string();
                                self.mark_groq_rotate(idx, 500, 5).await;
                            }
                            Err(e) => {
                                last_err = format!("Read bytes error: {e}");
                                self.mark_groq_rotate(idx, code, 5).await;
                            }
                        }
                    } else {
                        let text_body = resp.text().await.unwrap_or_default();
                        last_err = format!("Groq HTTP {code}: {text_body}");
                        let cd = if code == 429 { 60 } else if text_body.contains("terms") { 3600 } else { 10 };
                        tracing::warn!(
                            "Groq TTS key #{} rotated on HTTP {} (attempt {}): {}",
                            idx + 1, code, attempt + 1,
                            &text_body[..text_body.len().min(200)]
                        );
                        self.mark_groq_rotate(idx, code, cd).await;
                    }
                }
                Err(e) => {
                    last_err = format!("Groq network error: {e}");
                    self.mark_groq_rotate(idx, 504, 5).await;
                }
            }
        }

        Err(last_err)
    }

    // ── Gemini TTS fallback ──
    async fn generate_tts_ogg_gemini(
        &self,
        text: &str,
        output_ogg_path: &Path,
    ) -> Result<(), String> {
        let body = json!({
            "contents": [{"parts": [{"text": format!("Read the following text aloud exactly as written:\n\n{}", text)}]}],
            "generationConfig": {
                "responseModalities": ["AUDIO"],
                "speechConfig": {
                    "voiceConfig": {
                        "prebuiltVoiceConfig": {
                            "voiceName": "Fenrir"
                        }
                    }
                }
            }
        });

        let resp = self
            .generate_content("gemini-2.5-flash-preview-tts", &body)
            .await
            .map_err(|(_, e)| e)?;

        let b64 = resp["candidates"][0]["content"]["parts"][0]["inlineData"]["data"]
            .as_str()
            .ok_or_else(|| "No audio inlineData in Gemini TTS response".to_string())?;

        let pcm_bytes = B64
            .decode(b64)
            .map_err(|e| format!("Base64 decode error: {e}"))?;

        let pcm_path = output_ogg_path.with_extension("pcm");
        tokio::fs::write(&pcm_path, &pcm_bytes)
            .await
            .map_err(|e| format!("Write PCM failed: {e}"))?;

        let status = tokio::process::Command::new("ffmpeg")
            .args([
                "-y", "-f", "s16le", "-ar", "24000", "-ac", "1",
                "-i", pcm_path.to_str().unwrap_or_default(),
                "-c:a", "libopus", "-b:a", "32k",
                output_ogg_path.to_str().unwrap_or_default(),
            ])
            .output()
            .await
            .map_err(|e| format!("ffmpeg exec failed: {e}"))?;

        let _ = tokio::fs::remove_file(&pcm_path).await;

        if !status.status.success() {
            let err = String::from_utf8_lossy(&status.stderr);
            return Err(format!("ffmpeg conversion error: {err}"));
        }

        Ok(())
    }
}

// ── ffmpeg: MP3 → OGG Opus (for WhatsApp) ─────────────────────────────────
#[allow(dead_code)]
async fn convert_mp3_to_ogg(mp3_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mp3_tmp = format!("/tmp/af_tts_{}.mp3", uuid::Uuid::new_v4());
    let ogg_tmp = format!("/tmp/af_tts_{}.ogg", uuid::Uuid::new_v4());

    tokio::fs::write(&mp3_tmp, mp3_bytes)
        .await
        .map_err(|e| format!("Write MP3 tmp failed: {e}"))?;

    let status = tokio::process::Command::new("ffmpeg")
        .args([
            "-y", "-i", &mp3_tmp,
            "-c:a", "libopus", "-b:a", "32k",
            &ogg_tmp,
        ])
        .output()
        .await
        .map_err(|e| format!("ffmpeg mp3→ogg failed: {e}"))?;

    let _ = tokio::fs::remove_file(&mp3_tmp).await;

    if !status.status.success() {
        let _ = tokio::fs::remove_file(&ogg_tmp).await;
        return Err(format!(
            "ffmpeg mp3→ogg error: {}",
            String::from_utf8_lossy(&status.stderr)
        ));
    }

    let bytes = tokio::fs::read(&ogg_tmp)
        .await
        .map_err(|e| format!("Read OGG failed: {e}"))?;
    let _ = tokio::fs::remove_file(&ogg_tmp).await;
    Ok(bytes)
}

// ══════════════════════════════════════════════════════════════════════════
// HTTP Handlers
// ══════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
pub struct InternalTtsReq {
    pub text: String,
}

pub async fn internal_tts_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InternalTtsReq>,
) -> Response {
    if req.text.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "text is empty"})),
        )
            .into_response();
    }
    match state.key_pool.generate_tts_ogg_bytes(&req.text).await {
        Ok(bytes) => (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, "audio/ogg"),
                (header::CACHE_CONTROL, "no-store"),
            ],
            bytes,
        )
            .into_response(),
        Err(e) => {
            tracing::error!("TTS error: {}", e);
            (StatusCode::BAD_GATEWAY, Json(json!({"error": e}))).into_response()
        }
    }
}

pub async fn key_pool_status_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let keys = state.key_pool.status_snapshot().await;
    let groq_keys = state.key_pool.groq_status_snapshot().await;
    Json(json!({
        "total_keys": keys.len(),
        "keys": keys,
        "groq_tts_keys": groq_keys
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// OpenAI-compatible proxy handlers (LLM only — Gemini backend)
// ══════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
pub struct OpenAiChatRequest {
    pub model: Option<String>,
    pub messages: Vec<OpenAiMessage>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<u64>,
    pub stream: Option<bool>,
    pub tools: Option<Vec<Value>>,
}

#[derive(Deserialize, Serialize)]
pub struct OpenAiMessage {
    pub role: String,
    pub content: Option<Value>,
    pub tool_calls: Option<Vec<Value>>,
    pub tool_call_id: Option<String>,
    pub name: Option<String>,
}

fn extract_text(content: Option<&Value>) -> String {
    let Some(c) = content else {
        return String::new();
    };
    if let Some(s) = c.as_str() {
        return s.to_string();
    }
    if let Some(arr) = c.as_array() {
        return arr
            .iter()
            .filter_map(|part| part.get("text").and_then(|t| t.as_str()))
            .collect::<Vec<_>>()
            .join("\n");
    }
    if c.is_null() {
        return String::new();
    }
    c.to_string()
}

/// Strip unsupported JSON Schema fields for Gemini functionDeclarations.
fn clean_schema(val: &Value) -> Value {
    match val {
        Value::Object(map) => {
            let mut out = serde_json::Map::new();
            for (k, v) in map {
                if k == "additionalProperties" || k == "default" || k == "$schema" || k == "title" {
                    continue;
                }
                out.insert(k.clone(), clean_schema(v));
            }
            Value::Object(out)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(clean_schema).collect()),
        _ => val.clone(),
    }
}

pub async fn openai_models_handler() -> impl IntoResponse {
    Json(json!({
        "object": "list",
        "data": [
            {"id": "gemini-3.5-flash-lite", "object": "model", "owned_by": "google"},
            {"id": "gemini-3.1-flash-lite", "object": "model", "owned_by": "google"},
            {"id": "gemini-2.5-flash-preview", "object": "model", "owned_by": "google"},
        ]
    }))
}

pub async fn openai_chat_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<OpenAiChatRequest>,
) -> Response {
    let raw_model = req
        .model
        .as_deref()
        .unwrap_or("gemini-3.5-flash-lite");
    let target_model = if raw_model == "gemini-3-flash-preview" {
        "gemini-3.5-flash-lite".to_string()
    } else {
        raw_model.to_string()
    };
    let stream = req.stream.unwrap_or(false);

    // ── Build Gemini contents ──
    let mut contents: Vec<Value> = Vec::new();
    let mut system_parts: Vec<Value> = Vec::new();

    for msg in &req.messages {
        match msg.role.as_str() {
            "system" => {
                let t = extract_text(msg.content.as_ref());
                if !t.is_empty() {
                    system_parts.push(json!({"text": t}));
                }
            }
            "tool" => {
                let result_text = extract_text(msg.content.as_ref());
                let fn_name = msg.name.as_deref().unwrap_or("tool_result");
                contents.push(json!({
                    "role": "user",
                    "parts": [{"functionResponse": {
                        "name": fn_name,
                        "response": {"result": result_text}
                    }}]
                }));
            }
            "assistant" => {
                let mut parts: Vec<Value> = Vec::new();
                let t = extract_text(msg.content.as_ref());
                if !t.is_empty() {
                    parts.push(json!({"text": t}));
                }
                if let Some(tcs) = &msg.tool_calls {
                    for tc in tcs {
                        let call_id = tc["id"].as_str().unwrap_or("");
                        let fn_name = tc["function"]["name"].as_str().unwrap_or("unknown");
                        let raw_args = tc["function"]["arguments"].as_str().unwrap_or("{}");
                        let args: Value =
                            serde_json::from_str(raw_args).unwrap_or(json!({}));
                        let mut part_obj = json!({
                            "functionCall": {"name": fn_name, "args": args}
                        });
                        if let Some(sig) = state.key_pool.get_thought_sig(call_id).await {
                            part_obj["thoughtSignature"] = json!(sig);
                        }
                        parts.push(part_obj);
                    }
                }
                if !parts.is_empty() {
                    contents.push(json!({"role": "model", "parts": parts}));
                }
            }
            _ => {
                // user / function
                let t = extract_text(msg.content.as_ref());
                if !t.is_empty() {
                    contents.push(json!({"role": "user", "parts": [{"text": t}]}));
                }
            }
        }
    }

    let mut body = json!({
        "contents": contents,
        "generationConfig": {
            "temperature": req.temperature.unwrap_or(0.7),
            "maxOutputTokens": req.max_tokens.unwrap_or(2048),
            "candidateCount": 1
        }
    });

    if !system_parts.is_empty() {
        body["systemInstruction"] = json!({"parts": system_parts});
    }

    if let Some(tools) = &req.tools {
        let fn_decls: Vec<Value> = tools
            .iter()
            .filter_map(|t| {
                let fn_obj = t.get("function")?;
                let name = fn_obj.get("name")?.as_str()?.to_string();
                let desc = fn_obj
                    .get("description")
                    .and_then(|d| d.as_str())
                    .unwrap_or("")
                    .to_string();
                let params = fn_obj
                    .get("parameters")
                    .map(clean_schema)
                    .unwrap_or(json!({"type": "object", "properties": {}}));
                Some(json!({"name": name, "description": desc, "parameters": params}))
            })
            .collect();
        if !fn_decls.is_empty() {
            body["tools"] = json!([{"functionDeclarations": fn_decls}]);
        }
    }

    // ── Call Gemini ──
    let result = state.key_pool.generate_content(&target_model, &body).await;

    match result {
        Ok(resp) => {
            let cmpl_id = format!("chatcmpl-{}", uuid::Uuid::new_v4());
            let created = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            let candidate = &resp["candidates"][0];
            let finish_raw = candidate["finishReason"].as_str().unwrap_or("STOP");
            let finish_reason = match finish_raw {
                "STOP" | "MAX_TOKENS" => finish_raw.to_lowercase(),
                _ => "stop".to_string(),
            };

            let mut text_out = String::new();
            let mut tool_calls_out: Vec<Value> = Vec::new();

            let usage = &resp["usageMetadata"];
            let prompt_tokens = usage["promptTokenCount"].as_u64().unwrap_or(0);
            let completion_tokens = usage["candidatesTokenCount"].as_u64().unwrap_or(0);

            if let Some(parts) = candidate["content"]["parts"].as_array() {
                for part in parts {
                    if let Some(t) = part.get("text").and_then(|t| t.as_str()) {
                        text_out.push_str(t);
                    }
                    if let Some(fc) = part.get("functionCall") {
                        let fn_name = fc["name"].as_str().unwrap_or("unknown");
                        let args = fc.get("args").cloned().unwrap_or(json!({}));
                        let call_id = format!("call_{}", uuid::Uuid::new_v4());
                        if let Some(sig) = part.get("thoughtSignature").and_then(|s| s.as_str()) {
                            state.key_pool.store_thought_sig(&call_id, sig).await;
                        }
                        tool_calls_out.push(json!({
                            "id": call_id,
                            "type": "function",
                            "function": {
                                "name": fn_name,
                                "arguments": serde_json::to_string(&args).unwrap_or_default()
                            }
                        }));
                    }
                }
            }

            // ── SSE streaming format ──
            if stream {
                let role_chunk = json!({
                    "id": cmpl_id, "object": "chat.completion.chunk",
                    "created": created, "model": target_model,
                    "choices": [{"index":0,"delta":{"role":"assistant","content":""},"finish_reason":null}]
                });
                let content_chunk = json!({
                    "id": cmpl_id, "object": "chat.completion.chunk",
                    "created": created, "model": target_model,
                    "choices": [{
                        "index": 0,
                        "delta": if !tool_calls_out.is_empty() {
                            json!({"tool_calls": tool_calls_out})
                        } else {
                            json!({"content": text_out})
                        },
                        "finish_reason": null
                    }]
                });
                let done_chunk = json!({
                    "id": cmpl_id, "object": "chat.completion.chunk",
                    "created": created, "model": target_model,
                    "choices": [{"index":0,"delta":{},"finish_reason": finish_reason}],
                    "usage": {
                        "prompt_tokens": prompt_tokens,
                        "completion_tokens": completion_tokens,
                        "total_tokens": prompt_tokens + completion_tokens
                    }
                });
                let sse_body = format!(
                    "data: {}\n\ndata: {}\n\ndata: {}\n\ndata: [DONE]\n\n",
                    role_chunk, content_chunk, done_chunk
                );
                return (
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, "text/event-stream")],
                    sse_body,
                )
                    .into_response();
            }

            let mut msg_obj = json!({
                "role": "assistant",
                "content": if text_out.is_empty() && !tool_calls_out.is_empty() {
                    Value::Null
                } else {
                    json!(text_out)
                }
            });
            if !tool_calls_out.is_empty() {
                msg_obj["tool_calls"] = json!(tool_calls_out);
            }

            (
                StatusCode::OK,
                Json(json!({
                    "id": cmpl_id,
                    "object": "chat.completion",
                    "created": created,
                    "model": target_model,
                    "choices": [{
                        "index": 0,
                        "message": msg_obj,
                        "finish_reason": finish_reason
                    }],
                    "usage": {
                        "prompt_tokens": prompt_tokens,
                        "completion_tokens": completion_tokens,
                        "total_tokens": prompt_tokens + completion_tokens
                    }
                })),
            )
                .into_response()
        }
        Err((status, err)) => (
            status,
            Json(json!({"error": {"message": err, "type": "upstream_error"}})),
        )
            .into_response(),
    }
}
