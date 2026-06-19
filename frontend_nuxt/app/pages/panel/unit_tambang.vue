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
  { name: 'Dashboard',       path: '/panel/dashboard',         icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="7" height="9" x="3" y="3" rx="1"/><rect width="7" height="5" x="14" y="3" rx="1"/><rect width="7" height="9" x="14" y="12" rx="1"/><rect width="7" height="5" x="3" y="16" rx="1"/></svg>` },
  { name: 'Jenis Alat Berat', path: '/panel/jenis_alat_berat', icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 17h4V5H2v12h3"/><path d="M20 17h2v-9l-2.5-3.5H14v12h3"/><path d="M14 6h4.5"/><circle cx="18.5" cy="17.5" r="2.5"/><circle cx="5.5" cy="17.5" r="2.5"/></svg>` },
  { name: 'Unit Tambang',    path: '/panel/unit_tambang',      icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 12 12 17 22 12"/><polyline points="2 17 12 22 22 17"/></svg>` },
  { name: 'Analisa Kerusakan', path: '/panel/analisa',         icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>` },
  { name: 'Kembali',         path: '../',                       icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m12 19-7-7 7-7"/><path d="M19 12H5"/></svg>` },
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
const statusHex = (s: string) => ({ SEHAT: '#1FA971', WARNING: '#E0A106', CRITICAL: '#E0413E', RUSAK: '#7A848E' }[s] || '#7A848E')

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
  <div class="flex min-h-screen bg-mesh text-[color:var(--text)]">

    <!-- Sidebar -->
    <aside class="w-72 border-r border-[color:var(--border)] bg-[color:var(--surface)] p-5 flex flex-col justify-between z-10 shrink-0">
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
          <NuxtLink v-for="item in menuItems" :key="item.name" :to="item.path"
            @click="activeMenu = item.name"
            :class="['nav-link', activeMenu === item.name ? 'nav-link-active' : '']">
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

    <!-- Main -->
    <main class="flex-1 p-8 overflow-y-auto">
      <header class="flex justify-between items-start mb-8 gap-4 flex-wrap">
        <div>
          <h1 class="font-display text-4xl md:text-5xl font-bold uppercase tracking-wide leading-none">Unit Tambang</h1>
          <p class="mt-2 text-[color:var(--text-muted)]">Unit yang beroperasi saat ini dan masih aktif berjalan.</p>
        </div>
        <div class="flex items-center gap-3 panel-flat px-3 py-2">
          <div class="w-8 h-8 rounded-full bg-steel-gradient flex items-center justify-center text-white font-bold text-xs">{{ (user?.name || 'A').charAt(0).toUpperCase() }}</div>
          <span class="font-semibold text-sm">{{ user?.name || 'Admin' }}</span>
        </div>
      </header>

      <div v-if="errorMsg" class="mb-6 px-4 py-3 rounded-xl bg-critical/10 border border-critical/40 text-critical font-semibold flex items-center gap-2">⚠️ {{ errorMsg }}</div>

      <!-- Actions Bar -->
      <div class="flex justify-between items-center mb-6 gap-3 flex-wrap">
        <div class="flex gap-3 flex-1 flex-wrap">
          <div class="relative flex-1 min-w-48">
            <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-[color:var(--text-faint)]" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
            <input v-model="searchQuery" @keyup.enter="onSearch" type="text" placeholder="Cari kode / nama unit..." class="field !pl-9" />
          </div>
          <select v-model="filterStatus" @change="onFilterStatus" class="field !w-auto cursor-pointer">
            <option value="">Semua Status</option>
            <option v-for="s in statusOptions" :key="s" :value="s">{{ s }}</option>
          </select>
          <button @click="onSearch" class="btn btn-ghost px-6">Cari</button>
          <button @click="isExportOpen = true" class="btn btn-dark px-5">
            Ekspor
            <svg class="w-4 h-4 text-amber" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5"><path d="M12 3v12m0 0l-4-4m4 4l4-4M4 21h16"/></svg>
          </button>
        </div>
        <button @click="openAdd" class="btn btn-amber px-5">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5"><path d="M12 5v14M5 12h14"/></svg>
          Tambah Unit
        </button>
      </div>

      <!-- Table -->
      <section class="panel-raised overflow-hidden">
        <table class="table-industrial">
          <thead>
            <tr>
              <th v-for="h in ['Kode Unik','Nama Unit','Status','Health','Jadwal','Saving','Aksi']" :key="h">{{ h }}</th>
            </tr>
          </thead>
          <tbody>
            <template v-if="isLoading">
              <tr v-for="i in 5" :key="i">
                <td v-for="j in 7" :key="j"><div class="h-4 shimmer rounded"></div></td>
              </tr>
            </template>
            <tr v-else-if="units.length === 0">
              <td colspan="7" class="!py-12 text-center text-[color:var(--text-faint)] font-medium">
                {{ searchQuery || filterStatus ? 'Tidak ada unit yang cocok.' : 'Belum ada unit. Klik Tambah Unit.' }}
              </td>
            </tr>
            <tr v-else v-for="unit in units" :key="unit.id">
              <td class="font-mono font-semibold">{{ unit.code }}</td>
              <td class="text-sm">{{ unit.jenis_alat_berat_nama || '—' }}</td>
              <td>
                <span class="badge text-white" :style="{ backgroundColor: statusHex(unit.status), borderColor: statusHex(unit.status) }">{{ unit.status }}</span>
              </td>
              <td>
                <div class="flex items-center gap-2">
                  <div class="w-14 h-1.5 rounded-full bg-[color:var(--surface-3)] overflow-hidden">
                    <div class="h-full rounded-full" :style="{ width: unit.health + '%', backgroundColor: statusHex(unit.status) }"></div>
                  </div>
                  <span class="font-mono font-semibold text-sm">{{ unit.health }}%</span>
                </div>
              </td>
              <td class="text-xs text-[color:var(--text-muted)]">{{ unit.maintenance }}</td>
              <td class="font-mono font-semibold text-sm" :class="unit.savings >= 0 ? 'text-healthy' : 'text-critical'">
                {{ unit.savings >= 0 ? '+$' : '-$' }}{{ Math.abs(unit.savings).toLocaleString() }}
              </td>
              <td>
                <div class="flex gap-2 flex-wrap">
                  <button @click="openDetail(unit)" class="btn btn-ghost !px-3 !py-1.5 text-xs">Lihat</button>
                  <button @click="openEdit(unit)" class="btn btn-ghost !px-3 !py-1.5 text-xs">Edit</button>
                  <button @click="remove(unit)" class="btn btn-danger !px-3 !py-1.5 text-xs">Hapus</button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>

        <!-- Pagination -->
        <div class="px-5 py-4 border-t border-[color:var(--border)] bg-[color:var(--surface-2)] flex justify-between items-center flex-wrap gap-3">
          <span class="text-sm text-[color:var(--text-muted)] font-medium">Total: <span class="font-mono font-semibold">{{ total }}</span> unit</span>
          <div v-if="totalPages > 1" class="flex gap-1.5">
            <button @click="currentPage=1; fetchUnits()" :disabled="currentPage===1" class="pg-btn">«</button>
            <button @click="currentPage--; fetchUnits()" :disabled="currentPage===1" class="pg-btn">‹</button>
            <button v-for="page in visiblePages" :key="page" @click="currentPage=page; fetchUnits()"
              :class="currentPage===page ? 'pg-btn pg-active' : 'pg-btn'">{{ page }}</button>
            <button @click="currentPage++; fetchUnits()" :disabled="currentPage===totalPages" class="pg-btn">›</button>
            <button @click="currentPage=totalPages; fetchUnits()" :disabled="currentPage===totalPages" class="pg-btn">»</button>
          </div>
        </div>
      </section>
    </main>

    <!-- Detail Modal -->
    <div v-if="isDetailOpen && selectedUnit" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="modal-backdrop" @click="isDetailOpen = false"></div>
      <div class="modal-card w-full max-w-2xl flex flex-col max-h-[90vh] anim-pop">
        <div class="flex justify-between items-center px-6 py-4 border-b border-[color:var(--border)] bg-[color:var(--surface-2)]">
          <h3 class="font-display text-2xl font-bold uppercase tracking-wide">Detail Unit</h3>
          <button @click="isDetailOpen = false" class="w-9 h-9 rounded-lg hover:bg-critical/15 hover:text-critical text-[color:var(--text-muted)] flex items-center justify-center transition-colors">✕</button>
        </div>
        <div class="p-6 overflow-y-auto">
          <div class="flex flex-col md:flex-row gap-6 mb-6">
            <div class="viewer-3d w-full md:w-1/2 rounded-xl border border-[color:var(--border)] bg-steel-gradient flex items-center justify-center min-h-[200px] relative overflow-hidden cursor-grab active:cursor-grabbing">
              <model-viewer
                v-if="selectedUnit.model3d_url"
                :key="selectedUnit.id"
                :src="selectedUnit.model3d_url"
                alt="Model 3D unit alat berat"
                camera-controls
                auto-rotate
                auto-rotate-delay="0"
                rotation-per-second="35deg"
                shadow-intensity="1.4"
                exposure="1.1"
                environment-image="neutral"
                interaction-prompt="none"
                class="w-full h-full min-h-[200px] outline-none"
                style="background-color:transparent;"
              ></model-viewer>
              <img v-else-if="selectedUnit.img_url" :src="selectedUnit.img_url" class="w-full h-full object-cover" />
              <span v-else class="text-graphite-300 font-medium">No Visual</span>
              <div v-if="selectedUnit.model3d_url" class="absolute top-2 left-2 bg-steel/90 text-white text-[9px] font-semibold px-2 py-0.5 rounded-full pointer-events-none">● LIVE 3D</div>
              <div v-if="selectedUnit.model3d_url" class="absolute bottom-2 right-2 bg-graphite-900/80 text-graphite-100 text-[9px] font-medium px-2 py-0.5 rounded-full pointer-events-none">DRAG 360°</div>
            </div>
            <div class="w-full md:w-1/2 flex flex-col gap-4 justify-center">
              <div><p class="label">Kode Unik</p><p class="text-2xl font-mono font-bold">{{ selectedUnit.code }}</p></div>
              <div><p class="label">Jenis Alat Berat</p><p class="text-lg font-semibold">{{ selectedUnit.jenis_alat_berat_nama || '—' }}</p></div>
              <div>
                <span class="badge text-white" :style="{ backgroundColor: statusHex(selectedUnit.status), borderColor: statusHex(selectedUnit.status) }">Status: {{ selectedUnit.status }}</span>
              </div>
            </div>
          </div>
          <div class="grid grid-cols-3 gap-4 border-t border-[color:var(--border)] pt-6">
            <div class="panel-flat p-4 text-center">
              <p class="label">Health Score</p>
              <p class="text-3xl font-display font-bold" :style="{ color: statusHex(selectedUnit.status) }">{{ selectedUnit.health }}%</p>
            </div>
            <div class="panel-flat p-4 text-center">
              <p class="label">Jadwal MTC</p>
              <p class="text-base font-semibold mt-1 leading-tight">{{ selectedUnit.maintenance }}</p>
            </div>
            <div class="p-4 text-center rounded-[10px] bg-steel-gradient text-white border border-[color:var(--border)]">
              <p class="text-[10px] font-semibold uppercase tracking-wider text-graphite-300 mb-1">Est. Saving</p>
              <p class="text-xl font-mono font-bold" :class="selectedUnit.savings >= 0 ? 'text-amber' : 'text-critical'">
                {{ selectedUnit.savings >= 0 ? '+$' : '-$' }}{{ Math.abs(selectedUnit.savings).toLocaleString() }}
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Form Modal -->
    <div v-if="isFormOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="modal-backdrop" @click="isFormOpen = false"></div>
      <div class="modal-card w-full max-w-lg flex flex-col max-h-[90vh] anim-pop">
        <div class="flex justify-between items-center px-6 py-4 border-b border-[color:var(--border)] bg-[color:var(--surface-2)]">
          <h3 class="font-display text-2xl font-bold uppercase tracking-wide">{{ formMode==='add' ? 'Tambah Unit' : 'Edit Unit' }}</h3>
          <button @click="isFormOpen = false" class="w-9 h-9 rounded-lg hover:bg-critical/15 hover:text-critical text-[color:var(--text-muted)] flex items-center justify-center transition-colors">✕</button>
        </div>
        <div class="p-6 overflow-y-auto flex flex-col gap-4">
          <div v-if="formError" class="px-4 py-2.5 rounded-lg bg-critical/10 border border-critical/40 text-critical font-semibold text-sm">{{ formError }}</div>
          <div>
            <label class="label">Kode Unik <span class="text-critical">*</span></label>
            <input v-model="formData.code" type="text" placeholder="Cth: EXC-320-05" class="field" />
          </div>
          <div>
            <label class="label">Jenis Alat Berat <span class="text-critical">*</span></label>
            <select v-model="formData.jenis_alat_berat_id" class="field cursor-pointer">
              <option value="">— Pilih Jenis —</option>
              <option v-for="j in jenisOptions" :key="j.id" :value="j.id">{{ j.nama }}</option>
            </select>
          </div>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="label">Status</label>
              <select v-model="formData.status" class="field cursor-pointer">
                <option v-for="s in statusOptions" :key="s" :value="s">{{ s }}</option>
              </select>
            </div>
            <div>
              <label class="label">Health (%)</label>
              <input v-model.number="formData.health" type="number" min="0" max="100" class="field" />
            </div>
          </div>
          <div>
            <label class="label">Jadwal Maintenance</label>
            <input v-model="formData.maintenance" type="text" placeholder="Cth: 50 Jam Lagi" class="field" />
          </div>
          <div>
            <label class="label">Est. Savings ($)</label>
            <input v-model.number="formData.savings" type="number" class="field" />
          </div>
          <div>
            <label class="label">URL Gambar</label>
            <input v-model="formData.img_url" type="text" placeholder="https://..." class="field text-sm" />
          </div>
          <div>
            <label class="label">URL Model 3D (.glb)</label>
            <input v-model="formData.model3d_url" type="text" placeholder="https://....glb" class="field text-sm" />
            <p class="text-[10px] text-[color:var(--text-faint)] mt-1.5">Format GLB untuk visual 3D unit. Kosongkan untuk pakai gambar biasa.</p>
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

    <!-- Export Modal -->
    <div v-if="isExportOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="modal-backdrop" @click="isExportOpen = false"></div>
      <div class="modal-card w-full max-w-md p-8 text-center anim-pop">
        <div class="w-14 h-14 rounded-2xl bg-amber/15 border border-amber/40 flex items-center justify-center mx-auto mb-4 text-amber">
          <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path d="M12 3v12m0 0l-4-4m4 4l4-4M4 21h16"/></svg>
        </div>
        <h3 class="font-display text-2xl font-bold uppercase tracking-wide mb-2">Ekspor Data</h3>
        <p class="text-[color:var(--text-muted)] mb-6">Pilih format file untuk mengunduh data unit tambang.</p>
        <button @click="exportCSV" class="btn btn-ghost w-full !py-4 justify-between">
          <span class="font-semibold">Ekspor sebagai CSV</span>
          <span class="text-xl">📄</span>
        </button>
        <button @click="isExportOpen = false" class="mt-5 text-sm font-semibold text-[color:var(--text-muted)] hover:text-amber transition-colors">Tutup</button>
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
