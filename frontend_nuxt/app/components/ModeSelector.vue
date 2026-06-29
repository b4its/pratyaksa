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

interface SourceModeOption {
  value: PratyaksaSourceMode | 'simulasi'
  label: string
  desc: string
  color: string
  backendMode: 'live' | 'simulasi'
}

const baseModes: SourceModeOption[] = [
  { value: 'simulasi', label: 'Simulasi', desc: 'Data dari simulator internal (lokal, tanpa API eksternal)', color: 'text-warning border-warning/40 bg-warning/10', backendMode: 'simulasi' },
  { value: 'live-silent', label: 'Live (API)', desc: 'Data real-time dari endpoint ML Eksternal (192.168.101.3)', color: 'text-healthy border-healthy/40 bg-healthy/10', backendMode: 'live' },
  { value: 'live-telegram', label: 'Live + Telegram', desc: 'Data real-time + kirim notifikasi ke Telegram', color: 'text-steel border-steel/50 bg-steel/10', backendMode: 'live' },
]

const subModes: SourceModeOption[] = [
  { value: 'live-silent', label: 'Live (tanpa Telegram)', desc: 'Monitoring real-time via endpoint eksternal', color: 'text-healthy border-healthy/40 bg-healthy/10', backendMode: 'live' },
  { value: 'live-telegram', label: 'Live + Kirim Telegram', desc: 'Data real-time + kirim notifikasi WARNING/CRITICAL ke Telegram', color: 'text-steel border-steel/50 bg-steel/10', backendMode: 'live' },
  { value: 'hit-endpoint-sendiri', label: 'Hit Endpoint Sendiri', desc: 'Panggil endpoint ML eksternal langsung dari frontend', color: 'text-copper border-copper/50 bg-copper/10', backendMode: 'live' },
  { value: 'hit-endpoint-ml', label: 'Hit Endpoint ML', desc: 'Panggil endpoint ML terpisah (custom target)', color: 'text-amber border-amber/50 bg-amber/10', backendMode: 'live' },
]

const testTelegram = async () => {
  menuOpen.value = false
  emit('testTelegram')
}

const setMode = async (mode: PratyaksaSourceMode | 'simulasi') => {
  menuOpen.value = false

  // Cari option untuk tahu backendMode
  const allModes = [...(props.showSubModes ? subModes : baseModes), ...baseModes]
  const option = allModes.find(m => m.value === mode)
  if (!option) return

  // Set frontend source mode
  if (mode === 'simulasi') {
    pratyaksa.setSourceMode('live-silent') // fallback frontend mode
  } else {
    pratyaksa.setSourceMode(mode as PratyaksaSourceMode)
  }

  // Set backend mode via API
  switching.value = true
  await pratyaksa.setMode(option.backendMode)
  await pratyaksa.fetchAll()
  switching.value = false
}

const activeMode = computed(() => {
  const allModes = [...(props.showSubModes ? subModes : baseModes), ...baseModes]
  const bySource = allModes.find(m => m.value === pratyaksa.sourceMode.value)
  if (bySource) return bySource
  const backendMode = (pratyaksa.status as any)?.mode || 'simulasi'
  return allModes.find(m => m.backendMode === backendMode) || baseModes[0]
})

const modeColor = computed(() => {
  const m = activeMode.value
  return m?.color || 'border-warning/40 bg-warning/10 text-warning'
})

const modeLabel = computed(() => {
  if (switching.value) return 'MENYAMBUNG...'
  const m = activeMode.value
  if (m?.value === 'simulasi') return 'SIMULASI'
  if (m?.value === 'live-silent') return 'LIVE'
  if (m?.value === 'live-telegram') return 'LIVE+TG'
  if (m?.value === 'hit-endpoint-sendiri') return 'ENDPOINT'
  if (m?.value === 'hit-endpoint-ml') return 'ML API'
  return 'SIMULASI'
})

const isActuallyLive = computed(() => {
  const m = activeMode.value
  return m?.backendMode === 'live'
})

const isUsingSimulasi = computed(() => {
  const m = activeMode.value
  return m?.backendMode === 'simulasi'
})

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
    <div class="flex">
      <button
        class="flex items-center gap-2 px-3 py-2 rounded-xl text-xs font-semibold border transition-colors"
        :class="[modeColor, 'rounded-r-none cursor-pointer']"
        @click="menuOpen = !menuOpen"
      >
        <span v-if="switching" class="w-2 h-2 rounded-full bg-steel animate-spin border-2 border-transparent border-t-current"></span>
        <span v-else class="w-2 h-2 rounded-full"
          :class="activeMode?.value === 'live-telegram' ? 'bg-steel anim-live' : isActuallyLive ? 'bg-healthy anim-live' : 'bg-warning'">
        </span>
        <span>{{ modeLabel }}</span>
      </button>
      <button
        class="flex items-center justify-center px-2 rounded-xl rounded-l-none text-xs font-semibold border border-l-0 transition-colors"
        :class="modeColor"
        @click="menuOpen = !menuOpen"
      >
        <svg class="w-3.5 h-3.5 transition-transform" :class="{ 'rotate-180': menuOpen }" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5"><path d="m6 9 6 6 6-6"/></svg>
      </button>
    </div>

    <div v-if="menuOpen" class="absolute right-0 mt-2 w-72 z-50 panel-raised p-1.5 anim-pop">
      <template v-if="showSubModes">
        <button
          v-for="m in subModes" :key="m.value"
          class="w-full flex items-start gap-2.5 p-2.5 rounded-lg text-left hover:bg-[color:var(--surface-2)] transition-colors"
          @click="setMode(m.value)"
        >
          <span class="w-2.5 h-2.5 rounded-full mt-1 shrink-0"
            :class="m.value === 'live-silent' ? 'bg-healthy' : m.value === 'live-telegram' ? 'bg-steel' : m.value === 'hit-endpoint-sendiri' ? 'bg-copper' : 'bg-amber'">
          </span>
          <span class="flex-1 min-w-0">
            <span class="block font-semibold text-sm">{{ m.label }}</span>
            <span class="block text-[11px] text-[color:var(--text-muted)]">{{ m.desc }}</span>
          </span>
          <svg v-if="activeMode === m" class="w-4 h-4 ml-auto mt-0.5 shrink-0"
            :class="m.value === 'live-silent' ? 'text-healthy' : m.value === 'live-telegram' ? 'text-steel' : m.value === 'hit-endpoint-sendiri' ? 'text-copper' : 'text-amber'"
            fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5"><path d="M5 13l4 4L19 7"/></svg>
        </button>
        <hr class="my-1.5 border-[color:var(--border)]" />
        <button
          class="w-full flex items-center gap-2.5 p-2.5 rounded-lg text-left hover:bg-[color:var(--surface-2)] transition-colors text-sm font-medium text-[color:var(--text-muted)]"
          @click="testTelegram"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M8.29 6.293a9 9 0 1111.418 11.418M12 2a10 10 0 019.95 9M2 12h2m2-6l-2-2m14 14l2 2M12 20v2m-4-2l-2 2"/></svg>
          Uji Koneksi Telegram
        </button>
      </template>
      <template v-else>
        <button
          class="w-full flex items-start gap-2.5 p-2.5 rounded-lg text-left hover:bg-[color:var(--surface-2)] transition-colors"
          @click="setMode('simulasi')"
        >
          <span class="w-2.5 h-2.5 rounded-full bg-warning mt-1 shrink-0"></span>
          <span class="flex-1 min-w-0">
            <span class="block font-semibold text-sm">Simulasi</span>
            <span class="block text-[11px] text-[color:var(--text-muted)]">Data dari simulator internal Rust — tanpa koneksi ke API eksternal</span>
          </span>
          <svg v-if="isUsingSimulasi" class="w-4 h-4 text-warning ml-auto mt-0.5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5"><path d="M5 13l4 4L19 7"/></svg>
        </button>
        <button
          class="w-full flex items-start gap-2.5 p-2.5 rounded-lg text-left hover:bg-[color:var(--surface-2)] transition-colors"
          @click="setMode('live-silent')"
        >
          <span class="w-2.5 h-2.5 rounded-full bg-healthy mt-1 shrink-0"></span>
          <span class="flex-1 min-w-0">
            <span class="block font-semibold text-sm">Live (API)</span>
            <span class="block text-[11px] text-[color:var(--text-muted)]">Data real-time dari endpoint ML Eksternal (192.168.101.3:6000)</span>
          </span>
          <svg v-if="isActuallyLive && activeMode?.value !== 'live-telegram'" class="w-4 h-4 text-healthy ml-auto mt-0.5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5"><path d="M5 13l4 4L19 7"/></svg>
        </button>
        <button
          class="w-full flex items-start gap-2.5 p-2.5 rounded-lg text-left hover:bg-[color:var(--surface-2)] transition-colors"
          @click="setMode('live-telegram')"
        >
          <span class="w-2.5 h-2.5 rounded-full bg-steel mt-1 shrink-0"></span>
          <span class="flex-1 min-w-0">
            <span class="block font-semibold text-sm">Live + Telegram</span>
            <span class="block text-[11px] text-[color:var(--text-muted)]">Data real-time + kirim notifikasi ke Telegram</span>
          </span>
          <svg v-if="activeMode?.value === 'live-telegram'" class="w-4 h-4 text-steel ml-auto mt-0.5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5"><path d="M5 13l4 4L19 7"/></svg>
        </button>
        <hr class="my-1.5 border-[color:var(--border)]" />
        <div class="p-2.5">
          <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-faint)] mb-2">Status Backend</p>
          <div class="flex items-center gap-2 text-xs">
            <span class="w-2 h-2 rounded-full" :class="pratyaksa.isLive ? 'bg-healthy anim-live' : 'bg-warning'"></span>
            <span class="font-semibold" :class="pratyaksa.isLive ? 'text-healthy' : 'text-warning'">
              {{ (pratyaksa.status as any)?.mode === 'live' ? 'LIVE' : 'SIMULASI' }}
            </span>
          </div>
          <p v-if="(pratyaksa.status as any)?.manual_mode" class="text-[11px] text-steel mt-1">
            Mode manual: {{ (pratyaksa.status as any)?.manual_mode }}
          </p>
          <p v-else class="text-[11px] text-[color:var(--text-faint)] mt-1">
            Mode auto (deteksi otomatis dari health check)
          </p>
        </div>
      </template>
    </div>
  </div>
</template>