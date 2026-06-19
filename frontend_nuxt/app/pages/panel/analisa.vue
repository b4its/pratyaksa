<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'

// Muat web component <model-viewer> (Google) untuk render GLB
useHead({
  script: [
    { type: 'module', src: 'https://ajax.googleapis.com/ajax/libs/model-viewer/3.3.0/model-viewer.min.js' },
  ],
})

// --- SIDEBAR ---
interface MenuItem { name: string; path: string; icon: string }
const menuItems = ref<MenuItem[]>([
  { name: 'Dashboard', path: '/panel/dashboard', icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><rect width="7" height="9" x="3" y="3"/><rect width="7" height="5" x="14" y="3"/><rect width="7" height="9" x="14" y="12"/><rect width="7" height="5" x="3" y="16"/></svg>` },
  { name: 'Jenis Alat Berat', path: '/panel/jenis_alat_berat', icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><path d="M10 17h4V5H2v12h3"/><path d="M20 17h2v-9l-2.5-3.5H14v12h3"/><path d="M14 6h4.5"/><circle cx="18.5" cy="17.5" r="2.5"/><circle cx="5.5" cy="17.5" r="2.5"/></svg>` },
  { name: 'Unit Tambang', path: '/panel/unit_tambang', icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 12 12 17 22 12"/><polyline points="2 17 12 22 22 17"/></svg>` },
  { name: 'Analisa Kerusakan', path: '/panel/analisa', icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>` },
  { name: 'Kembali', path: '../', icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><path d="m12 19-7-7 7-7"/><path d="M19 12H5"/></svg>` },
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

// --- STATE ---
const overview = ref<Overview | null>(null)
const analysis = ref<UnitAnalysis | null>(null)
const selectedUnitId = ref<string | null>(null)
const isLoading = ref(true)
const errorMsg = ref('')
const autoRefresh = ref(true)
const lastUpdate = ref('')

const api = useApi()
let ChartLib: any = null
let refreshTimer: any = null

// chart instances
const charts: Record<string, any> = {}

// --- HELPERS ---
const riskColor = (level: string) => ({
  LOW: '#34d399', MEDIUM: '#facc15', HIGH: '#fb923c', CRITICAL: '#f87171',
}[level] || '#9ca3af')
const statusColor = (s: string) => ({
  SEHAT: '#34d399', WARNING: '#facc15', CRITICAL: '#f87171', RUSAK: '#9ca3af',
}[s] || '#9ca3af')

const selectedUnit = computed(() =>
  overview.value?.units.find(u => u.id === selectedUnitId.value) || null)

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
}

const selectUnit = async (id: string) => {
  selectedUnitId.value = id
  await fetchAnalysis()
  await nextTick()
  renderUnitCharts()
}

// --- CHART RENDERING ---
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

const neoFont = { family: 'Space Mono', weight: 'bold' as const }

const renderOverviewCharts = () => {
  if (!overview.value) return

  // Status pie (doughnut)
  const sd = overview.value.status_distribution
  upsertChart('statusPie', 'statusPie', {
    type: 'doughnut',
    data: {
      labels: sd.map(s => s.label),
      datasets: [{
        data: sd.map(s => s.count),
        backgroundColor: sd.map(s => statusColor(s.label)),
        borderColor: '#000', borderWidth: 3,
      }],
    },
    options: {
      responsive: true, maintainAspectRatio: false, cutout: '55%',
      plugins: { legend: { position: 'bottom', labels: { font: neoFont, boxWidth: 14 } } },
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
        borderColor: '#000', borderWidth: 3,
      }],
    },
    options: {
      responsive: true, maintainAspectRatio: false,
      plugins: { legend: { display: false } },
      scales: {
        x: { ticks: { font: neoFont, color: '#000' }, grid: { display: false }, border: { color: '#000', width: 3 } },
        y: { beginAtZero: true, ticks: { font: neoFont, color: '#000', stepSize: 1 }, grid: { color: '#E5E7EB' }, border: { color: '#000', width: 3 } },
      },
    },
  })
}

const renderUnitCharts = () => {
  if (!analysis.value) return

  // Risk gauge (doughnut as gauge)
  const rs = analysis.value.risk_score
  upsertChart('riskGauge', 'riskGauge', {
    type: 'doughnut',
    data: {
      labels: ['Risk', 'Sisa'],
      datasets: [{
        data: [rs, 100 - rs],
        backgroundColor: [riskColor(analysis.value.risk_level), '#e5e7eb'],
        borderColor: '#000', borderWidth: 3, circumference: 180, rotation: 270,
      }],
    },
    options: {
      responsive: true, maintainAspectRatio: false, cutout: '70%',
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
        backgroundColor: 'rgba(51,168,255,0.25)',
        borderColor: '#33A8FF', borderWidth: 3,
        pointBackgroundColor: '#000', pointBorderColor: '#000', pointRadius: 4,
      }],
    },
    options: {
      responsive: true, maintainAspectRatio: false,
      plugins: { legend: { display: false } },
      scales: {
        r: {
          min: 0, max: 100,
          ticks: { stepSize: 25, font: { family: 'Space Mono', size: 9 }, color: '#666', backdropColor: 'transparent' },
          grid: { color: '#d1d5db' }, angleLines: { color: '#d1d5db' },
          pointLabels: { font: neoFont, color: '#000' },
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
        { label: 'Suhu (°C)', data: hist.map(h => h.suhu_mesin), borderColor: '#f87171', backgroundColor: '#f87171', borderWidth: 2, pointRadius: 0, tension: 0.3, yAxisID: 'y' },
        { label: 'Akustik (dB)', data: hist.map(h => h.acoustic), borderColor: '#fb923c', backgroundColor: '#fb923c', borderWidth: 2, pointRadius: 0, tension: 0.3, yAxisID: 'y' },
        { label: 'Vibrasi (g)', data: hist.map(h => h.vibration), borderColor: '#33A8FF', backgroundColor: '#33A8FF', borderWidth: 2, pointRadius: 0, tension: 0.3, yAxisID: 'y1' },
        { label: 'Tekanan Oli (bar)', data: hist.map(h => h.tekanan_oli), borderColor: '#34d399', backgroundColor: '#34d399', borderWidth: 2, pointRadius: 0, tension: 0.3, yAxisID: 'y1' },
      ],
    },
    options: {
      responsive: true, maintainAspectRatio: false, interaction: { mode: 'index', intersect: false },
      plugins: { legend: { position: 'bottom', labels: { font: { family: 'Space Mono', size: 10, weight: 'bold' }, boxWidth: 12 } } },
      scales: {
        x: { ticks: { font: { family: 'Space Mono', size: 9 }, color: '#000', maxTicksLimit: 8 }, grid: { display: false }, border: { color: '#000', width: 3 } },
        y: { position: 'left', ticks: { font: { family: 'Space Mono', size: 9 }, color: '#000' }, grid: { color: '#E5E7EB' }, border: { color: '#000', width: 3 } },
        y1: { position: 'right', ticks: { font: { family: 'Space Mono', size: 9 }, color: '#000' }, grid: { display: false }, border: { color: '#000', width: 3 } },
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
        backgroundColor: shap.map(s => s.value >= 0 ? '#f87171' : '#34d399'),
        borderColor: '#000', borderWidth: 2,
      }],
    },
    options: {
      indexAxis: 'y', responsive: true, maintainAspectRatio: false,
      plugins: { legend: { display: false }, tooltip: { callbacks: { label: (c: any) => `${c.raw >= 0 ? '+' : ''}${c.raw} ke risiko` } } },
      scales: {
        x: { ticks: { font: { family: 'Space Mono', size: 9 }, color: '#000' }, grid: { color: '#E5E7EB' }, border: { color: '#000', width: 3 } },
        y: { ticks: { font: { family: 'Space Mono', size: 10, weight: 'bold' }, color: '#000' }, grid: { display: false }, border: { color: '#000', width: 3 } },
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
        backgroundColor: ['#f87171', '#fb923c', '#33A8FF', '#a78bfa'],
        borderColor: '#000', borderWidth: 3,
      }],
    },
    options: {
      responsive: true, maintainAspectRatio: false,
      plugins: { legend: { display: false } },
      scales: {
        x: { ticks: { font: { family: 'Space Mono', size: 9, weight: 'bold' }, color: '#000' }, grid: { display: false }, border: { color: '#000', width: 3 } },
        y: { beginAtZero: true, ticks: { font: { family: 'Space Mono', size: 9 }, color: '#000' }, grid: { color: '#E5E7EB' }, border: { color: '#000', width: 3 } },
      },
    },
  })
}

const renderAllCharts = () => { renderOverviewCharts(); renderUnitCharts() }

// --- LIFECYCLE ---
onMounted(async () => {
  const auth = useAuth()
  auth.initAuth()
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
  }, 5000)
})

onUnmounted(() => {
  if (refreshTimer) clearInterval(refreshTimer)
  Object.values(charts).forEach(c => c?.destroy())
})

const fmtHours = (h: number) => {
  if (h >= 24) return `${Math.floor(h / 24)} hari ${Math.round(h % 24)} jam`
  return `${Math.round(h)} jam`
}

const statusLabelColor = (s: string) => ({
  NORMAL: '#34d399', WARNING: '#facc15', CRITICAL: '#f87171',
}[s] || '#9ca3af')
</script>

<template>
  <div class="flex min-h-screen bg-[#F1F1F1] font-mono text-black">
    <!-- Sidebar -->
    <aside class="w-72 border-r-4 border-black bg-white p-6 flex flex-col justify-between z-10 shrink-0">
      <div>
        <div class="mb-10 flex justify-center">
          <div class="bg-miningYellow border-4 border-black p-4 shadow-neo w-24 h-24 flex items-center justify-center rounded-xl">
            <span class="text-5xl font-black italic">V</span>
          </div>
        </div>
        <nav class="space-y-4">
          <NuxtLink v-for="item in menuItems" :key="item.name" :to="item.path" @click="activeMenu = item.name"
            :class="['w-full flex items-center gap-3 p-3 border-2 font-bold transition-all cursor-pointer', activeMenu === item.name ? 'border-black bg-miningYellow shadow-neoHover' : 'border-transparent hover:border-black hover:bg-white hover:shadow-neoHover']">
            <div v-if="activeMenu === item.name" class="p-1 border-2 border-black bg-white shrink-0"><div class="w-2.5 h-2.5 bg-black"></div></div>
            <div v-else class="w-2.5 h-2.5 border-2 border-black ml-1.5 shrink-0"></div>
            <div class="flex items-center gap-3 w-full">
              <div class="flex items-center justify-center shrink-0" v-html="item.icon"></div>
              <span class="truncate">{{ item.name }}</span>
            </div>
          </NuxtLink>
        </nav>
      </div>
    </aside>

    <!-- Main -->
    <main class="flex-1 p-8 overflow-y-auto">
      <header class="flex justify-between items-start mb-8 flex-wrap gap-4">
        <div>
          <h1 class="text-5xl md:text-6xl font-black uppercase tracking-tighter leading-none">Analisa Kerusakan</h1>
          <p class="mt-2 font-bold text-gray-600 bg-white border-b-2 border-black inline-block">Monitoring kondisi & prediksi kegagalan armada secara real-time.</p>
        </div>
        <div class="flex items-center gap-3">
          <button @click="autoRefresh = !autoRefresh"
            :class="autoRefresh ? 'bg-emerald-400' : 'bg-white'"
            class="p-3 border-4 border-black font-black text-xs shadow-neoHover flex items-center gap-2 lift">
            <span class="w-3 h-3 rounded-full border-2 border-black" :class="autoRefresh ? 'bg-black anim-live' : 'bg-gray-300'"></span>
            {{ autoRefresh ? 'LIVE' : 'PAUSED' }}
          </button>
          <div class="p-2 border-4 border-black bg-white text-[10px] font-bold shadow-neoHover">
            Update:<br>{{ lastUpdate || '—' }}
          </div>
        </div>
      </header>

      <div v-if="errorMsg" class="mb-6 p-4 bg-neoRed text-white border-4 border-black font-bold shadow-neo">⚠️ {{ errorMsg }}</div>

      <div v-if="isLoading" class="flex items-center justify-center h-96 font-black text-gray-400 uppercase tracking-widest">Memuat analitik...</div>

      <template v-else-if="overview">
        <!-- KPI Cards -->
        <div class="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
          <div v-tilt class="tilt-card anim-pop d-1 relative overflow-hidden bg-white border-4 border-black p-5 shadow-neoHover">
            <span class="tilt-shine"></span>
            <p class="text-[10px] font-black uppercase text-gray-500 mb-1">Total Unit</p>
            <p class="text-4xl font-black">{{ overview.total_units }}</p>
          </div>
          <div v-tilt class="tilt-card anim-pop d-2 relative overflow-hidden bg-emerald-400 border-4 border-black p-5 shadow-neoHover">
            <span class="tilt-shine"></span>
            <p class="text-[10px] font-black uppercase text-emerald-900 mb-1">Rata-rata Health</p>
            <p class="text-4xl font-black">{{ overview.avg_health }}%</p>
          </div>
          <div v-tilt class="tilt-card anim-pop d-3 relative overflow-hidden bg-miningYellow border-4 border-black p-5 shadow-neoHover">
            <span class="tilt-shine"></span>
            <p class="text-[10px] font-black uppercase mb-1">Rata-rata Risk</p>
            <p class="text-4xl font-black">{{ overview.avg_risk_score }}</p>
          </div>
          <div v-tilt class="tilt-card anim-pop d-4 relative overflow-hidden bg-neoRed text-white border-4 border-black p-5 shadow-neoHover"
            :class="overview.units_at_risk > 0 ? 'anim-glow' : ''">
            <span class="tilt-shine"></span>
            <p class="text-[10px] font-black uppercase text-red-200 mb-1">Unit Berisiko</p>
            <p class="text-4xl font-black">{{ overview.units_at_risk }}</p>
          </div>
        </div>

        <!-- Fleet charts -->
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
          <div class="bg-white border-4 border-black shadow-neo p-6 anim-up d-2">
            <h2 class="text-xl font-black uppercase mb-4 border-b-4 border-black pb-2">Distribusi Status Armada</h2>
            <div class="h-64"><canvas id="statusPie"></canvas></div>
          </div>
          <div class="bg-white border-4 border-black shadow-neo p-6 anim-up d-3">
            <h2 class="text-xl font-black uppercase mb-4 border-b-4 border-black pb-2">Distribusi Tingkat Risiko</h2>
            <div class="h-64"><canvas id="riskBar"></canvas></div>
          </div>
        </div>

        <!-- Unit list + detail -->
        <div class="grid grid-cols-1 lg:grid-cols-4 gap-6">
          <!-- Unit list -->
          <div class="lg:col-span-1 bg-white border-4 border-black shadow-neo p-4 h-fit">
            <h2 class="text-lg font-black uppercase mb-4 border-b-4 border-black pb-2">Pilih Unit</h2>
            <div class="space-y-2 max-h-[600px] overflow-y-auto">
              <button v-for="u in overview.units" :key="u.id" @click="selectUnit(u.id)"
                :class="selectedUnitId === u.id ? 'bg-miningYellow border-black shadow-neoHover' : 'bg-white border-gray-300 hover:border-black'"
                class="w-full text-left p-3 border-2 transition-all">
                <div class="flex justify-between items-center">
                  <span class="font-black italic text-sm">{{ u.code }}</span>
                  <span class="px-2 py-0.5 border-2 border-black text-[9px] font-black text-white" :style="{ backgroundColor: riskColor(u.risk_level) }">{{ u.risk_level }}</span>
                </div>
                <div class="flex justify-between items-center mt-1">
                  <span class="text-[10px] font-bold text-gray-600 truncate">{{ u.jenis_alat_berat_nama }}</span>
                  <span class="text-[10px] font-black">Risk {{ u.risk_score }}</span>
                </div>
              </button>
            </div>
          </div>

          <!-- Detail -->
          <div v-if="analysis" class="lg:col-span-3 space-y-6">
            <!-- Header unit -->
            <div class="bg-black text-white border-4 border-black shadow-neo p-5 flex justify-between items-center flex-wrap gap-3">
              <div>
                <p class="text-miningYellow font-black text-xs">{{ analysis.unit.code }}</p>
                <p class="font-black text-xl uppercase">{{ analysis.unit.jenis_alat_berat_nama }}</p>
              </div>
              <div class="flex gap-3 items-center">
                <div class="text-center">
                  <p class="text-[9px] font-bold text-gray-400 uppercase">Health</p>
                  <p class="text-2xl font-black">{{ analysis.unit.health }}%</p>
                </div>
                <div class="px-3 py-2 border-2 text-black font-black text-sm" :style="{ backgroundColor: riskColor(analysis.risk_level) }">
                  {{ analysis.risk_level }}
                </div>
              </div>
            </div>

            <!-- Gauge + Radar -->
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
              <!-- 3D Model Viewer (GLB) -->
              <div class="bg-white border-4 border-black shadow-neo p-6 anim-up d-1">
                <h3 class="text-lg font-black uppercase mb-2 border-b-4 border-black pb-2">Visual 3D Unit</h3>
                <div class="viewer-3d border-4 border-black bg-gradient-to-br from-gray-100 to-gray-300 shadow-neoHover relative cursor-grab active:cursor-grabbing overflow-hidden h-48">
                  <model-viewer
                    v-if="analysis.unit.model3d_url"
                    :key="analysis.unit.id"
                    :src="analysis.unit.model3d_url"
                    alt="Model 3D unit alat berat"
                    camera-controls
                    auto-rotate
                    auto-rotate-delay="0"
                    rotation-per-second="35deg"
                    shadow-intensity="1.4"
                    exposure="1.1"
                    environment-image="neutral"
                    interaction-prompt="none"
                    class="w-full h-full outline-none"
                    style="background-color:transparent;"
                  ></model-viewer>
                  <div v-else class="w-full h-full flex items-center justify-center text-gray-400 font-bold text-sm">Model 3D tidak tersedia</div>
                  <div class="absolute top-2 left-2 bg-neoBlue text-white text-[8px] font-black px-2 py-0.5 border-2 border-black pointer-events-none">● LIVE 3D</div>
                  <div class="absolute bottom-2 right-2 bg-black text-white text-[8px] font-black px-1 pointer-events-none">DRAG 360°</div>
                </div>
              </div>

              <div class="bg-white border-4 border-black shadow-neo p-6">
                <h3 class="text-lg font-black uppercase mb-2 border-b-4 border-black pb-2">Risk Score</h3>
                <div class="h-48 relative">
                  <canvas id="riskGauge"></canvas>
                  <div class="absolute inset-0 flex flex-col items-center justify-end pb-2 pointer-events-none">
                    <span class="text-5xl font-black">{{ analysis.risk_score }}</span>
                    <span class="text-xs font-bold text-gray-500 uppercase">/ 100</span>
                  </div>
                </div>
              </div>
              <div class="bg-white border-4 border-black shadow-neo p-6">
                <h3 class="text-lg font-black uppercase mb-2 border-b-4 border-black pb-2">Kesehatan Komponen</h3>
                <div class="h-48"><canvas id="radar"></canvas></div>
              </div>
            </div>

            <!-- Sensor readings -->
            <div class="bg-white border-4 border-black shadow-neo p-6">
              <h3 class="text-lg font-black uppercase mb-4 border-b-4 border-black pb-2">Telemetri Sensor Real-Time</h3>
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
                ]" :key="s.label"
                  :class="s.danger ? 'bg-neoRed text-white' : 'bg-gray-50'"
                  class="border-2 border-black p-3 text-center shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">
                  <p class="text-[9px] font-black uppercase" :class="s.danger ? 'text-red-100' : 'text-gray-500'">{{ s.label }}</p>
                  <p class="text-lg font-black mt-1">{{ s.val }}</p>
                </div>
              </div>
            </div>

            <!-- Time-series -->
            <div class="bg-white border-4 border-black shadow-neo p-6">
              <h3 class="text-lg font-black uppercase mb-4 border-b-4 border-black pb-2">Tren Sensor 24 Jam Terakhir</h3>
              <div class="h-72"><canvas id="history"></canvas></div>
            </div>

            <!-- Telemetri lengkap 33 kolom (ECM/VIMS, FMS, CMMS, LIMS, Engineered) -->
            <div class="bg-white border-4 border-black shadow-neo p-6">
              <div class="flex justify-between items-center mb-4 border-b-4 border-black pb-2 flex-wrap gap-2">
                <h3 class="text-lg font-black uppercase">Telemetri On-Board (VIMS / KOMTRAX)</h3>
                <span class="px-3 py-1 border-2 border-black font-black text-xs text-black" :style="{ backgroundColor: statusLabelColor(analysis.telemetry.status_label) }">
                  STATUS: {{ analysis.telemetry.status_label }}
                </span>
              </div>

              <!-- ECM / VIMS sensor fisik -->
              <p class="text-[10px] font-black uppercase text-gray-400 mb-2">⚙️ ECM / VIMS — Sensor Fisik</p>
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
                ]" :key="s.l" :class="s.d ? 'bg-neoRed text-white' : 'bg-gray-50'" class="border-2 border-black p-2 text-center shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">
                  <p class="text-[9px] font-black uppercase" :class="s.d ? 'text-red-100' : 'text-gray-500'">{{ s.l }}</p>
                  <p class="text-base font-black mt-0.5">{{ s.v }}</p>
                </div>
              </div>

              <!-- FMS + CMMS + Fault -->
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                <div>
                  <p class="text-[10px] font-black uppercase text-gray-400 mb-2">🚚 FMS / Dispatch & 🔧 CMMS</p>
                  <div class="grid grid-cols-2 gap-3">
                    <div class="border-2 border-black p-2 shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">
                      <p class="text-[9px] font-black uppercase text-gray-500">Component Type</p>
                      <p class="text-sm font-black">{{ analysis.telemetry.component_type }}</p>
                    </div>
                    <div class="border-2 border-black p-2 shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">
                      <p class="text-[9px] font-black uppercase text-gray-500">Operator</p>
                      <p class="text-sm font-black">{{ analysis.telemetry.operator_id }}</p>
                    </div>
                    <div class="border-2 border-black p-2 shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">
                      <p class="text-[9px] font-black uppercase text-gray-500">Payload</p>
                      <p class="text-sm font-black">{{ analysis.telemetry.payload_tonnage }} t</p>
                    </div>
                    <div class="border-2 border-black p-2 shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">
                      <p class="text-[9px] font-black uppercase text-gray-500">Ambient Temp</p>
                      <p class="text-sm font-black">{{ analysis.telemetry.ambient_temp_c }}°C</p>
                    </div>
                    <div class="border-2 border-black p-2 shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">
                      <p class="text-[9px] font-black uppercase text-gray-500">Hour Meter</p>
                      <p class="text-sm font-black">{{ analysis.telemetry.hour_meter_actual.toLocaleString() }} HM</p>
                    </div>
                    <div class="border-2 border-black p-2 shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">
                      <p class="text-[9px] font-black uppercase text-gray-500">Design Life</p>
                      <p class="text-sm font-black">{{ analysis.telemetry.design_life_hm.toLocaleString() }} HM</p>
                    </div>
                    <div class="border-2 border-black p-2 shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">
                      <p class="text-[9px] font-black uppercase text-gray-500">Component Age</p>
                      <p class="text-sm font-black">{{ analysis.telemetry.component_age_hm.toLocaleString() }} HM</p>
                    </div>
                    <div class="border-2 border-black p-2 shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">
                      <p class="text-[9px] font-black uppercase text-gray-500">Remanufactured</p>
                      <p class="text-sm font-black">{{ analysis.telemetry.is_remanufactured ? 'YA' : 'TIDAK' }}</p>
                    </div>
                    <div class="border-2 border-black p-2 shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] col-span-2"
                      :class="analysis.telemetry.fault_code_severity >= 3 ? 'bg-neoRed text-white' : analysis.telemetry.fault_code_severity >= 2 ? 'bg-miningYellow' : 'bg-gray-50'">
                      <p class="text-[9px] font-black uppercase" :class="analysis.telemetry.fault_code_severity >= 3 ? 'text-red-100' : 'text-gray-500'">Fault Code Severity (DTC)</p>
                      <p class="text-sm font-black">Level {{ analysis.telemetry.fault_code_severity }} / 4</p>
                    </div>
                  </div>
                </div>

                <!-- LIMS lab oli -->
                <div>
                  <p class="text-[10px] font-black uppercase text-gray-400 mb-2">🧪 LIMS — Analisis Pelumas</p>
                  <div class="h-40 mb-3"><canvas id="labMetals"></canvas></div>
                  <div class="grid grid-cols-3 gap-2">
                    <div class="border-2 border-black p-2 text-center shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">
                      <p class="text-[8px] font-black uppercase text-gray-500">Viskositas 100C</p>
                      <p class="text-sm font-black">{{ analysis.telemetry.lab_viscosity_100c }}</p>
                    </div>
                    <div class="border-2 border-black p-2 text-center shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]"
                      :class="analysis.telemetry.lab_water_content_pct > 0.5 ? 'bg-neoRed text-white' : ''">
                      <p class="text-[8px] font-black uppercase" :class="analysis.telemetry.lab_water_content_pct > 0.5 ? 'text-red-100' : 'text-gray-500'">Water %</p>
                      <p class="text-sm font-black">{{ analysis.telemetry.lab_water_content_pct }}</p>
                    </div>
                    <div class="border-2 border-black p-2 text-center shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]"
                      :class="analysis.telemetry.lab_soot_pct > 3 ? 'bg-miningYellow' : ''">
                      <p class="text-[8px] font-black uppercase text-gray-500">Soot %</p>
                      <p class="text-sm font-black">{{ analysis.telemetry.lab_soot_pct }}</p>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Engineer generated -->
              <p class="text-[10px] font-black uppercase text-gray-400 mb-2">🧮 Engineer Generated Features</p>
              <div class="grid grid-cols-3 gap-3">
                <div class="border-2 border-black p-3 text-center shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] bg-neoBlue text-white">
                  <p class="text-[9px] font-black uppercase text-blue-100">Delta Eng Temp</p>
                  <p class="text-xl font-black">{{ analysis.telemetry.delta_eng_temp }}°C</p>
                </div>
                <div class="border-2 border-black p-3 text-center shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]" :style="{ backgroundColor: statusLabelColor(analysis.telemetry.status_label) }">
                  <p class="text-[9px] font-black uppercase">Status Label</p>
                  <p class="text-xl font-black">{{ analysis.telemetry.status_label }}</p>
                </div>
                <div class="border-2 border-black p-3 text-center shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] bg-black text-white">
                  <p class="text-[9px] font-black uppercase text-gray-400">RUL (Telemetri)</p>
                  <p class="text-xl font-black text-miningYellow">{{ fmtHours(analysis.telemetry.rul_hours) }}</p>
                </div>
              </div>
            </div>

            <!-- RUL + SHAP -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div class="bg-white border-4 border-black shadow-neo p-6">
                <h3 class="text-lg font-black uppercase mb-4 border-b-4 border-black pb-2">Prediksi Sisa Umur (RUL)</h3>
                <div class="text-center py-4">
                  <p class="text-xs font-bold text-gray-500 uppercase">Komponen Terlemah</p>
                  <p class="text-2xl font-black mb-4">{{ analysis.rul_prediction.component }}</p>
                  <div class="bg-black text-white border-4 border-black p-4 shadow-[4px_4px_0px_0px_#FFCC00]">
                    <p class="text-[10px] font-bold text-gray-400 uppercase">Estimasi Sebelum Kegagalan</p>
                    <p class="text-3xl font-black text-miningYellow mt-1">{{ fmtHours(analysis.rul_prediction.hours_remaining) }}</p>
                    <p class="text-[10px] font-bold text-gray-400 mt-2">
                      Rentang: {{ Math.round(analysis.rul_prediction.lower_bound) }}–{{ Math.round(analysis.rul_prediction.upper_bound) }} jam ·
                      Confidence {{ analysis.rul_prediction.confidence }}%
                    </p>
                  </div>
                </div>
              </div>
              <div class="bg-white border-4 border-black shadow-neo p-6">
                <h3 class="text-lg font-black uppercase mb-2 border-b-4 border-black pb-2">Faktor Penyebab (SHAP)</h3>
                <p class="text-[10px] font-bold text-gray-500 mb-2">Kontribusi tiap sensor terhadap skor risiko</p>
                <div class="h-56"><canvas id="shap"></canvas></div>
              </div>
            </div>
          </div>
        </div>
      </template>
    </main>
  </div>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Public+Sans:wght@900&family=Space+Mono:wght@400;700&display=swap');
body { font-family: 'Space Mono', monospace; }
h1,h2,h3,button,.font-black { font-family: 'Public Sans', sans-serif; font-weight: 900; }
canvas { max-width: 100%; }
</style>
