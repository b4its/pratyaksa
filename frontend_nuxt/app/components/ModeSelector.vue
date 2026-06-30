<script setup lang="ts">
import type { PratyaksaSourceMode } from '~/composables/usePratyaksa'

const props = defineProps<{
  showSubModes?: boolean
}>()

const emit = defineEmits<{
  testTelegram: []
}>()

const pratyaksa = usePratyaksa()
const menuOpen = ref(false)
const switching = ref(false)
const toast = ref<{ ok: boolean; msg: string } | null>(null)
let toastTimer: any = null

const showToast = (ok: boolean, msg: string) => {
  toast.value = { ok, msg }
  if (toastTimer) clearTimeout(toastTimer)
  toastTimer = setTimeout(() => { toast.value = null }, 4000)
}

interface BranchOption {
  id: 'simulasi' | 'live'
  label: string
  desc: string
  icon: string
  color: string
  activeColor: string
  dotColor: string
}

const branches: BranchOption[] = [
  {
    id: 'simulasi',
    label: 'Live Simulasi',
    desc: 'Data dari simulator internal — tanpa koneksi eksternal',
    icon: `<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M9.75 3.104v5.714a2.25 2.25 0 0 1-.659 1.591L5 14.5M9.75 3.104c-.251.023-.501.05-.75.082m.75-.082a24.301 24.301 0 0 1 4.5 0m0 0v5.714c0 .597.237 1.17.659 1.591L19.8 15.3M14.25 3.104c.251.023.501.05.75.082M19.8 15.3l-1.57.393A9.065 9.065 0 0 1 12 15a9.065 9.065 0 0 0-6.23.693L5 14.5m14.8.8 1.402 1.402c1.232 1.232.65 3.318-1.067 3.611A48.309 48.309 0 0 1 12 21c-2.773 0-5.491-.235-8.135-.687-1.718-.293-2.3-2.379-1.067-3.61L5 14.5"/></svg>`,
    color: 'border-warning/30 bg-warning/8 text-warning',
    activeColor: 'text-warning',
    dotColor: 'bg-warning',
  },
  {
    id: 'live',
    label: 'Live API',
    desc: 'Data real-time dari 192.168.101.3:6000 — koneksi eksternal',
    icon: `<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M12 21a9.004 9.004 0 0 0 8.716-6.747M12 21a9.004 9.004 0 0 1-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 0 1 7.843 4.582M12 3a8.997 8.997 0 0 0-7.843 4.582m15.686 0A11.953 11.953 0 0 1 12 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0 1 21 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0 1 12 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 0 1 3 12c0-1.605.42-3.113 1.157-4.418"/></svg>`,
    color: 'border-healthy/30 bg-healthy/8 text-healthy',
    activeColor: 'text-healthy',
    dotColor: 'bg-healthy',
  },
]

interface SubOption {
  id: 'silent' | 'telegram'
  label: string
  desc: string
  icon: string
}

const subOptions: SubOption[] = [
  { id: 'silent', label: 'Tanpa Telegram', desc: 'Monitoring tanpa notifikasi', icon: `<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75 11.25 15 15 9.75m-3-7.036A11.959 11.959 0 0 1 3.598 6 11.99 11.99 0 0 0 3 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285Z"/></svg>` },
  { id: 'telegram', label: '+ Kirim Telegram', desc: 'Alert otomatis ke Telegram', icon: `<svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M9.78 18.65l.28-4.23 7.68-6.92c.34-.31-.07-.46-.52-.19L7.74 13.3 3.64 12c-.88-.25-.89-.86.2-1.3l15.97-6.16c.73-.33 1.43.18 1.15 1.3l-2.72 12.81c-.19.91-.74 1.13-1.5.71L12.6 16.3l-1.99 1.93c-.23.23-.42.42-.83.42z"/></svg>` },
]

const statusInfo = computed(() => ({
  mode: pratyaksa.status.value?.mode || 'simulasi',
  manualMode: pratyaksa.status.value?.manual_mode as string | null | undefined,
  reachable: pratyaksa.status.value?.api_reachable as boolean,
}))

const activeBranchId = computed((): 'simulasi' | 'live' => {
  const info = statusInfo.value
  if (info.manualMode) return info.manualMode as 'simulasi' | 'live'
  return (info.mode as 'simulasi' | 'live') || 'simulasi'
})

const activeSubId = computed((): 'silent' | 'telegram' => {
  const sm = pratyaksa.sourceMode.value
  if (sm === 'live-telegram') return 'telegram'
  return 'silent'
})

const activeBranch = computed(() => branches.find(b => b.id === activeBranchId.value) || branches[1]!)
const activeSub = computed(() => subOptions.find(s => s.id === activeSubId.value) || subOptions[0]!)

const modeLabel = computed(() => {
  if (switching.value) return 'MENYAMBUNG...'
  const b = activeBranch.value
  const s = activeSub.value
  if (!b || !s) return 'LIVE'
  return `${b.label} ${s.id === 'telegram' ? '+ TG' : ''}`
})

const modeColor = computed(() => {
  return activeBranch.value?.color?.replace('border-', 'border ').replace('bg-', 'bg ').replace('text-', 'text ') || ''
})

const switchBranch = async (branchId: 'simulasi' | 'live') => {
  if (switching.value) return
  switching.value = true
  try {
    await pratyaksa.setMode(branchId)
    const sm = activeSubId.value === 'telegram' ? 'live-telegram' : 'live-silent'
    pratyaksa.setSourceMode(sm)
    await pratyaksa.fetchAll()
  } catch (e: any) {
    const msg = e?.data?.message || e?.message || 'Gagal terhubung ke backend'
    showToast(false, `✗ Gagal ganti mode: ${msg}`)
  } finally {
    switching.value = false
  }
}

const switchSub = (subId: 'silent' | 'telegram') => {
  const sm = subId === 'telegram' ? 'live-telegram' : 'live-silent'
  pratyaksa.setSourceMode(sm)
  const label = subOptions.find(s => s.id === subId)?.label || subId
  showToast(true, `✓ ${label}`)
}

const containerRef = ref<HTMLElement | null>(null)
onMounted(() => {
  document.addEventListener('click', (e) => {
    if (containerRef.value && !containerRef.value.contains(e.target as Node)) {
      menuOpen.value = false
    }
  })
})
</script>

<template>
  <div ref="containerRef" class="relative">
    <button
      class="flex items-center gap-2 px-3 py-2 rounded-xl text-xs font-semibold border transition-colors cursor-pointer whitespace-nowrap"
      :class="[
        activeBranch?.color || '',
        switching ? 'animate-pulse' : ''
      ]"
      @click="menuOpen = !menuOpen"
    >
      <span v-if="switching" class="w-2 h-2 rounded-full bg-steel animate-spin border-2 border-transparent border-t-current"></span>
      <span v-else class="w-2 h-2 rounded-full" :class="(activeBranch?.dotColor || 'bg-warning') + (activeBranchId === 'live' ? ' anim-live' : '')"></span>
      <span>{{ modeLabel }}</span>
      <svg class="w-3 h-3 transition-transform" :class="{ 'rotate-180': menuOpen }" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5"><path d="m6 9 6 6 6-6"/></svg>
    </button>

    <div v-if="menuOpen" class="absolute right-0 mt-2 w-80 z-50 panel-raised p-2 anim-pop">

      <!-- Branch: Live Simulasi -->
      <div class="mb-1">
        <div class="px-2 py-1 flex items-center gap-2 text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-faint)]">
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M9.75 3.104v5.714a2.25 2.25 0 0 1-.659 1.591L5 14.5M9.75 3.104c-.251.023-.501.05-.75.082m.75-.082a24.301 24.301 0 0 1 4.5 0m0 0v5.714c0 .597.237 1.17.659 1.591L19.8 15.3M14.25 3.104c.251.023.501.05.75.082M19.8 15.3l-1.57.393A9.065 9.065 0 0 1 12 15a9.065 9.065 0 0 0-6.23.693L5 14.5m14.8.8 1.402 1.402c1.232 1.232.65 3.318-1.067 3.611A48.309 48.309 0 0 1 12 21c-2.773 0-5.491-.235-8.135-.687-1.718-.293-2.3-2.379-1.067-3.61L5 14.5"/></svg>
          Simulasi (Internal)
        </div>
        <button
          class="w-full flex items-center gap-3 p-2.5 rounded-lg text-left transition-colors"
          :class="activeBranchId === 'simulasi' ? 'bg-warning/10 border border-warning/30' : 'hover:bg-[color:var(--surface-2)]'"
          @click="switchBranch('simulasi')"
        >
          <span class="w-2 h-2 rounded-full mt-0.5 shrink-0" :class="activeBranchId === 'simulasi' ? 'bg-warning' : 'bg-[color:var(--text-faint)]'"></span>
          <span class="flex-1 min-w-0">
            <span class="block text-sm font-semibold" :class="activeBranchId === 'simulasi' ? 'text-warning' : ''">Live Simulasi</span>
            <span class="block text-[10px] text-[color:var(--text-muted)]">Data dari engine Rust internal</span>
          </span>
          <svg v-if="activeBranchId === 'simulasi'" class="w-4 h-4 text-warning shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5"><path d="M5 13l4 4L19 7"/></svg>
        </button>
        <div v-if="activeBranchId === 'simulasi'" class="flex gap-1.5 pl-8 mt-1.5 mb-2">
          <button
            v-for="s in subOptions" :key="s.id"
            class="flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-[11px] font-medium border transition-all"
            :class="activeSubId === s.id
              ? 'bg-warning/15 border-warning/30 text-warning'
              : 'border-[color:var(--border)] text-[color:var(--text-muted)] hover:border-[color:var(--border-strong)]'"
            @click="switchSub(s.id)"
          >
            <span v-html="s.icon"></span>
            <span>{{ s.label }}</span>
          </button>
        </div>
      </div>

      <div class="border-t border-[color:var(--border)] my-1.5"></div>

      <!-- Branch: Live API -->
      <div class="mb-1">
        <div class="px-2 py-1 flex items-center gap-2 text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-faint)]">
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M12 21a9.004 9.004 0 0 0 8.716-6.747M12 21a9.004 9.004 0 0 1-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 0 1 7.843 4.582M12 3a8.997 8.997 0 0 0-7.843 4.582m15.686 0A11.953 11.953 0 0 1 12 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0 1 21 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0 1 12 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 0 1 3 12c0-1.605.42-3.113 1.157-4.418"/></svg>
          API Eksternal (192.168.101.3:6000)
        </div>
        <button
          class="w-full flex items-center gap-3 p-2.5 rounded-lg text-left transition-colors"
          :class="activeBranchId === 'live' ? 'bg-healthy/10 border border-healthy/30' : 'hover:bg-[color:var(--surface-2)]'"
          @click="switchBranch('live')"
        >
          <span class="w-2 h-2 rounded-full mt-0.5 shrink-0" :class="activeBranchId === 'live' ? 'bg-healthy anim-live' : 'bg-[color:var(--text-faint)]'"></span>
          <span class="flex-1 min-w-0">
            <span class="block text-sm font-semibold" :class="activeBranchId === 'live' ? 'text-healthy' : ''">Live API</span>
            <span class="block text-[10px] text-[color:var(--text-muted)]">Data dari 192.168.101.3:6000</span>
          </span>
          <svg v-if="activeBranchId === 'live'" class="w-4 h-4 text-healthy shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5"><path d="M5 13l4 4L19 7"/></svg>
        </button>
        <div v-if="activeBranchId === 'live'" class="flex gap-1.5 pl-8 mt-1.5 mb-2">
          <button
            v-for="s in subOptions" :key="s.id"
            class="flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-[11px] font-medium border transition-all"
            :class="activeSubId === s.id
              ? 'bg-healthy/15 border-healthy/30 text-healthy'
              : 'border-[color:var(--border)] text-[color:var(--text-muted)] hover:border-[color:var(--border-strong)]'"
            @click="switchSub(s.id)"
          >
            <span v-html="s.icon"></span>
            <span>{{ s.label }}</span>
          </button>
        </div>
      </div>

      <!-- Status Footer -->
      <div v-if="toast" class="px-2 pt-2">
      <div class="px-2.5 py-1.5 rounded-lg text-[11px] font-semibold flex items-center gap-1.5 transition-all anim-pop"
        :class="toast.ok ? 'bg-healthy/15 text-healthy' : 'bg-critical/15 text-critical'">
        <span>{{ toast.msg }}</span>
      </div>
    </div>
    <div class="border-t border-[color:var(--border)] mt-1.5 pt-2 px-2">
        <div class="flex items-center justify-between text-[10px]">
          <div class="flex items-center gap-1.5">
            <span class="font-semibold uppercase tracking-wider text-[color:var(--text-faint)]">Status Backend:</span>
            <span class="flex items-center gap-1" :class="activeBranch.activeColor">
              <span class="w-1.5 h-1.5 rounded-full" :class="activeBranch.dotColor"></span>
              <span class="font-bold">{{ activeBranch.label }}</span>
            </span>
          </div>
          <div class="flex items-center gap-1 text-[color:var(--text-faint)]">
            <span v-if="statusInfo.manualMode" class="text-amber font-semibold">(Manual)</span>
            <span v-else>(Auto)</span>
            <span v-if="activeBranchId === 'live'" class="flex items-center gap-1">
              ·
              <span class="w-1.5 h-1.5 rounded-full" :class="statusInfo.reachable ? 'bg-healthy' : 'bg-critical'"></span>
              <span :class="statusInfo.reachable ? 'text-healthy' : 'text-critical'">{{ statusInfo.reachable ? 'Online' : 'Offline' }}</span>
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>