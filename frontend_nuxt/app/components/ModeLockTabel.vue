<script setup lang="ts">
import type { PratyaksaSourceMode } from '~/composables/usePratyaksa'

const props = defineProps<{
  showSubModes?: boolean
  compact?: boolean
}>()

const emit = defineEmits<{
  testTelegram: []
}>()

const pratyaksa = usePratyaksa()
const switching = ref(false)
const switchingMode = ref<string | null>(null)

interface ModeOption {
  id: 'simulasi' | 'live'
  label: string
  icon: string
  desc: string
  details: string[]
  color: string
  bgColor: string
  borderColor: string
  dotColor: string
  badgeClass: string
}

const modeOptions: ModeOption[] = [
  {
    id: 'simulasi',
    label: 'SIMULASI',
    icon: `<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M9.75 3.104v5.714a2.25 2.25 0 0 1-.659 1.591L5 14.5M9.75 3.104c-.251.023-.501.05-.75.082m.75-.082a24.301 24.301 0 0 1 4.5 0m0 0v5.714c0 .597.237 1.17.659 1.591L19.8 15.3M14.25 3.104c.251.023.501.05.75.082M19.8 15.3l-1.57.393A9.065 9.065 0 0 1 12 15a9.065 9.065 0 0 0-6.23.693L5 14.5m14.8.8 1.402 1.402c1.232 1.232.65 3.318-1.067 3.611A48.309 48.309 0 0 1 12 21c-2.773 0-5.491-.235-8.135-.687-1.718-.293-2.3-2.379-1.067-3.61L5 14.5"/></svg>`,
    desc: 'Data dari simulator internal tanpa koneksi ke endpoint eksternal',
    details: [
      'Data deterministik dari engine Rust',
      'Tidak perlu koneksi jaringan',
      'Cocok untuk development & demo',
    ],
    color: 'text-warning',
    bgColor: 'bg-warning/8',
    borderColor: 'border-warning/30',
    dotColor: 'bg-warning',
    badgeClass: 'bg-warning/15 text-warning border-warning/30',
  },
  {
    id: 'live',
    label: 'LIVE API',
    icon: `<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M12 21a9.004 9.004 0 0 0 8.716-6.747M12 21a9.004 9.004 0 0 1-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 0 1 7.843 4.582M12 3a8.997 8.997 0 0 0-7.843 4.582m15.686 0A11.953 11.953 0 0 1 12 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0 1 21 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0 1 12 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 0 1 3 12c0-1.605.42-3.113 1.157-4.418"/></svg>`,
    desc: 'Data real-time dari endpoint ML Eksternal (192.168.101.3:6000)',
    details: [
      'Data dari ML API via GET /fleet, /result, dll',
      'Membutuhkan koneksi ke 192.168.101.3:6000',
      'Cocok untuk produksi & real monitoring',
    ],
    color: 'text-healthy',
    bgColor: 'bg-healthy/8',
    borderColor: 'border-healthy/30',
    dotColor: 'bg-healthy',
    badgeClass: 'bg-healthy/15 text-healthy border-healthy/30',
  },
]

// Sub-modes untuk halaman analisa
interface SubModeOption {
  id: PratyaksaSourceMode
  label: string
  desc: string
  icon: string
  color: string
  dotColor: string
}

const subModeOptions: SubModeOption[] = [
  {
    id: 'live-silent',
    label: 'Live (tanpa Telegram)',
    desc: 'Monitoring real-time tanpa notifikasi',
    icon: `<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75 11.25 15 15 9.75m-3-7.036A11.959 11.959 0 0 1 3.598 6 11.99 11.99 0 0 0 3 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285Z"/></svg>`,
    color: 'text-healthy',
    dotColor: 'bg-healthy',
  },
  {
    id: 'live-telegram',
    label: 'Live + Kirim Telegram',
    desc: 'Alert otomatis CRITICAL & WARNING ke Telegram',
    icon: `<svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M9.78 18.65l.28-4.23 7.68-6.92c.34-.31-.07-.46-.52-.19L7.74 13.3 3.64 12c-.88-.25-.89-.86.2-1.3l15.97-6.16c.73-.33 1.43.18 1.15 1.3l-2.72 12.81c-.19.91-.74 1.13-1.5.71L12.6 16.3l-1.99 1.93c-.23.23-.42.42-.83.42z"/></svg>`,
    color: 'text-steel',
    dotColor: 'bg-steel',
  },
  {
    id: 'hit-endpoint-sendiri',
    label: 'Hit Endpoint Sendiri',
    desc: 'Panggil endpoint kustom langsung dari frontend',
    icon: `<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M13.19 8.688a4.5 4.5 0 0 1 1.242 7.244l-4.5 4.5a4.5 4.5 0 0 1-6.364-6.364l1.757-1.757m13.35-.622 1.757-1.757a4.5 4.5 0 0 0-6.364-6.364l-4.5 4.5a4.5 4.5 0 0 0 1.242 7.244"/></svg>`,
    color: 'text-copper',
    dotColor: 'bg-copper',
  },
  {
    id: 'hit-endpoint-ml',
    label: 'Hit Endpoint ML',
    desc: 'Panggil endpoint ML terpisah untuk prediksi custom',
    icon: `<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M12 20V10m0 0L8 14m4-4 4 4M18 20V4M6 20v-4"/></svg>`,
    color: 'text-amber',
    dotColor: 'bg-amber',
  },
]

const isLocked = computed(() => {
  return (pratyaksa.status as any)?.manual_mode != null
})

const currentModeId = computed(() => {
  const m = (pratyaksa.status as any)?.mode || 'simulasi'
  return m
})

const selectMode = async (id: 'simulasi' | 'live') => {
  if (switching.value) return
  switching.value = true
  switchingMode.value = id

  try {
    await pratyaksa.setMode(id)
    await pratyaksa.fetchAll()
  } catch {
    // silent
  } finally {
    switching.value = false
    switchingMode.value = null
  }
}

const setSubMode = async (id: PratyaksaSourceMode) => {
  // Pastikan backend dalam mode live
  if (currentModeId.value !== 'live') {
    await selectMode('live')
  }
  pratyaksa.setSourceMode(id)
}

const testTelegramHandler = () => {
  emit('testTelegram')
}

const statusInfo = computed(() => {
  const s = pratyaksa.status as any
  return {
    mode: s?.mode || 'simulasi',
    manualMode: s?.manual_mode,
    reachable: s?.api_reachable,
    fleetCount: s?.fleet_count || 0,
  }
})
</script>

<template>
  <div>
    <!-- ── MODE LOCK TABLE ── -->
    <div class="panel overflow-hidden mb-5">
      <div class="px-5 py-3 border-b border-[color:var(--border)] flex items-center justify-between flex-wrap gap-2">
        <div class="flex items-center gap-2.5">
          <svg class="w-4 h-4 text-[color:var(--text-muted)]" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75 11.25 15 15 9.75m-3-7.036A11.959 11.959 0 0 1 3.598 6 11.99 11.99 0 0 0 3 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285Z"/></svg>
          <h3 class="font-semibold text-sm uppercase tracking-wider">Sumber Data</h3>
        </div>
        <div class="flex items-center gap-2">
          <span class="text-[10px] font-mono text-[color:var(--text-faint)]">
            Fleet: {{ statusInfo.fleetCount }} unit
          </span>
          <span v-if="isLocked" class="text-[10px] font-semibold px-2 py-0.5 rounded-full bg-amber/15 text-amber border border-amber/30 flex items-center gap-1">
            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5"><path stroke-linecap="round" stroke-linejoin="round" d="M16.5 10.5V6.75a4.5 4.5 0 1 0-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 0 0 2.25-2.25v-6.75a2.25 2.25 0 0 0-2.25-2.25H6.75a2.25 2.25 0 0 0-2.25 2.25v6.75a2.25 2.25 0 0 0 2.25 2.25Z"/></svg>
            TERKUNCI
          </span>
          <span v-else class="text-[10px] font-semibold px-2 py-0.5 rounded-full bg-steel/15 text-steel border border-steel/30 flex items-center gap-1">
            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5"><path stroke-linecap="round" stroke-linejoin="round" d="M13.5 10.5V6.75a4.5 4.5 0 1 1 9 0v3.75M3.75 21.75h10.5a2.25 2.25 0 0 0 2.25-2.25v-6.75a2.25 2.25 0 0 0-2.25-2.25H3.75a2.25 2.25 0 0 0-2.25 2.25v6.75a2.25 2.25 0 0 0 2.25 2.25Z"/></svg>
            AUTO
          </span>
        </div>
      </div>

      <!-- Dua kartu mode: Simulasi vs Live API -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-3 p-4">
        <button
          v-for="opt in modeOptions" :key="opt.id"
          :disabled="switching"
          class="relative rounded-xl border-2 p-4 text-left transition-all duration-200 disabled:opacity-70"
          :class="[
            currentModeId === opt.id
              ? `${opt.borderColor} ${opt.bgColor} ring-1 ring-inset ${opt.borderColor.replace('border', 'ring')}`
              : 'border-[color:var(--border)] hover:border-[color:var(--border-strong)] hover:bg-[color:var(--surface-2)]',
            switching && switchingMode === opt.id ? 'animate-pulse' : '',
          ]"
          @click="selectMode(opt.id)"
        >
          <!-- Status badge -->
          <div class="flex items-start justify-between mb-3">
            <div class="flex items-center gap-2.5">
              <span v-html="opt.icon" :class="currentModeId === opt.id ? opt.color : 'text-[color:var(--text-muted)]'"></span>
              <span class="font-display font-bold text-lg uppercase tracking-wide" :class="currentModeId === opt.id ? opt.color : 'text-[color:var(--text)]'">
                {{ opt.label }}
              </span>
            </div>
            <span
              v-if="currentModeId === opt.id"
              class="flex items-center gap-1 text-[10px] font-bold px-2 py-0.5 rounded-full"
              :class="opt.badgeClass"
            >
              <span class="w-1.5 h-1.5 rounded-full" :class="opt.dotColor + (opt.id === 'live' ? ' anim-live' : '')"></span>
              AKTIF
            </span>
          </div>

          <p class="text-xs text-[color:var(--text-muted)] mb-3">{{ opt.desc }}</p>

          <ul class="space-y-1">
            <li v-for="(d, i) in opt.details" :key="i" class="flex items-start gap-2 text-[11px] text-[color:var(--text-faint)]">
              <svg class="w-3 h-3 mt-0.5 shrink-0" :class="currentModeId === opt.id ? opt.color : 'text-[color:var(--text-faint)]'" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5"><path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5"/></svg>
              <span>{{ d }}</span>
            </li>
          </ul>

          <!-- Loading overlay -->
          <div v-if="switching && switchingMode === opt.id" class="absolute inset-0 rounded-xl bg-[color:var(--surface)]/60 flex items-center justify-center backdrop-blur-sm">
            <div class="flex items-center gap-2.5 font-semibold text-sm">
              <svg class="w-5 h-5 animate-spin" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/></svg>
              <span>{{ opt.id === 'live' ? 'Menghubungi 192.168.101.3...' : 'Mengaktifkan simulator...' }}</span>
            </div>
          </div>
        </button>
      </div>

      <!-- ── SUB-MODE (hanya untuk analisa) ── -->
      <div v-if="showSubModes && currentModeId === 'live'" class="border-t border-[color:var(--border)] px-4 py-3">
        <div class="flex items-center gap-2 mb-3">
          <svg class="w-3.5 h-3.5 text-[color:var(--text-muted)]" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5"/></svg>
          <span class="text-[11px] font-semibold uppercase tracking-wider text-[color:var(--text-muted)]">Mode Analisa Kerusakan</span>
        </div>
        <div class="flex flex-wrap gap-2">
          <button
            v-for="sm in subModeOptions" :key="sm.id"
            class="flex items-center gap-2 px-3 py-2 rounded-lg text-xs font-semibold border transition-all"
            :class="pratyaksa.sourceMode.value === sm.id
              ? `${sm.color} ${sm.color.replace('text', 'border').replace('copper', 'copper/50').replace('steel', 'steel/50').replace('amber', 'amber/50').replace('healthy', 'healthy/50')} ${sm.color.replace('text', 'bg').replace('copper', 'copper/10').replace('steel', 'steel/10').replace('amber', 'amber/10').replace('healthy', 'healthy/10')}`
              : 'border-[color:var(--border)] text-[color:var(--text-muted)] hover:bg-[color:var(--surface-2)]'"
            @click="setSubMode(sm.id)"
          >
            <span v-html="sm.icon" class="shrink-0"></span>
            <span>{{ sm.label }}</span>
            <span v-if="pratyaksa.sourceMode.value === sm.id" class="w-1.5 h-1.5 rounded-full" :class="sm.dotColor"></span>
          </button>
          <button
            class="flex items-center gap-2 px-3 py-2 rounded-lg text-xs font-semibold border border-[color:var(--border)] text-[color:var(--text-muted)] hover:bg-[color:var(--surface-2)] transition-all"
            @click="testTelegramHandler"
          >
            <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M8.29 6.293a9 9 0 1111.418 11.418M12 2a10 10 0 019.95 9M2 12h2m2-6-2-2m14 14l2 2M12 20v2m-4-2l-2 2"/></svg>
            Test Telegram
          </button>
        </div>
      </div>

      <!-- ── Connection status ── -->
      <div v-if="currentModeId === 'live'" class="border-t border-[color:var(--border)] px-4 py-2.5 flex items-center gap-3 flex-wrap text-[11px]">
        <div class="flex items-center gap-1.5">
          <span class="w-2 h-2 rounded-full" :class="statusInfo.reachable ? 'bg-healthy anim-live' : 'bg-critical'"></span>
          <span class="font-medium" :class="statusInfo.reachable ? 'text-healthy' : 'text-critical'">
            {{ statusInfo.reachable ? 'Terhubung' : 'Tidak Terhubung' }}
          </span>
          <span class="text-[color:var(--text-faint)]">ke 192.168.101.3:6000</span>
        </div>
        <span v-if="!isLocked" class="text-[color:var(--text-faint)]">· Mode auto — akan live jika API reachable</span>
        <span v-else class="text-amber">· Mode manual — terkunci oleh pengguna</span>
      </div>
      <div v-else class="border-t border-[color:var(--border)] px-4 py-2.5 flex items-center gap-1.5 text-[11px] text-[color:var(--text-faint)]">
        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M9.75 3.104v5.714a2.25 2.25 0 0 1-.659 1.591L5 14.5M9.75 3.104c-.251.023-.501.05-.75.082m.75-.082a24.301 24.301 0 0 1 4.5 0m0 0v5.714c0 .597.237 1.17.659 1.591L19.8 15.3M14.25 3.104c.251.023.501.05.75.082M19.8 15.3l-1.57.393A9.065 9.065 0 0 1 12 15a9.065 9.065 0 0 0-6.23.693L5 14.5m14.8.8 1.402 1.402c1.232 1.232.65 3.318-1.067 3.611A48.309 48.309 0 0 1 12 21c-2.773 0-5.491-.235-8.135-.687-1.718-.293-2.3-2.379-1.067-3.61L5 14.5"/></svg>
        <span>Mode simulasi lokal — semua data dari engine Rust internal, tidak ada koneksi eksternal</span>
      </div>
    </div>

    <!-- ── COMPACT VERSION (untuk halaman non-utama seperti jenis_alat_berat) ── -->
    <div v-if="compact" class="flex items-center gap-3 flex-wrap mb-5">
      <div class="flex rounded-xl overflow-hidden border border-[color:var(--border)]">
        <button
          v-for="opt in modeOptions" :key="opt.id"
          class="flex items-center gap-2 px-4 py-2 text-xs font-semibold transition-all"
          :class="currentModeId === opt.id
            ? `${opt.bgColor} ${opt.color}`
            : 'text-[color:var(--text-muted)] hover:bg-[color:var(--surface-2)]'"
          @click="selectMode(opt.id)"
        >
          <span class="w-2 h-2 rounded-full" :class="currentModeId === opt.id ? opt.dotColor + (opt.id === 'live' ? ' anim-live' : '') : 'bg-[color:var(--text-faint)]'"></span>
          {{ opt.label }}
        </button>
      </div>
      <span v-if="isLocked" class="text-[10px] font-semibold px-2 py-0.5 rounded-full bg-amber/15 text-amber border border-amber/30 flex items-center gap-1">
        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5"><path stroke-linecap="round" stroke-linejoin="round" d="M16.5 10.5V6.75a4.5 4.5 0 1 0-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 0 0 2.25-2.25v-6.75a2.25 2.25 0 0 0-2.25-2.25H6.75a2.25 2.25 0 0 0-2.25 2.25v6.75a2.25 2.25 0 0 0 2.25 2.25Z"/></svg>
        TERKUNCI
      </span>
      <span v-if="currentModeId === 'live'" class="text-[10px] flex items-center gap-1.5">
        <span class="w-1.5 h-1.5 rounded-full" :class="statusInfo.reachable ? 'bg-healthy anim-live' : 'bg-critical'"></span>
        <span :class="statusInfo.reachable ? 'text-healthy' : 'text-critical'">{{ statusInfo.reachable ? 'Online' : 'Offline' }}</span>
      </span>
    </div>
  </div>
</template>