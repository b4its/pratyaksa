<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue'

// --- LOGIKA SKELETON LOADING ---
const isLoading = ref(true)

// --- LOGIKA SIDEBAR NAVIGASI ---
interface MenuItem {
  name: string
  path: string
  icon: string
}

const menuItems = ref<MenuItem[]>([
  { 
    name: 'Dashboard', 
    path: '/panel/dashboard',
    icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square" stroke-linejoin="miter"><rect width="7" height="9" x="3" y="3"/><rect width="7" height="5" x="14" y="3"/><rect width="7" height="9" x="14" y="12"/><rect width="7" height="5" x="3" y="16"/></svg>`
  },
  { 
    name: 'Jenis Alat Berat', 
    path: '/panel/jenis_alat_berat',
    icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square" stroke-linejoin="miter"><path d="M10 17h4V5H2v12h3"/><path d="M20 17h2v-9l-2.5-3.5H14v12h3"/><path d="M14 6h4.5"/><circle cx="18.5" cy="17.5" r="2.5"/><circle cx="5.5" cy="17.5" r="2.5"/></svg>`
  },
  { 
    name: 'Unit Tambang', 
    path: '/panel/unit_tambang',
    icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square" stroke-linejoin="miter"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 12 12 17 22 12"/><polyline points="2 17 12 22 22 17"/></svg>`
  },
  { 
    name: 'Analisa Kerusakan', 
    path: '/panel/analisa',
    icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square" stroke-linejoin="miter"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>`
  },
  { 
    name: 'Kembali', 
    path: '../',
    icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square" stroke-linejoin="miter"><path d="m12 19-7-7 7-7"/><path d="M19 12H5"/></svg>`
  },
])

const activeMenu = ref('Dashboard') 

const setActiveMenu = (menu: MenuItem) => {
  activeMenu.value = menu.name
}

// --- DATA DUMMY DASHBOARD (CHART & KPI) ---
const dashboardKPI = ref({
  totalUnits: 124,
  activeUnits: 98,
  criticalUnits: 12,
  totalSavings: 45200
})

const statusDistribution = ref([
  { label: 'Sehat', jumlah: 65, color: 'bg-emerald-400' },
  { label: 'Warning', jumlah: 20, color: 'bg-miningYellow' },
  { label: 'Critical', jumlah: 10, color: 'bg-neoRed' },
  { label: 'Rusak', jumlah: 7, color: 'bg-red-800' },
  { label: 'Sedang Perbaikan', jumlah: 5, color: 'bg-gray-400' },
])

// --- DATA MULTI-LINE CHART (SEHAT, WARNING, CRITICAL) ---
interface FleetStatusData {
  month: string
  sehat: { val: number, units: string[] }
  warning: { val: number, units: string[] }
  critical: { val: number, units: string[] }
}

const monthlyFleetData = ref<FleetStatusData[]>([
  { month: 'Jan', sehat: { val: 75, units: ['EX-001', 'EX-002', 'HT-012'] }, warning: { val: 15, units: ['DZ-003', 'DZ-004', 'DZ-005', 'DZ-006'] }, critical: { val: 10, units: ['HT-005'] } },
  { month: 'Feb', sehat: { val: 72, units: ['EX-001', 'HT-012'] }, warning: { val: 18, units: ['EX-002', 'DZ-003'] }, critical: { val: 10, units: ['HT-005'] } },
  { month: 'Mar', sehat: { val: 68, units: ['EX-001', 'EX-015', 'EX-016', 'EX-017', 'HT-019'] }, warning: { val: 22, units: ['HT-012', 'EX-002'] }, critical: { val: 10, units: ['HT-005', 'DZ-003'] } },
  { month: 'Apr', sehat: { val: 80, units: ['EX-001', 'EX-002', 'HT-012', 'DZ-003'] }, warning: { val: 15, units: ['HT-005'] }, critical: { val: 5, units: ['EX-009'] } },
  { month: 'Mei', sehat: { val: 85, units: ['EX-001', 'EX-002', 'HT-012', 'DZ-003', 'HT-005'] }, warning: { val: 10, units: ['EX-009'] }, critical: { val: 5, units: ['DZ-008'] } },
  { month: 'Jun', sehat: { val: 82, units: ['EX-001', 'EX-002', 'HT-012', 'DZ-003'] }, warning: { val: 10, units: ['HT-005'] }, critical: { val: 8, units: ['DZ-008', 'EX-009'] } },
  { month: 'Jul', sehat: { val: 78, units: ['EX-001', 'HT-012', 'DZ-003'] }, warning: { val: 15, units: ['EX-002', 'HT-005'] }, critical: { val: 7, units: ['DZ-008'] } },
  { month: 'Ags', sehat: { val: 70, units: ['EX-001', 'DZ-003'] }, warning: { val: 20, units: ['HT-012', 'EX-002'] }, critical: { val: 10, units: ['HT-005', 'DZ-008'] } },
  { month: 'Sep', sehat: { val: 65, units: ['EX-001'] }, warning: { val: 20, units: ['DZ-003', 'HT-012'] }, critical: { val: 15, units: ['EX-002', 'HT-005', 'DZ-008'] } },
  { month: 'Okt', sehat: { val: 60, units: ['EX-011'] }, warning: { val: 25, units: ['EX-001', 'DZ-003'] }, critical: { val: 15, units: ['HT-012', 'EX-002', 'HT-005'] } },
  { month: 'Nov', sehat: { val: 75, units: ['EX-011', 'EX-001', 'HT-012'] }, warning: { val: 15, units: ['DZ-003'] }, critical: { val: 10, units: ['EX-002'] } },
  { month: 'Des', sehat: { val: 88, units: ['EX-011', 'EX-001', 'HT-012', 'DZ-003', 'EX-002'] }, warning: { val: 10, units: ['HT-005'] }, critical: { val: 2, units: ['DZ-008'] } },
])

// --- LOGIKA MODAL DETAIL BULAN ---
const isMonthDetailModalOpen = ref(false)
const selectedMonthData = ref<FleetStatusData | null>(null)

const openMonthDetail = (dataIndex: number) => {
  selectedMonthData.value = monthlyFleetData.value[dataIndex]
  isMonthDetailModalOpen.value = true
}

const closeMonthDetail = () => {
  isMonthDetailModalOpen.value = false
  setTimeout(() => { selectedMonthData.value = null }, 200)
}

// --- DATA KOORDINAT & TELEMETRI LENGKAP PETA ---
const mapLocations = ref([
  { id: 1, unit: 'EX-003', type: 'Excavator 320', lat: -0.5005, lng: 117.1510, status: 'Normal', level: 'L', colorHex: '#34d399', fuel: '78%', operator: 'Budi S.', speed: '0 km/h', temp: '85°C', lastUpdate: '2 mnt lalu' },
  { id: 2, unit: 'HT-019', type: 'Haul Truck', lat: -0.5030, lng: 117.1470, status: 'Normal', level: 'M', colorHex: '#34d399', fuel: '45%', operator: 'Joko P.', speed: '25 km/h', temp: '90°C', lastUpdate: '1 mnt lalu' },
  { id: 3, unit: 'EX-007', type: 'Excavator 336', lat: -0.4990, lng: 117.1560, status: 'High', level: 'H', colorHex: '#facc15', fuel: '20%', operator: 'Agus T.', speed: '0 km/h', temp: '105°C', lastUpdate: 'Baru saja' },
  { id: 4, unit: 'DZ-011', type: 'Dozer D85A', lat: -0.5050, lng: 117.1495, status: 'High', level: 'H', colorHex: '#facc15', fuel: '55%', operator: 'Rian M.', speed: '5 km/h', temp: '98°C', lastUpdate: '5 mnt lalu' },
  { id: 5, unit: 'HT-023', type: 'Haul Truck', lat: -0.5075, lng: 117.1450, status: 'Critical', level: 'I', colorHex: '#f87171', fuel: '10%', operator: 'Deni R.', speed: '0 km/h', temp: '120°C', lastUpdate: '10 mnt lalu' },
  { id: 6, unit: 'HT-021', type: 'Haul Truck', lat: -0.5040, lng: 117.1530, status: 'Normal', level: 'L', colorHex: '#34d399', fuel: '80%', operator: 'Siti N.', speed: '30 km/h', temp: '88°C', lastUpdate: 'Baru saja' },
  { id: 7, unit: 'HT-025', type: 'Haul Truck', lat: -0.5060, lng: 117.1585, status: 'Normal', level: 'L', colorHex: '#34d399', fuel: '65%', operator: 'Eko W.', speed: '22 km/h', temp: '82°C', lastUpdate: '3 mnt lalu' },
])

// --- INISIALISASI PETA, SKELETON, DAN CHART.JS ---
let utilizationChart: any = null;

onMounted(async () => {
  setTimeout(async () => {
    isLoading.value = false
    await nextTick()

    if (typeof window !== 'undefined') {
      
      // 1. INISIALISASI CHART.JS
      if (document.getElementById('utilizationChart')) {
        // Menggunakan dynamic import untuk menghindari error SSR di Nuxt
        const { default: Chart } = await import('chart.js/auto')
        
        const ctx = document.getElementById('utilizationChart') as HTMLCanvasElement
        
        utilizationChart = new Chart(ctx, {
          type: 'line',
          data: {
            labels: monthlyFleetData.value.map(d => d.month),
            datasets: [
              {
                label: 'Sehat',
                data: monthlyFleetData.value.map(d => d.sehat.val),
                borderColor: '#34d399',
                backgroundColor: '#34d399',
                borderWidth: 3, // Garis tidak terlalu tebal
                pointRadius: 4, // Titik lebih rapi
                pointHoverRadius: 6,
                pointBorderColor: '#000',
                pointBorderWidth: 2,
                tension: 0, // Garis kaku ala brutalism
              },
              {
                label: 'Warning',
                data: monthlyFleetData.value.map(d => d.warning.val),
                borderColor: '#facc15',
                backgroundColor: '#facc15',
                borderWidth: 3,
                pointRadius: 4,
                pointHoverRadius: 6,
                pointBorderColor: '#000',
                pointBorderWidth: 2,
                tension: 0,
              },
              {
                label: 'Critical',
                data: monthlyFleetData.value.map(d => d.critical.val),
                borderColor: '#f87171',
                backgroundColor: '#f87171',
                borderWidth: 3,
                pointRadius: 4,
                pointHoverRadius: 6,
                pointBorderColor: '#000',
                pointBorderWidth: 2,
                tension: 0,
              }
            ]
          },
          options: {
            responsive: true,
            maintainAspectRatio: false,
            layout: { padding: 10 },
            interaction: {
              mode: 'index',
              intersect: false,
            },
            plugins: {
              legend: { display: false },
              tooltip: {
                backgroundColor: '#fff',
                titleColor: '#000',
                bodyColor: '#000',
                borderColor: '#000',
                borderWidth: 3,
                padding: 12,
                cornerRadius: 0,
                titleFont: { family: 'Public Sans', size: 14, weight: 'bold' },
                bodyFont: { family: 'Space Mono', size: 12, weight: 'bold' },
                boxPadding: 6,
                usePointStyle: true,
                callbacks: {
                  title: (context) => `Bulan: ${context[0].label}`,
                  label: (context) => {
                    const dataIndex = context.dataIndex;
                    const datasetIndex = context.datasetIndex;
                    const monthData = monthlyFleetData.value[dataIndex];
                    
                    let units: string[] = [];
                    let status = '';
                    
                    if (datasetIndex === 0) { units = monthData.sehat.units; status = 'Sehat'; }
                    else if (datasetIndex === 1) { units = monthData.warning.units; status = 'Warning'; }
                    else if (datasetIndex === 2) { units = monthData.critical.units; status = 'Critical'; }

                    // LIMITASI TEXT AGAR TOOLTIP TIDAK OVERFLOW
                    const maxDisplay = 2;
                    let unitStr = units.slice(0, maxDisplay).join(', ');
                    if (units.length > maxDisplay) {
                      unitStr += ` (+${units.length - maxDisplay} unit...)`;
                    }

                    return `${status}: ${context.raw}% | ${units.length > 0 ? unitStr : '-'}`;
                  },
                  afterBody: () => '\n(Klik titik untuk rincian data)'
                }
              }
            },
            scales: {
              x: {
                grid: { color: '#E5E7EB', tickColor: '#000' },
                ticks: { font: { family: 'Public Sans', weight: 'bold' }, color: '#000' },
                border: { color: '#000', width: 4 }
              },
              y: {
                min: 0, max: 100,
                grid: { color: '#E5E7EB', borderDash: [4, 4] },
                ticks: { font: { family: 'Space Mono', weight: 'bold' }, color: '#000', stepSize: 20 },
                border: { color: '#000', width: 4 }
              }
            },
            onClick: (event, elements) => {
              if (elements.length > 0) {
                // Ambil index data bulan yang diklik, dan buka modal
                const dataIndex = elements[0].index;
                openMonthDetail(dataIndex);
              }
            }
          }
        })
      }

      // 2. INISIALISASI LEAFLET MAP
      if (document.getElementById('mining-map')) {
        const L = (await import('leaflet')).default
        const map = L.map('mining-map', {
          center: [-0.5032, 117.1536],
          zoom: 15,
          zoomControl: false 
        })

        L.tileLayer('https://{s}.basemaps.cartocdn.com/rastertiles/voyager/{z}/{x}/{y}{r}.png', {
          attribution: '&copy; OpenStreetMap contributors'
        }).addTo(map)

        mapLocations.value.forEach(loc => {
          const customIcon = L.divIcon({
            className: 'custom-fleet-marker',
            html: `
              <div style="display: flex; flex-direction: column; align-items: center; justify-content: center;">
                <div style="background-color: ${loc.colorHex}; width: 26px; height: 26px; border-radius: 50%; border: 2px solid black; display: flex; align-items: center; justify-content: center; color: black; font-weight: 900; font-size: 12px; z-index: 10; box-shadow: 2px 2px 0px rgba(0,0,0,1); font-family: sans-serif;">
                  ${loc.level}
                </div>
                <div style="background-color: #000; color: white; font-size: 11px; font-weight: 900; padding: 2px 6px; border-radius: 4px; margin-top: -6px; box-shadow: 2px 2px 0px rgba(0,0,0,1); white-space: nowrap; font-family: 'Space Mono', monospace; letter-spacing: -0.5px;">
                  ${loc.unit}
                </div>
              </div>
            `,
            iconSize: [50, 50],
            iconAnchor: [25, 25],
            popupAnchor: [0, -20]
          })

          const marker = L.marker([loc.lat, loc.lng], { icon: customIcon }).addTo(map)
          
          const popupHtml = `
            <div style="font-family: 'Public Sans', sans-serif; min-width: 220px; border: 4px solid black; box-shadow: 4px 4px 0px rgba(0,0,0,1);">
              <div style="background-color: ${loc.colorHex}; color: #000; padding: 10px; border-bottom: 4px solid black;">
                <h4 style="font-weight: 900; font-size: 18px; margin: 0; text-transform: uppercase;">${loc.unit}</h4>
                <p style="margin: 0; font-size: 12px; font-weight: bold;">${loc.type}</p>
              </div>
              <div style="padding: 10px; background-color: #fff;">
                <table style="width: 100%; font-size: 12px; font-weight: bold; border-collapse: collapse;">
                  <tr><td style="padding-bottom: 4px;">Status</td><td style="font-weight: 900; text-align: right;">${loc.status}</td></tr>
                  <tr><td style="padding-bottom: 4px;">Operator</td><td style="font-weight: 900; text-align: right;">${loc.operator}</td></tr>
                  <tr><td style="padding-bottom: 4px;">Speed</td><td style="font-weight: 900; text-align: right;">${loc.speed}</td></tr>
                  <tr><td style="padding-bottom: 4px;">Suhu Mesin</td><td style="font-weight: 900; text-align: right; color: ${loc.temp > '100' ? 'red' : 'inherit'}">${loc.temp}</td></tr>
                  <tr><td style="padding-bottom: 8px;">Fuel Level</td><td style="font-weight: 900; text-align: right;">${loc.fuel}</td></tr>
                </table>
                <div style="margin-top: 8px; padding-top: 8px; border-top: 2px dashed #000; font-size: 10px; text-align: center;">
                  Kordinat: ${loc.lat.toFixed(4)}, ${loc.lng.toFixed(4)}<br/>
                  <b>Terakhir update: ${loc.lastUpdate}</b>
                </div>
              </div>
            </div>
          `
          marker.bindPopup(popupHtml, {
            closeButton: false, 
            className: 'custom-leaflet-popup'
          })
        })
      }
    }
  }, 1500)
})

// --- LOGIKA MODAL LAPORAN KESELURUHAN ---
const isReportModalOpen = ref(false)
const openReportModal = () => { isReportModalOpen.value = true }
const closeReportModal = () => { isReportModalOpen.value = false }
</script>

<template>
  <div class="flex min-h-screen bg-[#F1F1F1] font-mono text-black relative overflow-x-hidden">
    
    <aside class="w-72 border-r-4 border-black bg-white p-6 flex flex-col justify-between z-[60] shrink-0">
      <div>
        <div class="mb-10 flex justify-center">
          <div class="bg-miningYellow border-4 border-black p-4 shadow-neo w-24 h-24 flex items-center justify-center rounded-xl translate-x-[-4px] translate-y-[-4px]">
            <span class="text-5xl font-black italic">V</span>
          </div>
        </div>

        <nav class="space-y-4">
        <NuxtLink 
            v-for="item in menuItems" 
            :key="item.name"
            :to="item.path"
            @click="setActiveMenu(item)"
            :class="[
            'w-full flex items-center gap-3 p-3 border-2 font-bold group transition-all cursor-pointer',
            activeMenu === item.name 
                ? 'border-black bg-miningYellow shadow-neoHover' 
                : 'border-transparent hover:border-black hover:bg-white hover:shadow-neoHover'
            ]"
        >
            <div v-if="activeMenu === item.name" class="p-1 border-2 border-black bg-white shrink-0 flex items-center justify-center">
              <div class="w-2.5 h-2.5 bg-black"></div>
            </div>
            <div v-else class="w-2.5 h-2.5 border-2 border-black ml-1.5 shrink-0 bg-transparent"></div>
            
            <div class="flex items-center gap-3 w-full">
               <div class="flex items-center justify-center shrink-0" v-html="item.icon"></div>
               <span class="truncate">{{ item.name }}</span>
            </div>
        </NuxtLink>
        </nav>


        
      </div>
    </aside>

    <main class="flex-1 p-8 overflow-y-auto z-10 relative">
      <header class="flex justify-between items-start mb-10">
        <div>
          <h1 class="text-6xl font-black uppercase tracking-tighter leading-none">{{ activeMenu }}</h1>
          <p class="mt-2 font-bold text-gray-600 bg-white border-b-2 border-black inline-block">
            {{ activeMenu === 'Dashboard' ? 'Ringkasan data analitik armada.' : 'Halaman ini sedang dalam pengembangan.' }}
          </p>
        </div>
        <div class="flex items-center gap-3 p-2 border-4 border-black bg-white shadow-neoHover shrink-0">
          <div class="w-8 h-8 rounded-full border-2 border-black bg-neoBlue"></div>
          <span class="font-black text-sm">Admin Suki</span>
        </div>
      </header>

      <div v-show="activeMenu === 'Dashboard'" class="space-y-8 animate-[popup_0.3s_ease-out]">
        
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          <template v-if="isLoading">
            <div v-for="i in 4" :key="'kpi-skel-'+i" class="bg-gray-200 border-4 border-black p-6 animate-pulse shadow-neo">
              <div class="h-4 w-24 bg-gray-300 mb-4 border border-black"></div>
              <div class="h-12 w-32 bg-gray-300 border border-black"></div>
            </div>
          </template>
          <template v-else>
            <div class="bg-white border-4 border-black p-6 shadow-neoHover hover:-translate-y-1 hover:shadow-neo transition-all">
              <p class="text-xs font-black uppercase text-gray-500 mb-2">Total Unit</p>
              <p class="text-5xl font-black">{{ dashboardKPI.totalUnits }}</p>
            </div>
            <div class="bg-emerald-400 border-4 border-black p-6 shadow-neoHover hover:-translate-y-1 hover:shadow-neo transition-all">
              <p class="text-xs font-black uppercase text-emerald-900 mb-2">Unit Aktif (Sehat)</p>
              <p class="text-5xl font-black text-black">{{ dashboardKPI.activeUnits }}</p>
            </div>
            <div class="bg-neoRed border-4 border-black p-6 shadow-neoHover hover:-translate-y-1 hover:shadow-neo transition-all text-white">
              <p class="text-xs font-black uppercase text-red-200 mb-2">Kritis / Rusak</p>
              <p class="text-5xl font-black">{{ dashboardKPI.criticalUnits }}</p>
            </div>
            <div class="bg-black text-white border-4 border-black p-6 shadow-[4px_4px_0px_0px_#FFCC00] hover:-translate-y-1 transition-all">
              <p class="text-xs font-black uppercase text-miningYellow mb-2">Total Saving</p>
              <p class="text-4xl font-black mt-2">${{ dashboardKPI.totalSavings.toLocaleString() }}</p>
            </div>
          </template>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
          
          <div class="lg:col-span-2 bg-white border-4 border-black shadow-neo p-6 flex flex-col relative overflow-visible">
            <template v-if="isLoading">
              <div class="flex justify-between items-center mb-8 border-b-4 border-black pb-4">
                <div class="h-8 w-64 bg-gray-200 border-2 border-black animate-pulse shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]"></div>
                <div class="h-8 w-24 bg-gray-200 border-2 border-black animate-pulse shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]"></div>
              </div>
              <div class="flex-1 w-full h-72 mt-4 relative bg-gray-50 border-4 border-black border-dashed animate-pulse flex items-center justify-center">
                <div class="text-gray-400 font-black tracking-widest uppercase">Memuat Chart...</div>
              </div>
            </template>

            <template v-else>
              <div class="flex justify-between items-start md:items-center flex-col md:flex-row mb-4 border-b-4 border-black pb-4 z-10 relative bg-white gap-4">
                <div>
                  <h2 class="text-2xl font-black uppercase">Statistik Kondisi Unit</h2>
                  <div class="flex gap-4 mt-2">
                    <div class="flex items-center gap-2 text-xs font-black uppercase"><div class="w-4 h-4 bg-[#34d399] border-2 border-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]"></div> Sehat</div>
                    <div class="flex items-center gap-2 text-xs font-black uppercase"><div class="w-4 h-4 bg-[#facc15] border-2 border-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]"></div> Warning</div>
                    <div class="flex items-center gap-2 text-xs font-black uppercase"><div class="w-4 h-4 bg-[#f87171] border-2 border-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]"></div> Critical</div>
                  </div>
                </div>
                <div class="flex gap-2">
                  <p class="text-[10px] font-bold text-gray-500 uppercase flex items-center bg-gray-100 px-2 border-2 border-black border-dashed pointer-events-none">💡 Klik titik untuk detail informasi</p>
                  <button class="border-4 border-black bg-miningYellow px-4 py-2 font-black text-sm shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:translate-y-1 hover:translate-x-1 hover:shadow-none transition-all">
                    Tahun 2026
                  </button>
                </div>
              </div>
              
              <div class="flex-1 w-full h-80 relative mt-4 cursor-pointer">
                <canvas id="utilizationChart"></canvas>
              </div>
            </template>
          </div>

          <div class="bg-white border-4 border-black shadow-neo p-6 flex flex-col">
            <template v-if="isLoading">
               <div class="h-8 w-48 bg-gray-200 mb-6 border-b-4 border-black animate-pulse shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]"></div>
               <div class="flex-1 flex flex-col justify-center gap-6">
                 <div v-for="i in 5" :key="'dist-skel-'+i">
                   <div class="flex justify-between mb-2">
                     <div class="h-4 w-16 bg-gray-200 border border-black animate-pulse"></div>
                     <div class="h-4 w-12 bg-gray-200 border border-black animate-pulse"></div>
                   </div>
                   <div class="w-full h-6 border-4 border-black bg-gray-100 shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]"></div>
                 </div>
               </div>
            </template>
            <template v-else>
              <h2 class="text-2xl font-black uppercase mb-6 border-b-4 border-black pb-4">Distribusi Total</h2>
              <div class="flex-1 flex flex-col justify-center gap-6">
                <div v-for="(item, index) in statusDistribution" :key="item.label">
                  <div class="flex justify-between font-black text-sm mb-2 uppercase">
                    <span>{{ item.label }}</span>
                    <span>{{ item.jumlah }}/{{ dashboardKPI.totalUnits }}</span>
                  </div>
                  <div class="w-full h-6 border-4 border-black bg-gray-200 shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] relative overflow-hidden">
                    <div 
                      :class="[item.color, 'h-full border-r-4 border-black animate-progress-grow']" 
                      :style="{ width: `${(item.jumlah / dashboardKPI.totalUnits) * 100}%`, animationDelay: `${index * 0.1}s` }"
                    ></div>
                  </div>
                </div>
              </div>
              <button @click="openReportModal" class="w-full mt-8 bg-black text-white border-4 border-black p-3 font-black shadow-[4px_4px_0px_0px_#FFCC00] hover:-translate-y-1 hover:shadow-[6px_6px_0px_0px_#FFCC00] transition-all uppercase">
                Lihat Detail Laporan
              </button>
            </template>
          </div>
        </div>

        <div class="bg-white border-4 border-black shadow-neo p-6 mt-8 flex flex-col relative z-0">
          <div class="flex justify-between items-center mb-4 border-b-4 border-black pb-4">
            <template v-if="isLoading">
              <div class="h-8 w-64 bg-gray-200 border-2 border-black animate-pulse"></div>
              <div class="h-8 w-48 bg-gray-200 border-2 border-black animate-pulse"></div>
            </template>
            <template v-else>
              <h2 class="text-2xl font-black uppercase">Peta &bull; Deretan Sebaran Unit</h2>
              <div class="flex gap-4 text-xs font-bold bg-gray-100 px-3 py-1 border-2 border-black hidden sm:flex">
                <div class="flex items-center gap-1"><span class="w-3 h-3 rounded-full bg-red-500 border border-black inline-block"></span> Critical</div>
                <div class="flex items-center gap-1"><span class="w-3 h-3 rounded-full bg-yellow-500 border border-black inline-block"></span> High</div>
                <div class="flex items-center gap-1"><span class="w-3 h-3 rounded-full bg-green-500 border border-black inline-block"></span> Normal</div>
              </div>
            </template>
          </div>
          
          <template v-if="isLoading">
            <div class="w-full h-[500px] border-4 border-black bg-gray-200 animate-pulse flex items-center justify-center font-black uppercase tracking-widest text-gray-500">
               Memuat Peta...
            </div>
          </template>
          <div v-show="!isLoading" id="mining-map" class="w-full h-[500px] border-4 border-black shadow-neoHover relative z-0 bg-[#e8f4f8]"></div>
        </div>

      </div>
    </main>

    <div v-if="isReportModalOpen" class="fixed inset-0 z-[100] flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="closeReportModal"></div>
      <div class="bg-white border-4 border-black shadow-neo w-full max-w-3xl relative z-10 flex flex-col max-h-[90vh] animate-[popup_0.2s_ease-out]">
        
        <div class="flex justify-between items-center p-6 border-b-4 border-black bg-black text-white">
          <h3 class="text-3xl font-black uppercase text-miningYellow">Detail Laporan Analitik</h3>
          <button @click="closeReportModal" class="bg-neoRed text-white border-2 border-white w-10 h-10 font-black shadow-[2px_2px_0px_0px_#FFCC00] hover:shadow-none hover:translate-x-[2px] hover:translate-y-[2px] transition-all">X</button>
        </div>
        
        <div class="p-6 overflow-y-auto bg-[#F1F1F1]">
          <h4 class="text-2xl font-black uppercase mb-4 border-b-4 border-black pb-2 inline-block">Ringkasan Eksekutif</h4>
          <div class="grid grid-cols-2 gap-4 mb-8">
            <div class="bg-white border-4 border-black p-4 shadow-neoHover">
              <p class="text-xs font-bold text-gray-500 uppercase">Total Utilisasi Tahun Ini</p>
              <p class="text-3xl font-black mt-2">3,455 <span class="text-sm font-bold text-gray-500">Jam Operasi</span></p>
            </div>
            <div class="bg-white border-4 border-black p-4 shadow-neoHover">
              <p class="text-xs font-bold text-gray-500 uppercase">Proyeksi Cost Reduction</p>
              <p class="text-3xl font-black text-emerald-600 mt-2">12.5% <span class="text-sm font-bold text-gray-500">MOM</span></p>
            </div>
          </div>
          <h4 class="text-xl font-black uppercase mb-4">Breakdown Status Armada</h4>
          <table class="w-full border-collapse border-4 border-black bg-white shadow-neo">
            <thead>
              <tr class="bg-black text-white">
                <th class="p-3 text-left font-black uppercase text-xs border-r border-white/20">Kategori Status</th>
                <th class="p-3 text-left font-black uppercase text-xs border-r border-white/20">Persentase</th>
                <th class="p-3 text-left font-black uppercase text-xs">Aksi Lanjutan Rekomendasi</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(item, index) in statusDistribution" :key="index" class="border-b-4 border-black hover:bg-gray-50 transition-colors">
                <td class="p-3 font-black">
                  <span :class="[item.color, 'px-2 py-1 border-2 border-black text-xs shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] text-black']">
                    {{ item.label }}
                  </span>
                </td>
                <td class="p-3 font-black text-xl">{{ item.jumlah }}/{{ dashboardKPI.totalUnits }}</td>
                <td class="p-3 font-bold text-sm text-gray-700">
                  {{ item.label === 'Sehat' ? 'Pertahankan jadwal maintenance rutin.' : 
                     item.label === 'Warning' ? 'Lakukan pengecekan dalam 48 jam ke depan.' : 
                     item.label === 'Critical' ? 'Segera jadwalkan overhaul, stop operasi.' : 'Tunggu suku cadang dari supplier utama.' }}
                </td>
              </tr>
            </tbody>
          </table>
          <div class="mt-8 flex justify-end">
             <button @click="closeReportModal" class="bg-miningYellow text-black border-4 border-black px-6 py-3 font-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:translate-y-1 hover:shadow-none transition-all flex items-center gap-2">
               TUTUP / UNDUH PDF
             </button>
          </div>
        </div>
      </div>
    </div>

    <div v-if="isMonthDetailModalOpen && selectedMonthData" class="fixed inset-0 z-[100] flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="closeMonthDetail"></div>
      <div class="bg-white border-4 border-black shadow-neo w-full max-w-4xl relative z-10 flex flex-col max-h-[90vh] animate-[popup_0.2s_ease-out]">
        
        <div class="flex justify-between items-center p-6 border-b-4 border-black bg-miningYellow">
          <h3 class="text-3xl font-black uppercase text-black">Detail Unit Bulan {{ selectedMonthData.month }}</h3>
          <button @click="closeMonthDetail" class="bg-neoRed text-white border-2 border-black w-10 h-10 font-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:shadow-none hover:translate-x-[2px] hover:translate-y-[2px] transition-all">X</button>
        </div>
        
        <div class="p-6 overflow-y-auto bg-white flex flex-col gap-6">
          <div class="border-4 border-black shadow-neoHover p-4 relative">
             <div class="absolute -top-4 left-4 bg-[#34d399] border-2 border-black px-3 py-1 font-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">SEHAT ({{ selectedMonthData.sehat.val }}%)</div>
             <p class="font-bold text-gray-700 mt-4 leading-relaxed">
               {{ selectedMonthData.sehat.units.length > 0 ? selectedMonthData.sehat.units.join(', ') : 'Tidak ada unit yang terdata dalam kategori ini.' }}
             </p>
             <p class="text-xs font-black mt-4 text-emerald-700 uppercase pt-2 border-t-2 border-dashed border-gray-300">Total: {{ selectedMonthData.sehat.units.length }} Armada</p>
          </div>

          <div class="border-4 border-black shadow-neoHover p-4 relative">
             <div class="absolute -top-4 left-4 bg-[#facc15] border-2 border-black px-3 py-1 font-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">WARNING ({{ selectedMonthData.warning.val }}%)</div>
             <p class="font-bold text-gray-700 mt-4 leading-relaxed">
               {{ selectedMonthData.warning.units.length > 0 ? selectedMonthData.warning.units.join(', ') : 'Tidak ada unit yang terdata dalam kategori ini.' }}
             </p>
             <p class="text-xs font-black mt-4 text-yellow-700 uppercase pt-2 border-t-2 border-dashed border-gray-300">Total: {{ selectedMonthData.warning.units.length }} Armada</p>
          </div>

          <div class="border-4 border-black shadow-neoHover p-4 relative">
             <div class="absolute -top-4 left-4 bg-[#f87171] border-2 border-black px-3 py-1 font-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] text-white">CRITICAL ({{ selectedMonthData.critical.val }}%)</div>
             <p class="font-bold text-gray-700 mt-4 leading-relaxed">
               {{ selectedMonthData.critical.units.length > 0 ? selectedMonthData.critical.units.join(', ') : 'Tidak ada unit yang terdata dalam kategori ini.' }}
             </p>
             <p class="text-xs font-black mt-4 text-red-700 uppercase pt-2 border-t-2 border-dashed border-gray-300">Total: {{ selectedMonthData.critical.units.length }} Armada</p>
          </div>
        </div>

      </div>
    </div>
    
  </div>
</template>

<style>
@import url('https://unpkg.com/leaflet@1.9.4/dist/leaflet.css');
@import url('https://fonts.googleapis.com/css2?family=Public+Sans:wght@900&family=Space+Mono:wght@400;700&display=swap');

body {
  font-family: 'Space Mono', monospace;
}

h1, h2, h3, h4, button, .font-black {
  font-family: 'Public Sans', sans-serif;
  font-weight: 900;
}

.leaflet-container {
  z-index: 1 !important;
  font-family: inherit;
}

.custom-leaflet-popup .leaflet-popup-content-wrapper {
  background: transparent;
  box-shadow: none;
  padding: 0;
}
.custom-leaflet-popup .leaflet-popup-content {
  margin: 0;
  width: auto !important; 
}
.custom-leaflet-popup .leaflet-popup-tip {
  display: none;
}

@keyframes popup {
  0% { transform: scale(0.95) translateY(10px); opacity: 0; }
  100% { transform: scale(1) translateY(0); opacity: 1; }
}

.animate-progress-grow {
  transform-origin: left;
  animation: scaleXRight 0.8s ease-out forwards;
}

@keyframes scaleXRight {
  0% { transform: scaleX(0); }
  100% { transform: scaleX(1); }
}

/* Modifikasi bawaan default Chart.js Canvas Container jika diperlukan */
canvas {
  width: 100% !important;
  height: 100% !important;
}
</style>