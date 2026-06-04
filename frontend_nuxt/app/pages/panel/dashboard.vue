<script setup lang="ts">
import { ref, computed, onMounted, nextTick } from 'vue'

// --- LOGIKA SIDEBAR NAVIGASI ---
interface MenuItem {
  name: string
  path: string
}

const menuItems = ref<MenuItem[]>([
  { name: 'Dashboard', path: '/panel/dashboard' },
  { name: 'Jenis Alat Berat', path: '/panel/jenis-alat' },
  { name: 'Unit Tambang', path: '/panel/unit_tambang' },
  { name: 'Analisa Kerusakan', path: '/panel/analisa' },
  { name: 'Kembali', path: '../' },
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

const histogramData = ref([
  { month: 'Jan', value: 210, color: 'bg-neoBlue' },
  { month: 'Feb', value: 500, color: 'bg-miningYellow' },
  { month: 'Mar', value: 135, color: 'bg-neoRed' },
  { month: 'Apr', value: 800, color: 'bg-emerald-400' },
  { month: 'Mei', value: 1200, color: 'bg-neoBlue' },
  { month: 'Jun', value: 470, color: 'bg-miningYellow' },
  { month: 'Jul', value: 140, color: 'bg-neoRed' },
])

const maxHistogramValue = computed(() => {
  const max = Math.max(...histogramData.value.map(d => d.value))
  return max === 0 ? 1 : max
})

const statusDistribution = ref([
  { label: 'Sehat', jumlah: 65, color: 'bg-emerald-400' },
  { label: 'Warning', jumlah: 20, color: 'bg-miningYellow' },
  { label: 'Critical', jumlah: 10, color: 'bg-neoRed' },
  { label: 'Rusak Dalam Perbaikan', jumlah: 5, color: 'bg-gray-400' },
])

// --- LOGIKA MAP LEAFLET (DUMMY DATA KOORDINAT) ---
const mapLocations = ref([
  { id: 1, name: 'Pit Alpha - Excavator 320', lat: -0.4900, lng: 117.1400, status: 'Sehat' },
  { id: 2, name: 'Pit Beta - Dump Truck P410', lat: -0.5102, lng: 117.1536, status: 'Warning' },
  { id: 3, name: 'Pit Gamma - Dozer D85A', lat: -0.5250, lng: 117.1300, status: 'Critical' },
  { id: 4, name: 'Stockpile - Wheel Loader', lat: -0.4850, lng: 117.1650, status: 'Rusak' },
])

// Inisialisasi Peta (Hanya dieksekusi di sisi Klien)
onMounted(async () => {
  if (typeof window !== 'undefined') {
    // Import Leaflet secara dinamis untuk menghindari error SSR di Nuxt 3
    const L = (await import('leaflet')).default
    
    // Perbaikan URL Icon bawaan Leaflet untuk Webpack/Vite
    delete (L.Icon.Default.prototype as any)._getIconUrl;
    L.Icon.Default.mergeOptions({
      iconRetinaUrl: 'https://unpkg.com/leaflet@1.9.4/dist/images/marker-icon-2x.png',
      iconUrl: 'https://unpkg.com/leaflet@1.9.4/dist/images/marker-icon.png',
      shadowUrl: 'https://unpkg.com/leaflet@1.9.4/dist/images/marker-shadow.png',
    });

    await nextTick()
    
    // Titik tengah peta difokuskan ke Samarinda
    const map = L.map('mining-map').setView([-0.5022, 117.1536], 12)

    // Layer Peta Dasar (OpenStreetMap)
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
    }).addTo(map)

    // Memasukkan Data Dummy ke Peta
    mapLocations.value.forEach(loc => {
      const marker = L.marker([loc.lat, loc.lng]).addTo(map)
      
      // Kustomisasi Popup Leaflet agar agak selaras dengan desain
      marker.bindPopup(`
        <div style="font-family: 'Public Sans', sans-serif; padding: 4px;">
          <h4 style="font-weight: 900; font-size: 14px; text-transform: uppercase; margin-bottom: 4px;">${loc.name}</h4>
          <span style="font-weight: bold; background: #000; color: #FFCC00; padding: 2px 6px; font-size: 10px;">STATUS: ${loc.status}</span>
        </div>
      `)
    })
  }
})

// --- LOGIKA MODAL LAPORAN DASHBOARD ---
const isReportModalOpen = ref(false)

const openReportModal = () => {
  isReportModalOpen.value = true
}
const closeReportModal = () => {
  isReportModalOpen.value = false
}
</script>

<template>
  <div class="flex min-h-screen bg-[#F1F1F1] font-mono text-black relative">
    
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
            :href="`{item.path}`"
            :class="[
            'w-full flex items-center gap-3 p-3 border-2 font-bold group transition-all',
            activeMenu === item.name 
                ? 'border-black bg-miningYellow shadow-neoHover' 
                : 'border-transparent hover:border-black hover:bg-white hover:shadow-neoHover'
            ]"
        >
            <div v-if="activeMenu === item.name" class="p-1 border-2 border-black bg-white">
            <div class="w-4 h-4 bg-black"></div>
            </div>
            <div v-else class="w-4 h-4 border-2 border-black ml-1.5"></div>
            
            {{ item.name }}
        </NuxtLink>
        </nav>
      </div>

      <div class="border-4 border-black p-4 bg-white shadow-neo mt-8">
        <p class="text-[10px] font-black uppercase text-gray-500">Logged in as</p>
        <div class="flex items-center gap-3 mt-1">
          <div class="w-10 h-10 border-2 border-black bg-neoBlue shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] shrink-0"></div>
          <p class="font-black text-sm truncate">Admin Suki</p>
        </div>
      </div>
    </aside>

    <main class="flex-1 p-8 overflow-y-auto z-10 relative">
      <header class="flex justify-between items-start mb-10">
        <div>
          <h1 class="text-6xl font-black uppercase tracking-tighter leading-none">{{ activeMenu }}</h1>
          <p class="mt-2 font-bold text-gray-600 bg-white border-b-2 border-black inline-block">
            {{ activeMenu === 'Dashboard' ? 'Ringkasan data analitik armada.' : 'Halaman ini sedang dalam pengembangan atau dialihkan.' }}
          </p>
        </div>
        <div class="flex items-center gap-3 p-2 border-4 border-black bg-white shadow-neoHover shrink-0">
          <div class="w-8 h-8 rounded-full border-2 border-black bg-neoBlue"></div>
          <span class="font-black text-sm">Admin Suki</span>
        </div>
      </header>

      <div v-show="activeMenu === 'Dashboard'" class="space-y-8 animate-[popup_0.3s_ease-out]">
        
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
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
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
          
          <div class="lg:col-span-2 bg-white border-4 border-black shadow-neo p-6 flex flex-col">
            <div class="flex justify-between items-center mb-8 border-b-4 border-black pb-4">
              <h2 class="text-2xl font-black uppercase">Statistik Utilisasi</h2>
              <button class="border-2 border-black bg-miningYellow px-3 py-1 font-black text-xs shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:translate-y-px hover:shadow-none transition-all">Tahun Ini</button>
            </div>
            
            <div class="flex-1 flex items-end justify-between gap-2 h-64 mt-4">
              <div v-for="(bar, index) in histogramData" :key="bar.month" class="flex flex-col items-center flex-1 group h-full justify-end">
                <div class="w-full relative flex justify-center h-[calc(100%-40px)] items-end">
                  <div class="absolute -top-10 opacity-0 group-hover:opacity-100 bg-black text-white text-xs font-black py-1 px-2 pointer-events-none transition-opacity z-10">
                    {{ bar.value }}
                  </div>
                  <div 
                    :class="[bar.color, 'animate-bar-grow w-3/4 max-w-[50px] border-4 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] transition-colors duration-300 group-hover:brightness-110']"
                    :style="{ height: `${(bar.value / maxHistogramValue) * 100}%`, animationDelay: `${index * 0.1}s` }"
                  ></div>
                </div>
                <p class="font-black text-xs uppercase mt-2 border-t-2 border-black w-full text-center pt-2">{{ bar.month }}</p>
              </div>
            </div>
          </div>

          <div class="bg-white border-4 border-black shadow-neo p-6 flex flex-col">
            <h2 class="text-2xl font-black uppercase mb-6 border-b-4 border-black pb-4">Distribusi Status</h2>
            
            <div class="flex-1 flex flex-col justify-center gap-6">
              <div v-for="(item, index) in statusDistribution" :key="item.label">
                <div class="flex justify-between font-black text-sm mb-2 uppercase">
                  <span>{{ item.label }}</span>
                  <span>{{ item.jumlah }}/{{ dashboardKPI.totalUnits }}</span>
                </div>
                <div class="w-full h-6 border-4 border-black bg-gray-200 shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] relative overflow-hidden">
                  <div 
                    :class="[item.color, 'h-full border-r-4 border-black animate-progress-grow']" 
                    :style="{ width: `${item.jumlah}%`, animationDelay: `${index * 0.1}s` }"
                  ></div>
                </div>
              </div>
            </div>
            
            <button @click="openReportModal" class="w-full mt-8 bg-black text-white border-4 border-black p-3 font-black shadow-[4px_4px_0px_0px_#FFCC00] hover:-translate-y-1 hover:shadow-[6px_6px_0px_0px_#FFCC00] transition-all uppercase">
              Lihat Detail Laporan
            </button>
          </div>
        </div>

        <div class="bg-white border-4 border-black shadow-neo p-6 mt-8 flex flex-col relative z-0">
          <div class="flex justify-between items-center mb-6 border-b-4 border-black pb-4">
            <h2 class="text-2xl font-black uppercase">Peta Persebaran Unit</h2>
          </div>
          <div id="mining-map" class="w-full h-96 border-4 border-black shadow-neoHover relative z-0"></div>
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
              <tr v-for="(item, index) in statusDistribution" :key="index" class="border-b-4 border-black">
                <td class="p-3 font-black">
                  <span :class="[item.color, 'px-2 py-1 border-2 border-black text-xs shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]']">
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
    
  </div>
</template>

<style>
/* Penting untuk mengimpor CSS Leaflet agar tampilan peta tidak berantakan */
@import url('https://unpkg.com/leaflet@1.9.4/dist/leaflet.css');
@import url('https://fonts.googleapis.com/css2?family=Public+Sans:wght@900&family=Space+Mono:wght@400;700&display=swap');

body {
  font-family: 'Space Mono', monospace;
}

h1, h2, h3, h4, button, .font-black {
  font-family: 'Public Sans', sans-serif;
  font-weight: 900;
}

/* Penyesuaian z-index khusus Leaflet untuk modal agar tidak menabrak z-index map bawaan leaflet */
.leaflet-container {
  z-index: 1 !important;
}

/* Animasi Muncul Konten Tab & Modal */
@keyframes popup {
  0% { transform: scale(0.95) translateY(10px); opacity: 0; }
  100% { transform: scale(1) translateY(0); opacity: 1; }
}

/* Animasi Chart Naik ke Atas (Sumbu Y) */
.animate-bar-grow {
  transform-origin: bottom;
  animation: scaleYUp 0.8s ease-out forwards;
}

@keyframes scaleYUp {
  0% { transform: scaleY(0); }
  100% { transform: scaleY(1); }
}

/* Animasi Progress Bar ke Kanan (Sumbu X) */
.animate-progress-grow {
  transform-origin: left;
  animation: scaleXRight 0.8s ease-out forwards;
}

@keyframes scaleXRight {
  0% { transform: scaleX(0); }
  100% { transform: scaleX(1); }
}
</style>