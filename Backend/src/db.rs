use rusqlite::{Connection, Result};
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE,
                password_hash TEXT,
                google_id TEXT,
                google_access_token TEXT,
                google_refresh_token TEXT,
                whatsapp_number TEXT,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS whatsapp_otps (
                id TEXT PRIMARY KEY,
                user_id TEXT,
                full_number TEXT NOT NULL,
                otp_code TEXT NOT NULL,
                verified INTEGER NOT NULL DEFAULT 0,
                expires_at INTEGER NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS chat_sessions (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                sub_agent_id TEXT,
                title TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS chat_messages (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                session_id TEXT,
                sub_agent_id TEXT,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS sub_agents (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                name TEXT NOT NULL,
                role TEXT NOT NULL,
                system_prompt TEXT NOT NULL,
                profile_dir TEXT NOT NULL DEFAULT '',
                skills TEXT NOT NULL DEFAULT '[]',
                helpers TEXT NOT NULL DEFAULT '[]',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL DEFAULT ''
            );
            ",
        )?;

        // Idempotent column migrations for existing databases
        let migrations = [
            "ALTER TABLE chat_messages ADD COLUMN session_id TEXT",
            "ALTER TABLE chat_messages ADD COLUMN sub_agent_id TEXT",
            "ALTER TABLE sub_agents ADD COLUMN profile_dir TEXT NOT NULL DEFAULT ''",
            "ALTER TABLE sub_agents ADD COLUMN skills TEXT NOT NULL DEFAULT '[]'",
            "ALTER TABLE sub_agents ADD COLUMN helpers TEXT NOT NULL DEFAULT '[]'",
            "ALTER TABLE sub_agents ADD COLUMN updated_at TEXT NOT NULL DEFAULT ''",
        ];
        for sql in migrations {
            let _ = conn.execute(sql, []);
        }

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}
