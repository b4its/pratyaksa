<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

useHead({ script: [{ type: 'module', src: 'https://ajax.googleapis.com/ajax/libs/model-viewer/3.3.0/model-viewer.min.js' }] })

// ---- Sidebar ----
const menuItems = [
  { name: 'Dashboard',       path: '/panel/dashboard',         icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="7" height="9" x="3" y="3" rx="1"/><rect width="7" height="5" x="14" y="3" rx="1"/><rect width="7" height="9" x="14" y="12" rx="1"/><rect width="7" height="5" x="3" y="16" rx="1"/></svg>` },
  { name: 'Jenis Alat Berat', path: '/panel/jenis_alat_berat', icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 17h4V5H2v12h3"/><path d="M20 17h2v-9l-2.5-3.5H14v12h3"/><path d="M14 6h4.5"/><circle cx="18.5" cy="17.5" r="2.5"/><circle cx="5.5" cy="17.5" r="2.5"/></svg>` },
  { name: 'Unit Tambang',    path: '/panel/unit_tambang',      icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 12 12 17 22 12"/><polyline points="2 17 12 22 22 17"/></svg>` },
  { name: 'Analisa Kerusakan', path: '/panel/analisa',         icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>` },
  { name: 'Work Order',      path: '/panel/work_order',        icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.12 2.12 0 0 1 3 3L12 15l-4 1 1-4Z"/></svg>` },
  { name: 'Kembali',         path: '../',                       icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m12 19-7-7 7-7"/><path d="M19 12H5"/></svg>` },
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
const perPage = 5
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

// ---- Auth & Theme ----
const { initAuth, user } = useAuth()
const { isDark, toggleTheme, initTheme } = useTheme()
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
  initTheme()
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
  <div class="flex h-screen overflow-hidden bg-mesh text-[color:var(--text)]">

    <!-- Sidebar -->
    <PanelSidebar />

    <!-- Main -->
    <main class="flex-1 min-w-0 w-full p-4 sm:p-6 lg:p-8 pt-16 lg:pt-8 overflow-y-auto">
      <header class="flex justify-between items-start mb-8 gap-4 flex-wrap">
        <div>
          <h1 class="font-display text-4xl md:text-5xl font-bold uppercase tracking-wide leading-none">Jenis Alat Berat</h1>
          <p class="mt-2 text-[color:var(--text-muted)]">Daftar kategori alat berat yang terdaftar di sistem.</p>
        </div>
        <div class="flex items-center gap-3 panel-flat px-3 py-2">
          <div class="w-8 h-8 rounded-full bg-steel-gradient flex items-center justify-center text-white font-bold text-xs">{{ (user?.name || 'A').charAt(0).toUpperCase() }}</div>
          <span class="font-semibold text-sm">{{ user?.name || 'Admin' }}</span>
        </div>
      </header>

      <!-- Error -->
      <div v-if="errorMsg" class="mb-6 px-4 py-3 rounded-xl bg-critical/10 border border-critical/40 text-critical font-semibold flex items-center gap-2">⚠️ {{ errorMsg }}</div>

      <!-- Actions -->
      <div class="flex justify-between items-center mb-6 gap-3 flex-wrap">
        <div class="flex gap-3 flex-1 flex-wrap">
          <div class="relative flex-1 min-w-48">
            <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-[color:var(--text-faint)]" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
            <input v-model="searchQuery" @keyup.enter="onSearch" type="text" placeholder="Cari jenis alat berat..." class="field !pl-9" />
          </div>
          <button @click="onSearch" class="btn btn-ghost px-6">Cari</button>
        </div>
        <button @click="openAdd" class="btn btn-amber px-5">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5"><path d="M12 5v14M5 12h14"/></svg>
          Tambah Jenis
        </button>
      </div>

      <!-- Table -->
      <section class="panel-raised overflow-hidden">
        <div class="overflow-x-auto">
        <table class="table-industrial">
          <thead>
            <tr>
              <th v-for="h in ['Nama Jenis', 'Deskripsi', 'Dibuat', 'Aksi']" :key="h">{{ h }}</th>
            </tr>
          </thead>
          <tbody>
            <template v-if="isLoading">
              <tr v-for="i in 5" :key="i">
                <td v-for="j in 4" :key="j"><div class="h-4 shimmer rounded"></div></td>
              </tr>
            </template>
            <tr v-else-if="items.length === 0">
              <td colspan="4" class="!py-12 text-center text-[color:var(--text-faint)] font-medium">
                {{ searchQuery ? `Tidak ada hasil untuk "${searchQuery}"` : 'Belum ada data. Klik Tambah Jenis.' }}
              </td>
            </tr>
            <tr v-else v-for="item in items" :key="item.id">
              <td class="font-semibold">{{ item.nama }}</td>
              <td class="text-sm text-[color:var(--text-muted)] max-w-xs truncate">{{ item.deskripsi || '—' }}</td>
              <td class="text-xs text-[color:var(--text-faint)] font-mono">{{ formatDate(item.created_at) }}</td>
              <td>
                <div class="flex gap-2 flex-wrap">
                  <button @click="openDetail(item)" class="btn btn-ghost !px-3 !py-1.5 text-xs">Lihat</button>
                  <button @click="openEdit(item)" class="btn btn-ghost !px-3 !py-1.5 text-xs">Edit</button>
                  <button @click="remove(item)" class="btn btn-danger !px-3 !py-1.5 text-xs">Hapus</button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
        </div>

        <!-- Pagination -->
        <div class="px-5 py-4 border-t border-[color:var(--border)] bg-[color:var(--surface-2)] flex justify-between items-center flex-wrap gap-3">
          <span class="text-sm text-[color:var(--text-muted)] font-medium">Total: <span class="font-mono font-semibold">{{ total }}</span> jenis</span>
          <div v-if="totalPages > 1" class="flex gap-1.5">
            <button @click="currentPage = 1; fetchData()" :disabled="currentPage === 1" class="pg-btn">«</button>
            <button @click="currentPage--; fetchData()" :disabled="currentPage === 1" class="pg-btn">‹</button>
            <button v-for="page in visiblePages" :key="page" @click="currentPage = page; fetchData()"
              :class="currentPage === page ? 'pg-btn pg-active' : 'pg-btn'">{{ page }}</button>
            <button @click="currentPage++; fetchData()" :disabled="currentPage === totalPages" class="pg-btn">›</button>
            <button @click="currentPage = totalPages; fetchData()" :disabled="currentPage === totalPages" class="pg-btn">»</button>
          </div>
        </div>
      </section>
    </main>

    <!-- Detail Modal -->
    <div v-if="isDetailOpen && selectedItem" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="modal-backdrop" @click="isDetailOpen = false"></div>
      <div class="modal-card w-full max-w-lg anim-pop">
        <div class="flex justify-between items-center px-6 py-4 border-b border-[color:var(--border)] bg-[color:var(--surface-2)]">
          <h3 class="font-display text-2xl font-bold uppercase tracking-wide">Detail Jenis Alat Berat</h3>
          <button @click="isDetailOpen = false" class="w-9 h-9 rounded-lg hover:bg-critical/15 hover:text-critical text-[color:var(--text-muted)] flex items-center justify-center transition-colors">✕</button>
        </div>
        <div class="p-6 space-y-4">
          <div class="panel-flat p-4">
            <p class="label">Nama Jenis</p>
            <p class="text-xl font-semibold">{{ selectedItem.nama }}</p>
          </div>
          <div class="panel-flat p-4">
            <p class="label">Deskripsi</p>
            <p class="leading-relaxed text-[color:var(--text-muted)]">{{ selectedItem.deskripsi || 'Tidak ada deskripsi.' }}</p>
          </div>
          <div class="grid grid-cols-2 gap-4">
            <div class="panel-flat p-4">
              <p class="label">Dibuat</p>
              <p class="font-semibold text-sm font-mono">{{ formatDate(selectedItem.created_at) }}</p>
            </div>
            <div class="panel-flat p-4">
              <p class="label">Diperbarui</p>
              <p class="font-semibold text-sm font-mono">{{ formatDate(selectedItem.updated_at) }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Form Modal (Add / Edit) -->
    <div v-if="isFormOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="modal-backdrop" @click="isFormOpen = false"></div>
      <div class="modal-card w-full max-w-lg flex flex-col anim-pop">
        <div class="flex justify-between items-center px-6 py-4 border-b border-[color:var(--border)] bg-[color:var(--surface-2)]">
          <h3 class="font-display text-2xl font-bold uppercase tracking-wide">{{ formMode === 'add' ? 'Tambah Jenis' : 'Edit Jenis' }}</h3>
          <button @click="isFormOpen = false" class="w-9 h-9 rounded-lg hover:bg-critical/15 hover:text-critical text-[color:var(--text-muted)] flex items-center justify-center transition-colors">✕</button>
        </div>
        <div class="p-6 flex flex-col gap-4">
          <div v-if="formError" class="px-4 py-2.5 rounded-lg bg-critical/10 border border-critical/40 text-critical font-semibold text-sm">{{ formError }}</div>
          <div>
            <label class="label">Nama Jenis <span class="text-critical">*</span></label>
            <input v-model="formData.nama" type="text" placeholder="Cth: Caterpillar Excavator 320" class="field" />
          </div>
          <div>
            <label class="label">Deskripsi</label>
            <textarea v-model="formData.deskripsi" rows="3" placeholder="Deskripsi singkat tentang jenis alat berat ini..." class="field resize-none"></textarea>
          </div>
        </div>
        <div class="px-6 py-4 border-t border-[color:var(--border)] bg-[color:var(--surface-2)] flex justify-end gap-3">
          <button @click="isFormOpen = false" class="btn btn-ghost px-6">Batal</button>
          <button @click="save" :disabled="formLoading" class="btn btn-amber px-6 disabled:opacity-60">
            {{ formLoading ? 'Menyimpan…' : 'Simpan' }}
          </button>
        </div>
      </div>
    </div>

  </div>
</template>

<style scoped>
.pg-btn {
  width: 2.25rem; height: 2.25rem;
  display: flex; align-items: center; justify-content: center;
  border-radius: 9px; border: 1px solid var(--border-strong);
  background: var(--surface); color: var(--text);
  font-weight: 600; font-size: .85rem;
  transition: all .15s ease;
}
.pg-btn:hover:not(:disabled) { background: var(--surface-3); border-color: var(--steel); }
.pg-btn:disabled { opacity: .4; cursor: not-allowed; }
.pg-active { background: var(--amber); border-color: var(--amber); color: #1b1206; }
.pg-active:hover { background: var(--amber); }
</style>
