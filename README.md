# Pratyaksa — Mining Intelligence Platform

Platform manajemen armada alat berat pertambangan dengan **predictive maintenance** berbasis AI/IoT. Pratyaksa memantau kesehatan unit secara realtime, memprediksi sisa umur komponen (RUL), menjelaskan akar masalah lewat analisis SHAP, dan mengeskalasi alarm kritis menjadi Work Order — termasuk notifikasi instan ke Telegram.

> **Contributor — Fullstack & Data Engineering**
> https://github.com/virgiawanprima/project-pratyaksa-kic

## Fitur Utama

- **Dashboard armada** — KPI realtime, distribusi status, dan data chart (Chart.js).
- **Peta sebaran unit** — lokasi unit tambang di peta interaktif (Leaflet).
- **Analisa kesehatan** — risk score, telemetri sensor (33 parameter), RUL per komponen, radar kesehatan, kontribusi SHAP, dan riwayat sensor 24 jam.
- **Visualisasi 3D unit** — model `.glb/.gltf` lewat `<model-viewer>` (Google).
- **Manajemen master data** — CRUD jenis alat berat & unit tambang.
- **Work Order (CMMS)** — buat dan lacak perintah kerja dari alarm.
- **Notifikasi Telegram** — alert unit CRITICAL dikirim ke semua subscriber lewat service gRPC low-latency; command `/start` & `/status` untuk ringkasan fleet.

## Arsitektur

```
pratyaksa/
├── frontend_nuxt/         # Frontend SSR (Nuxt 4 + Tailwind + shadcn)
│   ├── app/               # srcDir: pages, components, composables, plugins
│   └── server/routes/svc/ # Bridge non-/api: gRPC ke bot + upload model 3D
├── backend_rust/          # Backend API (Rust + Actix-Web)
│   ├── src/
│   │   ├── main.rs
│   │   ├── config.rs
│   │   ├── db/            # Handler PostgreSQL & MongoDB
│   │   ├── models/        # Struct data & tipe request
│   │   ├── routes/        # Handler endpoint API
│   │   └── middleware/    # Middleware JWT auth
│   └── migrations/        # Migrasi SQL (auto-run saat startup)
├── telegram_bot_grpc/     # Service notifikasi Telegram (Rust + tonic/gRPC)
│   ├── proto/alert.proto  # Kontrak gRPC AlertService
│   └── src/main.rs
├── nginx/
│   └── nginx.conf         # Reverse proxy (satu pintu masuk)
└── docker-compose.yml     # Orkestrasi multi-service (berbasis profile)
```

### Arsitektur jaringan (Docker)

```
Browser
   │
   ▼  :80
[ Nginx ]  ← satu pintu masuk
   │
   ├── /api/*  ──→  [ backend:8080 ]  (Rust + Actix-Web)
   │                      │
   │               ┌──────┴──────┐
   │          [postgres:5432] [mongodb:27017]
   │
   └── /*      ──→  [ frontend:3000 ] (Nuxt 4, SSR)
                          │
                          │  server route /svc/send-alert (gRPC)
                          ▼
                  [ telegram-bot:50051 ] ──→ Telegram Bot API
```

Alur alert: `Browser → Nuxt /svc/send-alert (HTTP) → telegram-bot (gRPC/tonic) → Telegram API → subscriber`. Endpoint bot tidak diekspos ke host; hanya dipanggil dari dalam network internal.

## Stack

| Layer        | Teknologi                                             |
|--------------|-------------------------------------------------------|
| Frontend     | Nuxt 4, Vue 3, Tailwind CSS, shadcn-nuxt              |
| Visualisasi  | Chart.js, Leaflet, `<model-viewer>` (3D)             |
| Backend      | Rust, Actix-Web 4, SQLx, validator                    |
| Telegram bot | Rust, tonic (gRPC/HTTP2), reqwest, prost              |
| DB 1         | PostgreSQL 16 (users, units, telemetry, work orders)  |
| DB 2         | MongoDB 7 (laporan analisa manual)                    |
| Auth         | JWT (jsonwebtoken) + bcrypt                            |
| Proxy        | Nginx 1.27                                             |

## API Endpoints

Semua di-prefix `/api/v1`. Endpoint bertanda **(auth)** butuh header `Authorization: Bearer <token>`.

```
GET    /health                    — Health check (public)
GET    /fleet-summary             — Ringkasan fleet untuk bot Telegram (internal, no-auth)

POST   /auth/register             — Daftar akun baru
POST   /auth/login                — Login, dapat JWT token
GET    /auth/me                   — Info user aktif (auth)

GET    /dashboard                 — Stats KPI + data chart (auth)

GET    /jenis-alat-berat          — List jenis alat berat (auth)
POST   /jenis-alat-berat          — Tambah jenis (auth)
GET    /jenis-alat-berat/:id      — Detail (auth)
PUT    /jenis-alat-berat/:id      — Update (auth)
DELETE /jenis-alat-berat/:id      — Hapus (auth)

GET    /unit-tambang              — List unit + filter/search (auth)
POST   /unit-tambang              — Tambah unit (auth)
GET    /unit-tambang/:id          — Detail (auth)
PUT    /unit-tambang/:id          — Update (auth)
DELETE /unit-tambang/:id          — Hapus (auth)

GET    /analisa/overview          — Analitik kesehatan seluruh armada, realtime (auth)
GET    /analisa/unit/:id          — Analitik detail satu unit, realtime (auth)
GET    /analisa                   — List laporan analisa manual / MongoDB (auth)
POST   /analisa                   — Buat laporan baru (auth)
GET    /analisa/:id               — Detail laporan (auth)
PUT    /analisa/:id               — Update laporan (auth)
DELETE /analisa/:id               — Hapus laporan (auth)

POST   /telemetry                 — Ingest data telemetri (auth)
GET    /telemetry/unit/:id        — Riwayat telemetri per unit (auth)

GET    /work-orders               — List work order (auth)
POST   /work-orders               — Buat work order (auth)
GET    /work-orders/:id           — Detail (auth)
PUT    /work-orders/:id           — Update status/data (auth)
```

> Catatan: `/analisa/overview` dan `/analisa/unit/:id` menurunkan analitik kesehatan secara deterministik-realtime dari data unit di PostgreSQL (tidak ada sensor IoT fisik di environment ini). Endpoint CRUD `/analisa` lainnya menyimpan laporan manual di MongoDB.

### Server routes Nuxt (di luar `/api`)

Ditempatkan di `/svc/` agar Nginx tidak meneruskannya ke backend Rust:

```
POST /svc/send-alert     — Proxy kirim alert ke bot Telegram via gRPC
POST /svc/upload-model   — Upload model 3D (.glb/.gltf, maks 50 MB) ke public/media/models
```

### Kontrak gRPC bot (`alert.proto`)

```
service AlertService {
  rpc SendAlert(AlertRequest)  returns (AlertResponse);   // kirim alert CRITICAL
  rpc HealthCheck(HealthRequest) returns (HealthResponse); // cek service + cache fleet
}
```

## Menjalankan dengan Docker Compose

### Profiles yang tersedia

| Profile    | Services yang dijalankan                                   |
|------------|------------------------------------------------------------|
| `db`       | PostgreSQL + MongoDB saja                                  |
| `backend`  | PostgreSQL + MongoDB + Backend Rust                        |
| `bot`      | Service Telegram bot (gRPC) saja                           |
| `full`     | Semua: db + backend + telegram-bot + frontend + nginx      |
| `tools`    | Mongo Express + pgAdmin (UI database)                      |

### Setup awal

```bash
# Salin file environment
cp .env.example .env

# Edit nilai sesuai kebutuhan (password, JWT secret, token Telegram, dll)
nano .env
```

Untuk fitur notifikasi Telegram, isi minimal `TELEGRAM_BOT_TOKEN` dan set `WO_LINK_BASE` ke IP/domain ber-titik (bukan `localhost`, karena Telegram tidak menjadikan `localhost` sebagai link).

### Jalankan semua service

```bash
docker compose --profile full up -d
```

### Jalankan sebagian

```bash
# Hanya database
docker compose --profile db up -d

# Database + backend
docker compose --profile backend up -d

# Backend + tools UI (pgAdmin + Mongo Express)
docker compose --profile backend --profile tools up -d
```

### Akses services

| Service       | URL                            | Keterangan                      |
|---------------|--------------------------------|---------------------------------|
| Aplikasi      | http://localhost               | Nginx (frontend + API)          |
| Backend API   | http://localhost/api/v1        | Via Nginx                       |
| API Health    | http://localhost/api/v1/health | Health check                    |
| Telegram bot  | gRPC `telegram-bot:50051`      | Internal only (tidak ke host)   |
| pgAdmin       | http://localhost:5050          | Profile: tools                  |
| Mongo Express | http://localhost:8081          | Profile: tools                  |

### Login default

```
Email    : admin@pratyaksa.id
Password : admin123
```

## Development Lokal (tanpa Docker)

### Backend Rust

```bash
# Jalankan database dulu
docker compose --profile db up -d

cd backend_rust
cargo run
```

Migrasi SQL otomatis dijalankan saat startup.

### Telegram bot (opsional)

```bash
cd telegram_bot_grpc
cp .env.example .env   # isi TELEGRAM_BOT_TOKEN
cargo run
```

### Frontend Nuxt

```bash
cd frontend_nuxt
pnpm install
pnpm dev
```

Pastikan env berikut sesuai saat lokal:

```bash
NUXT_PUBLIC_API_BASE=http://localhost:8080/api/v1
NUXT_TELEGRAM_GRPC_TARGET=127.0.0.1:50051
```

## Environment Variables

Lihat `.env.example` untuk daftar lengkap. Yang paling penting:

| Variable                    | Keterangan                                                        |
|-----------------------------|-------------------------------------------------------------------|
| `POSTGRES_*`                | Kredensial & port PostgreSQL                                      |
| `MONGO_*`                   | Kredensial & port MongoDB                                        |
| `JWT_SECRET`                | Secret penandatangan JWT — **ganti di production**               |
| `JWT_EXPIRY_HOURS`          | Masa berlaku token (default 24 jam)                              |
| `NUXT_PUBLIC_API_BASE`      | Base URL API dari sisi browser (`/api/v1` di balik Nginx)        |
| `NUXT_TELEGRAM_GRPC_TARGET` | Target gRPC bot (`telegram-bot:50051` di Docker)                 |
| `TELEGRAM_BOT_TOKEN`        | Token bot Telegram                                               |
| `WO_LINK_BASE`              | Base URL link "Buat Work Order" di pesan Telegram (IP/domain)    |
| `APP_PORT`                  | Port Nginx (default 80)                                          |

## Operasional

### Menghentikan semua service

```bash
docker compose --profile full down

# Hapus volume juga (data hilang permanen)
docker compose --profile full down -v
```

### Rebuild setelah perubahan kode

```bash
# Frontend berubah
docker compose --profile full build frontend
docker compose --profile full up -d --force-recreate --no-deps frontend

# Backend berubah
docker compose --profile full build backend
docker compose --profile full up -d --force-recreate --no-deps backend

# Telegram bot berubah
docker compose --profile full build telegram-bot
docker compose --profile full up -d --force-recreate --no-deps telegram-bot

# Kalau backend baru restart & nginx kasih 502, restart nginx
docker restart pratyaksa-nginx

# Rebuild + jalankan sekaligus
docker compose --profile full up -d --build
```
