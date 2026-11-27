# Agent Father

**Agent Father** is a multi-user, self-hosted personal AI executive assistant platform with a React/Vite web dashboard, a Rust (Axum) backend, SQLite persistence, custom Sub-Agent workspaces, native WhatsApp messaging & voice notes (PTT), and an internal rotating LLM/TTS proxy.

---

## Key Features

- **Multi-User Isolated Sessions:** Every authenticated user receives an isolated chat history and agent memory scope (`X-Hermes-Session-Key: web-user:<user_id>`).
- **Workspace & Custom Sub-Agents:** Create, manage, and chat with custom AI sub-agents (each with its own name, role, custom system prompt, and isolated session key `web-subagent:<user_id>:<sub_agent_id>`).
- **Native WhatsApp Integration:**
  - Secure 6-digit OTP verification (with optional voice OTP) to link a user's WhatsApp number.
  - Send text messages and Opus/OGG push-to-talk (PTT) voice notes directly from the web chat to WhatsApp.
- **Voice Note Synthesis & Playback:** Generates voice notes via internal TTS (`/internal/tts`), converts audio via `ffmpeg` (`libopus` for WhatsApp PTT), and serves web audio at `/media-audio/` with an inline `<audio>` player.
- **Multilingual UI (EN → FR → AR):** Full English (default), French, and Arabic (RTL) interface with instant language switching.
- **30+ Integration Roadmap:** Organized 5×2 grids across **Business**, **Social Media**, and **Developers** categories (WhatsApp live; others marked `SOON`).
- **Rotating API Key Pool:** Built-in OpenAI-compatible proxy (`/internal/llm/v1/chat/completions`) and TTS endpoint with automatic key rotation and cooldown handling.

---

## Tech Stack

| Layer | Technology |
|---|---|
| **Frontend** | React 19 (`^19.3.1`), Vite 8 (`^8.4.0`), Vanilla CSS |
| **Backend** | Rust (Edition 2024), Axum `0.8`, Tokio `1`, Reqwest `0.12`, Tower-HTTP `0.6` |
| **Database** | SQLite (`rusqlite 0.32` with `bundled`, WAL mode) |
| **Auth** | JWT (`jsonwebtoken 9`, 7-day expiry) + SHA-256 salted password hashing |
| **Audio** | `ffmpeg` (`libopus` encoding for WhatsApp PTT) + Gemini/Internal TTS |
| **Agent Runtime** | `nousresearch/hermes-agent:latest` (internal gateway on port `8642`) |
| **Tunnel / Ingress** | Cloudflare Tunnel (`cloudflare/cloudflared:latest`) |
| **Containerization** | Docker & Docker Compose |

---

## Project Structure

```text
AgentFather/
├── Backend/
│   ├── Cargo.toml              # Rust dependencies & package manifest
│   ├── Cargo.lock              # Locked Rust dependency tree
│   ├── Dockerfile              # Multi-stage build (rust:1-slim-bookworm -> debian:bookworm-slim + ffmpeg)
│   ├── .gitignore
│   └── src/
│       ├── main.rs             # Axum router, AppState, static serving (/brand-assets, /media-audio, SPA)
│       ├── auth.rs             # Signup, login, logout, /me, JWT claims, Google OAuth & Gmail handlers
│       ├── chat.rs             # Web chat, history, sub-agent CRUD, [[wa_send:]] & MEDIA: post-processing
│       ├── db.rs               # SQLite schema & WAL initialization
│       ├── key_pool.rs         # Rotating Gemini/Groq key pool, OpenAI-compat LLM proxy, /internal/tts
│       └── whatsapp.rs         # WhatsApp OTP request & verification (text + voice OTP)
├── frontend/
│   ├── index.html              # SPA entry HTML
│   ├── package.json            # React 19 + Vite 8 scripts
│   ├── vite.config.js          # Vite configuration
│   ├── public/                 # Static public SVG assets
│   └── src/
│       ├── main.jsx            # React DOM mount
│       ├── App.jsx             # Complete multilingual SPA (Home, Features, Pricing, Status, Dashboard)
│       ├── App.css             # Application styling
│       ├── index.css           # Global theme & CSS variables
│       └── assets/             # Local frontend assets
├── assets/
│   ├── logo.png                # Brand icon
│   └── main.png                # Hero banner artwork
├── docker-compose.yml          # Multi-container orchestration (hermes-agent, backend, cloudflared)
├── .gitignore                  # Root ignore rules (target/, node_modules/, dist/, *.db*, *.log, runtime data)
├── README.md                   # This file
└── system_desgain.md           # Detailed system architecture & design specification
```

---

## Prerequisites

- **Docker** & **Docker Compose** (for full containerized deployment)
- **Node.js 20+** & **npm** (to build `frontend/dist` before building the backend Docker image)
- **Rust toolchain** (only required if running/compiling `Backend` natively outside Docker)

---

## Quick Start (Docker Compose)

1. **Build the frontend bundle** (the `Backend/Dockerfile` copies `frontend/dist` into `/app/frontend/dist`):
   ```bash
   cd frontend
   npm install
   npm run build
   cd ..
   ```

2. **Build and start all services**:
   ```bash
   docker compose build backend
   docker compose up -d
   ```

3. **Verify the backend is healthy**:
   ```bash
   curl http://127.0.0.1:8090/api/health
   # {"service":"Agent Father","status":"ok"}
   ```

---

## Local Development (Without Docker)

### Frontend
```bash
cd frontend
npm install
npm run dev
```

### Backend
```bash
cd Backend
export DATABASE_PATH="./agentfather.db"
export FRONTEND_DIST="../frontend/dist"
export ASSETS_DIR="../assets"
export AUDIO_DIR="./audio"
cargo run
```

---

## Environment Variables

| Variable | Default / Example | Description |
|---|---|---|
| `DATABASE_PATH` | `/app/data/agentfather.db` | Path to the SQLite database file |
| `FRONTEND_DIST` | `/app/frontend/dist` | Path to built Vite static assets |
| `ASSETS_DIR` | `/app/assets` | Path to brand assets served at `/brand-assets` |
| `AUDIO_DIR` | `/app/hermes_data/audio` | Directory for generated audio files served at `/media-audio` |
| `JWT_SECRET` | `agentfather-jwt-super-secret-2026` | HMAC secret for signing 7-day JWT tokens |
| `HERMES_API_URL` | `http://127.0.0.1:8642/v1/chat/completions` | Internal agent gateway completion endpoint |
| `HERMES_API_KEY` | `agentfather-hermes-secret-2026` | Bearer key for the internal agent gateway |
| `WHATSAPP_BRIDGE_URL` | `http://127.0.0.1:3000` | Internal WhatsApp bridge URL |
| `HERMES_WHATSAPP_BRIDGE_TOKEN` | `agentfather-wa-bridge-token-2026` | Bearer token for WhatsApp bridge (also read from `.bridge-token`) |
| `GEMINI_KEYS` | *(comma-separated)* | Optional override for Gemini LLM API keys |
| `GROQ_KEYS` | *(comma-separated)* | Optional override for Groq API keys |

---

## API Overview

| Method | Endpoint | Auth | Purpose |
|---|---|---|---|
| `GET` | `/api/health` | Public | Service health check |
| `POST` | `/api/auth/signup` | Public | Create user account & return JWT |
| `POST` | `/api/auth/login` | Public | Authenticate user & return JWT |
| `POST` | `/api/auth/logout` | Public | Logout acknowledgment |
| `GET` | `/api/auth/me` | Bearer JWT | Get current user profile |
| `POST` | `/api/whatsapp/request-otp` | Optional JWT | Send 6-digit OTP (+ voice note) via WhatsApp |
| `POST` | `/api/whatsapp/verify-otp` | Optional JWT | Verify OTP & link WhatsApp number to user |
| `POST` | `/api/chat` | Bearer JWT | Send message (optional `sub_agent_id`), returns `reply` & `audio_url` |
| `GET` | `/api/chat/history` | Bearer JWT | Fetch user's isolated chat history (up to 100 messages) |
| `GET` | `/api/sub-agents` | Bearer JWT | List user's custom sub-agents |
| `POST` | `/api/sub-agents` | Bearer JWT | Create a new sub-agent (`name`, `role`, `system_prompt`) |
| `DELETE` | `/api/sub-agents/{id}` | Bearer JWT | Delete a user's sub-agent |
| `POST` | `/internal/llm/v1/chat/completions` | Internal | Rotating Gemini LLM OpenAI-compatible proxy |
| `POST` | `/internal/tts` | Internal | Text-to-speech synthesis endpoint |
| `GET` | `/internal/keys/status` | Internal | Inspect key pool health & rotation stats |
