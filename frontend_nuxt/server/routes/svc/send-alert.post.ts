import { credentials, loadPackageDefinition, Metadata, type ServiceError } from '@grpc/grpc-js'
import { loadSync } from '@grpc/proto-loader'
import { mkdtempSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'
import { tmpdir } from 'node:os'

/**
 * Proxy pengiriman alert ke service bot Telegram via gRPC (low latency).
 *
 * Alur: Browser → route ini (HTTP) → gRPC ke service Rust (tonic) → Telegram API.
 * Ditempatkan di /svc/ (bukan /api/) agar di Docker tidak diarahkan Nginx ke backend Rust.
 *
 * Target gRPC di-override via env NUXT_TELEGRAM_GRPC_TARGET (default host:port).
 */

// Definisi proto di-embed sebagai string supaya andal di dev maupun build .output
// (tidak bergantung pada file yang ikut ter-bundle). Harus sinkron dengan
// telegram_bot_grpc/proto/alert.proto.
const PROTO = `
syntax = "proto3";
package alert;

service AlertService {
  rpc SendAlert(AlertRequest) returns (AlertResponse);
  rpc HealthCheck(HealthRequest) returns (HealthResponse);
}

message AlertRequest {
  string asset_id  = 1;
  string model     = 2;
  string lokasi    = 3;
  string status    = 4;
  string rul       = 5;
  string shap1     = 6;
  string shap2     = 7;
  string part_name = 8;
  string part_no   = 9;
  string stok      = 10;
}

message AlertResponse {
  bool   success  = 1;
  string message  = 2;
  string asset_id = 3;
}

message HealthRequest {}

message HealthResponse {
  bool  ok             = 1;
  int32 fleet_total    = 2;
  int32 fleet_critical = 3;
}
`

interface AlertResponse {
  success: boolean
  message: string
  asset_id: string
}

// Cache client gRPC per target agar koneksi HTTP/2 dipakai ulang antar request.
const clientCache = new Map<string, any>()

function getClient(target: string) {
  const cached = clientCache.get(target)
  if (cached) return cached

  const dir = mkdtempSync(join(tmpdir(), 'pratyaksa-proto-'))
  const protoPath = join(dir, 'alert.proto')
  writeFileSync(protoPath, PROTO)

  const packageDefinition = loadSync(protoPath, {
    keepCase: true,
    longs: String,
    enums: String,
    defaults: true,
    oneofs: true,
  })
  const proto: any = loadPackageDefinition(packageDefinition)
  const client = new proto.alert.AlertService(target, credentials.createInsecure())
  clientCache.set(target, client)
  return client
}

function sendAlertGrpc(target: string, payload: Record<string, string>): Promise<AlertResponse> {
  const client = getClient(target)
  const deadline = new Date(Date.now() + 20000) // 20 dtk (pengiriman Telegram bisa lambat)
  return new Promise((resolve, reject) => {
    client.SendAlert(
      payload,
      new Metadata(),
      { deadline },
      (err: ServiceError | null, res: AlertResponse) => {
        if (err) reject(err)
        else resolve(res)
      },
    )
  })
}

export default defineEventHandler(async (event) => {
  const body = (await readBody(event)) as Record<string, any>
  const config = useRuntimeConfig(event)
  const target = (config.telegramGrpcTarget as string) || '127.0.0.1:50051'

  // Normalisasi payload ke string (proto AlertRequest semua field string)
  const payload: Record<string, string> = {}
  for (const [k, v] of Object.entries(body || {})) {
    payload[k] = v == null ? '' : String(v)
  }

  try {
    const upstream = await sendAlertGrpc(target, payload)
    if (!upstream.success) {
      throw createError({ statusCode: 502, statusMessage: upstream.message || 'Service bot menolak alert.' })
    }
    return { status: 'success', upstream }
  } catch (e: any) {
    // gRPC status code: 14 = UNAVAILABLE, 4 = DEADLINE_EXCEEDED, 9 = FAILED_PRECONDITION
    const code = e?.code
    let message: string
    if (code === 14) {
      message = `Tidak dapat terhubung ke service bot di ${target} (UNAVAILABLE). Pastikan container telegram-bot aktif.`
    } else if (code === 4) {
      message = `Service bot terlalu lama merespons (timeout) di ${target}.`
    } else if (code === 9) {
      message = e?.details || 'Token/Chat ID Telegram belum dikonfigurasi pada service bot.'
    } else if (e?.statusCode) {
      throw e
    } else {
      message = e?.details || e?.message || `Gagal menghubungi service bot via gRPC (${target}).`
    }
    throw createError({ statusCode: 502, statusMessage: message })
  }
})
