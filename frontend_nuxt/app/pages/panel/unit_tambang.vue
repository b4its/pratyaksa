<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

// Muat web component <model-viewer> (Google) untuk render GLB
useHead({
  script: [
    { type: 'module', src: 'https://ajax.googleapis.com/ajax/libs/model-viewer/3.3.0/model-viewer.min.js' },
  ],
})

// ---- Sidebar ----
const menuItems = [
  { name: 'Dashboard',       path: '/panel/dashboard',         icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><rect width="7" height="9" x="3" y="3"/><rect width="7" height="5" x="14" y="3"/><rect width="7" height="9" x="14" y="12"/><rect width="7" height="5" x="3" y="16"/></svg>` },
  { name: 'Jenis Alat Berat', path: '/panel/jenis_alat_berat', icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><path d="M10 17h4V5H2v12h3"/><path d="M20 17h2v-9l-2.5-3.5H14v12h3"/><path d="M14 6h4.5"/><circle cx="18.5" cy="17.5" r="2.5"/><circle cx="5.5" cy="17.5" r="2.5"/></svg>` },
  { name: 'Unit Tambang',    path: '/panel/unit_tambang',      icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 12 12 17 22 12"/><polyline points="2 17 12 22 22 17"/></svg>` },
  { name: 'Analisa Kerusakan', path: '/panel/analisa',         icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>` },
  { name: 'Kembali',         path: '../',                       icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><path d="m12 19-7-7 7-7"/><path d="M19 12H5"/></svg>` },
]
const activeMenu = ref('Unit Tambang')

// ---- Types ----
interface UnitTambang {
  id: string
  code: string
  jenis_alat_berat_id: string
  jenis_alat_berat_nama: string | null
  status: string
  health: number
  maintenance: string
  savings: number
  img_url: string | null
  model3d_url: string | null
  created_at: string
  updated_at: string
}

interface JenisOption { id: string; nama: string }

// ---- State ----
const units = ref<UnitTambang[]>([])
const total = ref(0)
const currentPage = ref(1)
const perPage = 10
const searchQuery = ref('')
const filterStatus = ref('')
const isLoading = ref(false)
const errorMsg = ref('')

const jenisOptions = ref<JenisOption[]>([])
const statusOptions = ['SEHAT', 'WARNING', 'CRITICAL', 'RUSAK']
const totalPages = computed(() => Math.ceil(total.value / perPage))

// ---- Modal ----
const isDetailOpen = ref(false)
const selectedUnit = ref<UnitTambang | null>(null)
const isFormOpen = ref(false)
const formMode = ref<'add' | 'edit'>('add')
const formData = ref<any>({})
const formError = ref('')
const formLoading = ref(false)
const isExportOpen = ref(false)

const { initAuth, user } = useAuth()
const api = useApi()

// ---- Fetch Units ----
const fetchUnits = async () => {
  isLoading.value = true
  errorMsg.value = ''
  try {
    const res = await api.getUnitTambang({
      page: currentPage.value,
      per_page: perPage,
      search: searchQuery.value || undefined,
      status: filterStatus.value || undefined,
    }) as any
    units.value = res.data.data
    total.value = res.data.total
  } catch (e: any) {
    errorMsg.value = e?.data?.message || 'Gagal memuat data unit.'
  } finally {
    isLoading.value = false
  }
}

// ---- Fetch Jenis Options (for form dropdown) ----
const fetchJenisOptions = async () => {
  try {
    const res = await api.getJenisAlatBerat({ per_page: 100 }) as any
    jenisOptions.value = res.data.data.map((j: any) => ({ id: j.id, nama: j.nama }))
  } catch { /* ignore */ }
}

onMounted(() => {
  initAuth()
  fetchUnits()
  fetchJenisOptions()
})

const onSearch = () => { currentPage.value = 1; fetchUnits() }
const onFilterStatus = () => { currentPage.value = 1; fetchUnits() }

// ---- CRUD ----
const openDetail = (unit: UnitTambang) => { selectedUnit.value = unit; isDetailOpen.value = true }

const openAdd = () => {
  formMode.value = 'add'
  formData.value = {
    code: '', jenis_alat_berat_id: jenisOptions.value[0]?.id || '',
    status: 'SEHAT', health: 100, maintenance: '', savings: 0,
    img_url: 'https://placehold.co/400x250?text=Unit+Baru',
    model3d_url: 'https://modelviewer.dev/shared-assets/models/RobotExpressive.glb'
  }
  formError.value = ''
  isFormOpen.value = true
}

const openEdit = (unit: UnitTambang) => {
  formMode.value = 'edit'
  formData.value = {
    id: unit.id, code: unit.code,
    jenis_alat_berat_id: unit.jenis_alat_berat_id,
    status: unit.status, health: unit.health,
    maintenance: unit.maintenance, savings: unit.savings,
    img_url: unit.img_url || '', model3d_url: unit.model3d_url || ''
  }
  formError.value = ''
  isFormOpen.value = true
}

const save = async () => {
  if (!formData.value.code?.trim()) { formError.value = 'Kode unit wajib diisi.'; return }
  if (!formData.value.jenis_alat_berat_id) { formError.value = 'Jenis alat berat wajib dipilih.'; return }
  formLoading.value = true
  formError.value = ''
  try {
    const payload = {
      code: formData.value.code,
      jenis_alat_berat_id: formData.value.jenis_alat_berat_id,
      status: formData.value.status,
      health: Number(formData.value.health),
      maintenance: formData.value.maintenance,
      savings: Number(formData.value.savings),
      img_url: formData.value.img_url || undefined,
      model3d_url: formData.value.model3d_url || undefined,
    }
    if (formMode.value === 'add') {
      await api.createUnitTambang(payload)
    } else {
      await api.updateUnitTambang(formData.value.id, payload)
    }
    isFormOpen.value = false
    await fetchUnits()
  } catch (e: any) {
    formError.value = e?.data?.message || 'Gagal menyimpan.'
  } finally {
    formLoading.value = false
  }
}

const remove = async (unit: UnitTambang) => {
  if (!confirm(`Hapus unit "${unit.code}"?`)) return
  try {
    await api.deleteUnitTambang(unit.id)
    await fetchUnits()
  } catch (e: any) {
    alert(e?.data?.message || 'Gagal menghapus.')
  }
}

// ---- Helpers ----
const getStatusColor = (s: string) => ({ SEHAT: 'bg-emerald-400', WARNING: 'bg-miningYellow', CRITICAL: 'bg-neoRed text-white', RUSAK: 'bg-gray-400' }[s] || 'bg-white')

const visiblePages = computed(() => {
  const max = 3, tp = totalPages.value, cp = currentPage.value
  if (tp <= max) return Array.from({ length: tp }, (_, i) => i + 1)
  let start = Math.max(1, cp - 1)
  let end = Math.min(tp, start + max - 1)
  if (end === tp) start = Math.max(1, tp - max + 1)
  return Array.from({ length: end - start + 1 }, (_, i) => start + i)
})

// CSV export
const exportCSV = () => {
  const headers = ['Kode', 'Jenis', 'Status', 'Health', 'Maintenance', 'Savings']
  const rows = units.value.map(u => [u.code, u.jenis_alat_berat_nama || '', u.status, u.health, u.maintenance, u.savings])
  const csv = [headers, ...rows].map(r => r.join(',')).join('\n')
  const blob = new Blob([csv], { type: 'text/csv' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a'); a.href = url; a.download = 'unit_tambang.csv'; a.click()
  URL.revokeObjectURL(url)
  isExportOpen.value = false
}
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
          <NuxtLink v-for="item in menuItems" :key="item.name" :to="item.path"
            @click="activeMenu = item.name"
            :class="['w-full flex items-center gap-3 p-3 border-2 font-bold transition-all cursor-pointer',
              activeMenu === item.name ? 'border-black bg-miningYellow shadow-neoHover' : 'border-transparent hover:border-black hover:bg-white hover:shadow-neoHover']">
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
      <header class="flex justify-between items-start mb-10">
        <div>
          <h1 class="text-6xl font-black uppercase tracking-tighter leading-none">Unit Tambang</h1>
          <p class="mt-2 font-bold text-gray-600 bg-white border-b-2 border-black inline-block">Unit yang beroperasi saat ini dan masih aktif berjalan.</p>
        </div>
        <div class="flex items-center gap-3 p-2 border-4 border-black bg-white shadow-neoHover">
          <div class="w-8 h-8 rounded-full border-2 border-black bg-neoBlue"></div>
          <span class="font-black text-sm">{{ user?.name || 'Admin' }}</span>
        </div>
      </header>

      <div v-if="errorMsg" class="mb-6 p-4 bg-neoRed text-white border-4 border-black font-bold shadow-neo">⚠️ {{ errorMsg }}</div>

      <!-- Actions Bar -->
      <div class="flex justify-between items-center mb-8 gap-4 flex-wrap">
        <div class="flex gap-4 flex-1 flex-wrap">
          <input v-model="searchQuery" @keyup.enter="onSearch" type="text" placeholder="Cari kode / nama unit..."
            class="flex-1 min-w-48 p-4 border-4 border-black shadow-neoHover focus:outline-none focus:bg-miningYellow/10 font-bold placeholder:text-gray-400" />
          <select v-model="filterStatus" @change="onFilterStatus"
            class="p-4 border-4 border-black bg-white font-black shadow-neoHover appearance-none cursor-pointer px-6">
            <option value="">Semua Status</option>
            <option v-for="s in statusOptions" :key="s" :value="s">{{ s }}</option>
          </select>
          <button @click="onSearch" class="p-4 border-4 border-black bg-white font-black shadow-neoHover hover:translate-x-1 hover:translate-y-1 hover:shadow-none transition-all px-8">CARI</button>
          <button @click="isExportOpen = true" class="bg-black text-white p-4 font-black shadow-[4px_4px_0px_0px_#FFCC00] flex items-center gap-2 hover:translate-x-1 hover:translate-y-1 hover:shadow-none transition-all border-2 border-black">
            EKSPORT <span class="text-miningYellow">↓</span>
          </button>
        </div>
        <button @click="openAdd" class="bg-emerald-400 text-black p-4 font-black shadow-[4px_4px_0px_0px_#000] flex items-center gap-2 hover:translate-x-1 hover:translate-y-1 hover:shadow-none transition-all border-4 border-black">
          + TAMBAH UNIT
        </button>
      </div>

      <!-- Table -->
      <section class="border-4 border-black bg-white shadow-neo overflow-hidden">
        <table class="w-full border-collapse">
          <thead>
            <tr class="bg-black text-white">
              <th v-for="h in ['Kode Unik','Nama Unit','Status','Health','Jadwal','Saving','Aksi']" :key="h"
                class="p-4 text-left font-black uppercase text-xs tracking-widest border-r border-white/20">{{ h }}</th>
            </tr>
          </thead>
          <tbody>
            <template v-if="isLoading">
              <tr v-for="i in 5" :key="i" class="border-b-4 border-black">
                <td v-for="j in 7" :key="j" class="p-4"><div class="h-4 bg-gray-200 animate-pulse rounded"></div></td>
              </tr>
            </template>
            <tr v-else-if="units.length === 0">
              <td colspan="7" class="p-12 text-center font-bold text-gray-400">
                {{ searchQuery || filterStatus ? 'Tidak ada unit yang cocok.' : 'Belum ada unit. Klik + TAMBAH UNIT.' }}
              </td>
            </tr>
            <tr v-else v-for="unit in units" :key="unit.id" class="border-b-4 border-black hover:bg-gray-50 transition-colors">
              <td class="p-4 font-black italic">{{ unit.code }}</td>
              <td class="p-4 font-bold text-sm">{{ unit.jenis_alat_berat_nama || '—' }}</td>
              <td class="p-4">
                <span :class="getStatusColor(unit.status)" class="px-2 py-1 border-2 border-black font-black text-[10px] shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">{{ unit.status }}</span>
              </td>
              <td class="p-4 font-black">{{ unit.health }}%</td>
              <td class="p-4 font-bold text-xs">{{ unit.maintenance }}</td>
              <td class="p-4 font-black text-sm" :class="unit.savings >= 0 ? 'text-emerald-600' : 'text-neoRed'">
                {{ unit.savings >= 0 ? '+$' : '-$' }}{{ Math.abs(unit.savings).toLocaleString() }}
              </td>
              <td class="p-4 flex gap-2 flex-wrap">
                <button @click="openDetail(unit)" class="bg-miningYellow border-2 border-black px-3 py-1 font-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:shadow-none hover:translate-x-[1px] hover:translate-y-[1px] transition-all text-xs">VIEW</button>
                <button @click="openEdit(unit)" class="bg-white border-2 border-black px-3 py-1 font-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:shadow-none hover:translate-x-[1px] hover:translate-y-[1px] transition-all text-xs">EDIT</button>
                <button @click="remove(unit)" class="bg-neoRed text-white border-2 border-black px-3 py-1 font-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:shadow-none hover:translate-x-[1px] hover:translate-y-[1px] transition-all text-xs">DEL</button>
              </td>
            </tr>
          </tbody>
        </table>

        <!-- Pagination -->
        <div class="p-4 border-t-4 border-black bg-white flex justify-between items-center flex-wrap gap-2">
          <span class="font-bold text-sm text-gray-500">Total: {{ total }} unit</span>
          <div v-if="totalPages > 1" class="flex gap-2">
            <button @click="currentPage=1; fetchUnits()" :disabled="currentPage===1" class="w-10 h-10 border-2 border-black flex items-center justify-center font-black hover:bg-black hover:text-white disabled:opacity-50 transition-colors">&laquo;</button>
            <button @click="currentPage--; fetchUnits()" :disabled="currentPage===1" class="w-10 h-10 border-2 border-black flex items-center justify-center font-black hover:bg-black hover:text-white disabled:opacity-50 transition-colors">&lt;</button>
            <button v-for="page in visiblePages" :key="page" @click="currentPage=page; fetchUnits()"
              :class="currentPage===page ? 'bg-black text-white' : 'bg-white hover:bg-black hover:text-white'"
              class="w-10 h-10 border-2 border-black flex items-center justify-center font-black transition-colors">{{ page }}</button>
            <button @click="currentPage++; fetchUnits()" :disabled="currentPage===totalPages" class="w-10 h-10 border-2 border-black flex items-center justify-center font-black hover:bg-black hover:text-white disabled:opacity-50 transition-colors">&gt;</button>
            <button @click="currentPage=totalPages; fetchUnits()" :disabled="currentPage===totalPages" class="w-10 h-10 border-2 border-black flex items-center justify-center font-black hover:bg-black hover:text-white disabled:opacity-50 transition-colors">&raquo;</button>
          </div>
        </div>
      </section>
    </main>

    <!-- Detail Modal -->
    <div v-if="isDetailOpen && selectedUnit" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="isDetailOpen = false"></div>
      <div class="bg-white border-4 border-black shadow-neo w-full max-w-2xl relative z-10 flex flex-col max-h-[90vh] animate-[popup_0.2s_ease-out]">
        <div class="flex justify-between items-center p-6 border-b-4 border-black bg-miningYellow">
          <h3 class="text-3xl font-black uppercase">Detail Unit</h3>
          <button @click="isDetailOpen = false" class="bg-neoRed text-white border-2 border-black w-10 h-10 font-black hover:shadow-none transition-all">X</button>
        </div>
        <div class="p-6 overflow-y-auto">
          <div class="flex flex-col md:flex-row gap-6 mb-6">
            <div class="w-full md:w-1/2 border-4 border-black bg-gray-100 shadow-neoHover flex items-center justify-center min-h-[200px] relative overflow-hidden cursor-grab active:cursor-grabbing">
              <model-viewer
                v-if="selectedUnit.model3d_url"
                :key="selectedUnit.id"
                :src="selectedUnit.model3d_url"
                alt="Model 3D unit alat berat"
                camera-controls
                auto-rotate
                shadow-intensity="1"
                class="w-full h-full min-h-[200px] outline-none"
                style="background-color:#f3f4f6;"
              ></model-viewer>
              <img v-else-if="selectedUnit.img_url" :src="selectedUnit.img_url" class="w-full h-full object-cover" />
              <span v-else class="text-gray-400 font-bold">No Visual</span>
              <div v-if="selectedUnit.model3d_url" class="absolute bottom-2 right-2 bg-black text-white text-[8px] font-black px-1 pointer-events-none">DRAG 360°</div>
            </div>
            <div class="w-full md:w-1/2 flex flex-col gap-4 justify-center">
              <div><p class="text-xs font-bold text-gray-500 uppercase">Kode Unik</p><p class="text-2xl font-black italic">{{ selectedUnit.code }}</p></div>
              <div><p class="text-xs font-bold text-gray-500 uppercase">Jenis Alat Berat</p><p class="text-xl font-bold">{{ selectedUnit.jenis_alat_berat_nama || '—' }}</p></div>
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
              <p class="text-xl font-black mt-2" :class="selectedUnit.savings >= 0 ? 'text-miningYellow' : 'text-neoRed'">
                {{ selectedUnit.savings >= 0 ? '+$' : '-$' }}{{ Math.abs(selectedUnit.savings).toLocaleString() }}
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Form Modal -->
    <div v-if="isFormOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="isFormOpen = false"></div>
      <div class="bg-white border-4 border-black shadow-neo w-full max-w-lg relative z-10 flex flex-col max-h-[90vh] animate-[popup_0.2s_ease-out]">
        <div class="flex justify-between items-center p-6 border-b-4 border-black" :class="formMode==='add' ? 'bg-emerald-400' : 'bg-white'">
          <h3 class="text-2xl font-black uppercase">{{ formMode==='add' ? 'Tambah Unit' : 'Edit Unit' }}</h3>
          <button @click="isFormOpen = false" class="bg-neoRed text-white border-2 border-black w-10 h-10 font-black hover:shadow-none transition-all">X</button>
        </div>
        <div class="p-6 overflow-y-auto flex flex-col gap-4">
          <div v-if="formError" class="p-3 bg-neoRed text-white border-2 border-black font-bold text-sm">{{ formError }}</div>
          <div>
            <label class="block text-sm font-black uppercase mb-2">Kode Unik <span class="text-neoRed">*</span></label>
            <input v-model="formData.code" type="text" placeholder="Cth: EXC-320-05" class="w-full p-3 border-4 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] focus:outline-none focus:bg-miningYellow/10 font-bold" />
          </div>
          <div>
            <label class="block text-sm font-black uppercase mb-2">Jenis Alat Berat <span class="text-neoRed">*</span></label>
            <select v-model="formData.jenis_alat_berat_id" class="w-full p-3 border-4 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] focus:outline-none font-bold appearance-none cursor-pointer">
              <option value="">— Pilih Jenis —</option>
              <option v-for="j in jenisOptions" :key="j.id" :value="j.id">{{ j.nama }}</option>
            </select>
          </div>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-black uppercase mb-2">Status</label>
              <select v-model="formData.status" class="w-full p-3 border-4 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] focus:outline-none font-bold appearance-none cursor-pointer">
                <option v-for="s in statusOptions" :key="s" :value="s">{{ s }}</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-black uppercase mb-2">Health (%)</label>
              <input v-model.number="formData.health" type="number" min="0" max="100" class="w-full p-3 border-4 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] focus:outline-none font-bold" />
            </div>
          </div>
          <div>
            <label class="block text-sm font-black uppercase mb-2">Jadwal Maintenance</label>
            <input v-model="formData.maintenance" type="text" placeholder="Cth: 50 Jam Lagi" class="w-full p-3 border-4 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] focus:outline-none font-bold" />
          </div>
          <div>
            <label class="block text-sm font-black uppercase mb-2">Est. Savings ($)</label>
            <input v-model.number="formData.savings" type="number" class="w-full p-3 border-4 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] focus:outline-none font-bold" />
          </div>
          <div>
            <label class="block text-sm font-black uppercase mb-2">URL Gambar</label>
            <input v-model="formData.img_url" type="text" placeholder="https://..." class="w-full p-3 border-4 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] focus:outline-none font-bold text-sm" />
          </div>
          <div>
            <label class="block text-sm font-black uppercase mb-2">URL Model 3D (.glb)</label>
            <input v-model="formData.model3d_url" type="text" placeholder="https://....glb" class="w-full p-3 border-4 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] focus:outline-none font-bold text-sm" />
            <p class="text-[10px] font-bold text-gray-500 mt-1">Format GLB untuk visual 3D unit. Kosongkan untuk pakai gambar biasa.</p>
          </div>
        </div>
        <div class="p-6 border-t-4 border-black bg-gray-50 flex justify-end gap-4">
          <button @click="isFormOpen = false" class="px-6 py-3 border-4 border-black bg-white font-black hover:bg-gray-200 transition-colors">BATAL</button>
          <button @click="save" :disabled="formLoading" class="px-6 py-3 border-4 border-black bg-black text-white font-black shadow-[4px_4px_0px_0px_#FFCC00] hover:translate-x-1 hover:translate-y-1 hover:shadow-none transition-all disabled:opacity-60">
            {{ formLoading ? 'MENYIMPAN...' : 'SIMPAN' }}
          </button>
        </div>
      </div>
    </div>

    <!-- Export Modal -->
    <div v-if="isExportOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="isExportOpen = false"></div>
      <div class="bg-miningYellow border-4 border-black shadow-neo w-full max-w-md relative z-10 p-8 text-center animate-[popup_0.2s_ease-out]">
        <h3 class="text-3xl font-black uppercase mb-4">Eksport Data</h3>
        <p class="font-bold mb-8">Pilih format file untuk mengunduh data unit tambang.</p>
        <div class="flex flex-col gap-4">
          <button @click="exportCSV" class="w-full p-4 border-4 border-black bg-white font-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:translate-x-1 hover:translate-y-1 hover:shadow-none transition-all flex justify-between items-center">
            <span>Eksport sebagai CSV</span><span class="text-xl">📄</span>
          </button>
        </div>
        <button @click="isExportOpen = false" class="mt-8 underline font-bold hover:text-white transition-colors">Tutup</button>
      </div>
    </div>

  </div>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Public+Sans:wght@900&family=Space+Mono:wght@400;700&display=swap');
body { font-family: 'Space Mono', monospace; }
h1,h2,h3,button,.font-black { font-family: 'Public Sans', sans-serif; font-weight: 900; }
@keyframes popup { 0%{transform:scale(0.95) translateY(10px);opacity:0} 100%{transform:scale(1) translateY(0);opacity:1} }
</style>
