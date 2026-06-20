# Pratyaksa — Mining Intelligence Platform

Platform manajemen armada alat berat pertambangan dengan predictive maintenance berbasis IoT.

## contributor repo - FUllstack Data Engineering
https://github.com/virgiawanprima/project-pratyaksa-kic

## Arsitektur

```
pratyaksa/
├── frontend_nuxt/      # Frontend (Nuxt 4 + Tailwind CSS)
├── backend_rust/       # Backend API (Rust + Actix-Web)
│   ├── src/
│   │   ├── main.rs
│   │   ├── config.rs
│   │   ├── db/         # PostgreSQL & MongoDB handlers
│   │   ├── models/     # Data structures & request types
│   │   ├── routes/     # API endpoint handlers
│   │   └── middleware/ # JWT auth middleware
│   └── migrations/     # SQL migrations (auto-run on startup)
├── nginx/
│   └── nginx.conf      # Reverse proxy config
└── docker-compose.yml  # Multi-service Docker setup
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
   └── /*      ──→  [ frontend:3000 ] (Nuxt 4)
```

## Stack

| Layer     | Teknologi                        |
|-----------|----------------------------------|
| Frontend  | Nuxt 4, Vue 3, Tailwind CSS      |
| Backend   | Rust, Actix-Web 4                |
| DB 1      | PostgreSQL 16 (users, units)     |
| DB 2      | MongoDB 7 (analisa, sensor data) |
| Auth      | JWT (jsonwebtoken)               |

## API Endpoints

```
GET  /api/v1/health              — Health check (public)

POST /api/v1/auth/register       — Daftar akun baru
POST /api/v1/auth/login          — Login, dapat JWT token
GET  /api/v1/auth/me             — Info user aktif (auth)

GET  /api/v1/dashboard           — Stats KPI + chart data (auth)

GET  /api/v1/jenis-alat-berat    — List semua jenis (auth)
POST /api/v1/jenis-alat-berat    — Tambah jenis baru (auth)
GET  /api/v1/jenis-alat-berat/:id
PUT  /api/v1/jenis-alat-berat/:id
DELETE /api/v1/jenis-alat-berat/:id

GET  /api/v1/unit-tambang        — List unit + filter/search (auth)
POST /api/v1/unit-tambang        — Tambah unit (auth)
GET  /api/v1/unit-tambang/:id
PUT  /api/v1/unit-tambang/:id
DELETE /api/v1/unit-tambang/:id

GET  /api/v1/analisa             — List analisa (MongoDB) (auth)
POST /api/v1/analisa             — Buat laporan baru (auth)
GET  /api/v1/analisa/:id
PUT  /api/v1/analisa/:id
DELETE /api/v1/analisa/:id
```

## Menjalankan dengan Docker Compose

### Profiles yang tersedia

| Profile    | Services yang dijalankan                    |
|------------|---------------------------------------------|
| `db`       | PostgreSQL + MongoDB saja                   |
| `backend`  | PostgreSQL + MongoDB + Backend Rust         |
| `frontend` | Frontend Nuxt saja (butuh backend aktif)    |
| `full`     | Semua service (db + backend + frontend)     |
| `tools`    | Mongo Express + pgAdmin (UI database)       |

### Setup awal

```bash
# Salin file environment
cp .env.example .env

# Edit jika perlu (password, port, dll)
nano .env
```

### Jalankan semua service

```bash
docker compose --profile full up -d
```

### Jalankan hanya database

```bash
docker compose --profile db up -d
```

### Jalankan database + backend

```bash
docker compose --profile backend up -d
```

### Jalankan dengan tools UI (pgAdmin + Mongo Express)

```bash
docker compose --profile backend --profile tools up -d
```

### Akses services

| Service       | URL                        | Keterangan                    |
|---------------|----------------------------|-------------------------------|
| Aplikasi      | http://localhost           | Nginx (frontend + API)        |
| Backend API   | http://localhost/api/v1    | Via Nginx (internal only)     |
| API Health    | http://localhost/api/v1/health | Health check               |
| pgAdmin       | http://localhost:5050      | Profile: tools                |
| Mongo Express | http://localhost:8081      | Profile: tools                |

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

### Frontend Nuxt

```bash
cd frontend_nuxt
pnpm install
pnpm dev
```

Pastikan `NUXT_PUBLIC_API_BASE=http://localhost:8080/api/v1` sudah diset.

## Menghentikan semua service

```bash
docker compose --profile full down

# Hapus volume juga (data hilang)
docker compose --profile full down -v
```

```bash
# Frontend berubah
docker compose --profile full build frontend
docker compose --profile full up -d --force-recreate --no-deps frontend

# Backend berubah
docker compose --profile full build backend
docker compose --profile full up -d --force-recreate --no-deps backend

# Kalau backend restart & nginx kasih 502, restart nginx
docker restart pratyaksa-nginx

docker compose --profile full up -d --build

```