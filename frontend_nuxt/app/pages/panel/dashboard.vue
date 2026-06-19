<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue'

// Muat web component <model-viewer> untuk render GLB
useHead({
  script: [
    { type: 'module', src: 'https://ajax.googleapis.com/ajax/libs/model-viewer/3.3.0/model-viewer.min.js' },
  ],
})

// ---- Sidebar ----
const menuItems = [
  { name: 'Dashboard',        path: '/panel/dashboard',         icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="7" height="9" x="3" y="3" rx="1"/><rect width="7" height="5" x="14" y="3" rx="1"/><rect width="7" height="9" x="14" y="12" rx="1"/><rect width="7" height="5" x="3" y="16" rx="1"/></svg>` },
  { name: 'Jenis Alat Berat', path: '/panel/jenis_alat_berat',  icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 17h4V5H2v12h3"/><path d="M20 17h2v-9l-2.5-3.5H14v12h3"/><path d="M14 6h4.5"/><circle cx="18.5" cy="17.5" r="2.5"/><circle cx="5.5" cy="17.5" r="2.5"/></svg>` },
  { name: 'Unit Tambang',     path: '/panel/unit_tambang',       icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 12 12 17 22 12"/><polyline points="2 17 12 22 22 17"/></svg>` },
  { name: 'Analisa Kerusakan', path: '/panel/analisa',           icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>` },
  { name: 'Kembali',          path: '../',                       icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m12 19-7-7 7-7"/><path d="M19 12H5"/></svg>` },
]
const activeMenu = ref('Dashboard')
const setActiveMenu = (menu: any) => { activeMenu.value = menu.name }

// ---- Loading & Auth ----
const isLoading = ref(true)
const { initAuth, user } = useAuth()
const api = useApi()

// ---- KPI dari backend ----
const dashboardKPI = ref({ totalUnits: 0, activeUnits: 0, criticalUnits: 0, totalSavings: 0 })

// ---- Status colors (palet industrial) ----
const statusHexMap: Record<string, string> = {
  Sehat: '#1FA971',
  Warning: '#E0A106',
  Critical: '#E0413E',
  Rusak: '#7A848E',
}
const statusDistribution = ref<{ label: string; jumlah: number; color: string }[]>([])

// ---- Monthly fleet data & map locations dari backend ----
interface FleetStatusData {
  month: string
  sehat: { val: number; units: string[] }
  warning: { val: number; units: string[] }
  critical: { val: number; units: string[] }
}
const monthlyFleetData = ref<FleetStatusData[]>([])
const mapLocations = ref<any[]>([])

// ---- Armada 3D ----
interface FleetUnit {
  id: string; code: string; jenis_alat_berat_nama: string | null
  status: string; health: number; savings: number
  maintenance: string; img_url: string | null; model3d_url: string | null
}
const units = ref<FleetUnit[]>([])
const featuredUnit = ref<FleetUnit | null>(null)
const selectFeatured = (u: FleetUnit) => { featuredUnit.value = u }

const statusHex = (s: string) => ({
  SEHAT: '#1FA971', WARNING: '#E0A106', CRITICAL: '#E0413E', RUSAK: '#7A848E',
}[s] || '#7A848E')

const loadUnits = async () => {
  try {
    const res = await api.getUnitTambang({ per_page: 100 }) as any
    units.value = res.data.data
    if (units.value.length > 0) featuredUnit.value = units.value[0]
  } catch (e) {
    console.error('Failed to load units', e)
  }
}

// ---- Modal bulan ----
const isMonthDetailModalOpen = ref(false)
const selectedMonthData = ref<FleetStatusData | null>(null)
const openMonthDetail = (i: number) => { selectedMonthData.value = monthlyFleetData.value[i]; isMonthDetailModalOpen.value = true }
const closeMonthDetail = () => { isMonthDetailModalOpen.value = false; setTimeout(() => { selectedMonthData.value = null }, 200) }

// ---- Chart instance ----
let utilizationChart: any = null

// ---- Theme-aware chart palette ----
const css = (v: string) => (typeof window !== 'undefined' ? getComputedStyle(document.documentElement).getPropertyValue(v).trim() : '')
const chartTheme = () => ({
  tick: css('--text-muted') || '#5d6b7a',
  grid: css('--border') || '#d7dde4',
  axis: css('--border-strong') || '#c2cad3',
  surface: css('--surface') || '#ffffff',
  text: css('--text') || '#1b2128',
})

// ---- Fetch dashboard stats ----
const loadDashboard = async () => {
  try {
    const res = await api.getDashboardStats() as any
    const data = res.data

    dashboardKPI.value = {
      totalUnits: data.total_units,
      activeUnits: data.active_units,
      criticalUnits: data.critical_units,
      totalSavings: data.total_savings,
    }

    statusDistribution.value = data.status_distribution.map((s: any) => ({
      label: s.label,
      jumlah: s.jumlah,
      color: statusHexMap[s.label] || '#7A848E',
    }))

    monthlyFleetData.value = data.monthly_fleet_data
    mapLocations.value = data.map_locations
  } catch (e) {
    console.error('Failed to load dashboard stats', e)
  }
}

onMounted(async () => {
  initAuth()
  await Promise.all([loadDashboard(), loadUnits()])
  isLoading.value = false
  await nextTick()

  if (typeof window !== 'undefined') {
    const t = chartTheme()

    // Chart.js
    if (document.getElementById('utilizationChart') && monthlyFleetData.value.length > 0) {
      const { default: Chart } = await import('chart.js/auto')
      const ctx = document.getElementById('utilizationChart') as HTMLCanvasElement
      utilizationChart = new Chart(ctx, {
        type: 'line',
        data: {
          labels: monthlyFleetData.value.map(d => d.month),
          datasets: [
            { label: 'Sehat',    data: monthlyFleetData.value.map(d => d.sehat.val),    borderColor: '#1FA971', backgroundColor: 'rgba(31,169,113,0.12)', borderWidth: 2.5, pointRadius: 3, pointHoverRadius: 6, pointBackgroundColor: '#1FA971', pointBorderColor: t.surface, pointBorderWidth: 2, tension: 0.35, fill: true },
            { label: 'Warning',  data: monthlyFleetData.value.map(d => d.warning.val),  borderColor: '#E0A106', backgroundColor: 'rgba(224,161,6,0.10)', borderWidth: 2.5, pointRadius: 3, pointHoverRadius: 6, pointBackgroundColor: '#E0A106', pointBorderColor: t.surface, pointBorderWidth: 2, tension: 0.35 },
            { label: 'Critical', data: monthlyFleetData.value.map(d => d.critical.val), borderColor: '#E0413E', backgroundColor: 'rgba(224,65,62,0.10)', borderWidth: 2.5, pointRadius: 3, pointHoverRadius: 6, pointBackgroundColor: '#E0413E', pointBorderColor: t.surface, pointBorderWidth: 2, tension: 0.35 },
          ],
        },
        options: {
          responsive: true, maintainAspectRatio: false,
          layout: { padding: 10 },
          interaction: { mode: 'index', intersect: false },
          plugins: {
            legend: { display: false },
            tooltip: {
              backgroundColor: t.text, titleColor: '#fff', bodyColor: '#e6eaef', borderColor: t.axis, borderWidth: 1, padding: 12, cornerRadius: 10,
              titleFont: { family: 'Inter', size: 13, weight: 'bold' },
              bodyFont: { family: 'JetBrains Mono', size: 11 },
              boxPadding: 6, usePointStyle: true,
              callbacks: {
                title: (ctx) => `Bulan: ${ctx[0].label}`,
                label: (ctx) => {
                  const m = monthlyFleetData.value[ctx.dataIndex]
                  const map = [m?.sehat, m?.warning, m?.critical]
                  const item = map[ctx.datasetIndex]
                  const labels = ['Sehat', 'Warning', 'Critical']
                  const units = item?.units?.slice(0, 2).join(', ') || '-'
                  const extra = item?.units?.length > 2 ? ` (+${item.units.length - 2} unit...)` : ''
                  return `${labels[ctx.datasetIndex]}: ${ctx.raw}% | ${units}${extra}`
                },
                afterBody: () => '\n(Klik titik untuk rincian data)',
              },
            },
          },
          scales: {
            x: { grid: { color: t.grid }, ticks: { font: { family: 'Inter', weight: 'bold' }, color: t.tick }, border: { color: t.axis } },
            y: { min: 0, max: 100, grid: { color: t.grid }, ticks: { font: { family: 'JetBrains Mono' }, color: t.tick, stepSize: 20 }, border: { color: t.axis } },
          },
          onClick: (_, elements) => { if (elements.length > 0) openMonthDetail(elements[0].index) },
        },
      })
    }

    // Leaflet Map
    if (document.getElementById('mining-map') && mapLocations.value.length > 0) {
      const L = (await import('leaflet')).default
      const map = L.map('mining-map', { center: [-0.5032, 117.1536], zoom: 15, zoomControl: false })
      L.tileLayer('https://{s}.basemaps.cartocdn.com/rastertiles/voyager/{z}/{x}/{y}{r}.png', {
        attribution: '&copy; OpenStreetMap contributors',
      }).addTo(map)

      mapLocations.value.forEach((loc: any) => {
        const icon = L.divIcon({
          className: 'custom-fleet-marker',
          html: `<div style="display:flex;flex-direction:column;align-items:center;">
            <div style="background-color:${loc.color_hex};width:26px;height:26px;border-radius:50%;border:2px solid #fff;display:flex;align-items:center;justify-content:center;color:#fff;font-weight:800;font-size:12px;box-shadow:0 4px 12px -2px rgba(0,0,0,0.4);font-family:'Inter',sans-serif;">${loc.level}</div>
            <div style="background-color:#171d24;color:#fff;font-size:11px;font-weight:700;padding:2px 7px;border-radius:6px;margin-top:-4px;box-shadow:0 4px 10px -2px rgba(0,0,0,0.35);white-space:nowrap;font-family:'JetBrains Mono',monospace;">${loc.unit}</div>
          </div>`,
          iconSize: [50, 50], iconAnchor: [25, 25], popupAnchor: [0, -20],
        })
        const marker = L.marker([loc.lat, loc.lng], { icon }).addTo(map)
        marker.bindPopup(`
          <div style="font-family:'Inter',sans-serif;min-width:220px;border:1px solid rgba(0,0,0,0.08);border-radius:12px;overflow:hidden;box-shadow:0 14px 34px -12px rgba(0,0,0,0.4);">
            <div style="background-color:${loc.color_hex};color:#fff;padding:12px;">
              <h4 style="font-weight:800;font-size:17px;margin:0;">${loc.unit}</h4>
              <p style="margin:0;font-size:12px;opacity:.9;">${loc.unit_type}</p>
            </div>
            <div style="padding:12px;background-color:#fff;">
              <table style="width:100%;font-size:12px;border-collapse:collapse;color:#1b2128;">
                <tr><td style="padding-bottom:5px;color:#5d6b7a;">Status</td><td style="font-weight:700;text-align:right;">${loc.status}</td></tr>
                <tr><td style="padding-bottom:5px;color:#5d6b7a;">Operator</td><td style="font-weight:700;text-align:right;">${loc.operator}</td></tr>
                <tr><td style="padding-bottom:5px;color:#5d6b7a;">Speed</td><td style="font-weight:700;text-align:right;">${loc.speed}</td></tr>
                <tr><td style="padding-bottom:5px;color:#5d6b7a;">Suhu Mesin</td><td style="font-weight:700;text-align:right;">${loc.temp}</td></tr>
                <tr><td style="color:#5d6b7a;">Fuel Level</td><td style="font-weight:700;text-align:right;">${loc.fuel}</td></tr>
              </table>
              <div style="margin-top:10px;padding-top:8px;border-top:1px dashed #d7dde4;font-size:10px;text-align:center;color:#5d6b7a;font-family:'JetBrains Mono',monospace;">
                ${loc.lat.toFixed(4)}, ${loc.lng.toFixed(4)} · <b>${loc.last_update}</b>
              </div>
            </div>
          </div>`, { closeButton: false, className: 'custom-leaflet-popup' })
      })
    }
  }
})

// ---- Modal laporan ----
const isReportModalOpen = ref(false)
const openReportModal = () => { isReportModalOpen.value = true }
const closeReportModal = () => { isReportModalOpen.value = false }
</script>

<template>
  <div class="flex min-h-screen bg-mesh text-[color:var(--text)] relative overflow-x-hidden">

    <aside class="w-72 border-r border-[color:var(--border)] bg-[color:var(--surface)] p-5 flex flex-col justify-between z-[60] shrink-0">
      <div>
        <div class="flex items-center gap-3 mb-9 px-1">
          <div class="w-11 h-11 rounded-xl bg-amber-gradient flex items-center justify-center shadow-[0_8px_18px_-8px_rgba(242,166,12,0.7)]">
            <svg class="w-6 h-6 text-graphite-900" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 12 12 17 22 12"/><polyline points="2 17 12 22 22 17"/></svg>
          </div>
          <div>
            <p class="font-display text-xl font-bold tracking-wide leading-none">PRAT<span class="text-amber">YAKSA</span></p>
            <p class="text-[10px] font-semibold uppercase tracking-[0.18em] text-[color:var(--text-faint)] mt-1">Control Panel</p>
          </div>
        </div>

        <nav class="space-y-1.5">
          <NuxtLink
            v-for="item in menuItems"
            :key="item.name"
            :to="item.path"
            @click="setActiveMenu(item)"
            :class="['nav-link', activeMenu === item.name ? 'nav-link-active' : '']"
          >
            <span class="flex items-center justify-center shrink-0" v-html="item.icon"></span>
            <span class="truncate">{{ item.name }}</span>
          </NuxtLink>
        </nav>
      </div>

      <div class="panel-flat p-4">
        <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-faint)]">Logged in as</p>
        <div class="flex items-center gap-3 mt-2">
          <div class="w-9 h-9 rounded-full bg-steel-gradient flex items-center justify-center text-white font-bold text-sm">{{ (user?.name || 'A').charAt(0).toUpperCase() }}</div>
          <p class="font-semibold text-sm truncate">{{ user?.name || 'Admin' }}</p>
        </div>
      </div>
    </aside>

    <main class="flex-1 p-8 overflow-y-auto z-10 relative">
      <header class="flex justify-between items-start mb-8 gap-4 flex-wrap">
        <div>
          <h1 class="font-display text-4xl md:text-5xl font-bold uppercase tracking-wide leading-none">Dashboard</h1>
          <p class="mt-2 text-[color:var(--text-muted)]">Ringkasan data analitik armada secara menyeluruh.</p>
        </div>
        <div class="flex items-center gap-3 panel-flat px-3 py-2">
          <div class="w-8 h-8 rounded-full bg-steel-gradient flex items-center justify-center text-white font-bold text-xs">{{ (user?.name || 'A').charAt(0).toUpperCase() }}</div>
          <span class="font-semibold text-sm">{{ user?.name || 'Admin' }}</span>
        </div>
      </header>

      <div class="space-y-7">

        <!-- KPI -->
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-5">
          <template v-if="isLoading">
            <div v-for="i in 4" :key="'kpi-skel-'+i" class="panel p-6">
              <div class="h-4 w-24 shimmer rounded mb-4"></div>
              <div class="h-10 w-32 shimmer rounded"></div>
            </div>
          </template>
          <template v-else>
            <div v-tilt class="tilt-card kpi anim-pop d-1 p-6" style="--accent:#3E92CC">
              <span class="tilt-shine"></span>
              <p class="text-[11px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-2">Total Unit</p>
              <p class="font-display text-5xl font-bold">{{ dashboardKPI.totalUnits }}</p>
            </div>
            <div v-tilt class="tilt-card kpi anim-pop d-2 p-6" style="--accent:#1FA971">
              <span class="tilt-shine"></span>
              <p class="text-[11px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-2">Unit Aktif (Sehat)</p>
              <p class="font-display text-5xl font-bold text-healthy">{{ dashboardKPI.activeUnits }}</p>
            </div>
            <div v-tilt class="tilt-card kpi anim-pop d-3 p-6" style="--accent:#E0413E"
              :class="dashboardKPI.criticalUnits > 0 ? 'anim-glow' : ''">
              <span class="tilt-shine"></span>
              <p class="text-[11px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)] mb-2">Kritis / Rusak</p>
              <p class="font-display text-5xl font-bold text-critical">{{ dashboardKPI.criticalUnits }}</p>
            </div>
            <div v-tilt class="tilt-card kpi anim-pop d-4 p-6 !bg-steel-gradient text-white" style="--accent:#F2A60C">
              <span class="tilt-shine"></span>
              <p class="text-[11px] font-semibold uppercase tracking-wider text-amber mb-2">Total Saving</p>
              <p class="font-display text-4xl font-bold mt-1">${{ dashboardKPI.totalSavings.toLocaleString() }}</p>
            </div>
          </template>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">

          <!-- Chart -->
          <div class="lg:col-span-2 panel p-6 flex flex-col">
            <template v-if="isLoading">
              <div class="flex justify-between items-center mb-6 pb-4 border-b border-[color:var(--border)]">
                <div class="h-7 w-64 shimmer rounded"></div>
                <div class="h-7 w-24 shimmer rounded"></div>
              </div>
              <div class="flex-1 w-full h-72 shimmer rounded-xl"></div>
            </template>

            <template v-else>
              <div class="flex justify-between items-start md:items-center flex-col md:flex-row mb-5 pb-4 border-b border-[color:var(--border)] gap-4">
                <div>
                  <h2 class="font-display text-2xl font-bold uppercase tracking-wide">Statistik Kondisi Unit</h2>
                  <div class="flex gap-4 mt-2 flex-wrap">
                    <div class="flex items-center gap-2 text-xs font-semibold"><span class="w-3 h-3 rounded-full bg-healthy"></span> Sehat</div>
                    <div class="flex items-center gap-2 text-xs font-semibold"><span class="w-3 h-3 rounded-full bg-warning"></span> Warning</div>
                    <div class="flex items-center gap-2 text-xs font-semibold"><span class="w-3 h-3 rounded-full bg-critical"></span> Critical</div>
                  </div>
                </div>
                <div class="flex gap-2 items-center">
                  <span class="text-[10px] font-medium text-[color:var(--text-faint)] hidden sm:inline">💡 Klik titik untuk detail</span>
                  <button class="btn btn-ghost !py-2 text-sm">Tahun 2026</button>
                </div>
              </div>

              <div class="flex-1 w-full h-80 relative mt-2 cursor-pointer">
                <canvas id="utilizationChart"></canvas>
              </div>
            </template>
          </div>

          <!-- Distribusi -->
          <div class="panel p-6 flex flex-col">
            <template v-if="isLoading">
              <div class="h-7 w-40 shimmer rounded mb-6"></div>
              <div class="flex-1 flex flex-col justify-center gap-6">
                <div v-for="i in 4" :key="'dist-skel-'+i">
                  <div class="flex justify-between mb-2"><div class="h-4 w-16 shimmer rounded"></div><div class="h-4 w-12 shimmer rounded"></div></div>
                  <div class="w-full h-3 shimmer rounded-full"></div>
                </div>
              </div>
            </template>
            <template v-else>
              <h2 class="font-display text-2xl font-bold uppercase tracking-wide mb-6 pb-4 border-b border-[color:var(--border)]">Distribusi Total</h2>
              <div class="flex-1 flex flex-col justify-center gap-5">
                <div v-for="(item, index) in statusDistribution" :key="item.label">
                  <div class="flex justify-between font-semibold text-sm mb-2">
                    <span>{{ item.label }}</span>
                    <span class="font-mono text-[color:var(--text-muted)]">{{ item.jumlah }}/{{ dashboardKPI.totalUnits }}</span>
                  </div>
                  <div class="w-full h-3 rounded-full bg-[color:var(--surface-3)] overflow-hidden">
                    <div
                      class="h-full rounded-full animate-progress-grow"
                      :style="{ width: `${(item.jumlah / dashboardKPI.totalUnits) * 100}%`, backgroundColor: item.color, animationDelay: `${index * 0.1}s` }"
                    ></div>
                  </div>
                </div>
              </div>
              <button @click="openReportModal" class="btn btn-dark w-full mt-7 !py-3">Lihat Detail Laporan</button>
            </template>
          </div>
        </div>

        <!-- ============ VISUAL 3D ARMADA ============ -->
        <div class="panel p-6 relative z-0">
          <div class="flex justify-between items-center mb-6 pb-4 border-b border-[color:var(--border)] flex-wrap gap-3">
            <div>
              <h2 class="font-display text-2xl font-bold uppercase tracking-wide">Visual 3D Armada</h2>
              <p class="text-xs text-[color:var(--text-muted)] mt-1">Klik unit untuk menampilkan model 3D interaktif · drag untuk putar 360°</p>
            </div>
            <span class="badge bg-steel/15 border-steel/40 text-steel">
              <span class="w-1.5 h-1.5 rounded-full bg-steel anim-live"></span> {{ units.length }} Unit 3D
            </span>
          </div>

          <template v-if="isLoading">
            <div class="w-full h-[420px] shimmer rounded-xl"></div>
          </template>

          <div v-else class="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <!-- Featured big viewer -->
            <div class="lg:col-span-2">
              <div v-if="featuredUnit" class="viewer-3d rounded-xl border border-[color:var(--border)] bg-steel-gradient shadow-elev-sm relative overflow-hidden h-[420px] cursor-grab active:cursor-grabbing">
                <model-viewer
                  :key="featuredUnit.id"
                  :src="featuredUnit.model3d_url || 'https://modelviewer.dev/shared-assets/models/RobotExpressive.glb'"
                  :alt="'Model 3D ' + featuredUnit.code"
                  camera-controls auto-rotate auto-rotate-delay="0" rotation-per-second="32deg"
                  shadow-intensity="1.5" exposure="1.15" environment-image="neutral" interaction-prompt="none"
                  class="w-full h-full outline-none" style="background-color:transparent;"
                ></model-viewer>
                <div class="absolute top-3 left-3 flex flex-col gap-2 pointer-events-none">
                  <div class="bg-steel/90 text-white text-[10px] font-semibold px-2.5 py-1 rounded-full flex items-center gap-1.5">
                    <span class="w-1.5 h-1.5 rounded-full bg-white anim-live"></span> LIVE 3D
                  </div>
                  <div class="bg-graphite-900/80 text-amber text-sm font-mono font-bold px-3 py-1 rounded-lg">{{ featuredUnit.code }}</div>
                </div>
                <div class="absolute top-3 right-3 badge text-white" :style="{ backgroundColor: statusHex(featuredUnit.status), borderColor: statusHex(featuredUnit.status) }">{{ featuredUnit.status }}</div>
                <div class="absolute bottom-0 left-0 right-0 bg-graphite-900/85 backdrop-blur text-white p-4 flex justify-between items-end">
                  <div>
                    <p class="font-display font-bold text-lg uppercase leading-tight">{{ featuredUnit.jenis_alat_berat_nama }}</p>
                    <p class="text-xs text-graphite-300">{{ featuredUnit.maintenance }}</p>
                  </div>
                  <div class="text-right">
                    <p class="text-[10px] text-graphite-400 uppercase tracking-wider">Health</p>
                    <p class="text-3xl font-display font-bold" :style="{ color: statusHex(featuredUnit.status) }">{{ featuredUnit.health }}%</p>
                  </div>
                </div>
              </div>
            </div>

            <!-- Unit selector grid -->
            <div class="grid grid-cols-2 gap-3 max-h-[420px] overflow-y-auto pr-1">
              <button
                v-for="(u, i) in units" :key="u.id"
                @click="selectFeatured(u)"
                :class="[
                  'anim-pop relative overflow-hidden rounded-xl border text-left transition-all group',
                  featuredUnit && featuredUnit.id === u.id ? 'border-amber ring-2 ring-amber/30' : 'border-[color:var(--border)] hover:-translate-y-1 hover:shadow-elev-sm'
                ]"
                :style="{ animationDelay: (i * 0.05) + 's' }"
              >
                <div class="viewer-3d h-28 bg-steel-gradient relative overflow-hidden">
                  <model-viewer
                    :key="'mini-' + u.id"
                    :src="u.model3d_url || 'https://modelviewer.dev/shared-assets/models/RobotExpressive.glb'"
                    :alt="'Model 3D ' + u.code"
                    auto-rotate auto-rotate-delay="0" rotation-per-second="40deg" disable-zoom interaction-prompt="none"
                    shadow-intensity="1" exposure="1.1" environment-image="neutral"
                    class="w-full h-full outline-none pointer-events-none" style="background-color:transparent;"
                  ></model-viewer>
                  <div class="absolute top-1.5 right-1.5 w-3 h-3 rounded-full border-2 border-white" :style="{ backgroundColor: statusHex(u.status) }"></div>
                </div>
                <div class="p-2.5 bg-[color:var(--surface)] border-t border-[color:var(--border)]">
                  <p class="font-mono font-semibold text-xs truncate">{{ u.code }}</p>
                  <div class="flex items-center justify-between mt-1">
                    <span class="text-[10px] text-[color:var(--text-faint)]">HP {{ u.health }}%</span>
                    <span class="text-[8px] font-semibold px-1.5 py-0.5 rounded text-white" :style="{ backgroundColor: statusHex(u.status) }">{{ u.status }}</span>
                  </div>
                </div>
              </button>
            </div>
          </div>
        </div>

        <!-- PETA -->
        <div class="panel p-6 flex flex-col relative z-0">
          <div class="flex justify-between items-center mb-5 pb-4 border-b border-[color:var(--border)] flex-wrap gap-3">
            <template v-if="isLoading">
              <div class="h-7 w-64 shimmer rounded"></div>
              <div class="h-7 w-48 shimmer rounded"></div>
            </template>
            <template v-else>
              <h2 class="font-display text-2xl font-bold uppercase tracking-wide">Peta Sebaran Unit</h2>
              <div class="hidden sm:flex gap-4 text-xs font-semibold badge-wrap">
                <div class="flex items-center gap-1.5"><span class="w-2.5 h-2.5 rounded-full bg-critical"></span> Critical</div>
                <div class="flex items-center gap-1.5"><span class="w-2.5 h-2.5 rounded-full bg-warning"></span> High</div>
                <div class="flex items-center gap-1.5"><span class="w-2.5 h-2.5 rounded-full bg-healthy"></span> Normal</div>
              </div>
            </template>
          </div>

          <template v-if="isLoading">
            <div class="w-full h-[500px] shimmer rounded-xl"></div>
          </template>
          <div v-show="!isLoading" id="mining-map" class="w-full h-[500px] rounded-xl border border-[color:var(--border)] relative z-0 overflow-hidden bg-[#e8f4f8]"></div>
        </div>

      </div>
    </main>

    <!-- Report Modal -->
    <div v-if="isReportModalOpen" class="fixed inset-0 z-[100] flex items-center justify-center p-4">
      <div class="modal-backdrop" @click="closeReportModal"></div>
      <div class="modal-card w-full max-w-3xl flex flex-col max-h-[90vh] anim-pop">

        <div class="flex justify-between items-center px-6 py-4 bg-steel-gradient text-white">
          <h3 class="font-display text-2xl font-bold uppercase tracking-wide text-amber">Detail Laporan Analitik</h3>
          <button @click="closeReportModal" class="w-9 h-9 rounded-lg bg-white/10 hover:bg-critical text-white flex items-center justify-center transition-colors">✕</button>
        </div>

        <div class="p-6 overflow-y-auto bg-[color:var(--surface-2)]">
          <h4 class="font-display text-xl font-bold uppercase tracking-wide mb-4">Ringkasan Eksekutif</h4>
          <div class="grid grid-cols-2 gap-4 mb-7">
            <div class="panel p-4">
              <p class="label">Total Utilisasi Tahun Ini</p>
              <p class="font-display text-3xl font-bold mt-1">3.455 <span class="text-sm font-normal text-[color:var(--text-muted)]">Jam Operasi</span></p>
            </div>
            <div class="panel p-4">
              <p class="label">Proyeksi Cost Reduction</p>
              <p class="font-display text-3xl font-bold text-healthy mt-1">12,5% <span class="text-sm font-normal text-[color:var(--text-muted)]">MOM</span></p>
            </div>
          </div>
          <h4 class="font-display text-lg font-bold uppercase tracking-wide mb-4">Breakdown Status Armada</h4>
          <div class="panel overflow-hidden">
            <table class="table-industrial">
              <thead>
                <tr>
                  <th>Kategori Status</th>
                  <th>Persentase</th>
                  <th>Aksi Lanjutan Rekomendasi</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(item, index) in statusDistribution" :key="index">
                  <td><span class="badge text-white" :style="{ backgroundColor: item.color, borderColor: item.color }">{{ item.label }}</span></td>
                  <td class="font-mono font-bold text-lg">{{ item.jumlah }}/{{ dashboardKPI.totalUnits }}</td>
                  <td class="text-sm text-[color:var(--text-muted)]">
                    {{ item.label === 'Sehat' ? 'Pertahankan jadwal maintenance rutin.' :
                       item.label === 'Warning' ? 'Lakukan pengecekan dalam 48 jam ke depan.' :
                       item.label === 'Critical' ? 'Segera jadwalkan overhaul, stop operasi.' : 'Tunggu suku cadang dari supplier utama.' }}
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
          <div class="mt-7 flex justify-end">
            <button @click="closeReportModal" class="btn btn-amber px-6">Tutup / Unduh PDF</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Month Detail Modal -->
    <div v-if="isMonthDetailModalOpen && selectedMonthData" class="fixed inset-0 z-[100] flex items-center justify-center p-4">
      <div class="modal-backdrop" @click="closeMonthDetail"></div>
      <div class="modal-card w-full max-w-4xl flex flex-col max-h-[90vh] anim-pop">

        <div class="flex justify-between items-center px-6 py-4 border-b border-[color:var(--border)] bg-[color:var(--surface-2)]">
          <h3 class="font-display text-2xl font-bold uppercase tracking-wide">Detail Unit Bulan {{ selectedMonthData.month }}</h3>
          <button @click="closeMonthDetail" class="w-9 h-9 rounded-lg hover:bg-critical/15 hover:text-critical text-[color:var(--text-muted)] flex items-center justify-center transition-colors">✕</button>
        </div>

        <div class="p-6 overflow-y-auto flex flex-col gap-4">
          <div class="panel p-4 border-l-4" style="border-left-color:#1FA971">
            <div class="flex items-center justify-between mb-2">
              <span class="badge text-white" style="background:#1FA971;border-color:#1FA971">Sehat ({{ selectedMonthData.sehat.val }}%)</span>
              <span class="text-xs font-semibold text-healthy">{{ selectedMonthData.sehat.units.length }} Armada</span>
            </div>
            <p class="text-[color:var(--text-muted)] leading-relaxed text-sm">{{ selectedMonthData.sehat.units.length > 0 ? selectedMonthData.sehat.units.join(', ') : 'Tidak ada unit dalam kategori ini.' }}</p>
          </div>

          <div class="panel p-4 border-l-4" style="border-left-color:#E0A106">
            <div class="flex items-center justify-between mb-2">
              <span class="badge text-white" style="background:#E0A106;border-color:#E0A106">Warning ({{ selectedMonthData.warning.val }}%)</span>
              <span class="text-xs font-semibold text-warning">{{ selectedMonthData.warning.units.length }} Armada</span>
            </div>
            <p class="text-[color:var(--text-muted)] leading-relaxed text-sm">{{ selectedMonthData.warning.units.length > 0 ? selectedMonthData.warning.units.join(', ') : 'Tidak ada unit dalam kategori ini.' }}</p>
          </div>

          <div class="panel p-4 border-l-4" style="border-left-color:#E0413E">
            <div class="flex items-center justify-between mb-2">
              <span class="badge text-white" style="background:#E0413E;border-color:#E0413E">Critical ({{ selectedMonthData.critical.val }}%)</span>
              <span class="text-xs font-semibold text-critical">{{ selectedMonthData.critical.units.length }} Armada</span>
            </div>
            <p class="text-[color:var(--text-muted)] leading-relaxed text-sm">{{ selectedMonthData.critical.units.length > 0 ? selectedMonthData.critical.units.join(', ') : 'Tidak ada unit dalam kategori ini.' }}</p>
          </div>
        </div>

      </div>
    </div>

  </div>
</template>

<style>
@import url('https://unpkg.com/leaflet@1.9.4/dist/leaflet.css');
</style>
