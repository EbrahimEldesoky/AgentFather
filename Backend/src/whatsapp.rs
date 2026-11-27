use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json},
};
use chrono::Utc;
use rand::Rng;
use rusqlite::OptionalExtension;
use serde::Deserialize;
use serde_json::json;
use std::{path::PathBuf, sync::Arc};
use uuid::Uuid;

use crate::{auth::extract_claims, AppState};

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
    std::env::var("WHATSAPP_BRIDGE_URL").unwrap_or_else(|_| "http://hermes-agent:3000".to_string())
}

fn audio_host_dir() -> PathBuf {
    PathBuf::from(
        std::env::var("AUDIO_DIR").unwrap_or_else(|_| "/app/hermes_data/audio".to_string()),
    )
}

fn normalize_phone(country_code: &str, phone: &str) -> Option<String> {
    let cc: String = country_code.chars().filter(|c| c.is_ascii_digit()).collect();
    let mut num: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
    if cc.is_empty() || num.is_empty() {
        return None;
    }
    if num.starts_with('0') && !cc.is_empty() {
        num = num.trim_start_matches('0').to_string();
    }
    Some(format!("{}{}", cc, num))
}

#[derive(Deserialize)]
pub struct RequestOtpReq {
    pub country_code: String,
    pub phone_number: String,
    pub include_voice: Option<bool>,
}

pub async fn request_otp_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<RequestOtpReq>,
) -> impl IntoResponse {
    let Some(full_digits) = normalize_phone(&req.country_code, &req.phone_number) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "يرجى إدخال كود الدولة ورقم الواتساب بشكل صحيح"})),
        );
    };

    let otp_code: String = {
        let mut rng = rand::thread_rng();
        format!("{:06}", rng.gen_range(100000..999999))
    };

    let user_id = extract_claims(&headers).map(|c| c.sub);
    let now = Utc::now();
    let expires_at = now.timestamp() + 600;
    let otp_id = Uuid::new_v4().to_string();

    {
        let conn = state.db.conn.lock().unwrap();
        let _ = conn.execute(
            "INSERT INTO whatsapp_otps (id, user_id, full_number, otp_code, verified, expires_at, created_at) VALUES (?1, ?2, ?3, ?4, 0, ?5, ?6)",
            (&otp_id, &user_id, &full_digits, &otp_code, expires_at, &now.to_rfc3339()),
        );
    }

    let chat_id = format!("{}@s.whatsapp.net", full_digits);
    let otp_msg = format!(
        "مرحباً بك في *Agent Father*.\n\nرمز التحقق الخاص بك (Authentication Number) هو:\n*{}*\n\nأدخل هذا الرمز في الموقع لإتمام ربط حسابك.",
        otp_code
    );

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap();

    let send_res = client
        .post(format!("{}/send", bridge_url()))
        .bearer_auth(bridge_token())
        .json(&json!({
            "chatId": chat_id,
            "message": otp_msg
        }))
        .send()
        .await;

    let mut text_sent = false;
    let mut err_msg = None;

    match send_res {
        Ok(r) if r.status().is_success() => text_sent = true,
        Ok(r) => err_msg = Some(r.text().await.unwrap_or_default()),
        Err(e) => err_msg = Some(e.to_string()),
    }

    if text_sent && req.include_voice.unwrap_or(true) {
        let file_name = format!("otp_{}.ogg", Uuid::new_v4());
        let host_path = audio_host_dir().join(&file_name);
        let container_path = format!("/opt/data/audio/{}", file_name);
        let spaced_otp: String = otp_code.chars().map(|c| format!("{} ", c)).collect();
        let voice_script = format!(
            "مرحباً بك في Agent Father. رمز التحقق الخاص بك هو: {}.",
            spaced_otp
        );
        if match state.key_pool.generate_tts_ogg_bytes(&voice_script).await {
                Ok(bytes) => std::fs::write(&host_path, bytes).is_ok(),
                Err(e) => { tracing::warn!("TTS for OTP failed: {}", e); false }
            }
        {
            let _ = client
                .post(format!("{}/send-media", bridge_url()))
                .bearer_auth(bridge_token())
                .json(&json!({
                    "chatId": chat_id,
                    "filePath": container_path,
                    "mediaType": "audio"
                }))
                .send()
                .await;
        }
    }

    if text_sent {
        (
            StatusCode::OK,
            Json(json!({
                "ok": true,
                "message": "تم إرسال رمز التحقق إلى واتساب الخاص بك."
            })),
        )
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "ok": false,
                "error": err_msg.unwrap_or_else(|| "WhatsApp service unavailable".to_string())
            })),
        )
    }
}

#[derive(Deserialize)]
pub struct VerifyOtpReq {
    pub country_code: String,
    pub phone_number: String,
    pub otp_code: String,
}

pub async fn verify_otp_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<VerifyOtpReq>,
) -> impl IntoResponse {
    let Some(full_digits) = normalize_phone(&req.country_code, &req.phone_number) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "رقم غير صالح"})),
        );
    };

    let code_clean = req.otp_code.trim().to_string();
    let now_ts = Utc::now().timestamp();
    let user_id = extract_claims(&headers).map(|c| c.sub);

    let valid: Option<String> = {
        let conn = state.db.conn.lock().unwrap();
        conn.query_row(
            "SELECT id FROM whatsapp_otps WHERE full_number = ?1 AND otp_code = ?2 AND verified = 0 AND expires_at >= ?3 ORDER BY created_at DESC LIMIT 1",
            (&full_digits, &code_clean, now_ts),
            |r| r.get(0),
        )
        .optional()
        .unwrap_or(None)
    };

    let Some(otp_id) = valid else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "رمز التحقق غير صحيح أو منتهي الصلاحية"})),
        );
    };

    {
        let conn = state.db.conn.lock().unwrap();
        let _ = conn.execute(
            "UPDATE whatsapp_otps SET verified = 1 WHERE id = ?1",
            [&otp_id],
        );
        if let Some(ref uid) = user_id {
            let _ = conn.execute(
                "UPDATE users SET whatsapp_number = ?1 WHERE id = ?2",
                (&full_digits, uid),
            );
        }
    }

    (
        StatusCode::OK,
        Json(json!({
            "ok": true,
            "verified_number": format!("+{}", full_digits)
        })),
    )
}
