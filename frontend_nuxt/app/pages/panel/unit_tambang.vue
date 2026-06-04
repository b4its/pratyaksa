<script setup lang="ts">
import { ref, computed } from 'vue'

// --- LOGIKA SIDEBAR NAVIGASI ---
// Mengubah tipe menu menjadi object agar memiliki link/path masing-masing
interface MenuItem {
  name: string
  path: string
}

const menuItems = ref<MenuItem[]>([
  { name: 'Dashboard', path: '/panel/dashboard' },
  { name: 'Jenis Alat Berat', path: '/panel/jenis-alat' },
  { name: 'Unit Tambang', path: '/panel/unit-tambang' },
  { name: 'Analisa Kerusakan', path: '/panel/analisa' },
  { name: 'Kembali', path: '../' },
])

const activeMenu = ref('Unit Tambang') // Default menu yang aktif

const setActiveMenu = (menu: MenuItem) => {
  activeMenu.value = menu.name
  
  // LOGIKA ROUTING NUXT 3
  // Hapus tanda komentar (//) pada baris di bawah ini jika file pages/ rute sudah Anda buat.
  // navigateTo(menu.path)
}

// Tipe Data Unit
interface Unit {
  id: number
  code: string
  type: string
  status: string
  health: number
  maintenance: string
  savings: number
  img: string
}

// Dummy Data JSON
const units = ref<Unit[]>([
  { id: 1, code: 'EXC-320-01', type: 'Caterpillar Excavator 320', status: 'SEHAT', health: 96, maintenance: '120 Jam Lagi', savings: 2456, img: 'https://placehold.co/400x250?text=Excavator' },
  { id: 2, code: 'DT-SCN-12', type: 'Scania Dump Truck P410', status: 'WARNING', health: 78, maintenance: '35 Jam Lagi', savings: 820, img: 'https://placehold.co/400x250?text=Dump+Truck' },
  { id: 3, code: 'DZ-KMTS-08', type: 'Komatsu Dozer D85A', status: 'CRITICAL', health: 42, maintenance: '8 Jam Lagi (Segera)', savings: -4150, img: 'https://placehold.co/400x250?text=Dozer' },
  { id: 4, code: 'WL-SDLG-03', type: 'SDLG Wheel Loader LG956', status: 'RUSAK', health: 15, maintenance: 'Sedang Perbaikan', savings: -18000, img: 'https://placehold.co/400x250?text=Loader' },
  { id: 5, code: 'EXC-320-02', type: 'Caterpillar Excavator 320', status: 'SEHAT', health: 92, maintenance: '110 Jam Lagi', savings: 1200, img: 'https://placehold.co/400x250?text=Excavator+2' },
  { id: 6, code: 'DT-SCN-15', type: 'Scania Dump Truck P410', status: 'SEHAT', health: 88, maintenance: '80 Jam Lagi', savings: 450, img: 'https://placehold.co/400x250?text=Dump+Truck+2' },
])

// --- LOGIKA PAGINATION ---
const currentPage = ref(1)
const itemsPerPage = 5

const totalPages = computed(() => Math.ceil(units.value.length / itemsPerPage))

const paginatedUnits = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage
  const end = start + itemsPerPage
  return units.value.slice(start, end)
})

const prevPage = () => { if (currentPage.value > 1) currentPage.value-- }
const nextPage = () => { if (currentPage.value < totalPages.value) currentPage.value++ }
const goToPage = (page: number) => { currentPage.value = page }

// LOGIKA BARU: Batasan Halaman yang Tampil (Sliding Window)
const maxVisiblePages = 3 // Jumlah maksimal tombol angka yang tampil
const visiblePages = computed(() => {
  const pages = []
  if (totalPages.value <= maxVisiblePages) {
    for (let i = 1; i <= totalPages.value; i++) {
      pages.push(i)
    }
  } else {
    let start = Math.max(1, currentPage.value - Math.floor(maxVisiblePages / 2))
    let end = start + maxVisiblePages - 1

    if (end > totalPages.value) {
      end = totalPages.value
      start = Math.max(1, end - maxVisiblePages + 1)
    }

    for (let i = start; i <= end; i++) {
      pages.push(i)
    }
  }
  return pages
})

// --- LOGIKA MODAL VIEW ---
const isModalOpen = ref(false)
const selectedUnit = ref<Unit | null>(null)

const openModal = (unit: Unit) => {
  selectedUnit.value = unit
  isModalOpen.value = true
}

const closeModal = () => {
  isModalOpen.value = false
  setTimeout(() => { selectedUnit.value = null }, 200)
}

// Helper untuk warna status
const getStatusColor = (status: string) => {
  switch (status) {
    case 'SEHAT': return 'bg-emerald-400'
    case 'WARNING': return 'bg-miningYellow'
    case 'CRITICAL': return 'bg-neoRed text-white'
    case 'RUSAK': return 'bg-gray-400'
    default: return 'bg-white'
  }
}
</script>

<template>
  <div class="flex min-h-screen bg-[#F1F1F1] font-mono text-black relative">
    
    <aside class="w-72 border-r-4 border-black bg-white p-6 flex flex-col justify-between z-10">
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

      <div class="border-4 border-black p-4 bg-white shadow-neo">
        <p class="text-[10px] font-black uppercase text-gray-500">Logged in as</p>
        <div class="flex items-center gap-3 mt-1">
          <div class="w-10 h-10 border-2 border-black bg-neoBlue shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]"></div>
          <p class="font-black text-sm">Admin Suki</p>
        </div>
      </div>
    </aside>

    <main class="flex-1 p-8 overflow-y-auto">
      <header class="flex justify-between items-start mb-10">
        <div>
          <h1 class="text-6xl font-black uppercase tracking-tighter leading-none">{{ activeMenu }}</h1>
          <p class="mt-2 font-bold text-gray-600 bg-white border-b-2 border-black inline-block">Unit yang beroperasi saat ini dan masih aktif berjalan.</p>
        </div>
        <div class="flex items-center gap-3 p-2 border-4 border-black bg-white shadow-neoHover">
          <div class="w-8 h-8 rounded-full border-2 border-black bg-neoBlue"></div>
          <span class="font-black text-sm">Admin Suki</span>
        </div>
      </header>

      <div class="flex justify-between items-center mb-8 gap-4">
        <div class="flex gap-4 flex-1">
          <div class="relative flex-1 max-w-md">
            <input type="text" placeholder="Cari Armada..." class="w-full p-4 border-4 border-black shadow-neoHover focus:outline-none focus:bg-miningYellow/10 font-bold placeholder:text-gray-400" />
          </div>
          <select class="p-4 border-4 border-black bg-white font-black shadow-neoHover appearance-none px-10">
            <option>STATUS UNIT</option>
          </select>
        </div>
        <button class="bg-black text-white p-4 font-black shadow-[4px_4px_0px_0px_#FFCC00] flex items-center gap-2 hover:translate-x-1 hover:translate-y-1 hover:shadow-none transition-all border-2 border-black">
          EKSPORT DATA <span class="text-miningYellow">↓</span>
        </button>
      </div>

      <section class="border-4 border-black bg-white shadow-neo mb-12 overflow-hidden">
        <table class="w-full border-collapse">
          <thead>
            <tr class="bg-black text-white">
              <th v-for="h in ['Kode Unik', 'Jenis Alat Berat', 'Status', 'Health Score', 'Jadwal', 'Saving', 'Aksi']" :key="h" class="p-4 text-left font-black uppercase text-xs tracking-widest border-r border-white/20">{{ h }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="unit in paginatedUnits" :key="unit.code" class="border-b-4 border-black hover:bg-gray-50 transition-colors">
              <td class="p-4 font-black italic">{{ unit.code }}</td>
              <td class="p-4 font-bold">{{ unit.type }}</td>
              <td class="p-4">
                <span :class="getStatusColor(unit.status)" class="px-3 py-1 border-2 border-black font-black text-[10px] shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">
                  {{ unit.status }}
                </span>
              </td>
              <td class="p-4 font-black">{{ unit.health }}%</td>
              <td class="p-4 font-bold text-xs">{{ unit.maintenance }}</td>
              <td class="p-4 font-black" :class="unit.savings > 0 ? 'text-emerald-600' : 'text-neoRed'">
                {{ unit.savings > 0 ? '+$' : '-$' }}{{ Math.abs(unit.savings).toLocaleString() }}
              </td>
              <td class="p-4">
                <button @click="openModal(unit)" class="bg-miningYellow border-2 border-black px-4 py-1 font-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:shadow-none hover:translate-x-[1px] hover:translate-y-[1px] transition-all">VIEW</button>
              </td>
            </tr>
          </tbody>
        </table>

<div class="p-4 border-t-4 border-black bg-white flex justify-end gap-2 flex-wrap">
  <!-- Tombol Halaman Pertama (First) -->
  <button 
    @click="goToPage(1)" 
    :disabled="currentPage === 1"
    class="w-10 h-10 border-2 border-black flex items-center justify-center font-black hover:bg-black hover:text-white disabled:opacity-50 disabled:hover:bg-white disabled:hover:text-black transition-colors"
    title="Halaman Pertama"
  >
    &laquo;
  </button>

  <!-- Tombol Prev (<) -->
  <button 
    @click="prevPage" 
    :disabled="currentPage === 1"
    class="w-10 h-10 border-2 border-black flex items-center justify-center font-black hover:bg-black hover:text-white disabled:opacity-50 disabled:hover:bg-white disabled:hover:text-black transition-colors"
  >
    &lt;
  </button>
  
  <!-- Tombol Angka Dinamis (Maksimal 5) -->
  <button 
    v-for="page in visiblePages" 
    :key="page" 
    @click="goToPage(page)"
    :class="currentPage === page ? 'bg-black text-white' : 'bg-white text-black hover:bg-black hover:text-white'"
    class="w-10 h-10 border-2 border-black flex items-center justify-center font-black transition-colors"
  >
    {{ page }}
  </button>

  <!-- Tombol Next (>) -->
  <button 
    @click="nextPage" 
    :disabled="currentPage === totalPages"
    class="w-10 h-10 border-2 border-black flex items-center justify-center font-black hover:bg-black hover:text-white disabled:opacity-50 disabled:hover:bg-white disabled:hover:text-black transition-colors"
  >
    &gt;
  </button>

  <!-- Tombol Halaman Terakhir (Last) -->
  <button 
    @click="goToPage(totalPages)" 
    :disabled="currentPage === totalPages"
    class="w-10 h-10 border-2 border-black flex items-center justify-center font-black hover:bg-black hover:text-white disabled:opacity-50 disabled:hover:bg-white disabled:hover:text-black transition-colors"
    title="Halaman Terakhir"
  >
    &raquo;
  </button>
</div>
      </section>

      <h2 class="text-4xl font-black mb-6 uppercase">Visual Armada</h2>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
        <div v-for="unit in paginatedUnits" :key="'card-'+unit.id" class="bg-white border-4 border-black p-4 shadow-neo group hover:-translate-y-2 transition-all">
          <div class="flex justify-between items-start mb-4">
            <div class="bg-black text-white px-2 py-1 text-xs font-black shadow-[2px_2px_0px_0px_#FFCC00]">{{ unit.health }}%</div>
            <div :class="getStatusColor(unit.status)" class="border-2 border-black px-2 py-1 text-[10px] font-black">{{ unit.status }}</div>
          </div>
          
          <p class="text-miningYellow font-black text-xs mb-2">{{ unit.code }}</p>
          
          <div class="border-4 border-black mb-4 bg-gray-100 overflow-hidden shadow-neoHover">
            <img :src="unit.img" class="w-full h-32 object-cover grayscale group-hover:grayscale-0 transition-all" />
          </div>

          <p class="font-black text-sm mb-4 h-10 leading-tight uppercase">{{ unit.type }}</p>
          
          <button @click="openModal(unit)" class="w-full bg-miningYellow border-4 border-black p-2 font-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] active:shadow-none active:translate-x-1 active:translate-y-1 transition-all">
            VIEW UNIT
          </button>
        </div>
      </div>
    </main>

    <div v-if="isModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="closeModal"></div>
      
      <div class="bg-white border-4 border-black shadow-neo w-full max-w-2xl relative z-10 flex flex-col max-h-[90vh] animate-[popup_0.2s_ease-out]">
        <div class="flex justify-between items-center p-6 border-b-4 border-black bg-miningYellow">
          <h3 class="text-3xl font-black uppercase">Detail Unit</h3>
          <button @click="closeModal" class="bg-neoRed text-white border-2 border-black w-10 h-10 font-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:shadow-none hover:translate-x-[2px] hover:translate-y-[2px] transition-all">
            X
          </button>
        </div>

        <div class="p-6 overflow-y-auto" v-if="selectedUnit">
          <div class="flex flex-col md:flex-row gap-6 mb-6">
            <div class="w-full md:w-1/2 border-4 border-black bg-gray-100 shadow-neoHover">
              <img :src="selectedUnit.img" class="w-full h-full object-cover" />
            </div>
            <div class="w-full md:w-1/2 flex flex-col gap-4 justify-center">
              <div>
                <p class="text-xs font-bold text-gray-500 uppercase">Kode Unik</p>
                <p class="text-2xl font-black italic">{{ selectedUnit.code }}</p>
              </div>
              <div>
                <p class="text-xs font-bold text-gray-500 uppercase">Tipe Alat Berat</p>
                <p class="text-xl font-bold">{{ selectedUnit.type }}</p>
              </div>
              <div>
                <span :class="getStatusColor(selectedUnit.status)" class="px-3 py-1 border-2 border-black font-black text-sm shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] inline-block">
                  Status: {{ selectedUnit.status }}
                </span>
              </div>
            </div>
          </div>

          <div class="grid grid-cols-3 gap-4 border-t-4 border-black pt-6">
            <div class="border-2 border-black p-4 text-center shadow-[4px_4px_0px_0px_rgba(0,0,0,1)]">
              <p class="text-xs font-bold uppercase mb-1">Health Score</p>
              <p class="text-3xl font-black">{{ selectedUnit.health }}%</p>
            </div>
            <div class="border-2 border-black p-4 text-center shadow-[4px_4px_0px_0px_rgba(0,0,0,1)]">
              <p class="text-xs font-bold uppercase mb-1">Jadwal MTC</p>
              <p class="text-lg font-black mt-2 leading-tight">{{ selectedUnit.maintenance }}</p>
            </div>
            <div class="border-2 border-black p-4 text-center shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] bg-black text-white">
              <p class="text-xs font-bold uppercase mb-1 text-gray-300">Est. Saving</p>
              <p class="text-xl font-black mt-2" :class="selectedUnit.savings > 0 ? 'text-miningYellow' : 'text-neoRed'">
                 {{ selectedUnit.savings > 0 ? '+$' : '-$' }}{{ Math.abs(selectedUnit.savings).toLocaleString() }}
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
    
  </div>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Public+Sans:wght@900&family=Space+Mono:wght@400;700&display=swap');

body {
  font-family: 'Space Mono', monospace;
}

h1, h2, h3, button, .font-black {
  font-family: 'Public Sans', sans-serif;
  font-weight: 900;
}

/* Animasi Muncul Modal Pop-up */
@keyframes popup {
  0% { transform: scale(0.95) translateY(10px); opacity: 0; }
  100% { transform: scale(1) translateY(0); opacity: 1; }
}
</style>