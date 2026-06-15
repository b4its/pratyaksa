<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

useHead({ script: [{ type: 'module', src: 'https://ajax.googleapis.com/ajax/libs/model-viewer/3.3.0/model-viewer.min.js' }] })

// ---- Sidebar ----
const menuItems = [
  { name: 'Dashboard',       path: '/panel/dashboard',         icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><rect width="7" height="9" x="3" y="3"/><rect width="7" height="5" x="14" y="3"/><rect width="7" height="9" x="14" y="12"/><rect width="7" height="5" x="3" y="16"/></svg>` },
  { name: 'Jenis Alat Berat', path: '/panel/jenis_alat_berat', icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><path d="M10 17h4V5H2v12h3"/><path d="M20 17h2v-9l-2.5-3.5H14v12h3"/><path d="M14 6h4.5"/><circle cx="18.5" cy="17.5" r="2.5"/><circle cx="5.5" cy="17.5" r="2.5"/></svg>` },
  { name: 'Unit Tambang',    path: '/panel/unit_tambang',      icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 12 12 17 22 12"/><polyline points="2 17 12 22 22 17"/></svg>` },
  { name: 'Analisa Kerusakan', path: '/panel/analisa',         icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>` },
  { name: 'Kembali',         path: '../',                       icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><path d="m12 19-7-7 7-7"/><path d="M19 12H5"/></svg>` },
]
const activeMenu = ref('Jenis Alat Berat')

// ---- Types ----
interface JenisAlatBerat {
  id: string
  nama: string
  deskripsi: string | null
  created_at: string
  updated_at: string
}

// ---- State ----
const items = ref<JenisAlatBerat[]>([])
const total = ref(0)
const currentPage = ref(1)
const perPage = 10
const searchQuery = ref('')
const isLoading = ref(false)
const errorMsg = ref('')

const totalPages = computed(() => Math.ceil(total.value / perPage))

// ---- Modal ----
const isFormOpen = ref(false)
const formMode = ref<'add' | 'edit'>('add')
const formData = ref({ id: '', nama: '', deskripsi: '' })
const formError = ref('')
const formLoading = ref(false)

const isDetailOpen = ref(false)
const selectedItem = ref<JenisAlatBerat | null>(null)

// ---- Auth ----
const { initAuth, user } = useAuth()
const api = useApi()

// ---- Fetch ----
const fetchData = async () => {
  isLoading.value = true
  errorMsg.value = ''
  try {
    const res = await api.getJenisAlatBerat({
      page: currentPage.value,
      per_page: perPage,
      search: searchQuery.value || undefined,
    }) as any
    items.value = res.data.data
    total.value = res.data.total
  } catch (e: any) {
    errorMsg.value = e?.data?.message || 'Gagal memuat data.'
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  initAuth()
  fetchData()
})

const onSearch = () => { currentPage.value = 1; fetchData() }

// ---- CRUD ----
const openAdd = () => {
  formMode.value = 'add'
  formData.value = { id: '', nama: '', deskripsi: '' }
  formError.value = ''
  isFormOpen.value = true
}

const openEdit = (item: JenisAlatBerat) => {
  formMode.value = 'edit'
  formData.value = { id: item.id, nama: item.nama, deskripsi: item.deskripsi || '' }
  formError.value = ''
  isFormOpen.value = true
}

const openDetail = (item: JenisAlatBerat) => {
  selectedItem.value = item
  isDetailOpen.value = true
}

const save = async () => {
  if (!formData.value.nama.trim()) { formError.value = 'Nama wajib diisi.'; return }
  formLoading.value = true
  formError.value = ''
  try {
    if (formMode.value === 'add') {
      await api.createJenisAlatBerat({ nama: formData.value.nama, deskripsi: formData.value.deskripsi || undefined })
    } else {
      await api.updateJenisAlatBerat(formData.value.id, { nama: formData.value.nama, deskripsi: formData.value.deskripsi || undefined })
    }
    isFormOpen.value = false
    await fetchData()
  } catch (e: any) {
    formError.value = e?.data?.message || 'Gagal menyimpan.'
  } finally {
    formLoading.value = false
  }
}

const remove = async (item: JenisAlatBerat) => {
  if (!confirm(`Hapus "${item.nama}"?`)) return
  try {
    await api.deleteJenisAlatBerat(item.id)
    await fetchData()
  } catch (e: any) {
    alert(e?.data?.message || 'Gagal menghapus.')
  }
}

// ---- Pagination ----
const visiblePages = computed(() => {
  const max = 3, tp = totalPages.value, cp = currentPage.value
  if (tp <= max) return Array.from({ length: tp }, (_, i) => i + 1)
  let start = Math.max(1, cp - 1)
  let end = Math.min(tp, start + max - 1)
  if (end === tp) start = Math.max(1, tp - max + 1)
  return Array.from({ length: end - start + 1 }, (_, i) => start + i)
})

const formatDate = (iso: string) => new Date(iso).toLocaleDateString('id-ID', { day: 'numeric', month: 'short', year: 'numeric' })
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
      <div class="border-4 border-black p-4 bg-white shadow-neo">
        <p class="text-[10px] font-black uppercase text-gray-500">Logged in as</p>
        <div class="flex items-center gap-3 mt-1">
          <div class="w-10 h-10 border-2 border-black bg-neoBlue shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]"></div>
          <p class="font-black text-sm">{{ user?.name || 'Admin' }}</p>
        </div>
      </div>
    </aside>

    <!-- Main -->
    <main class="flex-1 p-8 overflow-y-auto">
      <header class="flex justify-between items-start mb-10">
        <div>
          <h1 class="text-6xl font-black uppercase tracking-tighter leading-none">Jenis Alat Berat</h1>
          <p class="mt-2 font-bold text-gray-600 bg-white border-b-2 border-black inline-block">Daftar kategori alat berat yang terdaftar di sistem.</p>
        </div>
        <div class="flex items-center gap-3 p-2 border-4 border-black bg-white shadow-neoHover">
          <div class="w-8 h-8 rounded-full border-2 border-black bg-neoBlue"></div>
          <span class="font-black text-sm">{{ user?.name || 'Admin' }}</span>
        </div>
      </header>

      <!-- Error -->
      <div v-if="errorMsg" class="mb-6 p-4 bg-neoRed text-white border-4 border-black font-bold shadow-neo">⚠️ {{ errorMsg }}</div>

      <!-- Actions -->
      <div class="flex justify-between items-center mb-8 gap-4 flex-wrap">
        <div class="flex gap-4 flex-1 flex-wrap">
          <input v-model="searchQuery" @keyup.enter="onSearch" type="text" placeholder="Cari jenis alat berat..."
            class="flex-1 min-w-48 p-4 border-4 border-black shadow-neoHover focus:outline-none focus:bg-miningYellow/10 font-bold placeholder:text-gray-400" />
          <button @click="onSearch" class="p-4 border-4 border-black bg-white font-black shadow-neoHover hover:translate-x-1 hover:translate-y-1 hover:shadow-none transition-all px-8">
            CARI
          </button>
        </div>
        <button @click="openAdd" class="bg-emerald-400 text-black p-4 font-black shadow-[4px_4px_0px_0px_#000] flex items-center gap-2 hover:translate-x-1 hover:translate-y-1 hover:shadow-none transition-all border-4 border-black">
          + TAMBAH JENIS
        </button>
      </div>

      <!-- Table -->
      <section class="border-4 border-black bg-white shadow-neo overflow-hidden">
        <table class="w-full border-collapse">
          <thead>
            <tr class="bg-black text-white">
              <th v-for="h in ['Nama Jenis', 'Deskripsi', 'Dibuat', 'Aksi']" :key="h"
                class="p-4 text-left font-black uppercase text-xs tracking-widest border-r border-white/20">{{ h }}</th>
            </tr>
          </thead>
          <tbody>
            <!-- Loading skeleton -->
            <template v-if="isLoading">
              <tr v-for="i in 5" :key="i" class="border-b-4 border-black">
                <td v-for="j in 4" :key="j" class="p-4"><div class="h-4 bg-gray-200 animate-pulse rounded"></div></td>
              </tr>
            </template>
            <!-- Empty -->
            <tr v-else-if="items.length === 0">
              <td colspan="4" class="p-12 text-center font-bold text-gray-400">
                {{ searchQuery ? `Tidak ada hasil untuk "${searchQuery}"` : 'Belum ada data. Klik + TAMBAH JENIS.' }}
              </td>
            </tr>
            <!-- Data -->
            <tr v-else v-for="item in items" :key="item.id" class="border-b-4 border-black hover:bg-gray-50 transition-colors">
              <td class="p-4 font-black">{{ item.nama }}</td>
              <td class="p-4 font-bold text-sm text-gray-600 max-w-xs truncate">{{ item.deskripsi || '—' }}</td>
              <td class="p-4 font-bold text-xs text-gray-500">{{ formatDate(item.created_at) }}</td>
              <td class="p-4 flex gap-2 flex-wrap">
                <button @click="openDetail(item)" class="bg-miningYellow border-2 border-black px-3 py-1 font-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:shadow-none hover:translate-x-[1px] hover:translate-y-[1px] transition-all text-xs">VIEW</button>
                <button @click="openEdit(item)" class="bg-white border-2 border-black px-3 py-1 font-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:shadow-none hover:translate-x-[1px] hover:translate-y-[1px] transition-all text-xs">EDIT</button>
                <button @click="remove(item)" class="bg-neoRed text-white border-2 border-black px-3 py-1 font-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:shadow-none hover:translate-x-[1px] hover:translate-y-[1px] transition-all text-xs">DEL</button>
              </td>
            </tr>
          </tbody>
        </table>

        <!-- Pagination -->
        <div v-if="totalPages > 1" class="p-4 border-t-4 border-black bg-white flex justify-between items-center flex-wrap gap-2">
          <span class="font-bold text-sm text-gray-500">Total: {{ total }} jenis</span>
          <div class="flex gap-2">
            <button @click="currentPage = 1; fetchData()" :disabled="currentPage === 1" class="w-10 h-10 border-2 border-black flex items-center justify-center font-black hover:bg-black hover:text-white disabled:opacity-50 transition-colors">&laquo;</button>
            <button @click="currentPage--; fetchData()" :disabled="currentPage === 1" class="w-10 h-10 border-2 border-black flex items-center justify-center font-black hover:bg-black hover:text-white disabled:opacity-50 transition-colors">&lt;</button>
            <button v-for="page in visiblePages" :key="page" @click="currentPage = page; fetchData()"
              :class="currentPage === page ? 'bg-black text-white' : 'bg-white text-black hover:bg-black hover:text-white'"
              class="w-10 h-10 border-2 border-black flex items-center justify-center font-black transition-colors">{{ page }}</button>
            <button @click="currentPage++; fetchData()" :disabled="currentPage === totalPages" class="w-10 h-10 border-2 border-black flex items-center justify-center font-black hover:bg-black hover:text-white disabled:opacity-50 transition-colors">&gt;</button>
            <button @click="currentPage = totalPages; fetchData()" :disabled="currentPage === totalPages" class="w-10 h-10 border-2 border-black flex items-center justify-center font-black hover:bg-black hover:text-white disabled:opacity-50 transition-colors">&raquo;</button>
          </div>
        </div>
        <div v-else class="p-4 border-t-4 border-black text-sm font-bold text-gray-500">Total: {{ total }} jenis</div>
      </section>
    </main>

    <!-- Detail Modal -->
    <div v-if="isDetailOpen && selectedItem" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="isDetailOpen = false"></div>
      <div class="bg-white border-4 border-black shadow-neo w-full max-w-lg relative z-10 animate-[popup_0.2s_ease-out]">
        <div class="flex justify-between items-center p-6 border-b-4 border-black bg-miningYellow">
          <h3 class="text-2xl font-black uppercase">Detail Jenis Alat Berat</h3>
          <button @click="isDetailOpen = false" class="bg-neoRed text-white border-2 border-black w-10 h-10 font-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:shadow-none transition-all">X</button>
        </div>
        <div class="p-6 space-y-4">
          <div class="border-2 border-black p-4 shadow-neoHover">
            <p class="text-xs font-black uppercase text-gray-500 mb-1">Nama Jenis</p>
            <p class="text-2xl font-black">{{ selectedItem.nama }}</p>
          </div>
          <div class="border-2 border-black p-4 shadow-neoHover">
            <p class="text-xs font-black uppercase text-gray-500 mb-1">Deskripsi</p>
            <p class="font-bold leading-relaxed">{{ selectedItem.deskripsi || 'Tidak ada deskripsi.' }}</p>
          </div>
          <div class="grid grid-cols-2 gap-4">
            <div class="border-2 border-black p-3 shadow-neoHover">
              <p class="text-xs font-black uppercase text-gray-500 mb-1">Dibuat</p>
              <p class="font-bold text-sm">{{ formatDate(selectedItem.created_at) }}</p>
            </div>
            <div class="border-2 border-black p-3 shadow-neoHover">
              <p class="text-xs font-black uppercase text-gray-500 mb-1">Diperbarui</p>
              <p class="font-bold text-sm">{{ formatDate(selectedItem.updated_at) }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Form Modal (Add / Edit) -->
    <div v-if="isFormOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="isFormOpen = false"></div>
      <div class="bg-white border-4 border-black shadow-neo w-full max-w-lg relative z-10 flex flex-col animate-[popup_0.2s_ease-out]">
        <div class="flex justify-between items-center p-6 border-b-4 border-black" :class="formMode === 'add' ? 'bg-emerald-400' : 'bg-white'">
          <h3 class="text-2xl font-black uppercase">{{ formMode === 'add' ? 'Tambah Jenis' : 'Edit Jenis' }}</h3>
          <button @click="isFormOpen = false" class="bg-neoRed text-white border-2 border-black w-10 h-10 font-black hover:shadow-none transition-all">X</button>
        </div>
        <div class="p-6 flex flex-col gap-4">
          <div v-if="formError" class="p-3 bg-neoRed text-white border-2 border-black font-bold text-sm">{{ formError }}</div>
          <div>
            <label class="block text-sm font-black uppercase mb-2">Nama Jenis <span class="text-neoRed">*</span></label>
            <input v-model="formData.nama" type="text" placeholder="Cth: Caterpillar Excavator 320"
              class="w-full p-3 border-4 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] focus:outline-none focus:bg-miningYellow/10 font-bold" />
          </div>
          <div>
            <label class="block text-sm font-black uppercase mb-2">Deskripsi</label>
            <textarea v-model="formData.deskripsi" rows="3" placeholder="Deskripsi singkat tentang jenis alat berat ini..."
              class="w-full p-3 border-4 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] focus:outline-none focus:bg-miningYellow/10 font-bold resize-none"></textarea>
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

  </div>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Public+Sans:wght@900&family=Space+Mono:wght@400;700&display=swap');
body { font-family: 'Space Mono', monospace; }
h1,h2,h3,button,.font-black { font-family: 'Public Sans', sans-serif; font-weight: 900; }
@keyframes popup { 0%{transform:scale(0.95) translateY(10px);opacity:0} 100%{transform:scale(1) translateY(0);opacity:1} }
</style>
