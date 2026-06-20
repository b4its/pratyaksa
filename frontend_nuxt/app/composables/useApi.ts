/**
 * Composable untuk semua API calls ke backend Rust
 *
 * Base URL:
 *   - Development lokal : http://localhost:8080/api/v1  (dari nuxt.config runtimeConfig)
 *   - Docker production  : /api/v1  (Nginx meneruskan ke backend:8080)
 *
 * Nuxt $fetch otomatis resolve path relatif ke origin yang benar di SSR maupun client.
 */

export const useApi = () => {
  const config = useRuntimeConfig()
  const baseURL = config.public.apiBase as string

  const getAuthHeaders = (): Record<string, string> => {
    if (!import.meta.client) return {}
    const token = localStorage.getItem('auth_token')
    return token ? { Authorization: `Bearer ${token}` } : {}
  }

  const api = {
    // ---- Auth ----
    async login(email: string, password: string) {
      return await $fetch(`${baseURL}/auth/login`, {
        method: 'POST',
        body: { email, password },
      })
    },

    async register(name: string, email: string, password: string) {
      return await $fetch(`${baseURL}/auth/register`, {
        method: 'POST',
        body: { name, email, password },
      })
    },

    async me() {
      return await $fetch(`${baseURL}/auth/me`, {
        headers: getAuthHeaders(),
      })
    },

    // ---- Dashboard ----
    async getDashboardStats() {
      return await $fetch(`${baseURL}/dashboard`, {
        headers: getAuthHeaders(),
      })
    },

    // ---- Jenis Alat Berat ----
    async getJenisAlatBerat(params?: { page?: number; per_page?: number; search?: string }) {
      const query = new URLSearchParams()
      if (params?.page) query.set('page', String(params.page))
      if (params?.per_page) query.set('per_page', String(params.per_page))
      if (params?.search) query.set('search', params.search)
      const qs = query.toString() ? `?${query.toString()}` : ''
      return await $fetch(`${baseURL}/jenis-alat-berat${qs}`, {
        headers: getAuthHeaders(),
      })
    },

    async createJenisAlatBerat(body: { nama: string; deskripsi?: string }) {
      return await $fetch(`${baseURL}/jenis-alat-berat`, {
        method: 'POST',
        headers: getAuthHeaders(),
        body,
      })
    },

    async updateJenisAlatBerat(id: string, body: { nama?: string; deskripsi?: string }) {
      return await $fetch(`${baseURL}/jenis-alat-berat/${id}`, {
        method: 'PUT',
        headers: getAuthHeaders(),
        body,
      })
    },

    async deleteJenisAlatBerat(id: string) {
      return await $fetch(`${baseURL}/jenis-alat-berat/${id}`, {
        method: 'DELETE',
        headers: getAuthHeaders(),
      })
    },

    // ---- Unit Tambang ----
    async getUnitTambang(params?: {
      page?: number
      per_page?: number
      search?: string
      status?: string
      jenis_alat_berat_id?: string
    }) {
      const query = new URLSearchParams()
      if (params?.page) query.set('page', String(params.page))
      if (params?.per_page) query.set('per_page', String(params.per_page))
      if (params?.search) query.set('search', params.search)
      if (params?.status) query.set('status', params.status)
      if (params?.jenis_alat_berat_id) query.set('jenis_alat_berat_id', params.jenis_alat_berat_id)
      const qs = query.toString() ? `?${query.toString()}` : ''
      return await $fetch(`${baseURL}/unit-tambang${qs}`, {
        headers: getAuthHeaders(),
      })
    },

    async createUnitTambang(body: {
      code: string
      jenis_alat_berat_id: string
      status: string
      health: number
      maintenance: string
      savings: number
      img_url?: string
      model3d_url?: string
      lat?: number
      lng?: number
    }) {
      return await $fetch(`${baseURL}/unit-tambang`, {
        method: 'POST',
        headers: getAuthHeaders(),
        body,
      })
    },

    async updateUnitTambang(
      id: string,
      body: Partial<{
        code: string
        jenis_alat_berat_id: string
        status: string
        health: number
        maintenance: string
        savings: number
        img_url: string
        model3d_url: string
        lat: number
        lng: number
      }>,
    ) {
      return await $fetch(`${baseURL}/unit-tambang/${id}`, {
        method: 'PUT',
        headers: getAuthHeaders(),
        body,
      })
    },

    async deleteUnitTambang(id: string) {
      return await $fetch(`${baseURL}/unit-tambang/${id}`, {
        method: 'DELETE',
        headers: getAuthHeaders(),
      })
    },

    // ---- Upload model 3D (.glb/.gltf) ke media frontend (Nuxt server route) ----
    async uploadModel(file: File) {
      const fd = new FormData()
      fd.append('file', file)
      return await $fetch('/svc/upload-model', { method: 'POST', body: fd })
    },

    // ---- Kirim alert ke bot Telegram (via proxy server Nuxt → endpoint data science) ----
    async sendAlert(payload: {
      asset_id: string
      model: string
      lokasi: string
      status: string
      rul: string
      shap1: string
      shap2: string
      part_name: string
      part_no: string
      stok: string
    }) {
      return await $fetch('/svc/send-alert', { method: 'POST', body: payload })
    },

    // ---- Analisa Kerusakan (Health Analytics - realtime) ----
    async getAnalisaOverview() {
      return await $fetch(`${baseURL}/analisa/overview`, {
        headers: getAuthHeaders(),
      })
    },

    async getUnitAnalysis(id: string) {
      return await $fetch(`${baseURL}/analisa/unit/${id}`, {
        headers: getAuthHeaders(),
      })
    },

    // ---- Work Orders (persistensi PostgreSQL) ----
    async getWorkOrders(params?: { page?: number; per_page?: number; wo_status?: string; asset_code?: string }) {
      const query = new URLSearchParams()
      if (params?.page) query.set('page', String(params.page))
      if (params?.per_page) query.set('per_page', String(params.per_page))
      if (params?.wo_status) query.set('wo_status', params.wo_status)
      if (params?.asset_code) query.set('asset_code', params.asset_code)
      const qs = query.toString() ? `?${query.toString()}` : ''
      return await $fetch(`${baseURL}/work-orders${qs}`, { headers: getAuthHeaders() })
    },

    async createWorkOrder(body: {
      asset_code: string
      equipment_type?: string
      status_unit: string
      priority?: string
      component?: string
      part_no?: string
      rul_hours?: number
      est_cost?: number
      scheduled_at?: string
      est_completion_at?: string
      technician?: string
      notes?: string
    }) {
      return await $fetch(`${baseURL}/work-orders`, {
        method: 'POST',
        headers: getAuthHeaders(),
        body,
      })
    },

    async updateWorkOrder(
      id: string,
      body: { wo_status?: string; technician?: string; notes?: string; feedback?: string },
    ) {
      return await $fetch(`${baseURL}/work-orders/${id}`, {
        method: 'PUT',
        headers: getAuthHeaders(),
        body,
      })
    },

    // ---- Telemetry (ingestion & history) ----
    async getTelemetryHistory(unitId: string, limit = 100) {
      return await $fetch(`${baseURL}/telemetry/unit/${unitId}?limit=${limit}`, {
        headers: getAuthHeaders(),
      })
    },

    async postTelemetry(body: Record<string, any>) {
      return await $fetch(`${baseURL}/telemetry`, {
        method: 'POST',
        headers: getAuthHeaders(),
        body,
      })
    },

    // ---- Analisa Kerusakan (laporan manual - MongoDB) ----
    async getAnalisa(params?: {
      page?: number
      per_page?: number
      unit_tambang_id?: string
      severity?: string
      status_analisa?: string
    }) {
      const query = new URLSearchParams()
      if (params?.page) query.set('page', String(params.page))
      if (params?.per_page) query.set('per_page', String(params.per_page))
      if (params?.unit_tambang_id) query.set('unit_tambang_id', params.unit_tambang_id)
      if (params?.severity) query.set('severity', params.severity)
      if (params?.status_analisa) query.set('status_analisa', params.status_analisa)
      const qs = query.toString() ? `?${query.toString()}` : ''
      return await $fetch(`${baseURL}/analisa${qs}`, {
        headers: getAuthHeaders(),
      })
    },

    async createAnalisa(body: {
      unit_tambang_id: string
      unit_code: string
      tipe_kerusakan: string
      deskripsi: string
      severity: string
      sensor_data: {
        suhu_mesin: number
        tekanan_oli: number
        rpm: number
        fuel_level: number
        vibration: number
        jam_operasi: number
      }
      rekomendasi: string
      dilaporkan_oleh: string
    }) {
      return await $fetch(`${baseURL}/analisa`, {
        method: 'POST',
        headers: getAuthHeaders(),
        body,
      })
    },

    async updateAnalisa(
      id: string,
      body: { status_analisa?: string; rekomendasi?: string; deskripsi?: string },
    ) {
      return await $fetch(`${baseURL}/analisa/${id}`, {
        method: 'PUT',
        headers: getAuthHeaders(),
        body,
      })
    },

    async deleteAnalisa(id: string) {
      return await $fetch(`${baseURL}/analisa/${id}`, {
        method: 'DELETE',
        headers: getAuthHeaders(),
      })
    },
  }

  return api
}
