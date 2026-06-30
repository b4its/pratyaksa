# ⛏️ PRATYAKSA — Mining Intelligence Platform

**Predictive Analytics & Traceability for Heavy Asset Condition Surveillance and Actualization**

Sistem AIoT Predictive dan Prescriptive Maintenance untuk Armada Alat Berat Tambang Batubara.

---

| | |
|---|---|
| ⚙️ **Status** | MVP Fungsional |
| 🏆 **Kompetisi** | Kideco Innovation Challenge (KIC) 2026 |
| 👥 **Tim Oryphem** | Politeknik Negeri Samarinda |
| 📂 **UI Layer** | [github.com/virgiawanprima/project-pratyaksa-kic](https://github.com/virgiawanprima/project-pratyaksa-kic) |
| 📂 **ML Layer** | [github.com/b4its/ml-pratyaksa](https://github.com/b4its/ml-pratyaksa) |

---

## 📋 Daftar Isi

- [Ringkasan Eksekutif](#ringkasan-eksekutif)
- [Masalah & Solusi](#masalah--solusi)
- [Arsitektur Sistem](#arsitektur-sistem)
- [Alur Data](#alur-data)
- [Mode Operasi](#mode-operasi)
- [Tech Stack](#tech-stack)
- [Struktur Proyek](#struktur-proyek)
- [API Endpoints](#api-endpoints)
- [Panduan Menjalankan](#panduan-menjalankan)
- [Development](#development)
- [Environment Variables](#environment-variables)
- [Monitoring & MLOps](#monitoring--mlops)
- [Kontak](#kontak)

---

## Ringkasan Eksekutif

PRATYAKSA adalah platform AIoT terintegrasi yang mengubah pendekatan perawatan armada alat berat tambang dari **reaktif menjadi proaktif**. Sistem terdiri dari dua lapisan utama:

### 🖥️ Lapisan 1: UI & Simulasi (`pratyaksa/`)

Platform manajemen armada yang mencakup dashboard realtime, peta sebaran unit (Leaflet), visualisasi 3D unit (model-viewer), analisa kesehatan armada dengan risk score, telemetri sensor (33 parameter), RUL per komponen, radar kesehatan, kontribusi SHAP, manajemen master data (CRUD jenis alat berat & unit tambang), work order (CMMS), dan notifikasi Telegram via gRPC.

### 🧠 Lapisan 2: ML Inference (`ml-pratyaksa/`)

Mesin inferensi AI yang beroperasi secara independen dengan arsitektur **Edge-Cloud Continuum**. Menerima data sensor realtime melalui MQTT → Redis Streams, menjalankan model XGBoost (3-class anomaly) + LSTM MoE (RUL hierarkis) + Digital Twin (physics cross-check) + SHAP Explainability + Drift Detection (Z-score). Mendukung operasi edge dengan buffer offline 72 jam di area blank spot.

### 🔗 Sinkronisasi

Rust Backend menjembatani kedua lapisan:
- **Mode SIMULASI**: data dari PostgreSQL lokal (derivasi deterministik)
- **Mode LIVE**: polling HTTP ke ML API → simpan ke MongoDB (batch, high-speed) → tampilkan di frontend
- **Sync Background**: migrasi periodik data dari PostgreSQL `ml-pratyaksa` ke MongoDB lokal

### Target Dampak

| Metrik | Baseline | Target |
|--------|:--------:|:------:|
| Physical Availability (PA) Armada | 82–85% | **90–93%** |
| Pengurangan *Unplanned Downtime* | — | **30–45%** |
| Efisiensi Biaya Pemeliharaan | — | **20–30%** |
| Potensi Penyelamatan Produksi (50 unit) | — | **USD 8–12 Juta/tahun** |
| Latensi *Alert* di Kabin | — | **<500ms** |

---

## Masalah & Solusi

### Masalah

Berdasarkan analisis operasional PT. Kideco Jaya Agung, terdapat lima permasalahan utama:

| # | Masalah | Dampak Finansial |
|---|---------|:----------------:|
| 1 | Inefisiensi Preventive Maintenance Berbasis Jadwal | Biaya perbaikan darurat 2–5× lebih mahal |
| 2 | Dampak Finansial Masif dari *Unplanned Downtime* | **USD 25.000+ per insiden** |
| 3 | Tantangan Konektivitas & Kondisi Ekstrem | Data tidak reliabel, sensor mati prematur |
| 4 | Model AI *Black-Box* Tidak Terpercaya | Adopsi sistem rendah |
| 5 | Risiko Keselamatan pada Komponen Kritis | Potensi **USD 500.000–2 Juta** per insiden LTI |

### Solusi

PRATYAKSA beroperasi dalam **dua mode** yang dapat dipilih pengguna:

1. **SIMULASI Mode** — Menggunakan data dari PostgreSQL lokal dengan derivasi deterministik untuk demo/pengembangan tanpa koneksi ML API
2. **LIVE Mode** — Terhubung ke ML API eksternal untuk data prediktif realtime, disimpan di MongoDB untuk fleksibilitas dan kecepatan akses

---

## Arsitektur Sistem

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        PRATYAKSA — ARSITEKTUR SISTEM                         │
│                                                                              │
│  Browser                                                                     │
│     │                                                                        │
│     ▼  :80                                                                   │
│  ┌──────────┐                                                               │
│  │  Nginx   │  ← satu pintu masuk (reverse proxy)                           │
│  └────┬─────┘                                                               │
│       │                                                                      │
│  ┌────┴──────────────────┐                          ┌──────────────────────┐ │
│  │  /api/*               │                          │  /*                  │ │
│  ▼                       ▼                          ▼                      │ │
│  ┌──────────────────┐  ┌──────────────────────┐  ┌────────────────────┐   │ │
│  │  Rust Backend     │  │  Nuxt Frontend (SSR) │  │  Telegram Bot      │   │ │
│  │  (Actix-Web :8080)│  │  (Nuxt 4 :3000)     │  │  (gRPC :50051)     │   │ │
│  └──────┬───────────┘  └──────────────────────┘  └────────────────────┘   │ │
│         │                                                                   │ │
│    ┌────┴─────────────────┐                                                  │
│    │                      │                                                  │
│    ▼                      ▼                                                  │
│  ┌──────────────┐  ┌──────────────┐     ┌───────────────────────────────┐  │
│  │  PostgreSQL   │  │   MongoDB    │     │  ML API (FastAPI :6000)      │  │
│  │  (Simulasi)   │  │   (Live)     │◄────│  ── XGBoost + LSTM MoE       │  │
│  │               │  │              │     │  ── Digital Twin + SHAP      │  │
│  │  ── users     │  │  live_       │     │  ── Drift Detection          │  │
│  │  ── unit_     │  │  predictions │     └──────────────┬───────────────┘  │
│  │     tambang   │  │  live_fleet_ │                    │                   │
│  │  ── jenis_    │  │  snapshots   │                    ▼                   │
│  │     alat_berat│  │  live_sensor_│     ┌───────────────────────────────┐  │
│  │  ── sensor_   │  │  readings    │     │  Redis Streams + TimescaleDB  │  │
│  │     telemetry │  │  live_drift_ │     │  (ML Internal)                │  │
│  │  ── work_     │  │  logs        │     └───────────────────────────────┘  │
│  │     orders    │  │  live_work_  │                                        │
│  └──────────────┘  │  orders      │                                        │
│                    └──────────────┘                                        │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Lapisan ML Inference (detail)

```
Sensor (6 titik/unit) → Edge Node (ONNX, <500ms)
    ↓ jika online
Mobile Hotspot 4G/LTE → MQTT → Mosquitto → Bridge → Redis Streams
    ↓ jika offline
Buffer SQLite 72 jam → sinkronisasi asinkron saat sinyal pulih
    ↓
FastAPI Inference:
  ├─ XGBoost (3-class anomaly: NORMAL/WARNING/CRITICAL)
  ├─ LSTM MoE (RUL hierarkis: 7 sub-komponen)
  ├─ Digital Twin (physics cross-check: brake, bearing, hydraulic)
  ├─ SHAP Explainability (waterfall plot)
  └─ Drift Detection (Z-score real-time)
    ↓
TimescaleDB (sensor + prediksi) ↔ Redis Cache (result:{asset_id}, TTL 1h)
    ↓
Airflow (weekly retrain + daily drift + daily quality)
```

---

## Alur Data

### Mode SIMULASI

```
Frontend → API → PostgreSQL (unit_tambang, sensor_telemetry)
  ↓
Derivation deterministik: health% + seed → 33 parameter telemetri
  ↓
  ├─ Risk score + RUL prediction
  ├─ SHAP-like contributions
  ├─ Component health radar
  └─ Sensor history 24 jam
```

### Mode LIVE

```
ML API (sensor → edge → MQTT → Redis → FastAPI inference)
  ↓
Rust Backend polling tiap 5 detik:
  ├─ GET /health → cek ketersediaan
  ├─ GET /fleet → daftar armada
  └─ GET /result/{asset_id} → detail prediksi per unit
  ↓
MongoDB batch insert (100 item / 500ms):
  ├─ live_predictions → prediksi ML lengkap
  ├─ live_fleet_snapshots → snapshot fleet periodik
  ├─ live_drift_logs → event drift detection
  └─ live_work_orders → work order dari ML API
  ↓
Sync background tiap 60 detik:
  └─ ML PostgreSQL → MongoDB (data history 1 jam terakhir)
  ↓
Frontend membaca dari MongoDB via API /api/v1/live/*
```

---

## Mode Operasi

| Mode | Sumber Data | Storage | Use Case |
|------|------------|---------|----------|
| **SIMULASI** | PostgreSQL (derivasi deterministik) | PostgreSQL | Demo, development, tanpa koneksi ML API |
| **LIVE** | ML API (inference realtime) | MongoDB (cache) | Produksi, data prediktif akurat |

Pengguna dapat mengganti mode kapan saja melalui `POST /api/v1/pratyaksa/mode` dengan body `{"mode": "live"}` atau `{"mode": "simulasi"}`.

---

## Tech Stack

### Layer 1: UI & Simulasi (`pratyaksa/`)

| Layer | Teknologi |
|-------|-----------|
| Frontend | Nuxt 4, Vue 3, Tailwind CSS, shadcn-nuxt |
| Visualisasi | Chart.js, Leaflet, `<model-viewer>` (3D) |
| Backend API | Rust, Actix-Web 4, SQLx, validator |
| Database Simulasi | PostgreSQL 16 |
| Database Live | MongoDB 7 |
| Telegram Bot | Rust, tonic (gRPC/HTTP2), reqwest, prost |
| Auth | JWT (jsonwebtoken) + bcrypt |
| Proxy | Nginx 1.27 |

### Layer 2: ML Inference (`ml-pratyaksa/`)

| Layer | Teknologi |
|-------|-----------|
| Backend Analytics | Python FastAPI (ASGI) — XGBoost + LSTM inference, SHAP |
| Edge Inference | ONNX Runtime — XGBoost ONNX di ARM Cortex-A53 |
| Data Ingestion | Python (paho-mqtt, redis-py) — MQTT→Redis bridge |
| Message Queue | Redis 8.0 — Streams + consumer group |
| ML/DL | XGBoost 3.2, Keras 3.14 + TensorFlow 2.21, scikit-learn 1.8 |
| Explainability | SHAP 0.51 — TreeExplainer, waterfall plot |
| Database | PostgreSQL 16 + TimescaleDB — hypertable, kompresi >90% |
| Monitoring | Prometheus + Grafana |
| MLOps | Apache Airflow 2.9, MLflow 3.13 |
| Edge Hardware | Raspberry Pi Zero 2W / ESP32-S3, Nextion HMI, ADXL345, MAX6675 |
| Orkestrasi | Docker Compose |

---

## Struktur Proyek

```
kic/
│
├── pratyaksa/                          # 🖥️ Layer 1: UI & Simulasi
│   ├── frontend_nuxt/                  # Frontend SSR (Nuxt 4)
│   │   ├── app/
│   │   │   ├── pages/panel/            # Halaman dashboard, analisa, work order, dll
│   │   │   ├── composables/            # useApi, usePratyaksa, useAuth
│   │   │   └── components/             # ModeSelector, ModeLockTabel, Sidebar
│   │   └── server/routes/svc/          # gRPC bridge + upload model 3D
│   │
│   ├── backend_rust/                   # Rust Backend API
│   │   ├── src/
│   │   │   ├── main.rs                 # Entrypoint: init DBs, polling, sync
│   │   │   ├── config.rs               # AppConfig
│   │   │   ├── db/
│   │   │   │   ├── postgres.rs         # PostgreSQL pool + migrasi
│   │   │   │   └── mongo.rs            # MongoDB + batch consumer channels
│   │   │   ├── models/
│   │   │   │   ├── live.rs             # LivePrediction, LiveFleetSnapshot, dll
│   │   │   │   ├── telemetry.rs        # SensorTelemetry (33 kolom)
│   │   │   │   ├── unit_tambang.rs     # UnitTambang
│   │   │   │   └── ...                 # auth, analisa, work_order, jenis_alat_berat
│   │   │   ├── routes/
│   │   │   │   ├── dashboard.rs        # GET /dashboard (mode-aware)
│   │   │   │   ├── health_analytics.rs # GET /analisa/overview & /unit/{id} (mode-aware)
│   │   │   │   ├── live.rs            # GET /api/v1/live/* (MongoDB)
│   │   │   │   ├── pratyaksa.rs        # Proxy ke ML API + simulator fallback
│   │   │   │   └── ...                 # auth, telemetry, work_order, unit_tambang
│   │   │   └── pratyaksa/
│   │   │       ├── mod.rs             # Polling + MongoDB storage + state
│   │   │       ├── models.rs          # FleetAsset, HealthResponse, FEATURE_NAMES
│   │   │       └── sync.rs            # Background sync dari ML PostgreSQL
│   │   └── migrations/                 # 8 SQL migration files
│   │
│   ├── telegram_bot_grpc/              # Rust gRPC Telegram Bot
│   │   ├── proto/alert.proto
│   │   └── src/main.rs
│   │
│   └── nginx/nginx.conf                # Reverse proxy
│
├── ml-pratyaksa/                       # 🧠 Layer 2: ML Inference
│   ├── api/
│   │   ├── app.py                      # FastAPI — 8 endpoints, LSTM experts, SHAP, drift
│   │   └── prescriptive.py             # Recommendation engine
│   ├── edge/                           # Edge Computing
│   │   ├── main.py                     # Sensor loop → MQTT
│   │   ├── inference.py                # ONNX inference orchestrator
│   │   ├── digital_twin.py             # Physics models
│   │   ├── buffer.py                   # SQLite offline buffer (72h)
│   │   └── drivers/                    # ADXL345, MAX6675, pressure transducer
│   ├── artifacts/                      # Trained models
│   │   ├── artifact_xgb_model.json     # XGBoost classifier
│   │   ├── artifact_lstm_{type}.keras  # 4 LSTM experts
│   │   └── artifact_scaler.pkl         # StandardScaler (37 fitur)
│   ├── airflow/dags/                   # MLOps Pipeline
│   │   ├── retrain_pipeline.py         # Weekly retrain
│   │   ├── drift_detection.py          # Daily KS-test drift
│   │   └── data_quality_check.py       # Daily null check
│   ├── database/
│   │   └── schema.sql                  # TimescaleDB schema (8 tables + views)
│   ├── simulator/
│   │   └── stream_simulator.py         # Parquet replay → Redis Streams
│   ├── monitoring/
│   │   ├── prometheus.yml
│   │   └── grafana/dashboards/
│   └── docker-compose.yml              # 11 services orchestration
```

---

## API Endpoints

### Layer 1: UI Backend (Rust) — prefix `/api/v1`

#### Public (no auth)
| Method | Path | Deskripsi |
|--------|------|-----------|
| GET | `/health` | Health check service |
| GET | `/fleet-summary` | Ringkasan fleet untuk bot Telegram |

#### Auth (JWT)
| Method | Path | Deskripsi |
|--------|------|-----------|
| POST | `/auth/register` | Daftar akun baru |
| POST | `/auth/login` | Login, dapat JWT token |
| GET | `/auth/me` | Info user aktif |

#### Dashboard
| Method | Path | Deskripsi |
|--------|------|-----------|
| GET | `/dashboard` | Stats KPI + chart (mode-aware: live/simulasi) |

#### Master Data
| Method | Path | Deskripsi |
|--------|------|-----------|
| GET/POST | `/jenis-alat-berat` | List / Tambah jenis alat berat |
| GET/PUT/DELETE | `/jenis-alat-berat/{id}` | Detail / Update / Hapus |
| GET/POST | `/unit-tambang` | List / Tambah unit tambang |
| GET/PUT/DELETE | `/unit-tambang/{id}` | Detail / Update / Hapus |

#### Health Analytics
| Method | Path | Deskripsi |
|--------|------|-----------|
| GET | `/analisa/overview` | Analitik kesehatan armada (mode-aware) |
| GET | `/analisa/unit/{id}` | Analitik detail satu unit (mode-aware) |
| GET/POST | `/analisa` | List / Buat laporan manual (MongoDB) |
| GET/PUT/DELETE | `/analisa/{id}` | Detail / Update / Hapus laporan |

#### Telemetry
| Method | Path | Deskripsi |
|--------|------|-----------|
| POST | `/telemetry` | Ingest data telemetri |
| GET | `/telemetry/unit/{id}` | Riwayat telemetri per unit |

#### Work Orders
| Method | Path | Deskripsi |
|--------|------|-----------|
| GET/POST | `/work-orders` | List / Buat work order |
| GET/PUT | `/work-orders/{id}` | Detail / Update status |

#### Pratyaksa (ML API Bridge)
| Method | Path | Deskripsi |
|--------|------|-----------|
| GET | `/pratyaksa/status` | Status koneksi & mode |
| GET | `/pratyaksa/fleet` | Daftar fleet (live/simulasi) |
| GET | `/pratyaksa/fleet/health` | Ringkasan kesehatan fleet |
| GET | `/pratyaksa/result/{asset_id}` | Detail prediksi per unit |
| POST | `/pratyaksa/predict` | Prediksi (37 fitur sensor) |
| POST | `/pratyaksa/workorder` | Buat work order via ML API |
| GET | `/pratyaksa/features` | Daftar 37 fitur sensor |
| GET | `/pratyaksa/explain/{pred_id}` | SHAP explanation |
| POST | `/pratyaksa/reload-models` | Hot-reload model |
| POST | `/pratyaksa/mode` | Ganti mode (live/simulasi) |

#### Live Data (MongoDB)
| Method | Path | Deskripsi |
|--------|------|-----------|
| GET | `/live/predictions` | Prediksi tersimpan di MongoDB |
| GET | `/live/predictions/{asset_id}/latest` | Prediksi terbaru per unit |
| GET | `/live/fleet` | Fleet snapshots tersimpan |
| GET | `/live/work-orders` | Work orders dari ML API |
| GET | `/live/stats` | Statistik agregat MongoDB |

### Layer 2: ML API (FastAPI)

| Method | Path | Auth | Deskripsi |
|--------|------|:----:|-----------|
| GET | `/health` | ✗ | Health check (Redis, Postgres, models) |
| POST | `/predict` | API Key | Prediksi tunggal: risk, RUL, twin, drift |
| GET | `/result/{asset_id}` | ✗ | Latest cached prediction |
| GET | `/fleet` | ✗ | Fleet status agregat |
| GET | `/features` | API Key | Daftar 37 fitur sensor |
| GET | `/explain/{pred_id}` | API Key | SHAP waterfall plot (base64 PNG) |
| POST | `/workorder` | API Key | Rekomendasi work order preskriptif |
| POST | `/reload-models` | API Key | Hot-reload model tanpa downtime |
| GET | `/metrics` | ✗ | Prometheus metrics |

### Nuxt Server Routes (di luar `/api`)
| Method | Path | Deskripsi |
|--------|------|-----------|
| POST | `/svc/send-alert` | Proxy kirim alert ke Telegram via gRPC |
| POST | `/svc/upload-model` | Upload model 3D (.glb/.gltf, maks 50 MB) |

---

## Panduan Menjalankan

### Prasyarat

- Docker & Docker Compose
- Python 3.11+ (untuk development ML)
- Rust (untuk development backend)
- Node.js 20+ & pnpm (untuk development frontend)
- Port tersedia: 80, 3000, 5432, 6000, 6379, 8080, 27017, 50051

### 1. Setup Environment

```bash
# Layer UI
cd pratyaksa
cp .env.example .env
# Edit sesuai kebutuhan

# Layer ML (opsional, untuk LIVE mode)
cd ../ml-pratyaksa
cp .env.example .env
```

### 2. Jalankan Layer UI (Full Stack)

```bash
cd pratyaksa

# Semua service
docker compose --profile full up -d

# Atau bertahap
docker compose --profile db up -d          # PostgreSQL + MongoDB
docker compose --profile backend up -d     # + Backend Rust
docker compose --profile full up -d        # + Frontend + Nginx + Bot
```

### 3. Jalankan Layer ML (untuk LIVE mode)

```bash
cd ml-pratyaksa
docker compose up -d

# Verifikasi
curl http://localhost:6000/health
```

### 4. Akses Aplikasi

| Service | URL | Keterangan |
|---------|-----|------------|
| Aplikasi | http://localhost | Nginx (frontend + API) |
| Backend API | http://localhost/api/v1 | Via Nginx |
| API Health | http://localhost/api/v1/health | Health check |
| ML API Docs | http://localhost:6000/docs | Swagger UI |
| Grafana | http://localhost:6001 | `admin` / `pratyaksa2026` |
| MLflow | http://localhost:6050 | Experiment tracking |
| Airflow | http://localhost:6080 | DAG scheduler |
| Prometheus | http://localhost:6090 | Metrics |
| pgAdmin | http://localhost:5050 | Profile: tools |
| Mongo Express | http://localhost:8081 | Profile: tools |

### 5. Login Default

```
Email    : admin@pratyaksa.id
Password : admin123
```

### Profiles Docker Compose

| Profile | Services |
|---------|----------|
| `db` | PostgreSQL + MongoDB |
| `backend` | db + Backend Rust |
| `bot` | Telegram bot (gRPC) |
| `full` | Semua: db + backend + bot + frontend + nginx |
| `tools` | Mongo Express + pgAdmin |

---

## Development

### Backend Rust

```bash
cd pratyaksa/backend_rust

# Jalankan database
docker compose --profile db up -d

# Jalankan backend
cargo run
# Migrasi SQL otomatis dijalankan saat startup
```

### Frontend Nuxt

```bash
cd pratyaksa/frontend_nuxt
pnpm install
pnpm dev
```

Pastikan env berikut sesuai:

```bash
NUXT_PUBLIC_API_BASE=http://localhost:8080/api/v1
NUXT_TELEGRAM_GRPC_TARGET=127.0.0.1:50051
```

### ML API (Python)

```bash
cd ml-pratyaksa

# Virtual environment
python -m venv venv
source venv/bin/activate
pip install -r api/requirements.txt

# Jalankan
cd api
uvicorn app:app --reload --port 6000
```

### Telegram Bot

```bash
cd pratyaksa/telegram_bot_grpc
cp .env.example .env
# Isi TELEGRAM_BOT_TOKEN
cargo run
```

### Testing ML

```bash
cd ml-pratyaksa
ENV=development python test_core.py
python test_load.py
```

**Lingkup test:**
- ✅ Risk resolution (XGBoost vs LSTM conflict)
- ✅ Hierarchy enforcement (part ≤ component ≤ system)
- ✅ Digital Twin physics models (brake, bearing, hydraulic)
- ✅ Drift detection (Z-score)
- ✅ Dropout flag detection (flatline, NaN)

---

## Environment Variables

### Layer UI (`pratyaksa/.env`)

| Variable | Keterangan |
|----------|------------|
| `DATABASE_URL` | Koneksi PostgreSQL (simulasi) |
| `MONGODB_URL` | Koneksi MongoDB (live data) |
| `JWT_SECRET` | Secret JWT — **ganti di production** |
| `LIVE_API_URL` | URL ML API (`http://192.168.101.3:6000`) |
| `LIVE_API_KEY` | API Key untuk ML API |
| `ML_POSTGRES_URL` | Koneksi PostgreSQL ml-pratyaksa (sync) |
| `ML_SYNC_INTERVAL` | Interval sync (detik, default 60) |
| `PRATYAKSA_POLL_INTERVAL` | Interval polling ML API (detik, default 5) |
| `TELEGRAM_BOT_TOKEN` | Token bot Telegram |
| `WO_LINK_BASE` | Base URL link WO di Telegram |

### Layer ML (`ml-pratyaksa/.env`)

| Variable | Keterangan |
|----------|------------|
| `POSTGRES_PASSWORD` | Password PostgreSQL/TimescaleDB |
| `PRATYAKSA_API_KEYS` | Daftar API Key (dipisah koma) |
| `REDIS_URL` | `redis://redis:6379` |
| `MQTT_USER` / `MQTT_PASS` | Kredensial MQTT broker |
| `GRAFANA_PASSWORD` | Password Grafana |

---

## Monitoring & MLOps

### Grafana Dashboard

Tersedia di http://localhost:6001 dengan dashboard pratyaksa-monitoring yang mencakup:
- Fleet health overview
- Inference latency
- Drift metrics
- Resource usage

### Prometheus Metrics

Tersedia di http://localhost:6090 dengan endpoint `/metrics` pada FastAPI (scrape setiap 15s).

### Airflow Pipelines

| DAG | Schedule | Deskripsi |
|-----|----------|-----------|
| `retrain_pipeline` | Weekly (Minggu 00:00) | Retrain XGBoost + LSTM, hot-reload |
| `drift_detection` | Daily (01:00) | KS-test drift detection |
| `data_quality_check` | Daily (02:00) | Null/quality check |

### MLflow

Tersedia di http://localhost:6050 untuk experiment tracking dan model registry.

---

## Port yang Digunakan

| Service | Port | Layer |
|---------|:----:|:-----:|
| Nginx (aplikasi) | **80** | UI |
| Frontend Nuxt | **3000** | UI |
| PostgreSQL (simulasi) | **5432** | UI |
| MongoDB | **27017** | UI |
| Rust Backend | **8080** | UI |
| Telegram Bot (gRPC) | **50051** | UI |
| pgAdmin | **5050** | UI |
| Mongo Express | **8081** | UI |
| FastAPI Inference | **6000** | ML |
| Grafana | **6001** | ML |
| MLflow | **6050** | ML |
| Airflow Webserver | **6080** | ML |
| Prometheus | **6090** | ML |
| Mosquitto MQTT | **6883** | ML |
| Redis | **6379** | ML |
| PostgreSQL/TimescaleDB | **5432** | ML |

> **Catatan**: PostgreSQL UI dan ML menggunakan port yang sama (5432) tetapi container dan volume berbeda.

---

## Operasional

### Menghentikan service

```bash
cd pratyaksa
docker compose --profile full down

# Hapus volume (data hilang permanen)
docker compose --profile full down -v
```

### Rebuild setelah perubahan

```bash
cd pratyaksa

# Frontend
docker compose --profile full build frontend
docker compose --profile full up -d --force-recreate --no-deps frontend

# Backend
docker compose --profile full build backend
docker compose --profile full up -d --force-recreate --no-deps backend

# Telegram bot
docker compose --profile full build telegram-bot
docker compose --profile full up -d --force-recreate --no-deps telegram-bot

# Rebuild semua
docker compose --profile full up -d --build
```

### Logs

```bash
# Backend
docker logs -f pratyaksa-backend

# ML API
docker logs -f pratyaksa-api

# Semua service
docker compose logs -f
```

---

## Tim Pengembang

| Jabatan | Nama | Peran | Kontak |
|---------|------|-------|--------|
| **Ketua** | Baits Rika Saputra | Full Stack Developer | brsaputra14@gmail.com |
| **Anggota 1** | Virgiawan Prima Rizky | Data & ML Engineer | [LinkedIn](https://www.linkedin.com/in/virgiawan-prima-rizky) |
| **Anggota 2** | Raihan Akbar Ramadhan | UI/UX Designer | — |
| **Anggota 3** | Farhan Raditya Al Gazali | IoT Engineer | — |

**Perguruan Tinggi:** Politeknik Negeri Samarinda  
**Kompetisi:** Kideco Innovation Challenge (KIC) 2026

---

<div align="center">

**⛏️ PRATYAKSA — AIoT Predictive + Prescriptive Maintenance**

*Mewujudkan Zero Unplanned Breakdowns melalui Kecerdasan Buatan dan Internet of Things*

Tim Oryphem — Politeknik Negeri Samarinda — KIC 2026

</div>