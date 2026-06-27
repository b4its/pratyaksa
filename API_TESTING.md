# API Testing — Pratyaksa Backend

Panduan lengkap untuk menguji endpoint API backend Rust menggunakan `curl`.  
Semua request lewat Nginx di `http://localhost/api/v1`.

---

## Prasyarat

Pastikan container sudah jalan:

```bash
docker compose --profile full up -d
```

Cek status:

```bash
docker compose ps
```

Semua service (`backend`, `frontend`, `nginx`, `postgres`, `mongodb`, `telegram-bot`) harus berstatus **Up**.

---

## 1. Autentikasi

Semua endpoint kecuali `/health` dan `/fleet-summary` butuh JWT token.  
Token berlaku **24 jam** sejak login.

### Login

```bash
curl -s -X POST http://localhost/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@pratyaksa.id","password":"admin123"}'
```

Respons:

```json
{
  "status": "success",
  "data": {
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
    "user": {
      "id": "3e45bec6-67b6-4b5f-80e4-4272231adcb8",
      "name": "Budi Susanto",
      "email": "admin@pratyaksa.id",
      "role": "admin",
      "created_at": "2026-06-27T14:20:04.159489Z"
    }
  }
}
```

Simpan token ke file agar mudah dipakai ulang:

```bash
curl -s -X POST http://localhost/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@pratyaksa.id","password":"admin123"}' > login.json
```

Gunakan di setiap request berikutnya:

```bash
-H "Authorization: Bearer $(jq -r '.data.token' login.json)"
```

> **Catatan:** Karena shell fish tidak mendukung `VAR=$(...)` di inline, cara paling aman adalah simpan respons login ke file `.json` lalu referensikan dengan `$(jq -r ...)`.

---

## 2. Analisa Kerusakan — POST `/analisa`

Endpoint ini menyimpan laporan analisa kesehatan unit ke **MongoDB**.  
Data yang dikirim mencakup identitas unit, jenis kerusakan, data sensor realtime, dan rekomendasi tindak lanjut.

### Buat file payload

Buat file `analisa_payload.json`:

```json
{
  "unit_tambang_id": "2205270f-a0cd-485b-bb7e-dc66143c8a40",
  "unit_code": "DT-SCN-22",
  "tipe_kerusakan": "Overheating Mesin",
  "deskripsi": "Suhu mesin melebihi batas normal 105C, tekanan oli menurun drastis, indikasi kebocoran sistem pendingin dan penurunan performa mesin secara signifikan.",
  "severity": "CRITICAL",
  "sensor_data": {
    "suhu_mesin": 112.5,
    "tekanan_oli": 1.8,
    "rpm": 1850.0,
    "fuel_level": 32.0,
    "vibration": 4.7,
    "jam_operasi": 4821.5
  },
  "rekomendasi": "Hentikan operasi segera. Lakukan pemeriksaan sistem pendingin, ganti coolant, dan cek kebocoran gasket kepala silinder. Jadwalkan overhaul dalam 48 jam.",
  "dilaporkan_oleh": "Budi Susanto"
}
```

### Kirim request

```bash
curl -s -X POST http://localhost/api/v1/analisa \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $(jq -r '.data.token' login.json)" \
  -d @analisa_payload.json | jq '.'
```

### Respons sukses (201 Created)

```json
{
  "status": "success",
  "data": {
    "_id": { "$oid": "6a400eac88c2ed4f82808414" },
    "unit_tambang_id": "2205270f-a0cd-485b-bb7e-dc66143c8a40",
    "unit_code": "DT-SCN-22",
    "tipe_kerusakan": "Overheating Mesin",
    "deskripsi": "Suhu mesin melebihi batas normal 105C...",
    "severity": "CRITICAL",
    "sensor_data": {
      "suhu_mesin": 112.5,
      "tekanan_oli": 1.8,
      "rpm": 1850.0,
      "fuel_level": 32.0,
      "vibration": 4.7,
      "jam_operasi": 4821.5,
      "timestamp": "2026-06-27T17:55:56.091049586Z"
    },
    "rekomendasi": "Hentikan operasi segera...",
    "status_analisa": "OPEN",
    "dilaporkan_oleh": "Budi Susanto",
    "created_at": "2026-06-27T17:55:56.091049586Z",
    "updated_at": "2026-06-27T17:55:56.091049586Z"
  }
}
```

### Aturan validasi field

| Field              | Tipe     | Wajib | Keterangan                                      |
|--------------------|----------|-------|-------------------------------------------------|
| `unit_tambang_id`  | UUID     | ✅    | Harus UUID valid yang ada di PostgreSQL          |
| `unit_code`        | string   | ✅    | Kode unit, contoh: `DT-SCN-22`                  |
| `tipe_kerusakan`   | string   | ✅    | 2–200 karakter                                   |
| `deskripsi`        | string   | ✅    | Minimal 5 karakter                               |
| `severity`         | string   | ✅    | Nilai valid: `LOW`, `MEDIUM`, `HIGH`, `CRITICAL` |
| `sensor_data`      | object   | ✅    | Lihat sub-field di bawah                         |
| `rekomendasi`      | string   | ✅    | Teks rekomendasi tindak lanjut                   |
| `dilaporkan_oleh`  | string   | ✅    | Nama pelapor                                     |
| `status_analisa`   | —        | ❌    | Otomatis di-set `OPEN` saat create               |

**Sub-field `sensor_data`:**

| Field          | Tipe  | Satuan    |
|----------------|-------|-----------|
| `suhu_mesin`   | f64   | °C        |
| `tekanan_oli`  | f64   | bar       |
| `rpm`          | f64   | RPM       |
| `fuel_level`   | f64   | % (0–100) |
| `vibration`    | f64   | g-force   |
| `jam_operasi`  | f64   | jam       |

---

## 3. Endpoint Analisa Lainnya

### List semua laporan

```bash
curl -s http://localhost/api/v1/analisa \
  -H "Authorization: Bearer $(jq -r '.data.token' login.json)" | jq '.'
```

Filter opsional via query params:

```bash
# Filter by severity
curl -s "http://localhost/api/v1/analisa?severity=CRITICAL&page=1&per_page=5" \
  -H "Authorization: Bearer $(jq -r '.data.token' login.json)" | jq '.'

# Filter by unit
curl -s "http://localhost/api/v1/analisa?unit_tambang_id=2205270f-a0cd-485b-bb7e-dc66143c8a40" \
  -H "Authorization: Bearer $(jq -r '.data.token' login.json)" | jq '.'

# Filter by status
curl -s "http://localhost/api/v1/analisa?status_analisa=OPEN" \
  -H "Authorization: Bearer $(jq -r '.data.token' login.json)" | jq '.'
```

### Get by ID

```bash
curl -s http://localhost/api/v1/analisa/6a400eac88c2ed4f82808414 \
  -H "Authorization: Bearer $(jq -r '.data.token' login.json)" | jq '.'
```

### Update status laporan

```bash
curl -s -X PUT http://localhost/api/v1/analisa/6a400eac88c2ed4f82808414 \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $(jq -r '.data.token' login.json)" \
  -d '{"status_analisa":"IN_PROGRESS","rekomendasi":"Sedang dalam penanganan teknisi senior."}' | jq '.'
```

Nilai valid `status_analisa`: `OPEN`, `IN_PROGRESS`, `RESOLVED`

### Hapus laporan

```bash
curl -s -X DELETE http://localhost/api/v1/analisa/6a400eac88c2ed4f82808414 \
  -H "Authorization: Bearer $(jq -r '.data.token' login.json)" | jq '.'
```

---

## 4. Analitik Kesehatan Realtime

Endpoint ini membaca dari PostgreSQL dan menghitung kondisi unit secara deterministik — berbeda dengan CRUD analisa di atas yang ke MongoDB.

### Overview seluruh armada

```bash
curl -s http://localhost/api/v1/analisa/overview \
  -H "Authorization: Bearer $(jq -r '.data.token' login.json)" | jq '.'
```

### Detail satu unit

```bash
# Ganti {id} dengan UUID unit dari /unit-tambang
curl -s http://localhost/api/v1/analisa/unit/2205270f-a0cd-485b-bb7e-dc66143c8a40 \
  -H "Authorization: Bearer $(jq -r '.data.token' login.json)" | jq '.'
```

---

## 5. Referensi Unit Tambang

Untuk mendapatkan `unit_tambang_id` yang valid:

```bash
curl -s http://localhost/api/v1/unit-tambang \
  -H "Authorization: Bearer $(jq -r '.data.token' login.json)" | jq '[.data.data[] | {id, code, status, health}]'
```

Contoh output:

```json
[
  { "id": "2205270f-a0cd-485b-bb7e-dc66143c8a40", "code": "DT-SCN-22", "status": "CRITICAL", "health": 38 },
  { "id": "01ec354c-4861-48f4-add2-eed65e7c01cd", "code": "WL-SDLG-07", "status": "CRITICAL", "health": 45 },
  { "id": "4da0332b-3eed-463e-94d6-f66eb25e893e", "code": "DT-SCN-25", "status": "WARNING",  "health": 76 }
]
```

---

## 6. Health Check (tanpa auth)

```bash
curl -s http://localhost/api/v1/health | jq '.'
```

```json
{
  "status": "ok",
  "service": "Pratyaksa Backend",
  "version": "0.1.0"
}
```

---

## Troubleshooting

| Error                                      | Penyebab                              | Solusi                                           |
|--------------------------------------------|---------------------------------------|--------------------------------------------------|
| `Token tidak ditemukan`                    | Header Authorization tidak terkirim   | Pastikan format `Bearer <token>` benar           |
| `Token tidak valid atau sudah kadaluarsa`  | Token > 24 jam atau JWT secret beda   | Login ulang untuk dapat token baru               |
| `Severity harus: LOW, MEDIUM, HIGH, atau CRITICAL` | Nilai severity salah          | Gunakan salah satu nilai yang valid              |
| `unit_tambang_id` error / validation fail  | UUID tidak valid atau tidak ada di DB | Cek dulu via `GET /unit-tambang`                 |
| `502 Bad Gateway`                          | Backend container belum siap          | Tunggu beberapa detik, atau `docker restart pratyaksa-nginx` |
