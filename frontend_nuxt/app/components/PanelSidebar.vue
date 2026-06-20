<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'

/**
 * Sidebar panel reusable & responsif.
 * - Desktop: bisa di-collapse (lebar penuh ↔ ikon saja), state disimpan di localStorage.
 * - Mobile: jadi drawer off-canvas dengan tombol hamburger + backdrop.
 * Menu aktif dideteksi otomatis dari route saat ini.
 */
const route = useRoute()
const { user, initAuth } = useAuth()
const { isDark, toggleTheme, initTheme } = useTheme()

interface MenuItem { name: string; path: string; icon: string }
const menuItems: MenuItem[] = [
  { name: 'Dashboard',         path: '/panel/dashboard',        icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="7" height="9" x="3" y="3" rx="1"/><rect width="7" height="5" x="14" y="3" rx="1"/><rect width="7" height="9" x="14" y="12" rx="1"/><rect width="7" height="5" x="3" y="16" rx="1"/></svg>` },
  { name: 'Jenis Alat Berat',  path: '/panel/jenis_alat_berat', icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 17h4V5H2v12h3"/><path d="M20 17h2v-9l-2.5-3.5H14v12h3"/><path d="M14 6h4.5"/><circle cx="18.5" cy="17.5" r="2.5"/><circle cx="5.5" cy="17.5" r="2.5"/></svg>` },
  { name: 'Unit Tambang',      path: '/panel/unit_tambang',     icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 12 12 17 22 12"/><polyline points="2 17 12 22 22 17"/></svg>` },
  { name: 'Analisa Kerusakan', path: '/panel/analisa',          icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>` },
  { name: 'Work Order',        path: '/panel/work_order',       icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.12 2.12 0 0 1 3 3L12 15l-4 1 1-4Z"/></svg>` },
  { name: 'Kembali',           path: '/',                       icon: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m12 19-7-7 7-7"/><path d="M19 12H5"/></svg>` },
]

const isActive = (item: MenuItem) => item.path !== '/' && route.path === item.path

const collapsed = ref(false)   // desktop: ikon saja
const mobileOpen = ref(false)  // mobile: drawer terbuka

onMounted(() => {
  initTheme()
  initAuth()
  if (import.meta.client) {
    collapsed.value = localStorage.getItem('panel_sidebar_collapsed') === '1'
  }
})
const toggleCollapse = () => {
  collapsed.value = !collapsed.value
  if (import.meta.client) localStorage.setItem('panel_sidebar_collapsed', collapsed.value ? '1' : '0')
}
// Tutup drawer setiap kali pindah halaman
watch(() => route.path, () => { mobileOpen.value = false })
</script>

<template>
  <!-- Tombol hamburger (mobile) -->
  <button
    class="lg:hidden fixed top-3 left-3 z-[60] w-11 h-11 rounded-xl bg-[color:var(--surface)] border border-[color:var(--border)] shadow-elev-sm flex items-center justify-center text-[color:var(--text)]"
    aria-label="Buka menu" @click="mobileOpen = true">
    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.2"><path d="M4 6h16M4 12h16M4 18h16"/></svg>
  </button>

  <!-- Backdrop (mobile) -->
  <transition name="ps-fade">
    <div v-if="mobileOpen" class="lg:hidden fixed inset-0 z-[70] bg-black/50 backdrop-blur-sm" @click="mobileOpen = false"></div>
  </transition>

  <!-- Sidebar -->
  <aside
    :class="[
      'fixed lg:static inset-y-0 left-0 z-[71] lg:z-10 h-screen shrink-0 flex flex-col justify-between',
      'border-r border-[color:var(--border)] bg-[color:var(--surface)] overflow-y-auto overflow-x-hidden',
      'transition-all duration-300 ease-out',
      collapsed ? 'p-3' : 'p-5',
      collapsed ? 'lg:w-20' : 'lg:w-72',
      'w-72',
      mobileOpen ? 'translate-x-0' : '-translate-x-full lg:translate-x-0',
    ]">
    <div>
      <!-- Header logo + tombol -->
      <div class="flex items-start justify-between gap-2 mb-8" :class="collapsed ? 'flex-col items-center gap-3' : ''">
        <div :class="collapsed ? 'flex justify-center w-full' : ''">
          <AppLogo v-if="!collapsed" height="2.4rem" />
          <img v-else src="/assets/pratyaksa_icon.png" alt="PRATYAKSA" class="w-10 h-10 object-contain" />
          <p v-if="!collapsed" class="text-[10px] font-semibold uppercase tracking-[0.18em] text-[color:var(--text-faint)] mt-2">Control Panel</p>
        </div>
        <!-- Tutup (mobile) -->
        <button class="lg:hidden p-1.5 rounded-lg hover:bg-[color:var(--surface-2)]" aria-label="Tutup menu" @click="mobileOpen = false">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.2"><path d="M6 18L18 6M6 6l12 12"/></svg>
        </button>
        <!-- Collapse (desktop) -->
        <button class="hidden lg:flex p-1.5 rounded-lg hover:bg-[color:var(--surface-2)] text-[color:var(--text-muted)]"
          :aria-label="collapsed ? 'Lebarkan sidebar' : 'Ciutkan sidebar'" @click="toggleCollapse">
          <svg class="w-5 h-5 transition-transform" :class="collapsed ? 'rotate-180' : ''" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.2"><path d="M15 18l-6-6 6-6"/></svg>
        </button>
      </div>

      <!-- Navigasi -->
      <nav class="space-y-1.5">
        <NuxtLink v-for="item in menuItems" :key="item.name" :to="item.path"
          :title="collapsed ? item.name : undefined"
          :class="['nav-link', isActive(item) ? 'nav-link-active' : '', collapsed ? 'lg:justify-center lg:px-0' : '']">
          <span class="flex items-center justify-center shrink-0" v-html="item.icon"></span>
          <span v-if="!collapsed" class="truncate">{{ item.name }}</span>
        </NuxtLink>
      </nav>
    </div>

    <div class="space-y-2 mt-6">
      <!-- Toggle tema -->
      <button @click="toggleTheme" class="theme-toggle" :class="collapsed ? 'lg:justify-center' : ''" aria-label="Ganti tema">
        <span class="flex items-center gap-2.5 font-semibold text-sm">
          <svg v-if="isDark" class="w-4 h-4 text-amber" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"/></svg>
          <svg v-else class="w-4 h-4 text-amber" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"/></svg>
          <span v-if="!collapsed">{{ isDark ? 'Mode Gelap' : 'Mode Terang' }}</span>
        </span>
        <span v-if="!collapsed" class="tt-switch" :class="{ 'tt-on': isDark }"><span class="tt-knob"></span></span>
      </button>
      <!-- User -->
      <div class="panel-flat" :class="collapsed ? 'p-2 flex justify-center' : 'p-4'">
        <template v-if="!collapsed">
          <p class="text-[10px] font-semibold uppercase tracking-wider text-[color:var(--text-faint)]">Logged in as</p>
          <div class="flex items-center gap-3 mt-2">
            <div class="w-9 h-9 rounded-full bg-steel-gradient flex items-center justify-center text-white font-bold text-sm">{{ (user?.name || 'A').charAt(0).toUpperCase() }}</div>
            <p class="font-semibold text-sm truncate">{{ user?.name || 'Admin' }}</p>
          </div>
        </template>
        <div v-else class="w-9 h-9 rounded-full bg-steel-gradient flex items-center justify-center text-white font-bold text-sm" :title="user?.name || 'Admin'">{{ (user?.name || 'A').charAt(0).toUpperCase() }}</div>
      </div>
    </div>
  </aside>
</template>

<style scoped>
.ps-fade-enter-active, .ps-fade-leave-active { transition: opacity 0.25s ease; }
.ps-fade-enter-from, .ps-fade-leave-to { opacity: 0; }
</style>
