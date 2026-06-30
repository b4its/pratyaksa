<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick, watch } from 'vue'

// Muat web component <model-viewer> (Google) untuk render GLB
useHead({
  script: [
    { type: 'module', src: 'https://ajax.googleapis.com/ajax/libs/model-viewer/3.3.0/model-viewer.min.js' },
  ],
})

// --- SIDEBAR ---
interface MenuItem { name: string; path: string; icon: string }
const menuItems = ref<MenuItem[]>([
  { name: 'Dashboard', path: '/panel/dashboard', icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="7" height="9" x="3" y="3" rx="1"/><rect width="7" height="5" x="14" y="3" rx="1"/><rect width="7" height="9" x="14" y="12" rx="1"/><rect width="7" height="5" x="3" y="16" rx="1"/></svg>` },
  { name: 'Jenis Alat Berat', path: '/panel/jenis_alat_berat', icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 17h4V5H2v12h3"/><path d="M20 17h2v-9l-2.5-3.5H14v12h3"/><path d="M14 6h4.5"/><circle cx="18.5" cy="17.5" r="2.5"/><circle cx="5.5" cy="17.5" r="2.5"/></svg>` },
  { name: 'Unit Tambang', path: '/panel/unit_tambang', icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 12 12 17 22 12"/><polyline points="2 17 12 22 22 17"/></svg>` },
  { name: 'Analisa Kerusakan', path: '/panel/analisa', icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>` },
  { name: 'Work Order', path: '/panel/work_order', icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.12 2.12 0 0 1 3 3L12 15l-4 1 1-4Z"/></svg>` },
  { name: 'Kembali', path: '../', icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m12 19-7-7 7-7"/><path d="M19 12H5"/></svg>` },
])
const activeMenu = ref('Analisa Kerusakan')

// --- TYPES ---
interface UnitSummary {
  id: string; code: string; jenis_alat_berat_nama: string | null
  status: string; health: number; risk_score: number; risk_level: string
}
interface Overview {
  total_units: number; avg_health: number; avg_risk_score: number
  fleet_avg_suhu: number; fleet_avg_vibration: number; units_at_risk: number
  status_distribution: { label: string; count: number }[]
  risk_distribution: { label: string; count: number }[]
  units: UnitSummary[]
  updated_at: string
}
interface UnitAnalysis {
  unit: { id: string; code: string; jenis_alat_berat_nama: string | null; status: string; health: number; img_url: string | null; model3d_url: string | null }
  risk_score: number; risk_level: string
  sensor_readings: {
    suhu_mesin: number; vibration: number; tekanan_oli: number; rpm: number
    fuel_level: number; oil_particle_iso: number; acoustic_db: number; jam_operasi: number
  }
  component_health: { component: string; health: number }[]
  rul_prediction: { component: string; hours_remaining: number; lower_bound: number; upper_bound: number; confidence: number }
  shap_contributions: { feature: string; value: number }[]
  sensor_history: { time: string; suhu_mesin: number; vibration: number; tekanan_oli: number; acoustic: number }[]
  telemetry: Telemetry
  prediction: Prediction
  operational: Record<string, number | boolean>
  updated_at: string
}
interface Telemetry {
  component_type: string; operator_id: string; payload_tonnage: number
  hour_meter_actual: number; design_life_hm: number; component_age_hm: number; is_remanufactured: boolean
  ambient_temp_c: number; idle_time_ratio: number
  eng_coolant_temp_c: number; eng_oil_press_psi: number; eng_rpm: number; eng_load_pct: number
  hyd_pump_press_psi: number; hyd_oil_temp_c: number; trans_oil_temp_c: number
  torque_converter_temp_c: number; final_drive_temp_c: number; brake_cooling_temp_c: number
  battery_voltage: number; fault_code_severity: number
  lab_fe_ppm: number; lab_cu_ppm: number; lab_al_ppm: number; lab_si_ppm: number
  lab_viscosity_100c: number; lab_water_content_pct: number; lab_soot_pct: number
  delta_eng_temp: number; status_label: string; rul_hours: number
}
interface Prediction {
  asset_id: string
  equipment_type: string
  xgb_anomaly_class: number
  xgb_anomaly_label: string
  lstm_rul_hours: number
  rul_uncertainty: number
  risk_level: string
  risk_class: number
  model_agreement: boolean
  lstm_hydraulic_system: number
  lstm_hydraulic_pump: number
  lstm_pump_seal: number
  lstm_brake_system: number
  lstm_brake_caliper: number
  lstm_brake_pad: number
  lstm_steering_system: number
  digital_twin: { brake_twin_rul: number; bearing_twin_rul: number; hydraulic_twin_rul: number }
  drift_status: { drift_detected: boolean; drifted_features: string[]; max_z_score: number; n_drifted: number }
  latency_ms: number
}

// --- STATE ---
const overview = ref<Overview | null>(null)
const analysis = ref<UnitAnalysis | null>(null)
const selectedUnitId = ref<string | null>(null)
const isLoading = ref(true)
const errorMsg = ref('')
const autoRefresh = ref(true)
const lastUpdate = ref('')

// --- LIVE mode (silent / telegram) shared from usePratyaksa singleton ---
const pratyaksa = usePratyaksa()
type PratyaksaSourceMode = 'live-silent' | 'live-telegram' | 'hit-endpoint-sendiri' | 'hit-endpoint-ml'
const alertStatus = ref<{ ok: boolean; msg: string } | null>(null)
const alertTesting = ref(false)
const alertedAssets = new Set<string>() // throttle: 1 alert per episode CRITICAL (sukses)
const alertCooldown = new Map<string, number>() // asset -> timestamp boleh coba lagi (setelah gagal)
let alertStatusTimer: any = null

// Inisialisasi pratyaksa polling jika belum
const pratyaksaInit = ref(false)
if (!pratyaksaInit.value) {
  pratyaksaInit.value = true
  pratyaksa.fetchAll()
  pratyaksa.startPolling(10000)
}

const showAlertStatus = (ok: boolean, msg: string) => {
  alertStatus.value = { ok, msg }
  if (alertStatusTimer) clearTimeout(alertStatusTimer)
  alertStatusTimer = setTimeout(() => { alertStatus.value = null }, 6000)
}

const { initAuth, user } = useAuth()
const { isDark, toggleTheme, initTheme } = useTheme()
const { resolveModel } = useModels()
const api = useApi()
const config = useRuntimeConfig()
let ChartLib: any = null
let refreshTimer: any = null

// chart instances
const charts: Record<string, any> = {}

// --- HELPERS ---
const riskColor = (level: string) => ({
  LOW: '#1FA971', MEDIUM: '#E0A106', HIGH: '#E0843E', CRITICAL: '#E0413E',
}[level] || '#7A848E')
const statusColor = (s: string) => ({
  SEHAT: '#1FA971', WARNING: '#E0A106', CRITICAL: '#E0413E', RUSAK: '#7A848E',
}[s] || '#7A848E')

const selectedUnit = computed(() =>
  overview.value?.units.find(u => u.id === selectedUnitId.value) || null)

// Pagination daftar "Pilih Unit" (agar tidak memanjang ke bawah)
const unitListPage = ref(1)
const unitListPerPage = 6
const unitListTotalPages = computed(() => Math.max(1, Math.ceil((overview.value?.units.length || 0) / unitListPerPage)))
const pagedUnitList = computed(() => {
  const arr = overview.value?.units || []
  const start = (unitListPage.value - 1) * unitListPerPage
  return arr.slice(start, start + unitListPerPage)
})

// --- FETCH ---
const fetchOverview = async () => {
  try {
    const res = await api.getAnalisaOverview() as any
    overview.value = res.data
    if (!selectedUnitId.value && res.data.units.length > 0) {
      selectedUnitId.value = res.data.units[0].id
    }
    errorMsg.value = ''
  } catch (e: any) {
    errorMsg.value = e?.data?.message || 'Gagal memuat data analitik armada.'
  }
}

const fetchAnalysis = async () => {
  if (!selectedUnitId.value) return
  try {
    const res = await api.getUnitAnalysis(selectedUnitId.value) as any
    analysis.value = res.data
    lastUpdate.value = new Date().toLocaleTimeString('id-ID')
  } catch (e: any) {
    errorMsg.value = e?.data?.message || 'Gagal memuat analitik unit.'
  }
}

const refreshAll = async () => {
  await Promise.all([fetchOverview(), fetchAnalysis()])
  await nextTick()
  renderAllCharts()
  await maybeSendAlert()
}

// --- Telegram alert (mode live + telegram) ---
const buildAlertPayload = (a: any, statusOverride?: string) => {
  const shaps = [...(a.shap_contributions || [])].sort((x: any, y: any) => Math.abs(y.value) - Math.abs(x.value))
  const fmt = (s: any) => (s ? `${s.feature} (${Math.abs(Math.round(s.value))}%)` : '-')
  const urgent = lstmComponentsOf(a)[0]
  const partName = urgent?.label || a.rul_prediction?.component || 'Komponen Utama'
  const partNo =
    'PRT-' +
    String(a.unit.code).replace(/[^A-Za-z0-9]/g, '').toUpperCase().slice(0, 6) +
    '-' +
    String(Math.round(urgent?.hours || 0)).padStart(3, '0')
  return {
    asset_id: a.unit.code,
    model: a.unit.jenis_alat_berat_nama || '-',
    lokasi: 'Area Tambang Kutai',
    status: statusOverride || a.prediction.risk_level,
    rul: String(Math.round(a.prediction.lstm_rul_hours)),
    shap1: fmt(shaps[0]),
    shap2: fmt(shaps[1]),
    part_name: partName,
    part_no: partNo,
    stok: String(2 + (String(a.unit.code).length % 4)),
  }
}

// Status unit yang memicu kirim ke Telegram
const ALERT_STATUSES = ['CRITICAL', 'WARNING']

// Broadcast alert untuk SEMUA unit berstatus CRITICAL / WARNING (bukan hanya unit terpilih).
// Throttle per-unit: 1 alert per episode (sukses), cooldown 60 dtk setelah gagal.
const maybeSendAlert = async () => {
  if (pratyaksa.sourceMode.value !== 'live-telegram') return
  const ov = overview.value
  if (!ov || !Array.isArray(ov.units)) return

  const atRisk = ov.units.filter(u => ALERT_STATUSES.includes(u.status))
  const atRiskCodes = new Set(atRisk.map(u => u.code))

  // Reset throttle untuk unit yang sudah tidak berisiko (kembali SEHAT/RUSAK)
  for (const code of [...alertedAssets]) {
    if (!atRiskCodes.has(code)) { alertedAssets.delete(code); alertCooldown.delete(code) }
  }

  let sent = 0
  let failed = 0
  let lastDetail = ''
  for (const u of atRisk) {
    const asset = u.code
    if (alertedAssets.has(asset)) continue                       // sudah terkirim episode ini
    if (Date.now() < (alertCooldown.get(asset) || 0)) continue   // masih cooldown setelah gagal
    try {
      const detail: any = await api.getUnitAnalysis(u.id)
      const a = detail.data
      await api.sendAlert(buildAlertPayload(a, u.status))
      alertedAssets.add(asset)
      alertCooldown.delete(asset)
      sent++
    } catch (e: any) {
      alertCooldown.set(asset, Date.now() + 60000) // jangan spam: tunda 60 dtk
      failed++
      lastDetail = e?.data?.statusMessage || e?.data?.message || e?.statusMessage || e?.message || 'endpoint tak terjangkau'
    }
  }

  if (sent > 0 && failed === 0) {
    showAlertStatus(true, `Alert terkirim untuk ${sent} unit berisiko (CRITICAL/WARNING).`)
  } else if (sent > 0 && failed > 0) {
    showAlertStatus(false, `${sent} alert terkirim, ${failed} gagal: ${lastDetail}`)
  } else if (failed > 0) {
    showAlertStatus(false, `Gagal kirim alert (${failed} unit): ${lastDetail} — cek koneksi ke endpoint Telegram.`)
  }
}

// Uji koneksi ke endpoint Telegram (kirim test alert) — bisa dipanggil kapan saja
const testTelegram = async () => {
  alertTesting.value = true
  try {
    const a = analysis.value as any
    const payload = a
      ? { ...buildAlertPayload(a), lokasi: 'Uji Koneksi (TEST)' }
      : {
          asset_id: 'TEST-PING', model: 'Pratyaksa Test', lokasi: 'Uji Koneksi (TEST)',
          status: 'TEST', rul: '0', shap1: '-', shap2: '-',
          part_name: '-', part_no: '-', stok: '0',
        }
    const res: any = await api.sendAlert(payload)
    const upMsg = res?.upstream?.message
    showAlertStatus(true, `Koneksi Telegram OK — test alert terkirim${a ? ' (' + a.unit.code + ')' : ''}.${upMsg ? ' ' + upMsg : ''}`)
  } catch (e: any) {
    const detail = e?.data?.statusMessage || e?.data?.message || e?.statusMessage || e?.message || 'endpoint tak terjangkau'
    showAlertStatus(false, `Test koneksi gagal: ${detail}`)
  } finally {
    alertTesting.value = false
  }
}

const selectUnit = async (id: string) => {
  selectedUnitId.value = id
  await fetchAnalysis()
  await nextTick()
  renderUnitCharts()
}

// --- CHART RENDERING ---
const css = (v: string) => (typeof window !== 'undefined' ? getComputedStyle(document.documentElement).getPropertyValue(v).trim() : '')
const theme = () => ({
  tick: css('--text-muted') || '#5d6b7a',
  grid: css('--border') || '#d7dde4',
  axis: css('--border-strong') || '#c2cad3',
  surface: css('--surface') || '#ffffff',
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

const labelFont = { family: 'Inter', weight: 'bold' as const }
const monoFont = { family: 'JetBrains Mono' }

// Plugin: tampilkan angka pada tiap arc doughnut (selalu terlihat, bukan hanya saat hover)
const arcValueLabel = {
  id: 'arcValueLabel',
  afterDatasetsDraw(chart: any) {
    const { ctx } = chart
    const meta = chart.getDatasetMeta(0)
    const ds = chart.data.datasets[0]
    if (!meta || !meta.data) return
    meta.data.forEach((arc: any, i: number) => {
      const val = ds.data[i]
      if (val == null || val === 0) return
      const pos = arc.tooltipPosition()
      ctx.save()
      ctx.font = '800 14px Inter, sans-serif'
      ctx.textAlign = 'center'
      ctx.textBaseline = 'middle'
      ctx.shadowColor = 'rgba(0,0,0,0.55)'
      ctx.shadowBlur = 4
      ctx.fillStyle = '#ffffff'
      ctx.fillText(String(val), pos.x, pos.y)
      ctx.restore()
    })
  },
}

// Plugin: tampilkan angka di atas tiap batang bar
const barValueLabel = {
  id: 'barValueLabel',
  afterDatasetsDraw(chart: any) {
    const { ctx } = chart
    const meta = chart.getDatasetMeta(0)
    const ds = chart.data.datasets[0]
    if (!meta || !meta.data) return
    const tickColor = css('--text') || '#1b2128'
    meta.data.forEach((bar: any, i: number) => {
      const val = ds.data[i]
      if (val == null) return
      ctx.save()
      ctx.font = '800 13px JetBrains Mono, monospace'
      ctx.textAlign = 'center'
      ctx.textBaseline = 'bottom'
      ctx.fillStyle = tickColor
      ctx.fillText(String(val), bar.x, bar.y - 6)
      ctx.restore()
    })
  },
}

// Plugin: nilai di ujung kanan tiap bar horizontal (RUL multi-komponen)
const barValueLabelH = {
  id: 'barValueLabelH',
  afterDatasetsDraw(chart: any) {
    const { ctx } = chart
    const meta = chart.getDatasetMeta(0)
    const ds = chart.data.datasets[0]
    if (!meta || !meta.data) return
    const col = css('--text-muted') || '#5d6b7a'
    meta.data.forEach((bar: any, i: number) => {
      const val = ds.data[i]
      if (val == null) return
      ctx.save()
      ctx.font = '700 10px JetBrains Mono, monospace'
      ctx.textAlign = 'left'
      ctx.textBaseline = 'middle'
      ctx.fillStyle = col
      ctx.fillText(`${val}j`, bar.x + 6, bar.y)
      ctx.restore()
    })
  },
}

const rulColor = (l: string) => ({ CRITICAL: '#E0413E', WARNING: '#E0A106', NORMAL: '#1FA971', OK: '#1FA971' }[l] || '#7A848E')
const rulTone = (h: number) => (h < 150 ? '#E0413E' : h < 450 ? '#E0A106' : '#1FA971')
const rulToneLabel = (h: number) => (h < 150 ? 'CRITICAL' : h < 450 ? 'WARNING' : 'NORMAL')

// 7 RUL komponen LSTM (sesuai kontrak /predict), urut paling mendesak
// Hitung 7 RUL komponen LSTM (sesuai kontrak /predict) dari sebuah objek analisa,
// urut paling mendesak. Dipakai untuk unit terpilih maupun unit lain saat broadcast.
const lstmComponentsOf = (a: any): { label: string; hours: number }[] => {
  const p = a?.prediction
  if (!p) return []
  return [
    { label: 'Sistem Hidrolik', hours: p.lstm_hydraulic_system },
    { label: 'Pompa Hidrolik', hours: p.lstm_hydraulic_pump },
    { label: 'Seal Pompa', hours: p.lstm_pump_seal },
    { label: 'Sistem Rem', hours: p.lstm_brake_system },
    { label: 'Brake Caliper', hours: p.lstm_brake_caliper },
    { label: 'Brake Pad (Rear)', hours: p.lstm_brake_pad },
    { label: 'Sistem Kemudi', hours: p.lstm_steering_system },
  ].sort((a, b) => a.hours - b.hours)
}

const lstmComponents = computed(() => lstmComponentsOf(analysis.value))

const digitalTwins = computed(() => {
  const d = analysis.value?.prediction?.digital_twin
  if (!d) return [] as { label: string; hours: number }[]
  return [
    { label: 'Brake Twin', hours: d.brake_twin_rul },
    { label: 'Bearing Twin', hours: d.bearing_twin_rul },
    { label: 'Hydraulic Twin', hours: d.hydraulic_twin_rul },
  ]
})

// Parameter operasional & lingkungan (dari dataset industri)
const operationalFields = [
  { k: 'road_grade_pct', l: 'Road Grade', u: '%' },
  { k: 'haul_distance_km', l: 'Jarak Angkut', u: ' km' },
  { k: 'cycle_time_minutes', l: 'Cycle Time', u: ' min' },
  { k: 'dust_concentration_mgm3', l: 'Konsentrasi Debu', u: ' mg/m³' },
  { k: 'humidity_pct', l: 'Kelembapan', u: '%' },
  { k: 'days_since_last_pm', l: 'Hari sejak PM', u: ' hari' },
  { k: 'last_maintenance_hours', l: 'Jam sejak MTC', u: ' h' },
  { k: 'fuel_consumption_rate_lph', l: 'Konsumsi BBM', u: ' L/h' },
  { k: 'boost_pressure_kpa', l: 'Boost Pressure', u: ' kPa' },
  { k: 'exhaust_gas_temp_c', l: 'Suhu Gas Buang', u: '°C' },
  { k: 'engine_oil_temp_c', l: 'Suhu Oli Mesin', u: '°C' },
  { k: 'coolant_pressure_kpa', l: 'Tekanan Coolant', u: ' kPa' },
  { k: 'vibration_x_g', l: 'Vibrasi X', u: ' g' },
  { k: 'vibration_y_g', l: 'Vibrasi Y', u: ' g' },
  { k: 'vibration_z_g', l: 'Vibrasi Z', u: ' g' },
  { k: 'oil_viscosity_cst', l: 'Viskositas Oli', u: ' cSt' },
  { k: 'oil_particle_count_iso', l: 'Partikel Oli', u: ' ISO' },
  { k: 'oil_moisture_pct', l: 'Moisture Oli', u: '%' },
  { k: 'wear_metal_fe_ppm', l: 'Wear Metal Fe', u: ' ppm' },
  { k: 'wear_metal_cu_ppm', l: 'Wear Metal Cu', u: ' ppm' },
]

const urgentComponents = computed(() => [...lstmComponents.value].slice(0, 3))

const renderOverviewCharts = () => {
  if (!overview.value) return
  const t = theme()

  // Status pie (doughnut)
  const sd = overview.value.status_distribution
  upsertChart('statusPie', 'statusPie', {
    type: 'doughnut',
    data: {
      labels: sd.map(s => s.label),
      datasets: [{
        data: sd.map(s => s.count),
        backgroundColor: sd.map(s => statusColor(s.label)),
        borderColor: t.surface, borderWidth: 3,
      }],
    },
    plugins: [arcValueLabel],
    options: {
      responsive: true, maintainAspectRatio: false, cutout: '60%',
      plugins: { legend: { position: 'bottom', labels: { font: labelFont, color: t.tick, boxWidth: 12, usePointStyle: true } } },
    },
  })

  // Risk distribution bar
  const rd = overview.value.risk_distribution
  upsertChart('riskBar', 'riskBar', {
    type: 'bar',
    data: {
      labels: rd.map(r => r.label),
      datasets: [{
        data: rd.map(r => r.count),
        backgroundColor: rd.map(r => riskColor(r.label)),
        borderRadius: 6, borderSkipped: false,
      }],
    },
    plugins: [barValueLabel],
    options: {
      responsive: true, maintainAspectRatio: false,
      plugins: { legend: { display: false } },
      scales: {
        x: { ticks: { font: labelFont, color: t.tick }, grid: { display: false }, border: { color: t.axis } },
        y: { beginAtZero: true, ticks: { font: monoFont, color: t.tick, stepSize: 1 }, grid: { color: t.grid }, border: { color: t.axis } },
      },
    },
  })
}

const renderUnitCharts = () => {
  if (!analysis.value) return
  const t = theme()

  // Risk gauge (doughnut as gauge)
  const rs = analysis.value.risk_score
  upsertChart('riskGauge', 'riskGauge', {
    type: 'doughnut',
    data: {
      labels: ['Risk', 'Sisa'],
      datasets: [{
        data: [rs, 100 - rs],
        backgroundColor: [riskColor(analysis.value.risk_level), t.grid],
        borderColor: t.surface, borderWidth: 2, circumference: 180, rotation: 270,
      }],
    },
    options: {
      responsive: true, maintainAspectRatio: false, cutout: '72%',
      plugins: { legend: { display: false }, tooltip: { enabled: false } },
    },
  })

  // Component health radar
  const ch = analysis.value.component_health
  upsertChart('radar', 'radar', {
    type: 'radar',
    data: {
      labels: ch.map(c => c.component),
      datasets: [{
        label: 'Health Score',
        data: ch.map(c => c.health),
        backgroundColor: 'rgba(62,146,204,0.18)',
        borderColor: '#3E92CC', borderWidth: 2.5,
        pointBackgroundColor: '#3E92CC', pointBorderColor: t.surface, pointRadius: 4,
      }],
    },
    options: {
      responsive: true, maintainAspectRatio: false,
      plugins: { legend: { display: false } },
      scales: {
        r: {
          min: 0, max: 100,
          ticks: { stepSize: 25, font: { family: 'JetBrains Mono', size: 9 }, color: t.tick, backdropColor: 'transparent' },
          grid: { color: t.grid }, angleLines: { color: t.grid },
          pointLabels: { font: labelFont, color: t.text },
        },
      },
    },
  })

  // Multi-sensor time-series
  const hist = analysis.value.sensor_history
  upsertChart('history', 'history', {
    type: 'line',
    data: {
      labels: hist.map(h => h.time),
      datasets: [
        { label: 'Suhu (°C)', data: hist.map(h => h.suhu_mesin), borderColor: '#E0413E', backgroundColor: '#E0413E', borderWidth: 2, pointRadius: 0, tension: 0.35, yAxisID: 'y' },
        { label: 'Akustik (dB)', data: hist.map(h => h.acoustic), borderColor: '#E0843E', backgroundColor: '#E0843E', borderWidth: 2, pointRadius: 0, tension: 0.35, yAxisID: 'y' },
        { label: 'Vibrasi (g)', data: hist.map(h => h.vibration), borderColor: '#3E92CC', backgroundColor: '#3E92CC', borderWidth: 2, pointRadius: 0, tension: 0.35, yAxisID: 'y1' },
        { label: 'Tekanan Oli (bar)', data: hist.map(h => h.tekanan_oli), borderColor: '#1FA971', backgroundColor: '#1FA971', borderWidth: 2, pointRadius: 0, tension: 0.35, yAxisID: 'y1' },
      ],
    },
    options: {
      responsive: true, maintainAspectRatio: false, interaction: { mode: 'index', intersect: false },
      plugins: { legend: { position: 'bottom', labels: { font: { family: 'Inter', size: 10, weight: 'bold' }, color: t.tick, boxWidth: 12, usePointStyle: true } } },
      scales: {
        x: { ticks: { font: { family: 'JetBrains Mono', size: 9 }, color: t.tick, maxTicksLimit: 8 }, grid: { display: false }, border: { color: t.axis } },
        y: { position: 'left', ticks: { font: { family: 'JetBrains Mono', size: 9 }, color: t.tick }, grid: { color: t.grid }, border: { color: t.axis } },
        y1: { position: 'right', ticks: { font: { family: 'JetBrains Mono', size: 9 }, color: t.tick }, grid: { display: false }, border: { color: t.axis } },
      },
    },
  })

  // SHAP contributions (horizontal bar)
  const shap = [...analysis.value.shap_contributions].sort((a, b) => Math.abs(b.value) - Math.abs(a.value))
  upsertChart('shap', 'shap', {
    type: 'bar',
    data: {
      labels: shap.map(s => s.feature),
      datasets: [{
        data: shap.map(s => s.value),
        backgroundColor: shap.map(s => s.value >= 0 ? '#E0413E' : '#1FA971'),
        borderRadius: 5, borderSkipped: false,
      }],
    },
    options: {
      indexAxis: 'y', responsive: true, maintainAspectRatio: false,
      plugins: { legend: { display: false }, tooltip: { callbacks: { label: (c: any) => `${c.raw >= 0 ? '+' : ''}${c.raw} ke risiko` } } },
      scales: {
        x: { ticks: { font: { family: 'JetBrains Mono', size: 9 }, color: t.tick }, grid: { color: t.grid }, border: { color: t.axis } },
        y: { ticks: { font: { family: 'Inter', size: 10, weight: 'bold' }, color: t.text }, grid: { display: false }, border: { color: t.axis } },
      },
    },
  })

  // Lab oil metals (bar) — Fe/Cu/Al/Si ppm
  const tm = analysis.value.telemetry
  upsertChart('labMetals', 'labMetals', {
    type: 'bar',
    data: {
      labels: ['Fe (Besi)', 'Cu (Tembaga)', 'Al (Aluminium)', 'Si (Silika)'],
      datasets: [{
        label: 'ppm',
        data: [tm.lab_fe_ppm, tm.lab_cu_ppm, tm.lab_al_ppm, tm.lab_si_ppm],
        backgroundColor: ['#E0413E', '#E0843E', '#3E92CC', '#9B7BE0'],
        borderRadius: 5, borderSkipped: false,
      }],
    },
    options: {
      responsive: true, maintainAspectRatio: false,
      plugins: { legend: { display: false } },
      scales: {
        x: { ticks: { font: { family: 'Inter', size: 9, weight: 'bold' }, color: t.tick }, grid: { display: false }, border: { color: t.axis } },
        y: { beginAtZero: true, ticks: { font: { family: 'JetBrains Mono', size: 9 }, color: t.tick }, grid: { color: t.grid }, border: { color: t.axis } },
      },
    },
  })

  // Prediksi RUL per komponen (LSTM, sesuai /predict) — horizontal bar, urut paling mendesak
  const rc = lstmComponents.value
  upsertChart('rulComponents', 'rulComponents', {
    type: 'bar',
    data: {
      labels: rc.map(c => c.label),
      datasets: [{
        data: rc.map(c => c.hours),
        backgroundColor: rc.map(c => rulTone(c.hours)),
        borderRadius: 5, borderSkipped: false,
      }],
    },
    plugins: [barValueLabelH],
    options: {
      indexAxis: 'y', responsive: true, maintainAspectRatio: false,
      layout: { padding: { right: 40 } },
      plugins: {
        legend: { display: false },
        tooltip: { callbacks: { label: (c: any) => `${c.raw} jam tersisa` } },
      },
      scales: {
        x: { beginAtZero: true, ticks: { font: { family: 'JetBrains Mono', size: 9 }, color: t.tick }, grid: { color: t.grid }, border: { color: t.axis } },
        y: { ticks: { font: { family: 'Inter', size: 10, weight: 'bold' }, color: t.text }, grid: { display: false }, border: { color: t.axis } },
      },
    },
  })
}

const renderAllCharts = () => { renderOverviewCharts(); renderUnitCharts() }

// --- LIFECYCLE ---
onMounted(async () => {
  initTheme()
  initAuth()
  isLoading.value = true
  if (typeof window !== 'undefined') {
    const mod = await import('chart.js/auto')
    ChartLib = mod.default
  }
  await fetchOverview()
  await fetchAnalysis()
  isLoading.value = false
  await nextTick()
  renderAllCharts()

  refreshTimer = setInterval(() => {
    if (autoRefresh.value) refreshAll()
  }, 15000)

  // Timer khusus Telegram — kirim alert tiap 5 detik selama mode live-telegram
  const alertTimer = setInterval(() => {
    if (pratyaksa.sourceMode.value === 'live-telegram') {
      maybeSendAlert()
    }
  }, 5000)
  // Simpan alertTimer ke window agar bisa dibersihkan
  ;(window as any).__pratyaksa_alert_timer = alertTimer
})

onUnmounted(() => {
  if (refreshTimer) clearInterval(refreshTimer)
  const at = (window as any).__pratyaksa_alert_timer
  if (at) { clearInterval(at); (window as any).__pratyaksa_alert_timer = null }
  Object.values(charts).forEach(c => c?.destroy())
})

// Re-render chart dengan warna tema baru saat dark/light di-toggle
watch(isDark, () => { nextTick(() => renderAllCharts()) })

const fmtHours = (h: number) => {
  if (h >= 24) return `${Math.floor(h / 24)} hari ${Math.round(h % 24)} jam`
  return `${Math.round(h)} jam`
}

const statusLabelColor = (s: string) => ({
  NORMAL: '#1FA971', WARNING: '#E0A106', CRITICAL: '#E0413E',
}[s] || '#7A848E')
</script>

<template>
  <div class="flex h-screen overflow-hidden bg-mesh text-[color:var(--text)]">
    <!-- Sidebar -->
    <PanelSidebar />

    <!-- Main -->
    <main class="flex-1 min-w-0 w-full p-4 sm:p-6 lg:p-8 pt-16 lg:pt-8 overflow-y-auto">
      <header class="flex justify-between items-start mb-8 flex-wrap gap-4">
        <div>
          <h1 class="font-display text-4xl md:text-5xl font-bold uppercase tracking-wide leading-none">Analisa Kerusakan</h1>
          <p class="mt-2 text-[color:var(--text-muted)]">Monitoring kondisi & prediksi kegagalan armada secara real-time.</p>
        </div>
        <div class="flex items-center gap-3">
          <!-- PRATYAKSA Mode Selector (4 sub-modes for analisa) -->
          <ModeSelector showSubModes @testTelegram="testTelegram" />

          <div class="panel-flat px-3 py-2 text-[10px] font-mono text-[color:var(--text-muted)]">
            Update<br><span class="font-semibold text-[color:var(--text)]">{{ lastUpdate || '—' }}</span>
          </div>
        </div>
      </header>

      <!-- Mode Lock Table -->
      <ModeLockTabel showSubModes @testTelegram="testTelegram" />

      <!-- status pengiriman alert telegram -->
      <div v-if="alertStatus" class="mb-4 px-4 py-2.5 rounded-xl text-sm font-semibold flex items-center gap-2"
        :class="alertStatus.ok ? 'bg-healthy/10 border border-healthy/40 text-healthy' : 'bg-critical/10 border border-critical/40 text-critical'">
        <svg class="w-4 h-4 shrink-0" fill="currentColor" viewBox="0 0 24 24"><path d="M9.78 18.65l.28-4.23 7.68-6.92c.34-.31-.07-.46-.52-.19L7.74 13.3 3.64 12c-.88-.25-.89-.86.2-1.3l15.97-6.16c.73-.33 1.43.18 1.15 1.3l-2.72 12.81c-.19.91-.74 1.13-1.5.71L12.6 16.3l-1.99 1.93c-.23.23-.42.42-.83.42z"/></svg>
        {{ alertStatus.msg }}
      </div>

      <div v-if="errorMsg" class="mb-6 px-4 py-3 rounded-xl bg-critical/10 border border-critical/40 text-critical font-semibold">⚠️ {{ errorMsg }}</div>

      <div v-if="isLoading" class="flex items-center justify-center h-96 font-semibold text-[color:var(--text-faint)] uppercase tracking-widest">Memuat analitik…</div>

      <template v-else-if="overview">
        <!-- KPI Cards -->
        <div class="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-7">
          <div v-tilt class="tilt-card kpi anim-pop d-1 p-5" style="--accent:#3E92CC">
            <span class="tilt-shine"></span>
            <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-1">Total Unit</p>
            <p class="font-display text-4xl font-bold">{{ overview.total_units }}</p>
          </div>
          <div v-tilt class="tilt-card kpi anim-pop d-2 p-5" style="--accent:#1FA971">
            <span class="tilt-shine"></span>
            <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-1">Rata-rata Health</p>
            <p class="font-display text-4xl font-bold text-healthy">{{ overview.avg_health }}%</p>
          </div>
          <div v-tilt class="tilt-card kpi anim-pop d-3 p-5" style="--accent:#E0A106">
            <span class="tilt-shine"></span>
            <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-1">Rata-rata Risk</p>
            <p class="font-display text-4xl font-bold text-warning">{{ overview.avg_risk_score }}</p>
          </div>
          <div v-tilt class="tilt-card kpi anim-pop d-4 p-5" style="--accent:#E0413E"
            :class="overview.units_at_risk > 0 ? 'anim-glow' : ''">
            <span class="tilt-shine"></span>
            <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-1">Unit Berisiko</p>
            <p class="font-display text-4xl font-bold text-critical">{{ overview.units_at_risk }}</p>
          </div>
        </div>

        <!-- Fleet charts -->
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-7">
          <div class="panel p-6 anim-up d-2">
            <h2 class="font-display text-xl font-bold uppercase tracking-wide mb-4 pb-2 border-b border-[color:var(--border)]">Distribusi Status Armada</h2>
            <div class="h-64"><canvas id="statusPie"></canvas></div>
          </div>
          <div class="panel p-6 anim-up d-3">
            <h2 class="font-display text-xl font-bold uppercase tracking-wide mb-4 pb-2 border-b border-[color:var(--border)]">Distribusi Tingkat Risiko</h2>
            <div class="h-64"><canvas id="riskBar"></canvas></div>
          </div>
        </div>

        <!-- Unit list + detail -->
        <div class="grid grid-cols-1 lg:grid-cols-4 gap-6 items-start">
          <!-- Unit list -->
          <div class="lg:col-span-1 panel p-4 lg:sticky lg:top-0">
            <h2 class="font-display text-lg font-bold uppercase tracking-wide mb-4 pb-2 border-b border-[color:var(--border)]">Pilih Unit</h2>
            <div class="space-y-2">
              <button v-for="u in pagedUnitList" :key="u.id" @click="selectUnit(u.id)"
                :class="selectedUnitId === u.id ? 'border-amber bg-amber/10' : 'border-[color:var(--border)] hover:border-steel hover:bg-[color:var(--surface-2)]'"
                class="w-full text-left p-3 rounded-lg border transition-all">
                <div class="flex justify-between items-center">
                  <span class="font-mono font-semibold text-sm">{{ u.code }}</span>
                  <span class="px-2 py-0.5 rounded-full text-[9px] font-bold text-white" :style="{ backgroundColor: riskColor(u.risk_level) }">{{ u.risk_level }}</span>
                </div>
                <div class="flex justify-between items-center mt-1.5">
                  <span class="text-[11px] text-[color:var(--text-muted)] truncate">{{ u.jenis_alat_berat_nama }}</span>
                  <span class="text-[11px] font-mono font-semibold">Risk {{ u.risk_score }}</span>
                </div>
              </button>
            </div>
            <div v-if="unitListTotalPages > 1" class="flex items-center justify-between mt-3 pt-3 border-t border-[color:var(--border)]">
              <span class="text-[11px] font-medium text-[color:var(--text-faint)]">Hal {{ unitListPage }} / {{ unitListTotalPages }}</span>
              <div class="flex gap-1.5">
                <button @click="unitListPage = Math.max(1, unitListPage - 1)" :disabled="unitListPage === 1" class="mini-pg">‹</button>
                <button @click="unitListPage = Math.min(unitListTotalPages, unitListPage + 1)" :disabled="unitListPage === unitListTotalPages" class="mini-pg">›</button>
              </div>
            </div>
          </div>

          <!-- Detail -->
          <div v-if="analysis" class="lg:col-span-3 space-y-6">
            <!-- Header unit -->
            <div class="rounded-xl bg-steel-gradient text-white p-5 flex justify-between items-center flex-wrap gap-3">
              <div>
                <p class="text-amber font-mono font-semibold text-xs">{{ analysis.unit.code }}</p>
                <p class="font-display font-bold text-xl uppercase tracking-wide">{{ analysis.unit.jenis_alat_berat_nama }}</p>
              </div>
              <div class="flex gap-4 items-center">
                <div class="text-center">
                  <p class="text-[9px] text-graphite-300 uppercase tracking-wider">Health</p>
                  <p class="text-2xl font-display font-bold">{{ analysis.unit.health }}%</p>
                </div>
                <div class="px-3 py-2 rounded-lg text-white font-bold text-sm" :style="{ backgroundColor: riskColor(analysis.risk_level) }">
                  {{ analysis.risk_level }}
                </div>
              </div>
            </div>

            <!-- 3D + Gauge + Radar -->
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
              <div class="panel p-6 anim-up d-1">
                <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-3 pb-2 border-b border-[color:var(--border)]">Visual 3D Unit</h3>
                <div class="viewer-3d rounded-xl border border-[color:var(--border)] bg-steel-gradient relative cursor-grab active:cursor-grabbing overflow-hidden h-48">
                  <model-viewer
                    v-if="analysis.unit"
                    :key="analysis.unit.id"
                    :src="resolveModel(analysis.unit.model3d_url, analysis.unit.jenis_alat_berat_nama)"
                    alt="Model 3D unit alat berat"
                    camera-controls auto-rotate auto-rotate-delay="0" rotation-per-second="35deg"
                    shadow-intensity="1.4" exposure="1.1" environment-image="neutral" interaction-prompt="none"
                    class="w-full h-full outline-none" style="background-color:transparent;"
                  ></model-viewer>
                  <div v-else class="w-full h-full flex items-center justify-center text-graphite-300 font-medium text-sm">Model 3D tidak tersedia</div>
                  <div class="absolute top-2 left-2 bg-steel/90 text-white text-[8px] font-semibold px-2 py-0.5 rounded-full pointer-events-none">● LIVE 3D</div>
                  <div class="absolute bottom-2 right-2 bg-graphite-900/80 text-graphite-100 text-[8px] font-medium px-2 py-0.5 rounded-full pointer-events-none">DRAG 360°</div>
                </div>
              </div>

              <div class="panel p-6">
                <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-3 pb-2 border-b border-[color:var(--border)]">Risk Score</h3>
                <div class="h-48 relative">
                  <canvas id="riskGauge"></canvas>
                  <div class="absolute inset-0 flex flex-col items-center justify-end pb-2 pointer-events-none">
                    <span class="text-5xl font-display font-bold">{{ analysis.risk_score }}</span>
                    <span class="text-xs text-[color:var(--text-muted)] uppercase tracking-wider">/ 100</span>
                  </div>
                </div>
              </div>
              <div class="panel p-6">
                <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-3 pb-2 border-b border-[color:var(--border)]">Kesehatan Komponen</h3>
                <div class="h-48"><canvas id="radar"></canvas></div>
              </div>
            </div>

            <!-- Sensor readings -->
            <div class="panel p-6">
              <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-4 pb-2 border-b border-[color:var(--border)]">Telemetri Sensor Real-Time</h3>
              <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
                <div v-for="s in [
                  { label: 'Suhu Mesin', val: analysis.sensor_readings.suhu_mesin + '°C', danger: analysis.sensor_readings.suhu_mesin > 105 },
                  { label: 'Vibrasi', val: analysis.sensor_readings.vibration + ' g', danger: analysis.sensor_readings.vibration > 6 },
                  { label: 'Tekanan Oli', val: analysis.sensor_readings.tekanan_oli + ' bar', danger: analysis.sensor_readings.tekanan_oli < 3 },
                  { label: 'RPM', val: analysis.sensor_readings.rpm, danger: false },
                  { label: 'Fuel Level', val: analysis.sensor_readings.fuel_level + '%', danger: analysis.sensor_readings.fuel_level < 20 },
                  { label: 'Partikel Oli', val: 'ISO ' + analysis.sensor_readings.oil_particle_iso, danger: analysis.sensor_readings.oil_particle_iso > 19 },
                  { label: 'Emisi Akustik', val: analysis.sensor_readings.acoustic_db + ' dB', danger: analysis.sensor_readings.acoustic_db > 90 },
                  { label: 'Jam Operasi', val: analysis.sensor_readings.jam_operasi + ' h', danger: false },
                ]" :key="s.label" :class="s.danger ? 'cell cell-danger' : 'cell'">
                  <p class="cell-label">{{ s.label }}</p>
                  <p class="cell-val text-lg">{{ s.val }}</p>
                </div>
              </div>
            </div>

            <!-- Time-series -->
            <div class="panel p-6">
              <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-4 pb-2 border-b border-[color:var(--border)]">Tren Sensor 24 Jam Terakhir</h3>
              <div class="h-72"><canvas id="history"></canvas></div>
            </div>

            <!-- Telemetri lengkap -->
            <div class="panel p-6">
              <div class="flex justify-between items-center mb-4 pb-2 border-b border-[color:var(--border)] flex-wrap gap-2">
                <h3 class="font-display text-lg font-bold uppercase tracking-wide">Telemetri On-Board (VIMS / KOMTRAX)</h3>
                <span class="badge text-white" :style="{ backgroundColor: statusLabelColor(analysis.telemetry.status_label), borderColor: statusLabelColor(analysis.telemetry.status_label) }">
                  STATUS: {{ analysis.telemetry.status_label }}
                </span>
              </div>

              <!-- ECM / VIMS -->
              <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-faint)] mb-2">⚙️ ECM / VIMS — Sensor Fisik</p>
              <div class="grid grid-cols-2 md:grid-cols-4 gap-3 mb-6">
                <div v-for="s in [
                  { l: 'Eng Coolant', v: analysis.telemetry.eng_coolant_temp_c + '°C', d: analysis.telemetry.eng_coolant_temp_c > 110 },
                  { l: 'Eng Oil Press', v: analysis.telemetry.eng_oil_press_psi + ' PSI', d: analysis.telemetry.eng_oil_press_psi < 25 },
                  { l: 'Eng RPM', v: analysis.telemetry.eng_rpm, d: false },
                  { l: 'Eng Load', v: analysis.telemetry.eng_load_pct + '%', d: false },
                  { l: 'Hyd Pump Press', v: analysis.telemetry.hyd_pump_press_psi + ' PSI', d: false },
                  { l: 'Hyd Oil Temp', v: analysis.telemetry.hyd_oil_temp_c + '°C', d: analysis.telemetry.hyd_oil_temp_c > 100 },
                  { l: 'Trans Oil Temp', v: analysis.telemetry.trans_oil_temp_c + '°C', d: analysis.telemetry.trans_oil_temp_c > 110 },
                  { l: 'Torque Conv Temp', v: analysis.telemetry.torque_converter_temp_c + '°C', d: analysis.telemetry.torque_converter_temp_c > 115 },
                  { l: 'Final Drive Temp', v: analysis.telemetry.final_drive_temp_c + '°C', d: analysis.telemetry.final_drive_temp_c > 105 },
                  { l: 'Brake Cooling', v: analysis.telemetry.brake_cooling_temp_c + '°C', d: analysis.telemetry.brake_cooling_temp_c > 95 },
                  { l: 'Battery', v: analysis.telemetry.battery_voltage + ' V', d: analysis.telemetry.battery_voltage < 23 },
                  { l: 'Idle Ratio', v: (analysis.telemetry.idle_time_ratio * 100).toFixed(0) + '%', d: false },
                ]" :key="s.l" :class="s.d ? 'cell cell-danger' : 'cell'">
                  <p class="cell-label">{{ s.l }}</p>
                  <p class="cell-val text-base">{{ s.v }}</p>
                </div>
              </div>

              <!-- FMS + CMMS + LIMS -->
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                <div>
                  <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-faint)] mb-2">🚚 FMS / Dispatch & 🔧 CMMS</p>
                  <div class="grid grid-cols-2 gap-3">
                    <div class="cell !text-left"><p class="cell-label">Component Type</p><p class="cell-val text-sm">{{ analysis.telemetry.component_type }}</p></div>
                    <div class="cell !text-left"><p class="cell-label">Operator</p><p class="cell-val text-sm">{{ analysis.telemetry.operator_id }}</p></div>
                    <div class="cell !text-left"><p class="cell-label">Payload</p><p class="cell-val text-sm">{{ analysis.telemetry.payload_tonnage }} t</p></div>
                    <div class="cell !text-left"><p class="cell-label">Ambient Temp</p><p class="cell-val text-sm">{{ analysis.telemetry.ambient_temp_c }}°C</p></div>
                    <div class="cell !text-left"><p class="cell-label">Hour Meter</p><p class="cell-val text-sm">{{ analysis.telemetry.hour_meter_actual.toLocaleString() }} HM</p></div>
                    <div class="cell !text-left"><p class="cell-label">Design Life</p><p class="cell-val text-sm">{{ analysis.telemetry.design_life_hm.toLocaleString() }} HM</p></div>
                    <div class="cell !text-left"><p class="cell-label">Component Age</p><p class="cell-val text-sm">{{ analysis.telemetry.component_age_hm.toLocaleString() }} HM</p></div>
                    <div class="cell !text-left"><p class="cell-label">Remanufactured</p><p class="cell-val text-sm">{{ analysis.telemetry.is_remanufactured ? 'YA' : 'TIDAK' }}</p></div>
                    <div class="!text-left col-span-2 rounded-[10px] p-2.5 border"
                      :class="analysis.telemetry.fault_code_severity >= 3 ? 'cell-danger' : analysis.telemetry.fault_code_severity >= 2 ? 'cell-warn' : 'cell'">
                      <p class="cell-label">Fault Code Severity (DTC)</p>
                      <p class="cell-val text-sm">Level {{ analysis.telemetry.fault_code_severity }} / 4</p>
                    </div>
                  </div>
                </div>

                <!-- LIMS -->
                <div>
                  <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-faint)] mb-2">🧪 LIMS — Analisis Pelumas</p>
                  <div class="h-40 mb-3"><canvas id="labMetals"></canvas></div>
                  <div class="grid grid-cols-3 gap-2">
                    <div class="cell"><p class="cell-label">Viskositas 100C</p><p class="cell-val text-sm">{{ analysis.telemetry.lab_viscosity_100c }}</p></div>
                    <div :class="analysis.telemetry.lab_water_content_pct > 0.5 ? 'cell cell-danger' : 'cell'"><p class="cell-label">Water %</p><p class="cell-val text-sm">{{ analysis.telemetry.lab_water_content_pct }}</p></div>
                    <div :class="analysis.telemetry.lab_soot_pct > 3 ? 'cell cell-warn' : 'cell'"><p class="cell-label">Soot %</p><p class="cell-val text-sm">{{ analysis.telemetry.lab_soot_pct }}</p></div>
                  </div>
                </div>
              </div>

              <!-- Engineer generated -->
              <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-faint)] mb-2">🧮 Engineer Generated Features</p>
              <div class="grid grid-cols-3 gap-3">
                <div class="rounded-[10px] p-3 text-center text-white" style="background:#3E92CC">
                  <p class="text-[9px] font-semibold uppercase tracking-wider text-white/80">Delta Eng Temp</p>
                  <p class="text-xl font-display font-bold">{{ analysis.telemetry.delta_eng_temp }}°C</p>
                </div>
                <div class="rounded-[10px] p-3 text-center text-white" :style="{ backgroundColor: statusLabelColor(analysis.telemetry.status_label) }">
                  <p class="text-[9px] font-semibold uppercase tracking-wider text-white/80">Status Label</p>
                  <p class="text-xl font-display font-bold">{{ analysis.telemetry.status_label }}</p>
                </div>
                <div class="rounded-[10px] p-3 text-center bg-steel-gradient text-white">
                  <p class="text-[9px] font-semibold uppercase tracking-wider text-graphite-300">RUL (Telemetri)</p>
                  <p class="text-xl font-display font-bold text-amber">{{ fmtHours(analysis.telemetry.rul_hours) }}</p>
                </div>
              </div>
            </div>

            <!-- Prediksi AI — XGBoost + LSTM + Digital Twin (sesuai /predict) -->
            <div class="panel p-6">
              <div class="flex justify-between items-center mb-4 pb-2 border-b border-[color:var(--border)] flex-wrap gap-2">
                <h3 class="font-display text-lg font-bold uppercase tracking-wide">Prediksi AI — Anomali &amp; RUL</h3>
                <div class="flex items-center gap-2 flex-wrap">
                  <span class="badge" :class="analysis.prediction.model_agreement ? 'bg-healthy/15 border-healthy/40 text-healthy' : 'bg-warning/15 border-warning/40 text-warning'">
                    {{ analysis.prediction.model_agreement ? 'Model Sepakat' : 'Model Konflik' }}
                  </span>
                  <span class="text-[10px] font-mono text-[color:var(--text-faint)]">{{ analysis.prediction.equipment_type }} · {{ analysis.prediction.latency_ms }}ms</span>
                </div>
              </div>

              <!-- Ringkasan model -->
              <div class="grid grid-cols-1 md:grid-cols-4 gap-3 mb-5">
                <div class="rounded-xl p-4 border" :style="{ borderColor: rulColor(analysis.prediction.xgb_anomaly_label) + '55', background: rulColor(analysis.prediction.xgb_anomaly_label) + '12' }">
                  <p class="cell-label">XGBoost Anomaly</p>
                  <p class="font-display text-2xl font-bold mt-0.5" :style="{ color: rulColor(analysis.prediction.xgb_anomaly_label) }">{{ analysis.prediction.xgb_anomaly_label }}</p>
                  <p class="text-[11px] text-[color:var(--text-faint)] mt-0.5">Kelas {{ analysis.prediction.xgb_anomaly_class }} / 2</p>
                </div>
                <div class="rounded-xl p-4 border border-[color:var(--border)] bg-[color:var(--surface-2)]">
                  <p class="cell-label">LSTM RUL (Sistem)</p>
                  <p class="font-display text-2xl font-bold mt-0.5">{{ analysis.prediction.lstm_rul_hours }} <span class="text-sm font-normal text-[color:var(--text-muted)]">jam</span></p>
                  <p class="text-[11px] text-[color:var(--text-faint)] mt-0.5">± {{ analysis.prediction.rul_uncertainty }} jam (uncertainty)</p>
                </div>
                <div class="rounded-xl p-4 border" :style="{ borderColor: rulColor(analysis.prediction.risk_level) + '55', background: rulColor(analysis.prediction.risk_level) + '12' }">
                  <p class="cell-label">Risk Level (Final)</p>
                  <p class="font-display text-2xl font-bold mt-0.5" :style="{ color: rulColor(analysis.prediction.risk_level) }">{{ analysis.prediction.risk_level }}</p>
                  <p class="text-[11px] text-[color:var(--text-faint)] mt-0.5">Kelas {{ analysis.prediction.risk_class }} / 2</p>
                </div>
                <div class="rounded-xl p-4 border" :class="analysis.prediction.drift_status.drift_detected ? 'border-warning/40 bg-warning/10' : 'border-[color:var(--border)] bg-[color:var(--surface-2)]'">
                  <p class="cell-label">Feature Drift</p>
                  <p class="font-display text-2xl font-bold mt-0.5" :class="analysis.prediction.drift_status.drift_detected ? 'text-warning' : 'text-healthy'">
                    {{ analysis.prediction.drift_status.drift_detected ? 'TERDETEKSI' : 'STABIL' }}
                  </p>
                  <p class="text-[11px] text-[color:var(--text-faint)] mt-0.5">z-max {{ analysis.prediction.drift_status.max_z_score }} · {{ analysis.prediction.drift_status.n_drifted }} fitur</p>
                </div>
              </div>

              <!-- 3 komponen paling mendesak -->
              <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 mb-5">
                <div v-for="c in urgentComponents" :key="c.label" class="rounded-xl p-4 border"
                  :style="{ borderColor: rulTone(c.hours) + '66', background: rulTone(c.hours) + '14' }">
                  <div class="flex items-center justify-between mb-1">
                    <span class="text-[10px] font-bold uppercase tracking-wide" :style="{ color: rulTone(c.hours) }">{{ rulToneLabel(c.hours) }}</span>
                  </div>
                  <p class="font-semibold text-sm leading-tight">{{ c.label }}</p>
                  <p class="font-display text-2xl font-bold mt-1" :style="{ color: rulTone(c.hours) }">{{ fmtHours(c.hours) }}</p>
                </div>
              </div>

              <!-- RUL per komponen (7 LSTM) -->
              <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-faint)] mb-2">RUL per Komponen (LSTM)</p>
              <div class="h-72"><canvas id="rulComponents"></canvas></div>

              <!-- Digital twin -->
              <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-faint)] mt-5 mb-2">Digital Twin (Physics-based)</p>
              <div class="grid grid-cols-3 gap-3">
                <div v-for="d in digitalTwins" :key="d.label" class="cell">
                  <p class="cell-label">{{ d.label }}</p>
                  <p class="cell-val text-lg" :style="{ color: rulTone(d.hours) }">{{ d.hours }} <span class="text-xs font-normal">jam</span></p>
                </div>
              </div>

              <!-- Daftar fitur drift -->
              <div v-if="analysis.prediction.drift_status.drift_detected" class="mt-4 px-4 py-3 rounded-lg bg-warning/10 border border-warning/40">
                <span class="font-semibold text-warning text-sm">⚠ Fitur ter-drift:</span>
                <span class="font-mono text-xs text-[color:var(--text-muted)] ml-1">{{ analysis.prediction.drift_status.drifted_features.join(', ') }}</span>
              </div>
            </div>

            <!-- Parameter Operasional & Lingkungan -->
            <div class="panel p-6">
              <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-4 pb-2 border-b border-[color:var(--border)]">Parameter Operasional &amp; Lingkungan</h3>
              <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-5 gap-3">
                <div v-for="f in operationalFields" :key="f.k" class="cell">
                  <p class="cell-label">{{ f.l }}</p>
                  <p class="cell-val text-base">{{ analysis.operational[f.k] }}{{ f.u }}</p>
                </div>
                <div class="cell">
                  <p class="cell-label">Oil Change Flag</p>
                  <p class="cell-val text-base">{{ analysis.operational.oil_change_flag ? 'YA' : 'TIDAK' }}</p>
                </div>
              </div>
            </div>

            <!-- RUL + SHAP -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div class="panel p-6">
                <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-4 pb-2 border-b border-[color:var(--border)]">Prediksi Sisa Umur (RUL)</h3>
                <div class="py-2 space-y-3">
                  <!-- Komponen terlemah -->
                  <div class="rounded-xl border border-[color:var(--border)] bg-[color:var(--surface-2)] p-4">
                    <p class="cell-label">Komponen Terlemah</p>
                    <p class="font-display text-xl font-bold mt-0.5 leading-tight">{{ analysis.rul_prediction.component }}</p>
                  </div>

                  <!-- Bento responsif: estimasi · rentang · confidence -->
                  <div class="grid grid-cols-2 lg:grid-cols-4 gap-3">
                    <!-- Estimasi sebelum kegagalan (hero) -->
                    <div class="col-span-2 rounded-xl bg-steel-gradient text-white p-4 flex flex-col justify-center shadow-elev-sm">
                      <p class="text-[10px] text-graphite-300 uppercase tracking-wider">Estimasi Sebelum Kegagalan</p>
                      <p class="text-3xl md:text-4xl font-display font-bold text-amber mt-1 leading-none">{{ fmtHours(analysis.rul_prediction.hours_remaining) }}</p>
                      <p class="text-[10px] text-graphite-300 mt-1.5 font-mono">{{ Math.round(analysis.rul_prediction.hours_remaining) }} jam total</p>
                    </div>

                    <!-- Rentang estimasi -->
                    <div class="rounded-xl border border-[color:var(--border)] bg-[color:var(--surface-2)] p-4 flex flex-col justify-center">
                      <p class="cell-label">Rentang Estimasi</p>
                      <p class="font-display text-2xl font-bold mt-1 leading-none tabular-nums">
                        {{ Math.round(analysis.rul_prediction.lower_bound) }}<span class="text-[color:var(--text-faint)]">–</span>{{ Math.round(analysis.rul_prediction.upper_bound) }}
                      </p>
                      <p class="text-[10px] text-[color:var(--text-muted)] mt-1 font-mono">jam (min–max)</p>
                    </div>

                    <!-- Confidence -->
                    <div class="rounded-xl border border-[color:var(--border)] bg-[color:var(--surface-2)] p-4 flex flex-col justify-center">
                      <p class="cell-label">Confidence</p>
                      <p class="font-display text-2xl font-bold mt-1 leading-none"
                        :style="{ color: analysis.rul_prediction.confidence >= 80 ? '#1FA971' : analysis.rul_prediction.confidence >= 60 ? '#E0A106' : '#E0413E' }">
                        {{ analysis.rul_prediction.confidence }}%
                      </p>
                      <div class="mt-2 h-1.5 rounded-full bg-[color:var(--border)] overflow-hidden">
                        <div class="h-full rounded-full transition-all duration-700"
                          :style="{ width: analysis.rul_prediction.confidence + '%', background: analysis.rul_prediction.confidence >= 80 ? '#1FA971' : analysis.rul_prediction.confidence >= 60 ? '#E0A106' : '#E0413E' }"></div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
              <div class="panel p-6">
                <h3 class="font-display text-lg font-bold uppercase tracking-wide mb-2 pb-2 border-b border-[color:var(--border)]">Faktor Penyebab (SHAP)</h3>
                <p class="text-[10px] text-[color:var(--text-muted)] mb-2">Kontribusi tiap sensor terhadap skor risiko</p>
                <div class="h-56"><canvas id="shap"></canvas></div>
              </div>
            </div>
          </div>
        </div>
      </template>
    </main>
  </div>
</template>

<style scoped>
.cell {
  background: var(--surface-2);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: .6rem;
  text-align: center;
}
.cell-danger {
  background: rgba(224,65,62,0.12) !important;
  border-color: rgba(224,65,62,0.40) !important;
  color: var(--critical) !important;
}
.cell-warn {
  background: rgba(224,161,6,0.14) !important;
  border-color: rgba(224,161,6,0.45) !important;
  color: var(--warning) !important;
}
.cell-label {
  font-size: .6rem; font-weight: 700;
  text-transform: uppercase; letter-spacing: .05em;
  color: var(--text-faint);
}
.cell-danger .cell-label, .cell-warn .cell-label { color: inherit; opacity: .85; }
.cell-val { font-weight: 700; margin-top: .15rem; color: var(--text); }
.cell-danger .cell-val, .cell-warn .cell-val { color: inherit; }
</style>
