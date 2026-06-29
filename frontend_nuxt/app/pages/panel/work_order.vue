<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick, watch } from 'vue'

// --- SIDEBAR ---
const menuItems = [
  { name: 'Dashboard',         path: '/panel/dashboard',        icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="7" height="9" x="3" y="3" rx="1"/><rect width="7" height="5" x="14" y="3" rx="1"/><rect width="7" height="9" x="14" y="12" rx="1"/><rect width="7" height="5" x="3" y="16" rx="1"/></svg>` },
  { name: 'Jenis Alat Berat',  path: '/panel/jenis_alat_berat', icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 17h4V5H2v12h3"/><path d="M20 17h2v-9l-2.5-3.5H14v12h3"/><path d="M14 6h4.5"/><circle cx="18.5" cy="17.5" r="2.5"/><circle cx="5.5" cy="17.5" r="2.5"/></svg>` },
  { name: 'Unit Tambang',      path: '/panel/unit_tambang',     icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 12 12 17 22 12"/><polyline points="2 17 12 22 22 17"/></svg>` },
  { name: 'Analisa Kerusakan', path: '/panel/analisa',          icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>` },
  { name: 'Work Order',        path: '/panel/work_order',       icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.12 2.12 0 0 1 3 3L12 15l-4 1 1-4Z"/></svg>` },
  { name: 'Kembali',           path: '../',                     icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m12 19-7-7 7-7"/><path d="M19 12H5"/></svg>` },
]
const activeMenu = ref('Work Order')

const { initAuth, user } = useAuth()
const { isDark, toggleTheme, initTheme } = useTheme()
const api = useApi()
const route = useRoute()

let ChartLib: any = null
let refreshTimer: any = null
const charts: Record<string, any> = {}

const isLoading = ref(true)
const errorMsg = ref('')
const lastUpdate = ref('')
const autoRefresh = ref(true)

// --- TYPES ---
interface WoItem {
  id: string
  code: string
  type: string
  status: string            // CRITICAL | WARNING | RUSAK
  health: number
  riskLevel: string
  riskScore: number
  rulHours: number
  rulUncertainty: number
  failureDate: Date
  repairByDate: Date
  repairDurationHours: number
  estCompletionDate: Date
  priority: string          // HIGH | MEDIUM | LOW
  topComponent: string
  components: { component: string; health: number }[]
  shap: { feature: string; value: number }[]
  twins: { label: string; hours: number }[]
  estCost: number
  woId: string
}

const items = ref<WoItem[]>([])
const selectedCode = ref<string | null>(null)
const statusFilter = ref<'ALL' | 'CRITICAL' | 'WARNING' | 'RUSAK'>('ALL')

// --- MODAL DETAIL ---
const modalOpen = ref(false)
const modalItem = ref<WoItem | null>(null)
const modalStart = ref<number>(Date.now())   // snapshot waktu buka modal (untuk progress)
const nowTick = ref<number>(Date.now())      // di-update tiap detik → animasi countdown
let tickTimer: any = null

// Form pembuatan Work Order
const woTechnician = ref('')
const woNotes = ref('')
const woSaving = ref(false)
const woToast = ref<{ ok: boolean; msg: string } | null>(null)
let woToastTimer: any = null

// Daftar WO tersimpan (dari DB)
const savedWorkOrders = ref<any[]>([])

const showToast = (ok: boolean, msg: string) => {
  woToast.value = { ok, msg }
  if (woToastTimer) clearTimeout(woToastTimer)
  woToastTimer = setTimeout(() => { woToast.value = null }, 5000)
}

const openModal = (it: WoItem) => {
  modalItem.value = it
  modalStart.value = Date.now()
  nowTick.value = Date.now()
  woTechnician.value = ''
  woNotes.value = ''
  modalOpen.value = true
}
const closeModal = () => { modalOpen.value = false }

// --- Animasi estimasi penyelesaian perbaikan ---
// Progress 0..100 dari saat modal dibuka sampai estimasi selesai.
const repairProgress = computed(() => {
  const it = modalItem.value
  if (!it) return 0
  const total = it.estCompletionDate.getTime() - modalStart.value
  if (total <= 0) return 100
  const elapsed = nowTick.value - modalStart.value
  return Math.max(0, Math.min(100, (elapsed / total) * 100))
})
// Sisa waktu menuju estimasi selesai (ms)
const repairRemainingMs = computed(() => {
  const it = modalItem.value
  if (!it) return 0
  return Math.max(0, it.estCompletionDate.getTime() - nowTick.value)
})
const repairCountdown = computed(() => {
  let ms = repairRemainingMs.value
  const h = Math.floor(ms / 3_600_000); ms -= h * 3_600_000
  const m = Math.floor(ms / 60_000); ms -= m * 60_000
  const s = Math.floor(ms / 1000)
  const pad = (n: number) => String(n).padStart(2, '0')
  return `${pad(h)}:${pad(m)}:${pad(s)}`
})

const submitWorkOrder = async () => {
  const it = modalItem.value
  if (!it) return
  woSaving.value = true
  try {
    const res: any = await api.createWorkOrder({
      asset_code: it.code,
      equipment_type: it.type,
      status_unit: it.status,
      priority: it.priority,
      component: it.topComponent,
      part_no: 'PRT-' + it.code.replace(/[^A-Za-z0-9]/g, '').toUpperCase().slice(0, 6),
      rul_hours: it.rulHours,
      est_cost: it.estCost,
      scheduled_at: it.repairByDate.toISOString(),
      est_completion_at: it.estCompletionDate.toISOString(),
      technician: woTechnician.value || undefined,
      notes: woNotes.value || undefined,
    })
    const wo = res?.data
    showToast(true, `Work Order ${wo?.wo_number || ''} untuk ${it.code} berhasil dibuat.`)
    await fetchWorkOrders()
    closeModal()
  } catch (e: any) {
    showToast(false, e?.data?.message || e?.message || 'Gagal membuat Work Order.')
  } finally {
    woSaving.value = false
  }
}

const updateWoStatus = async (id: string, wo_status: string) => {
  try {
    await api.updateWorkOrder(id, { wo_status })
    await fetchWorkOrders()
  } catch (e: any) {
    showToast(false, e?.data?.message || 'Gagal memperbarui status WO.')
  }
}

const fetchWorkOrders = async () => {
  try {
    const res: any = await api.getWorkOrders({ per_page: 500 })
    savedWorkOrders.value = res?.data?.data || []
  } catch {
    // diamkan jika belum login / belum ada data
  }
}

// --- Saved WO: pencarian + pagination + export + detail ---
const woSearch = ref('')
const woPage = ref(1)
const woPerPage = 5

const filteredSavedWo = computed(() => {
  const q = woSearch.value.trim().toLowerCase()
  if (!q) return savedWorkOrders.value
  return savedWorkOrders.value.filter((wo: any) =>
    [wo.wo_number, wo.asset_code, wo.equipment_type, wo.component, wo.technician, wo.wo_status, wo.priority, wo.status_unit]
      .filter(Boolean)
      .some((v: string) => String(v).toLowerCase().includes(q)),
  )
})
const woTotalPages = computed(() => Math.max(1, Math.ceil(filteredSavedWo.value.length / woPerPage)))
const pagedSavedWo = computed(() => {
  const start = (woPage.value - 1) * woPerPage
  return filteredSavedWo.value.slice(start, start + woPerPage)
})
watch([woSearch, () => savedWorkOrders.value.length], () => {
  if (woPage.value > woTotalPages.value) woPage.value = woTotalPages.value
})

// Detail WO tersimpan (lengkap + data unit terkait)
const { createMap } = useFleetMap()
let woMap: any = null
const woDetailOpen = ref(false)
const woDetail = ref<any>(null)
const woDetailLoading = ref(false)

const renderWoMap = async () => {
  const u = woDetail.value?.unit
  if (!u || u.lat == null || u.lng == null) return
  await nextTick()
  if (woMap) { try { woMap.remove() } catch {} woMap = null }
  woMap = await createMap('wo-detail-map', [{
    unit: u.code,
    unit_type: u.jenis_alat_berat_nama || 'Heavy Equipment',
    status: u.status,
    color_hex: statusColor(u.status),
    level: (u.status || '•').charAt(0),
    lat: u.lat,
    lng: u.lng,
    health: u.health,
  }], { zoom: 14, dark: isDark.value })
}

const openWoDetail = async (id: string) => {
  woDetailOpen.value = true
  woDetailLoading.value = true
  woDetail.value = null
  if (woMap) { try { woMap.remove() } catch {} woMap = null }
  try {
    const res: any = await api.getWorkOrder(id)
    woDetail.value = res?.data || null
  } catch (e: any) {
    showToast(false, e?.data?.message || 'Gagal memuat detail Work Order.')
    woDetailOpen.value = false
  } finally {
    woDetailLoading.value = false
  }
  await nextTick()
  renderWoMap()
}
const closeWoDetail = () => {
  woDetailOpen.value = false
  if (woMap) { try { woMap.remove() } catch {} woMap = null }
}

// --- Export CSV & Excel ---
const EXPORT_COLUMNS: { key: string; label: string }[] = [
  { key: 'wo_number', label: 'WO Number' },
  { key: 'asset_code', label: 'Unit' },
  { key: 'equipment_type', label: 'Equipment' },
  { key: 'status_unit', label: 'Status Unit' },
  { key: 'priority', label: 'Prioritas' },
  { key: 'component', label: 'Komponen' },
  { key: 'part_no', label: 'Part No' },
  { key: 'rul_hours', label: 'RUL (jam)' },
  { key: 'est_cost', label: 'Est. Biaya (Rp)' },
  { key: 'technician', label: 'Teknisi' },
  { key: 'scheduled_at', label: 'Mulai Perbaikan' },
  { key: 'est_completion_at', label: 'Est. Selesai' },
  { key: 'wo_status', label: 'Status WO' },
  { key: 'notes', label: 'Catatan' },
  { key: 'created_at', label: 'Dibuat' },
]
const cellValue = (wo: any, key: string) => {
  const v = wo[key]
  if (v == null) return ''
  if (key.endsWith('_at')) return fmtDateLong(v)
  return String(v)
}
const downloadBlob = (content: string, mime: string, filename: string) => {
  const blob = new Blob([content], { type: mime })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = filename
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
}
const exportCsv = () => {
  const rows = filteredSavedWo.value
  const esc = (s: string) => `"${String(s).replace(/"/g, '""')}"`
  const header = EXPORT_COLUMNS.map(c => esc(c.label)).join(',')
  const body = rows.map((wo: any) => EXPORT_COLUMNS.map(c => esc(cellValue(wo, c.key))).join(',')).join('\n')
  const stamp = new Date().toISOString().slice(0, 10)
  downloadBlob('\uFEFF' + header + '\n' + body, 'text/csv;charset=utf-8;', `work-orders-${stamp}.csv`)
}
const exportExcel = () => {
  const rows = filteredSavedWo.value
  const esc = (s: string) => String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')
  const head = '<tr>' + EXPORT_COLUMNS.map(c => `<th style="background:#1d242e;color:#fff;padding:6px;border:1px solid #ccc">${esc(c.label)}</th>`).join('') + '</tr>'
  const body = rows.map((wo: any) =>
    '<tr>' + EXPORT_COLUMNS.map(c => `<td style="padding:6px;border:1px solid #ccc">${esc(cellValue(wo, c.key))}</td>`).join('') + '</tr>',
  ).join('')
  const html = `<html xmlns:o="urn:schemas-microsoft-com:office:office" xmlns:x="urn:schemas-microsoft-com:office:excel" xmlns="http://www.w3.org/TR/REC-html40"><head><meta charset="UTF-8"></head><body><table>${head}${body}</table></body></html>`
  const stamp = new Date().toISOString().slice(0, 10)
  downloadBlob(html, 'application/vnd.ms-excel', `work-orders-${stamp}.xls`)
}

const ATRISK = ['CRITICAL', 'WARNING', 'RUSAK']

// --- HELPERS ---
const statusColor = (s: string) => ({ CRITICAL: '#E0413E', WARNING: '#E0A106', RUSAK: '#7A848E', SEHAT: '#1FA971' }[s] || '#7A848E')
const priorityColor = (p: string) => ({ HIGH: '#E0413E', MEDIUM: '#E0A106', LOW: '#3E92CC' }[p] || '#7A848E')

const priorityOf = (status: string): string => {
  if (status === 'RUSAK' || status === 'CRITICAL') return 'HIGH'
  if (status === 'WARNING') return 'MEDIUM'
  return 'LOW'
}

// hash deterministik dari kode unit → variasi biaya yang stabil (bukan acak tiap refresh)
const hashCode = (s: string) => {
  let h = 0
  for (let i = 0; i < s.length; i++) h = (h * 31 + s.charCodeAt(i)) >>> 0
  return h
}

const estCostOf = (code: string, status: string): number => {
  const base = status === 'RUSAK' ? 280_000_000 : status === 'CRITICAL' ? 135_000_000 : 48_000_000
  const factor = 1 + ((hashCode(code) % 40) - 10) / 100 // -10%..+30%
  return Math.round((base * factor) / 1_000_000) * 1_000_000
}

const lstmComponentsOf = (a: any) => {
  const p = a?.prediction
  if (!p) return [] as { label: string; hours: number }[]
  return [
    { label: 'Sistem Hidrolik', hours: p.lstm_hydraulic_system },
    { label: 'Pompa Hidrolik', hours: p.lstm_hydraulic_pump },
    { label: 'Seal Pompa', hours: p.lstm_pump_seal },
    { label: 'Sistem Rem', hours: p.lstm_brake_system },
    { label: 'Brake Caliper', hours: p.lstm_brake_caliper },
    { label: 'Brake Pad (Rear)', hours: p.lstm_brake_pad },
    { label: 'Sistem Kemudi', hours: p.lstm_steering_system },
  ].filter(c => typeof c.hours === 'number').sort((x, y) => x.hours - y.hours)
}

const buildItem = (u: any, a: any): WoItem => {
  const now = Date.now()
  const rul = a?.prediction?.lstm_rul_hours ?? Math.max(2, Math.round((u.health || 50) * 1.5))
  const unc = a?.prediction?.rul_uncertainty ?? Math.round(rul * 0.15)
  // RUSAK = sudah breakdown → jadwal perbaikan segera (+8 jam)
  const failureMs = now + rul * 3600 * 1000
  const repairByMs = u.status === 'RUSAK'
    ? now + 8 * 3600 * 1000
    : now + Math.max(8, rul * 0.6) * 3600 * 1000
  // Estimasi durasi perbaikan per tingkat keparahan
  const repairDurationHours = u.status === 'RUSAK' ? 24 : u.status === 'CRITICAL' ? 12 : 6
  const estCompletionMs = repairByMs + repairDurationHours * 3600 * 1000
  const comps = lstmComponentsOf(a)
  const componentHealth: { component: string; health: number }[] =
    (a?.component_health && a.component_health.length)
      ? a.component_health
      : comps.slice(0, 6).map(c => ({ component: c.label, health: Math.max(5, Math.min(100, Math.round(c.hours / 5))) }))
  const shap = [...(a?.shap_contributions || [])]
    .sort((x: any, y: any) => Math.abs(y.value) - Math.abs(x.value))
    .slice(0, 5)
  const twinObj = a?.prediction?.digital_twin
  const twins = twinObj
    ? [
        { label: 'Brake Twin', hours: twinObj.brake_twin_rul },
        { label: 'Bearing Twin', hours: twinObj.bearing_twin_rul },
        { label: 'Hydraulic Twin', hours: twinObj.hydraulic_twin_rul },
      ]
    : []
  return {
    id: u.id,
    code: u.code,
    type: u.jenis_alat_berat_nama || 'Heavy Equipment',
    status: u.status,
    health: u.health ?? 0,
    riskLevel: u.risk_level || a?.risk_level || '-',
    riskScore: Math.round(u.risk_score ?? a?.risk_score ?? 0),
    rulHours: Math.round(rul),
    rulUncertainty: Math.round(unc),
    failureDate: new Date(failureMs),
    repairByDate: new Date(repairByMs),
    repairDurationHours,
    estCompletionDate: new Date(estCompletionMs),
    priority: priorityOf(u.status),
    topComponent: comps[0]?.label || 'Komponen Utama',
    components: componentHealth,
    shap,
    twins,
    estCost: estCostOf(u.code, u.status),
    woId: 'WO-' + String(hashCode(u.code) % 100000).padStart(5, '0'),
  }
}

// --- FETCH ---
const fetchData = async () => {
  try {
    const res = await api.getAnalisaOverview() as any
    const atRisk = (res.data.units || []).filter((u: any) => ATRISK.includes(u.status))
    const analyses = await Promise.all(
      atRisk.map((u: any) => api.getUnitAnalysis(u.id).then((r: any) => r.data).catch(() => null)),
    )
    items.value = atRisk.map((u: any, i: number) => buildItem(u, analyses[i]))
      .sort((a: WoItem, b: WoItem) => a.rulHours - b.rulHours)

    if (items.value.length) {
      const fromQuery = route.query.asset ? String(route.query.asset) : null
      if (!selectedCode.value || !items.value.find(it => it.code === selectedCode.value)) {
        const match = fromQuery && items.value.find(it => it.code.toLowerCase() === fromQuery.toLowerCase())
        selectedCode.value = match?.code || items.value[0].code
      }
    } else {
      selectedCode.value = null
    }
    errorMsg.value = ''
    lastUpdate.value = new Date().toLocaleTimeString('id-ID')
  } catch (e: any) {
    errorMsg.value = e?.data?.message || 'Gagal memuat data work order.'
  }
}

// --- COMPUTED ---
const filteredItems = computed(() =>
  statusFilter.value === 'ALL' ? items.value : items.value.filter(it => it.status === statusFilter.value))

// Pagination daftar work order (unit berisiko) — tampilkan 5 per halaman
const itemsPage = ref(1)
const itemsPerPage = 5
const itemsTotalPages = computed(() => Math.max(1, Math.ceil(filteredItems.value.length / itemsPerPage)))
const pagedItems = computed(() => {
  const start = (itemsPage.value - 1) * itemsPerPage
  return filteredItems.value.slice(start, start + itemsPerPage)
})
watch([statusFilter, () => filteredItems.value.length], () => {
  if (itemsPage.value > itemsTotalPages.value) itemsPage.value = itemsTotalPages.value
})

const selected = computed(() => items.value.find(it => it.code === selectedCode.value) || null)

const kpiCritical = computed(() => items.value.filter(i => i.status === 'CRITICAL').length)
const kpiWarning = computed(() => items.value.filter(i => i.status === 'WARNING').length)
const kpiRusak = computed(() => items.value.filter(i => i.status === 'RUSAK').length)
const kpiTotalCost = computed(() => items.value.reduce((s, i) => s + i.estCost, 0))
const kpiAvgRul = computed(() => {
  if (!items.value.length) return 0
  return Math.round(items.value.reduce((s, i) => s + i.rulHours, 0) / items.value.length)
})

// --- FORMATTERS ---
const fmtRupiah = (n: number) =>
  new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR', maximumFractionDigits: 0 }).format(n)
const fmtRupiahShort = (n: number) => {
  if (n >= 1_000_000_000) return `Rp ${(n / 1_000_000_000).toFixed(1)} M`
  if (n >= 1_000_000) return `Rp ${Math.round(n / 1_000_000)} jt`
  return fmtRupiah(n)
}
const fmtHours = (h: number) => {
  if (h >= 24) return `${Math.floor(h / 24)} hari ${Math.round(h % 24)} jam`
  return `${Math.round(h)} jam`
}
const fmtDate = (d: Date) =>
  d.toLocaleString('id-ID', { day: '2-digit', month: 'short', hour: '2-digit', minute: '2-digit' })
const fmtDateLong = (d: Date | string) =>
  new Date(d).toLocaleString('id-ID', { weekday: 'short', day: '2-digit', month: 'long', hour: '2-digit', minute: '2-digit' })
const woStatusColor = (s: string) => ({ OPEN: '#3E92CC', IN_PROGRESS: '#E0A106', COMPLETED: '#1FA971', CANCELLED: '#7A848E' }[s] || '#7A848E')

const selectUnit = (code: string) => { selectedCode.value = code; nextTick(() => renderDetailCharts()) }

// --- CHART RENDERING ---
const css = (v: string) => (typeof window !== 'undefined' ? getComputedStyle(document.documentElement).getPropertyValue(v).trim() : '')
const theme = () => ({
  tick: css('--text-muted') || '#5d6b7a',
  grid: css('--border') || '#d7dde4',
  axis: css('--border-strong') || '#c2cad3',
  text: css('--text') || '#1b2128',
})

const upsertChart = (key: string, canvasId: string, config: any) => {
  const el = document.getElementById(canvasId) as HTMLCanvasElement | null
  if (!el || !ChartLib) return
  if (charts[key]) {
    charts[key].data = config.data
    if (config.options) charts[key].options = config.options
    charts[key].update('none')
  } else {
    charts[key] = new ChartLib(el, config)
  }
}

// Plugin: tampilkan angka nilai pada tiap arc doughnut secara permanen (tanpa hover).
// Tooltip (hover) tetap aktif untuk detail.
const arcValueLabels = {
  id: 'arcValueLabels',
  afterDatasetsDraw(chart: any) {
    const { ctx } = chart
    const meta = chart.getDatasetMeta(0)
    const ds = chart.data.datasets[0]?.data || []
    meta.data.forEach((arc: any, i: number) => {
      const val = ds[i]
      if (!val) return
      const pos = arc.tooltipPosition()
      ctx.save()
      ctx.fillStyle = '#ffffff'
      ctx.font = 'bold 14px Inter, sans-serif'
      ctx.textAlign = 'center'
      ctx.textBaseline = 'middle'
      ctx.shadowColor = 'rgba(0,0,0,0.35)'
      ctx.shadowBlur = 3
      ctx.fillText(String(val), pos.x, pos.y)
      ctx.restore()
    })
  },
}

const renderFleetCharts = () => {
  const t = theme()
  const list = items.value

  // 1) Doughnut: distribusi status unit berisiko
  upsertChart('woStatus', 'woStatus', {
    type: 'doughnut',
    data: {
      labels: ['Critical', 'Warning', 'Rusak'],
      datasets: [{
        data: [kpiCritical.value, kpiWarning.value, kpiRusak.value],
        backgroundColor: ['#E0413E', '#E0A106', '#7A848E'],
        borderWidth: 0,
      }],
    },
    plugins: [arcValueLabels],
    options: {
      responsive: true, maintainAspectRatio: false, cutout: '62%',
      plugins: { legend: { position: 'bottom', labels: { color: t.text, font: { size: 11 } } } },
    },
  })

  // 2) Doughnut: distribusi prioritas
  const prio = ['HIGH', 'MEDIUM', 'LOW'].map(p => list.filter(i => i.priority === p).length)
  upsertChart('woPriority', 'woPriority', {
    type: 'doughnut',
    data: {
      labels: ['High', 'Medium', 'Low'],
      datasets: [{ data: prio, backgroundColor: ['#E0413E', '#E0A106', '#3E92CC'], borderWidth: 0 }],
    },
    plugins: [arcValueLabels],
    options: {
      responsive: true, maintainAspectRatio: false, cutout: '62%',
      plugins: { legend: { position: 'bottom', labels: { color: t.text, font: { size: 11 } } } },
    },
  })

  // 3) Bar horizontal: RUL (jam) per unit — paling mendesak di atas
  const byRul = [...list].sort((a, b) => a.rulHours - b.rulHours).slice(0, 12)
  upsertChart('woRul', 'woRul', {
    type: 'bar',
    data: {
      labels: byRul.map(i => i.code),
      datasets: [{
        label: 'RUL (jam)', data: byRul.map(i => i.rulHours),
        backgroundColor: byRul.map(i => statusColor(i.status)), borderRadius: 5,
      }],
    },
    options: {
      indexAxis: 'y', responsive: true, maintainAspectRatio: false,
      plugins: { legend: { display: false }, tooltip: { callbacks: { label: (c: any) => `${c.raw} jam tersisa` } } },
      scales: {
        x: { beginAtZero: true, ticks: { color: t.tick, font: { family: 'JetBrains Mono', size: 9 } }, grid: { color: t.grid } },
        y: { ticks: { color: t.text, font: { weight: 'bold', size: 10 } }, grid: { display: false } },
      },
    },
  })

  // 4) Bar: estimasi biaya per unit (juta Rupiah)
  const byCost = [...list].sort((a, b) => b.estCost - a.estCost).slice(0, 12)
  upsertChart('woCost', 'woCost', {
    type: 'bar',
    data: {
      labels: byCost.map(i => i.code),
      datasets: [{
        label: 'Estimasi Biaya (juta Rp)', data: byCost.map(i => Math.round(i.estCost / 1_000_000)),
        backgroundColor: '#3E92CC', borderRadius: 5,
      }],
    },
    options: {
      responsive: true, maintainAspectRatio: false,
      plugins: { legend: { display: false }, tooltip: { callbacks: { label: (c: any) => `Rp ${c.raw} juta` } } },
      scales: {
        x: { ticks: { color: t.text, font: { family: 'JetBrains Mono', size: 9 } }, grid: { display: false } },
        y: { beginAtZero: true, ticks: { color: t.tick, font: { size: 9 } }, grid: { color: t.grid } },
      },
    },
  })

  // 5) Line: proyeksi kumulatif unit jatuh tempo (breakdown) 14 hari ke depan
  const days = 14
  const now = Date.now()
  const labels: string[] = []
  const cumulative: number[] = []
  for (let d = 1; d <= days; d++) {
    const until = now + d * 24 * 3600 * 1000
    labels.push(`H+${d}`)
    cumulative.push(list.filter(i => i.failureDate.getTime() <= until).length)
  }
  upsertChart('woTimeline', 'woTimeline', {
    type: 'line',
    data: {
      labels,
      datasets: [{
        label: 'Akumulasi unit jatuh tempo perbaikan',
        data: cumulative, borderColor: '#E0413E', backgroundColor: 'rgba(224,65,62,0.12)',
        fill: true, tension: 0.35, pointRadius: 3, pointBackgroundColor: '#E0413E',
      }],
    },
    options: {
      responsive: true, maintainAspectRatio: false,
      plugins: { legend: { labels: { color: t.text, font: { size: 11 } } } },
      scales: {
        x: { ticks: { color: t.tick, font: { family: 'JetBrains Mono', size: 9 } }, grid: { color: t.grid } },
        y: { beginAtZero: true, ticks: { color: t.tick, stepSize: 1 }, grid: { color: t.grid } },
      },
    },
  })
}

const renderDetailCharts = () => {
  const t = theme()
  const s = selected.value
  if (!s) return

  // 6) Radar: kesehatan komponen unit terpilih
  upsertChart('woRadar', 'woRadar', {
    type: 'radar',
    data: {
      labels: s.components.map(c => c.component),
      datasets: [{
        label: `Health ${s.code}`, data: s.components.map(c => c.health),
        backgroundColor: 'rgba(62,146,204,0.18)', borderColor: '#3E92CC',
        pointBackgroundColor: '#3E92CC',
      }],
    },
    options: {
      responsive: true, maintainAspectRatio: false,
      plugins: { legend: { display: false } },
      scales: {
        r: {
          suggestedMin: 0, suggestedMax: 100,
          angleLines: { color: t.grid }, grid: { color: t.grid },
          pointLabels: { color: t.text, font: { size: 10 } },
          ticks: { display: false },
        },
      },
    },
  })

  // 7) Bar SHAP: kontributor utama (merah = menaikkan risiko, hijau = menurunkan)
  const shap = s.shap
  upsertChart('woShap', 'woShap', {
    type: 'bar',
    data: {
      labels: shap.map(x => x.feature),
      datasets: [{
        label: 'Kontribusi SHAP (%)', data: shap.map(x => Math.round(x.value)),
        backgroundColor: shap.map(x => (x.value >= 0 ? '#E0413E' : '#1FA971')), borderRadius: 5,
      }],
    },
    options: {
      indexAxis: 'y', responsive: true, maintainAspectRatio: false,
      plugins: { legend: { display: false } },
      scales: {
        x: { ticks: { color: t.tick, font: { size: 9 } }, grid: { color: t.grid } },
        y: { ticks: { color: t.text, font: { size: 10 } }, grid: { display: false } },
      },
    },
  })
}

const renderAll = () => { renderFleetCharts(); renderDetailCharts() }

const refreshAll = async () => {
  await fetchData()
  await nextTick()
  renderAll()
}

// --- LIFECYCLE ---
onMounted(async () => {
  initTheme()
  initAuth()
  // Inisialisasi pratyaksa polling
  const pratyaksa = usePratyaksa()
  pratyaksa.fetchAll()
  pratyaksa.startPolling(10000)
  isLoading.value = true
  if (typeof window !== 'undefined') {
    const mod = await import('chart.js/auto')
    ChartLib = mod.default
  }
  await fetchData()
  isLoading.value = false
  await nextTick()
  renderAll()
  fetchWorkOrders()

  // Jika dibuka dari link Telegram (?asset=KODE), langsung buka modal Buat Work Order
  // untuk unit tersebut alih-alih hanya menampilkan halaman daftar.
  const assetQuery = route.query.asset ? String(route.query.asset) : null
  if (assetQuery) {
    const target = items.value.find(it => it.code.toLowerCase() === assetQuery.toLowerCase())
    if (target) {
      selectedCode.value = target.code
      await nextTick()
      renderDetailCharts()
      openModal(target)
    }
  }

  refreshTimer = setInterval(() => {
    if (autoRefresh.value) refreshAll()
  }, 15000)

  // Ticker 1 detik untuk animasi countdown estimasi perbaikan
  tickTimer = setInterval(() => { nowTick.value = Date.now() }, 1000)
})

onUnmounted(() => {
  if (refreshTimer) clearInterval(refreshTimer)
  if (tickTimer) clearInterval(tickTimer)
  Object.values(charts).forEach(c => c?.destroy())
})

watch(isDark, () => { nextTick(() => renderAll()) })
</script>


<template>
  <div class="flex h-screen overflow-hidden bg-mesh text-[color:var(--text)]">
    <!-- Sidebar -->
    <PanelSidebar />

    <!-- Main -->
    <main class="flex-1 min-w-0 w-full p-4 sm:p-6 lg:p-8 pt-16 lg:pt-8 overflow-y-auto">
      <header class="flex justify-between items-start mb-8 flex-wrap gap-4">
        <div>
          <h1 class="font-display text-4xl md:text-5xl font-bold uppercase tracking-wide leading-none">Work Order</h1>
          <p class="mt-2 text-[color:var(--text-muted)]">Estimasi perbaikan unit CRITICAL, WARNING & RUSAK — dipicu dari alert Telegram.</p>
        </div>
        <div class="flex items-center gap-3">
          <!-- PRATYAKSA Mode Selector -->
          <ModeSelector :showSubModes="false" />
          <div class="panel-flat px-3 py-2 text-[10px] font-mono text-[color:var(--text-muted)]">
            Update<br><span class="font-semibold text-[color:var(--text)]">{{ lastUpdate || '—' }}</span>
          </div>
        </div>
      </header>

      <!-- Mode Lock Table -->
      <ModeLockTabel />

      <div v-if="errorMsg" class="mb-6 px-4 py-3 rounded-xl bg-critical/10 border border-critical/40 text-critical font-semibold">⚠️ {{ errorMsg }}</div>
      <div v-if="woToast" class="mb-4 px-4 py-2.5 rounded-xl text-sm font-semibold"
        :class="woToast.ok ? 'bg-healthy/10 border border-healthy/40 text-healthy' : 'bg-critical/10 border border-critical/40 text-critical'">
        {{ woToast.msg }}
      </div>
      <div v-if="isLoading" class="flex items-center justify-center h-96 font-semibold text-[color:var(--text-faint)] uppercase tracking-widest">Memuat work order…</div>

      <template v-else>
        <!-- KPI -->
        <div class="grid grid-cols-2 lg:grid-cols-5 gap-4 mb-7">
          <div class="panel-flat p-5">
            <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-1">Critical</p>
            <p class="font-display text-4xl font-bold text-critical">{{ kpiCritical }}</p>
          </div>
          <div class="panel-flat p-5">
            <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-1">Warning</p>
            <p class="font-display text-4xl font-bold text-warning">{{ kpiWarning }}</p>
          </div>
          <div class="panel-flat p-5">
            <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-1">Rusak</p>
            <p class="font-display text-4xl font-bold text-rusak">{{ kpiRusak }}</p>
          </div>
          <div class="panel-flat p-5">
            <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-1">Rata-rata RUL</p>
            <p class="font-display text-4xl font-bold text-steel">{{ kpiAvgRul }}<span class="text-base font-semibold"> jam</span></p>
          </div>
          <div class="panel-flat p-5">
            <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-1">Est. Biaya Total</p>
            <p class="font-display text-3xl font-bold text-amber">{{ fmtRupiahShort(kpiTotalCost) }}</p>
          </div>
        </div>

        <div v-if="!items.length" class="panel-flat p-10 text-center text-[color:var(--text-muted)]">
          🎉 Tidak ada unit berstatus CRITICAL / WARNING / RUSAK saat ini. Semua armada dalam kondisi sehat.
        </div>

        <template v-else>
          <!-- Grid Chart Fleet -->
          <div class="grid grid-cols-1 lg:grid-cols-3 gap-5 mb-6">
            <div class="panel-flat p-5">
              <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-3">Distribusi Status</h3>
              <div class="h-56"><canvas id="woStatus"></canvas></div>
            </div>
            <div class="panel-flat p-5">
              <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-3">Distribusi Prioritas</h3>
              <div class="h-56"><canvas id="woPriority"></canvas></div>
            </div>
            <div class="panel-flat p-5">
              <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-3">RUL per Unit (jam)</h3>
              <div class="h-56"><canvas id="woRul"></canvas></div>
            </div>
          </div>

          <div class="grid grid-cols-1 lg:grid-cols-2 gap-5 mb-6">
            <div class="panel-flat p-5">
              <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-3">Estimasi Biaya per Unit</h3>
              <div class="h-64"><canvas id="woCost"></canvas></div>
            </div>
            <div class="panel-flat p-5">
              <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-3">Proyeksi Jatuh Tempo Perbaikan (14 hari)</h3>
              <div class="h-64"><canvas id="woTimeline"></canvas></div>
            </div>
          </div>

          <!-- Tabel Work Order -->
          <div class="panel-flat p-5 mb-6">
            <div class="flex items-center justify-between mb-4 flex-wrap gap-3">
              <h3 class="font-display text-xl font-bold uppercase tracking-wide">Daftar Work Order</h3>
              <div class="flex gap-1.5">
                <button v-for="f in (['ALL','CRITICAL','WARNING','RUSAK'] as const)" :key="f"
                  @click="statusFilter = f"
                  :class="statusFilter === f ? 'bg-amber text-graphite-900 border-amber' : 'btn-ghost'"
                  class="btn !py-1.5 !px-3 text-[11px]">{{ f === 'ALL' ? 'Semua' : f }}</button>
              </div>
            </div>
            <div class="overflow-x-auto">
              <table class="w-full text-sm min-w-[720px]">
                <thead>
                  <tr class="text-left text-[10px] uppercase tracking-wider text-[color:var(--text-muted)] border-b border-[color:var(--border)]">
                    <th class="py-2.5 pr-3">WO ID</th>
                    <th class="py-2.5 pr-3">Unit</th>
                    <th class="py-2.5 pr-3">Status</th>
                    <th class="py-2.5 pr-3">Prioritas</th>
                    <th class="py-2.5 pr-3">RUL</th>
                    <th class="py-2.5 pr-3">Komponen Kritis</th>
                    <th class="py-2.5 pr-3">Perbaikan Sebelum</th>
                    <th class="py-2.5 pr-3">Est. Biaya</th>
                    <th class="py-2.5"></th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="it in pagedItems" :key="it.id"
                    @click="selectUnit(it.code)"
                    :class="['border-b border-[color:var(--border)] cursor-pointer hover:bg-[color:var(--surface-2)] transition-colors', selectedCode === it.code ? 'bg-[color:var(--surface-2)]' : '']">
                    <td class="py-2.5 pr-3 font-mono text-[11px]">{{ it.woId }}</td>
                    <td class="py-2.5 pr-3 font-semibold">{{ it.code }}<span class="block text-[10px] text-[color:var(--text-muted)] font-normal">{{ it.type }}</span></td>
                    <td class="py-2.5 pr-3">
                      <span class="px-2 py-0.5 rounded-full text-[10px] font-bold text-white" :style="{ background: statusColor(it.status) }">{{ it.status }}</span>
                    </td>
                    <td class="py-2.5 pr-3">
                      <span class="px-2 py-0.5 rounded-full text-[10px] font-bold text-white" :style="{ background: priorityColor(it.priority) }">{{ it.priority }}</span>
                    </td>
                    <td class="py-2.5 pr-3 font-mono">{{ fmtHours(it.rulHours) }}</td>
                    <td class="py-2.5 pr-3">{{ it.topComponent }}</td>
                    <td class="py-2.5 pr-3 font-mono text-[11px]">{{ fmtDate(it.repairByDate) }}</td>
                    <td class="py-2.5 pr-3 font-semibold text-amber">{{ fmtRupiahShort(it.estCost) }}</td>
                    <td class="py-2.5 text-right">
                      <button @click.stop="openModal(it)" class="btn btn-ghost !py-1.5 !px-3 text-[11px] text-steel">Detail</button>
                    </td>
                  </tr>
                  <tr v-if="!filteredItems.length">
                    <td colspan="9" class="py-6 text-center text-[color:var(--text-muted)]">Tidak ada unit berisiko untuk filter ini.</td>
                  </tr>
                </tbody>
              </table>
            </div>
            <!-- Pagination -->
            <div v-if="itemsTotalPages > 1" class="flex items-center justify-between mt-4">
              <p class="text-xs text-[color:var(--text-muted)]">
                Menampilkan {{ pagedItems.length }} dari {{ filteredItems.length }} unit
              </p>
              <div class="flex items-center gap-1.5">
                <button @click="itemsPage = Math.max(1, itemsPage - 1)" :disabled="itemsPage === 1"
                  class="btn btn-ghost !py-1.5 !px-3 text-[11px] disabled:opacity-40">‹ Prev</button>
                <span class="text-xs font-mono px-2">{{ itemsPage }} / {{ itemsTotalPages }}</span>
                <button @click="itemsPage = Math.min(itemsTotalPages, itemsPage + 1)" :disabled="itemsPage === itemsTotalPages"
                  class="btn btn-ghost !py-1.5 !px-3 text-[11px] disabled:opacity-40">Next ›</button>
              </div>
            </div>
          </div>

          <!-- Detail Unit Terpilih -->
          <div v-if="selected" class="panel-raised p-6 mb-4">
            <div class="flex items-start justify-between flex-wrap gap-4 mb-5">
              <div>
                <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)]">{{ selected.woId }} · Detail Work Order</p>
                <h2 class="font-display text-3xl font-bold tracking-wide">{{ selected.code }}</h2>
                <p class="text-[color:var(--text-muted)] text-sm">{{ selected.type }}</p>
              </div>
              <div class="flex items-center gap-2">
                <span class="px-3 py-1 rounded-full text-xs font-bold text-white" :style="{ background: statusColor(selected.status) }">{{ selected.status }}</span>
                <span class="px-3 py-1 rounded-full text-xs font-bold text-white" :style="{ background: priorityColor(selected.priority) }">PRIORITAS {{ selected.priority }}</span>
              </div>
            </div>

            <!-- Ringkasan estimasi -->
            <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
              <div class="panel-flat p-4">
                <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-1">Sisa Umur (RUL)</p>
                <p class="font-display text-2xl font-bold text-critical">{{ fmtHours(selected.rulHours) }}</p>
                <p class="text-[10px] text-[color:var(--text-muted)]">± {{ selected.rulUncertainty }} jam</p>
              </div>
              <div class="panel-flat p-4">
                <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-1">Estimasi Breakdown</p>
                <p class="font-display text-xl font-bold">{{ fmtDate(selected.failureDate) }}</p>
              </div>
              <div class="panel-flat p-4">
                <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-1">Perbaikan Sebelum</p>
                <p class="font-display text-xl font-bold text-warning">{{ fmtDate(selected.repairByDate) }}</p>
              </div>
              <div class="panel-flat p-4">
                <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-1">Estimasi Biaya</p>
                <p class="font-display text-2xl font-bold text-amber">{{ fmtRupiahShort(selected.estCost) }}</p>
              </div>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-5 mb-6">
              <div class="panel-flat p-5">
                <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-3">Kesehatan Komponen</h3>
                <div class="h-64"><canvas id="woRadar"></canvas></div>
              </div>
              <div class="panel-flat p-5">
                <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-3">Kontributor Risiko (SHAP)</h3>
                <div class="h-64"><canvas id="woShap"></canvas></div>
              </div>
            </div>

            <!-- Digital Twin -->
            <div v-if="selected.twins.length" class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div v-for="tw in selected.twins" :key="tw.label" class="panel-flat p-4 text-center">
                <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-1">{{ tw.label }}</p>
                <p class="font-display text-2xl font-bold text-steel">{{ fmtHours(tw.hours) }}</p>
              </div>
            </div>
          </div>

          <!-- Work Order Tersimpan (dari database) -->
          <div v-if="savedWorkOrders.length" class="panel-flat p-5 mb-6">
            <div class="flex items-center justify-between mb-4 flex-wrap gap-3">
              <h3 class="font-display text-xl font-bold uppercase tracking-wide">Work Order Tersimpan</h3>
              <div class="flex items-center gap-2 flex-wrap">
                <div class="relative">
                  <svg class="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-[color:var(--text-faint)]" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
                  <input v-model="woSearch" type="text" placeholder="Cari WO / unit / komponen…"
                    class="pl-9 pr-3 py-2 rounded-xl bg-[color:var(--surface-2)] border border-[color:var(--border)] text-sm focus:outline-none focus:border-amber w-60" />
                </div>
                <button @click="exportCsv" class="btn btn-ghost !py-2 !px-3 text-[11px] font-semibold">
                  <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
                  CSV
                </button>
                <button @click="exportExcel" class="btn btn-ghost !py-2 !px-3 text-[11px] font-semibold text-healthy">
                  <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
                  Excel
                </button>
              </div>
            </div>
            <div class="overflow-x-auto">
              <table class="w-full text-sm min-w-[760px]">
                <thead>
                  <tr class="text-left text-[10px] uppercase tracking-wider text-[color:var(--text-muted)] border-b border-[color:var(--border)]">
                    <th class="py-2.5 pr-3">WO Number</th>
                    <th class="py-2.5 pr-3">Unit</th>
                    <th class="py-2.5 pr-3">Komponen</th>
                    <th class="py-2.5 pr-3">Prioritas</th>
                    <th class="py-2.5 pr-3">Teknisi</th>
                    <th class="py-2.5 pr-3">Est. Selesai</th>
                    <th class="py-2.5 pr-3">Status</th>
                    <th class="py-2.5"></th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="wo in pagedSavedWo" :key="wo.id" class="border-b border-[color:var(--border)] hover:bg-[color:var(--surface-2)] transition-colors">
                    <td class="py-2.5 pr-3 font-mono text-[11px]">{{ wo.wo_number }}</td>
                    <td class="py-2.5 pr-3 font-semibold">{{ wo.asset_code }}</td>
                    <td class="py-2.5 pr-3">{{ wo.component }}</td>
                    <td class="py-2.5 pr-3">
                      <span class="px-2 py-0.5 rounded-full text-[10px] font-bold text-white" :style="{ background: priorityColor(wo.priority) }">{{ wo.priority }}</span>
                    </td>
                    <td class="py-2.5 pr-3">{{ wo.technician || '—' }}</td>
                    <td class="py-2.5 pr-3 font-mono text-[11px]">{{ wo.est_completion_at ? fmtDateLong(wo.est_completion_at) : '—' }}</td>
                    <td class="py-2.5 pr-3">
                      <span class="px-2 py-0.5 rounded-full text-[10px] font-bold text-white" :style="{ background: woStatusColor(wo.wo_status) }">{{ wo.wo_status }}</span>
                    </td>
                    <td class="py-2.5 text-right whitespace-nowrap">
                      <button @click="openWoDetail(wo.id)" class="btn btn-ghost !py-1 !px-2 text-[10px] text-steel">Detail</button>
                      <button v-if="wo.wo_status === 'OPEN'" @click="updateWoStatus(wo.id, 'IN_PROGRESS')" class="btn btn-ghost !py-1 !px-2 text-[10px] text-warning">Mulai</button>
                      <button v-if="wo.wo_status === 'IN_PROGRESS'" @click="updateWoStatus(wo.id, 'COMPLETED')" class="btn btn-ghost !py-1 !px-2 text-[10px] text-healthy">Selesai</button>
                    </td>
                  </tr>
                  <tr v-if="!filteredSavedWo.length">
                    <td colspan="8" class="py-6 text-center text-[color:var(--text-muted)]">Tidak ada WO yang cocok dengan pencarian.</td>
                  </tr>
                </tbody>
              </table>
            </div>
            <!-- Pagination -->
            <div v-if="woTotalPages > 1" class="flex items-center justify-between mt-4">
              <p class="text-xs text-[color:var(--text-muted)]">
                Menampilkan {{ pagedSavedWo.length }} dari {{ filteredSavedWo.length }} WO
              </p>
              <div class="flex items-center gap-1.5">
                <button @click="woPage = Math.max(1, woPage - 1)" :disabled="woPage === 1"
                  class="btn btn-ghost !py-1.5 !px-3 text-[11px] disabled:opacity-40">‹ Prev</button>
                <span class="text-xs font-mono px-2">{{ woPage }} / {{ woTotalPages }}</span>
                <button @click="woPage = Math.min(woTotalPages, woPage + 1)" :disabled="woPage === woTotalPages"
                  class="btn btn-ghost !py-1.5 !px-3 text-[11px] disabled:opacity-40">Next ›</button>
              </div>
            </div>
          </div>
        </template>
      </template>

      <!-- ===== MODAL DETAIL WORK ORDER ===== -->
      <Teleport to="body">
        <Transition name="wo-fade">
          <div v-if="modalOpen && modalItem" class="wo-modal-backdrop" @click.self="closeModal">
            <Transition name="wo-pop" appear>
              <div class="wo-modal panel-raised" v-if="modalOpen">
                <!-- Header -->
                <div class="flex items-start justify-between gap-4 p-6 border-b border-[color:var(--border)]">
                  <div>
                    <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)]">{{ modalItem.woId }} · Detail Lengkap</p>
                    <h2 class="font-display text-3xl font-bold tracking-wide">{{ modalItem.code }}</h2>
                    <p class="text-[color:var(--text-muted)] text-sm">{{ modalItem.type }}</p>
                  </div>
                  <div class="flex items-center gap-2">
                    <span class="px-3 py-1 rounded-full text-xs font-bold text-white" :style="{ background: statusColor(modalItem.status) }">{{ modalItem.status }}</span>
                    <span class="px-3 py-1 rounded-full text-xs font-bold text-white" :style="{ background: priorityColor(modalItem.priority) }">{{ modalItem.priority }}</span>
                    <button @click="closeModal" class="btn btn-ghost !p-2 ml-1" aria-label="Tutup">
                      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5"><path d="M6 18L18 6M6 6l12 12"/></svg>
                    </button>
                  </div>
                </div>

                <div class="p-6 overflow-y-auto wo-modal-body">
                  <!-- ANIMASI ESTIMASI SELESAI PERBAIKAN -->
                  <div class="panel-flat p-6 mb-6">
                    <div class="flex flex-col md:flex-row items-center gap-6">
                      <!-- Ring progress -->
                      <div class="wo-ring shrink-0" :style="{ '--p': repairProgress }">
                        <div class="wo-ring-inner">
                          <span class="font-display text-2xl font-bold text-amber tabular-nums">{{ repairCountdown }}</span>
                          <span class="text-[9px] uppercase tracking-wider text-[color:var(--text-muted)]">menuju selesai</span>
                        </div>
                      </div>
                      <div class="flex-1 w-full">
                        <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-1">Estimasi Perbaikan Selesai</p>
                        <p class="font-display text-2xl font-bold mb-1">{{ fmtDateLong(modalItem.estCompletionDate) }}</p>
                        <p class="text-sm text-[color:var(--text-muted)] mb-3">
                          Mulai perbaikan: <span class="font-semibold text-[color:var(--text)]">{{ fmtDateLong(modalItem.repairByDate) }}</span> ·
                          Durasi estimasi: <span class="font-semibold text-[color:var(--text)]">{{ modalItem.repairDurationHours }} jam</span>
                        </p>
                        <!-- Bar progress beranimasi -->
                        <div class="wo-bar">
                          <div class="wo-bar-fill" :style="{ width: repairProgress + '%' }"></div>
                        </div>
                        <div class="flex justify-between text-[10px] text-[color:var(--text-muted)] mt-1 font-mono">
                          <span>{{ Math.round(repairProgress) }}% berjalan</span>
                          <span>selesai {{ Math.round(100 - repairProgress) }}% lagi</span>
                        </div>
                      </div>
                    </div>
                  </div>

                  <!-- Ringkasan angka -->
                  <div class="grid grid-cols-2 md:grid-cols-4 gap-3 mb-6">
                    <div class="panel-flat p-4">
                      <p class="text-[10px] uppercase tracking-wider text-[color:var(--text-muted)] mb-1">Sisa Umur (RUL)</p>
                      <p class="font-display text-xl font-bold text-critical">{{ fmtHours(modalItem.rulHours) }}</p>
                      <p class="text-[10px] text-[color:var(--text-muted)]">± {{ modalItem.rulUncertainty }} jam</p>
                    </div>
                    <div class="panel-flat p-4">
                      <p class="text-[10px] uppercase tracking-wider text-[color:var(--text-muted)] mb-1">Risk Score</p>
                      <p class="font-display text-xl font-bold text-warning">{{ modalItem.riskScore }}</p>
                      <p class="text-[10px] text-[color:var(--text-muted)]">{{ modalItem.riskLevel }}</p>
                    </div>
                    <div class="panel-flat p-4">
                      <p class="text-[10px] uppercase tracking-wider text-[color:var(--text-muted)] mb-1">Estimasi Breakdown</p>
                      <p class="font-display text-base font-bold">{{ fmtDate(modalItem.failureDate) }}</p>
                    </div>
                    <div class="panel-flat p-4">
                      <p class="text-[10px] uppercase tracking-wider text-[color:var(--text-muted)] mb-1">Estimasi Biaya</p>
                      <p class="font-display text-xl font-bold text-amber">{{ fmtRupiahShort(modalItem.estCost) }}</p>
                    </div>
                  </div>

                  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
                    <!-- Kesehatan komponen (bar CSS) -->
                    <div>
                      <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-3">Kesehatan Komponen</h3>
                      <div class="space-y-2.5">
                        <div v-for="c in modalItem.components" :key="c.component">
                          <div class="flex justify-between text-xs mb-1">
                            <span class="font-semibold">{{ c.component }}</span>
                            <span class="font-mono" :style="{ color: c.health < 40 ? '#E0413E' : c.health < 70 ? '#E0A106' : '#1FA971' }">{{ Math.round(c.health) }}%</span>
                          </div>
                          <div class="wo-bar !h-2">
                            <div class="wo-bar-fill !transition-all" :style="{ width: c.health + '%', background: c.health < 40 ? '#E0413E' : c.health < 70 ? '#E0A106' : '#1FA971' }"></div>
                          </div>
                        </div>
                      </div>
                    </div>
                    <!-- SHAP -->
                    <div>
                      <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-3">Kontributor Risiko (SHAP)</h3>
                      <div class="space-y-2.5">
                        <div v-for="sh in modalItem.shap" :key="sh.feature">
                          <div class="flex justify-between text-xs mb-1">
                            <span class="font-semibold">{{ sh.feature }}</span>
                            <span class="font-mono" :style="{ color: sh.value >= 0 ? '#E0413E' : '#1FA971' }">{{ sh.value >= 0 ? '+' : '' }}{{ Math.round(sh.value) }}%</span>
                          </div>
                          <div class="wo-bar !h-2">
                            <div class="wo-bar-fill !transition-all" :style="{ width: Math.min(100, Math.abs(sh.value)) + '%', background: sh.value >= 0 ? '#E0413E' : '#1FA971' }"></div>
                          </div>
                        </div>
                        <p v-if="!modalItem.shap.length" class="text-sm text-[color:var(--text-muted)]">Data SHAP tidak tersedia.</p>
                      </div>
                    </div>
                  </div>

                  <!-- Digital Twin -->
                  <div v-if="modalItem.twins.length" class="grid grid-cols-3 gap-3 mb-6">
                    <div v-for="tw in modalItem.twins" :key="tw.label" class="panel-flat p-3 text-center">
                      <p class="text-[10px] uppercase tracking-wider text-[color:var(--text-muted)] mb-1">{{ tw.label }}</p>
                      <p class="font-display text-lg font-bold text-steel">{{ fmtHours(tw.hours) }}</p>
                    </div>
                  </div>

                  <!-- Form buat Work Order -->
                  <div class="panel-flat p-5">
                    <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-4">Buat Work Order</h3>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                      <div>
                        <label class="block text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-1.5">Teknisi</label>
                        <input v-model="woTechnician" type="text" placeholder="Nama mekanik / tim"
                          class="w-full px-3 py-2.5 rounded-xl bg-[color:var(--surface-2)] border border-[color:var(--border)] text-sm focus:outline-none focus:border-amber" />
                      </div>
                      <div>
                        <label class="block text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-1.5">Komponen Target</label>
                        <input :value="modalItem.topComponent" readonly
                          class="w-full px-3 py-2.5 rounded-xl bg-[color:var(--surface-2)] border border-[color:var(--border)] text-sm opacity-70" />
                      </div>
                    </div>
                    <div class="mb-4">
                      <label class="block text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-1.5">Catatan</label>
                      <textarea v-model="woNotes" rows="2" placeholder="Instruksi tambahan untuk teknisi…"
                        class="w-full px-3 py-2.5 rounded-xl bg-[color:var(--surface-2)] border border-[color:var(--border)] text-sm focus:outline-none focus:border-amber"></textarea>
                    </div>
                    <div class="flex justify-end gap-2">
                      <button @click="closeModal" class="btn btn-ghost !py-2.5 text-sm">Batal</button>
                      <button @click="submitWorkOrder" :disabled="woSaving"
                        class="btn !py-2.5 text-sm bg-amber text-graphite-900 border-amber font-bold disabled:opacity-60">
                        {{ woSaving ? 'Menyimpan…' : 'Simpan Work Order' }}
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </Transition>
          </div>
        </Transition>
      </Teleport>

      <!-- ===== MODAL DETAIL WORK ORDER TERSIMPAN ===== -->
      <Teleport to="body">
        <Transition name="wo-fade">
          <div v-if="woDetailOpen" class="wo-modal-backdrop" @click.self="closeWoDetail">
            <Transition name="wo-pop" appear>
              <div class="wo-modal panel-raised" v-if="woDetailOpen">
                <div class="flex items-start justify-between gap-4 p-6 border-b border-[color:var(--border)]">
                  <div>
                    <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)]">Detail Work Order Tersimpan</p>
                    <h2 class="font-display text-2xl font-bold tracking-wide">{{ woDetail?.work_order?.wo_number || '—' }}</h2>
                  </div>
                  <div class="flex items-center gap-2">
                    <span v-if="woDetail?.work_order" class="px-3 py-1 rounded-full text-xs font-bold text-white" :style="{ background: woStatusColor(woDetail.work_order.wo_status) }">{{ woDetail.work_order.wo_status }}</span>
                    <button @click="closeWoDetail" class="btn btn-ghost !p-2" aria-label="Tutup">
                      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5"><path d="M6 18L18 6M6 6l12 12"/></svg>
                    </button>
                  </div>
                </div>

                <div class="p-6 overflow-y-auto wo-modal-body">
                  <div v-if="woDetailLoading" class="py-12 text-center text-[color:var(--text-muted)] uppercase tracking-widest text-sm">Memuat detail…</div>

                  <template v-else-if="woDetail?.work_order">
                    <!-- Data Work Order -->
                    <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-3">Data Work Order</h3>
                    <div class="grid grid-cols-2 md:grid-cols-3 gap-3 mb-6">
                      <div class="panel-flat p-3"><p class="wo-k">Prioritas</p><p class="wo-v">{{ woDetail.work_order.priority }}</p></div>
                      <div class="panel-flat p-3"><p class="wo-k">Status Unit (saat dibuat)</p><p class="wo-v">{{ woDetail.work_order.status_unit }}</p></div>
                      <div class="panel-flat p-3"><p class="wo-k">Komponen</p><p class="wo-v">{{ woDetail.work_order.component }}</p></div>
                      <div class="panel-flat p-3"><p class="wo-k">Part No</p><p class="wo-v">{{ woDetail.work_order.part_no || '—' }}</p></div>
                      <div class="panel-flat p-3"><p class="wo-k">RUL (jam)</p><p class="wo-v">{{ woDetail.work_order.rul_hours }}</p></div>
                      <div class="panel-flat p-3"><p class="wo-k">Estimasi Biaya</p><p class="wo-v text-amber">{{ fmtRupiah(woDetail.work_order.est_cost) }}</p></div>
                      <div class="panel-flat p-3"><p class="wo-k">Teknisi</p><p class="wo-v">{{ woDetail.work_order.technician || '—' }}</p></div>
                      <div class="panel-flat p-3"><p class="wo-k">Mulai Perbaikan</p><p class="wo-v text-sm">{{ woDetail.work_order.scheduled_at ? fmtDateLong(woDetail.work_order.scheduled_at) : '—' }}</p></div>
                      <div class="panel-flat p-3"><p class="wo-k">Estimasi Selesai</p><p class="wo-v text-sm">{{ woDetail.work_order.est_completion_at ? fmtDateLong(woDetail.work_order.est_completion_at) : '—' }}</p></div>
                      <div class="panel-flat p-3"><p class="wo-k">Feedback</p><p class="wo-v">{{ woDetail.work_order.feedback || '—' }}</p></div>
                      <div class="panel-flat p-3"><p class="wo-k">Dibuat</p><p class="wo-v text-sm">{{ fmtDateLong(woDetail.work_order.created_at) }}</p></div>
                      <div class="panel-flat p-3"><p class="wo-k">Diperbarui</p><p class="wo-v text-sm">{{ fmtDateLong(woDetail.work_order.updated_at) }}</p></div>
                    </div>
                    <div v-if="woDetail.work_order.notes" class="panel-flat p-4 mb-6">
                      <p class="wo-k mb-1">Catatan</p>
                      <p class="text-sm">{{ woDetail.work_order.notes }}</p>
                    </div>

                    <!-- Data Unit Terkait -->
                    <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-3">Unit Terkait</h3>
                    <div v-if="woDetail.unit" class="grid grid-cols-2 md:grid-cols-3 gap-3">
                      <div class="panel-flat p-3"><p class="wo-k">Kode Unit</p><p class="wo-v">{{ woDetail.unit.code }}</p></div>
                      <div class="panel-flat p-3"><p class="wo-k">Jenis Alat Berat</p><p class="wo-v text-sm">{{ woDetail.unit.jenis_alat_berat_nama || '—' }}</p></div>
                      <div class="panel-flat p-3">
                        <p class="wo-k">Status Unit Saat Ini</p>
                        <p class="wo-v"><span class="px-2 py-0.5 rounded-full text-[11px] font-bold text-white" :style="{ background: statusColor(woDetail.unit.status) }">{{ woDetail.unit.status }}</span></p>
                      </div>
                      <div class="panel-flat p-3"><p class="wo-k">Health</p><p class="wo-v">{{ woDetail.unit.health }}%</p></div>
                      <div class="panel-flat p-3"><p class="wo-k">Maintenance</p><p class="wo-v text-sm">{{ woDetail.unit.maintenance }}</p></div>
                      <div class="panel-flat p-3"><p class="wo-k">Savings</p><p class="wo-v">{{ fmtRupiahShort(woDetail.unit.savings) }}</p></div>
                      <div v-if="woDetail.unit.lat && woDetail.unit.lng" class="panel-flat p-3"><p class="wo-k">Lokasi</p><p class="wo-v text-sm font-mono">{{ woDetail.unit.lat.toFixed(4) }}, {{ woDetail.unit.lng.toFixed(4) }}</p></div>
                    </div>
                    <!-- Peta lokasi unit -->
                    <div v-if="woDetail.unit && woDetail.unit.lat && woDetail.unit.lng" class="mt-4">
                      <p class="wo-k mb-1.5">Peta Lokasi Unit</p>
                      <div id="wo-detail-map" class="w-full h-64 rounded-xl overflow-hidden border border-[color:var(--border)] z-0"></div>
                    </div>
                    <div v-if="!woDetail.unit" class="panel-flat p-4 text-sm text-[color:var(--text-muted)]">Unit dengan kode {{ woDetail.work_order.asset_code }} tidak ditemukan di basis data unit.</div>

                    <!-- Aksi status -->
                    <div class="flex justify-end gap-2 mt-6">
                      <button v-if="woDetail.work_order.wo_status === 'OPEN'" @click="updateWoStatus(woDetail.work_order.id, 'IN_PROGRESS').then(() => openWoDetail(woDetail.work_order.id))" class="btn !py-2.5 text-sm border-warning/50 text-warning bg-warning/10">Mulai Perbaikan</button>
                      <button v-if="woDetail.work_order.wo_status === 'IN_PROGRESS'" @click="updateWoStatus(woDetail.work_order.id, 'COMPLETED').then(() => openWoDetail(woDetail.work_order.id))" class="btn !py-2.5 text-sm bg-healthy text-white border-healthy font-bold">Tandai Selesai (Unit → Sehat)</button>
                      <button @click="closeWoDetail" class="btn btn-ghost !py-2.5 text-sm">Tutup</button>
                    </div>
                  </template>
                </div>
              </div>
            </Transition>
          </div>
        </Transition>
      </Teleport>
    </main>
  </div>
</template>

<style scoped>
.wo-modal-backdrop {
  position: fixed; inset: 0; z-index: 100;
  background: rgba(12, 16, 20, 0.6);
  backdrop-filter: blur(3px);
  display: flex; align-items: center; justify-content: center;
  padding: 1.5rem;
}
.wo-modal {
  width: 100%; max-width: 56rem;
  max-height: 90vh; display: flex; flex-direction: column;
  border-radius: 1.25rem; overflow: hidden;
}
.wo-modal-body { max-height: calc(90vh - 110px); }

/* Ring progress beranimasi */
.wo-ring {
  --p: 0;
  width: 150px; height: 150px; border-radius: 50%;
  background: conic-gradient(#F2A60C calc(var(--p) * 1%), var(--surface-2, #e5e8ec) 0);
  display: grid; place-items: center;
  transition: background 0.9s linear;
  box-shadow: 0 0 0 1px rgba(242,166,12,0.25), 0 10px 30px -12px rgba(242,166,12,0.4);
  animation: wo-pulse 2.4s ease-in-out infinite;
}
.wo-ring-inner {
  width: 116px; height: 116px; border-radius: 50%;
  background: var(--surface, #fff);
  display: flex; flex-direction: column; align-items: center; justify-content: center;
  gap: 2px;
}
@keyframes wo-pulse {
  0%, 100% { box-shadow: 0 0 0 1px rgba(242,166,12,0.25), 0 10px 30px -12px rgba(242,166,12,0.35); }
  50%      { box-shadow: 0 0 0 3px rgba(242,166,12,0.4), 0 14px 40px -10px rgba(242,166,12,0.6); }
}

/* Bar progress */
.wo-bar { height: 12px; border-radius: 999px; background: var(--surface-2, #e5e8ec); overflow: hidden; }
.wo-bar-fill {
  height: 100%; border-radius: 999px;
  background: linear-gradient(135deg, #F2A60C 0%, #D9930A 100%);
  transition: width 0.9s linear;
}

/* Transisi modal */
.wo-fade-enter-active, .wo-fade-leave-active { transition: opacity 0.25s ease; }
.wo-fade-enter-from, .wo-fade-leave-to { opacity: 0; }
.wo-pop-enter-active { transition: transform 0.28s cubic-bezier(0.22,1,0.36,1), opacity 0.28s ease; }
.wo-pop-enter-from { transform: translateY(16px) scale(0.96); opacity: 0; }

/* Key-value di modal detail */
.wo-k { font-size: 10px; text-transform: uppercase; letter-spacing: 0.05em; color: var(--text-muted, #5d6b7a); margin-bottom: 2px; }
.wo-v { font-weight: 700; }
</style>
