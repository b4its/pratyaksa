<template>
  <div class="min-h-screen text-[color:var(--text)] bg-[color:var(--bg)] selection:bg-amber selection:text-graphite-900 font-sans transition-colors duration-500 overflow-x-clip">

    <!-- NAV -->
    <nav ref="navEl" :class="navOnDark ? 'nav-bar--dark' : 'nav-bar--light'" class="nav-bar sticky top-0 z-50 backdrop-blur-md border-b px-6 py-3.5 flex justify-between items-center w-full transition-colors duration-500">
      <div class="flex items-center cursor-pointer group">
        <AppLogo height="2.25rem" :on-dark="navOnDark" class="group-hover:scale-105 transition-transform" />
      </div>

      <div class="xl:hidden flex gap-2">
        <button @click="toggleTheme" aria-label="Toggle Dark Mode" class="btn btn-ghost !p-2.5">
          <svg v-if="!isDark" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"></path></svg>
          <svg v-else class="w-5 h-5 text-amber" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path></svg>
        </button>
        <button @click="toggleMenu" aria-label="Toggle Navigation Menu" class="btn btn-amber !p-2.5">
          <svg v-if="!isMenuOpen" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M4 6h16M4 12h16M4 18h16"></path></svg>
          <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>

      <div class="hidden xl:flex gap-7 font-semibold items-center text-sm">
        <a href="#tentang" class="hover:text-amber transition-colors">Tentang Kami</a>
        <a href="#solusi" class="hover:text-amber transition-colors">Solusi Pratyaksa</a>
        <a href="#keunggulan" class="hover:text-amber transition-colors">Keunggulan</a>
      </div>

      <div class="hidden xl:flex items-center gap-3">
        <button @click="toggleTheme" aria-label="Toggle Dark Mode" class="btn btn-ghost !p-2.5">
          <svg v-if="!isDark" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"></path></svg>
          <svg v-else class="w-5 h-5 text-amber" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path></svg>
        </button>

        <template v-if="isAuthenticated">
          <div class="flex items-center gap-2.5 pl-1 pr-3 py-1.5 rounded-xl border border-[color:var(--border)] bg-[color:var(--surface-2)]">
            <div class="w-8 h-8 rounded-full bg-steel-gradient flex items-center justify-center text-white font-bold text-xs">{{ (user?.name || 'A').charAt(0).toUpperCase() }}</div>
            <span class="font-semibold text-sm max-w-[120px] truncate text-[color:var(--text)]">{{ user?.name || 'Operator' }}</span>
          </div>
          <NuxtLink to="panel/dashboard" class="btn btn-amber px-6">Buka Dashboard →</NuxtLink>
          <button @click="logout" aria-label="Keluar" class="btn btn-ghost !p-2.5" title="Keluar">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"/></svg>
          </button>
        </template>
        <template v-else>
          <NuxtLink to="account/login" class="btn btn-ghost px-6">Masuk</NuxtLink>
          <NuxtLink to="account/register" class="btn btn-amber px-6">Daftar Gratis</NuxtLink>
        </template>
      </div>
    </nav>

    <!-- MOBILE MENU -->
    <div v-show="isMenuOpen" class="xl:hidden fixed inset-0 z-40 bg-[color:var(--surface)] pt-24 px-6 flex flex-col gap-2 font-semibold text-lg transition-colors duration-500">
      <a href="#tentang" @click="toggleMenu" class="py-3 border-b border-[color:var(--border)] hover:text-amber transition-colors">Tentang Kami</a>
      <a href="#solusi" @click="toggleMenu" class="py-3 border-b border-[color:var(--border)] hover:text-amber transition-colors">Solusi Pratyaksa</a>
      <a href="#keunggulan" @click="toggleMenu" class="py-3 border-b border-[color:var(--border)] hover:text-amber transition-colors">Keunggulan</a>
      <div class="mt-6 flex flex-col gap-3">
        <template v-if="isAuthenticated">
          <div class="flex items-center gap-3 p-3 rounded-xl border border-[color:var(--border)] bg-[color:var(--surface-2)]">
            <div class="w-9 h-9 rounded-full bg-steel-gradient flex items-center justify-center text-white font-bold text-sm">{{ (user?.name || 'A').charAt(0).toUpperCase() }}</div>
            <span class="font-semibold truncate">{{ user?.name || 'Operator' }}</span>
          </div>
          <NuxtLink to="panel/dashboard" @click="toggleMenu" class="btn btn-amber w-full !py-3.5">Buka Dashboard →</NuxtLink>
          <button @click="logout(); toggleMenu()" class="btn btn-ghost w-full !py-3.5">Keluar</button>
        </template>
        <template v-else>
          <NuxtLink to="account/login" @click="toggleMenu" class="btn btn-ghost w-full !py-3.5">Masuk</NuxtLink>
          <NuxtLink to="account/register" @click="toggleMenu" class="btn btn-amber w-full !py-3.5">Daftar Gratis</NuxtLink>
        </template>
      </div>
    </div>

    <!-- HERO -->
    <header class="relative overflow-hidden hero-bg text-white transition-colors duration-500">
      <!-- aurora gradient bergerak — memberi kedalaman & nuansa hidup -->
      <div class="aurora-wrap absolute inset-0 pointer-events-none">
        <span class="aurora aurora-a"></span>
        <span class="aurora aurora-b"></span>
        <span class="aurora aurora-c"></span>
      </div>
      <div class="absolute inset-0 bg-mesh opacity-25"></div>
      <div class="absolute inset-0 bg-topo opacity-20"></div>
      <!-- floating parallax accents -->
      <div ref="shape1" class="parallax absolute top-20 left-10 w-24 h-24 rounded-2xl border border-amber/40 bg-amber/10 backdrop-blur-sm hidden md:block anim-float"></div>
      <div ref="shape2" class="parallax absolute bottom-28 right-20 w-20 h-20 rounded-full border border-steel/50 bg-steel/10 hidden md:block anim-float" style="animation-delay:1s"></div>
      <div ref="shape3" class="parallax absolute top-1/3 right-1/3 w-14 h-14 rounded-xl border border-white/15 bg-white/5 hidden lg:block anim-float" style="animation-delay:.5s"></div>

      <div class="relative z-10 max-w-7xl mx-auto px-6 py-20 md:py-28 grid lg:grid-cols-2 gap-12 items-center">
        <!-- left: copy -->
        <div class="anim-left">
          <div class="inline-flex items-center gap-2 bg-amber/15 text-amber border border-amber/40 px-4 py-1.5 rounded-full font-semibold uppercase text-xs tracking-wider mb-6">
            <span class="w-2 h-2 rounded-full bg-amber anim-live"></span> Mining Intelligence Platform
          </div>
          <h1 class="font-display text-5xl md:text-7xl font-bold uppercase leading-[1.02] tracking-wide mb-6">
            Kendalikan<br>Tambang Anda<br>
            <span class="text-gradient-amber">Dengan Data</span>
          </h1>
          <p class="text-lg md:text-xl mb-8 max-w-xl text-graphite-100 leading-relaxed">
            Pratyaksa mengintegrasikan IoT dan Machine Learning untuk memprediksi kerusakan alat berat dan mengotomatisasi keputusan operasional secara real-time.
          </p>
          <div class="flex flex-col sm:flex-row gap-4">
            <NuxtLink v-if="isAuthenticated" to="panel/dashboard" class="btn btn-amber !py-4 px-8 text-base">Buka Dashboard →</NuxtLink>
            <NuxtLink v-else to="account/register" class="btn btn-amber !py-4 px-8 text-base">Mulai Sekarang →</NuxtLink>
            <a href="#solusi" class="btn !py-4 px-8 text-base bg-white/10 text-white border border-white/20 hover:bg-white/15 backdrop-blur-sm">Lihat Solusi</a>
          </div>
        </div>

        <!-- right: interactive 3D model -->
        <div class="anim-pop">
          <div class="viewer-3d rounded-2xl border border-white/10 bg-graphite-950/40 backdrop-blur-sm shadow-elev-lg relative overflow-hidden h-[340px] md:h-[460px] cursor-grab active:cursor-grabbing">
            <model-viewer
              v-if="isMounted"
              src="/media/models/dump_truck.glb"
              alt="Visual 3D armada Pratyaksa"
              camera-controls
              auto-rotate
              auto-rotate-delay="0"
              rotation-per-second="32deg"
              shadow-intensity="1.5"
              exposure="1.15"
              environment-image="neutral"
              interaction-prompt="none"
              class="w-full h-full outline-none"
              style="background-color:transparent;"
            ></model-viewer>
            <div class="absolute top-3 left-3 bg-steel/90 text-white text-[10px] font-semibold px-2.5 py-1 rounded-full pointer-events-none flex items-center gap-1.5">
              <span class="w-1.5 h-1.5 rounded-full bg-white anim-live"></span> LIVE 3D VISUAL
            </div>
            <div class="absolute bottom-3 right-3 bg-graphite-900/80 text-graphite-100 text-[10px] font-medium px-2.5 py-1 rounded-full pointer-events-none">DRAG UNTUK PUTAR 360°</div>
          </div>
        </div>
      </div>
    </header>

    <!-- MARQUEE -->
    <div class="bg-amber-gradient text-graphite-900 overflow-hidden flex whitespace-nowrap py-2.5 border-y border-amber-deep/30">
      <div class="animate-marquee font-semibold text-sm uppercase tracking-[0.15em] flex gap-6 items-center">
        <span>⚡ IoT Sensor Integration</span><span class="opacity-50">•</span>
        <span>🚧 Predictive Maintenance</span><span class="opacity-50">•</span>
        <span>📊 Machine Learning Analysis</span><span class="opacity-50">•</span>
        <span>⚙️ Automated Alerting</span><span class="opacity-50">•</span>
        <span>⚡ IoT Sensor Integration</span><span class="opacity-50">•</span>
        <span>🚧 Predictive Maintenance</span><span class="opacity-50">•</span>
        <span>📊 Machine Learning Analysis</span><span class="opacity-50">•</span>
        <span>⚙️ Automated Alerting</span>
      </div>
    </div>

    <!-- STATS COUNTER -->
    <section ref="statsSection" class="py-16 px-6 md:px-20 bg-[color:var(--bg)] bg-ore-dots">
      <div class="max-w-6xl mx-auto grid grid-cols-2 lg:grid-cols-4 gap-6">
        <div v-for="(s, i) in stats" :key="s.label" v-tilt class="tilt-card kpi p-6 text-center" :style="{ '--accent': s.accent }">
          <span class="tilt-shine"></span>
          <p class="font-display text-4xl md:text-5xl font-bold" :style="{ color: s.accent }">{{ statDisplay[i] }}{{ s.suffix }}</p>
          <p class="text-xs font-semibold uppercase tracking-wider mt-2 text-[color:var(--text-muted)]">{{ s.label }}</p>
        </div>
      </div>
    </section>

    <!-- TENTANG -->
    <section id="tentang" class="py-24 px-6 md:px-20 bg-[color:var(--surface)] relative border-y border-[color:var(--border)]">
      <div class="absolute inset-0 bg-mesh opacity-40"></div>
      <div v-reveal class="max-w-4xl mx-auto text-center relative z-10 panel-raised p-10">
        <span class="badge bg-copper/10 border-copper/40 text-copper mb-6">Tentang Kami</span>
        <h2 class="font-display text-4xl md:text-6xl font-bold uppercase mb-6 tracking-wide">Apa itu <span class="text-amber">Pratyaksa?</span></h2>
        <div class="w-16 h-1 bg-amber mx-auto mb-8 rounded-full"></div>
        <p class="text-xl md:text-2xl leading-relaxed text-[color:var(--text-muted)]">
          Pratyaksa adalah jembatan antara operasi alat berat dan kecerdasan buatan. Kami mengubah data mentah dari armada tambang menjadi
          <span class="text-amber font-semibold">wawasan preskriptif</span>, memastikan alat berat Anda beroperasi pada efisiensi maksimal.
        </p>
      </div>
    </section>

    <!-- SOLUSI -->
    <section id="solusi" class="py-24 px-6 md:px-20 section-dark text-white relative overflow-hidden">
      <span class="aurora aurora-soft"></span>
      <div class="absolute inset-0 bg-topo opacity-20"></div>
      <div class="max-w-7xl mx-auto relative z-10">
        <span v-reveal class="badge bg-amber/15 border-amber/40 text-amber mb-5">End-to-End</span>
        <h2 v-reveal class="font-display text-5xl md:text-6xl font-bold uppercase mb-14 tracking-wide">Solusi End-to-End</h2>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-7">
          <div v-reveal="{ anim: 'up', delay: 0 }" v-tilt class="tilt-card relative overflow-hidden rounded-2xl border border-white/10 bg-white/[0.04] backdrop-blur-sm p-8 flex flex-col h-full hover:border-amber/40 transition-colors">
            <span class="tilt-shine"></span>
            <div class="w-14 h-14 rounded-xl bg-amber-gradient flex items-center justify-center mb-6 shadow-[0_8px_20px_-8px_rgba(242,166,12,0.6)] anim-float">
              <svg class="w-7 h-7 text-graphite-900" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
            </div>
            <h3 class="font-display text-2xl font-bold uppercase mb-3 tracking-wide">Predictive Maintenance</h3>
            <p class="text-graphite-200 leading-relaxed flex-grow">Mendeteksi anomali suhu, tekanan, dan vibrasi sebelum kerusakan fatal terjadi via Machine Learning.</p>
          </div>

          <div v-reveal="{ anim: 'up', delay: 120 }" v-tilt class="tilt-card relative overflow-hidden rounded-2xl border border-white/10 bg-white/[0.04] backdrop-blur-sm p-8 flex flex-col h-full hover:border-steel/50 transition-colors">
            <span class="tilt-shine"></span>
            <div class="w-14 h-14 rounded-xl bg-copper-gradient flex items-center justify-center mb-6 shadow-[0_8px_20px_-8px_rgba(194,112,61,0.6)] anim-float" style="animation-delay:.5s">
              <svg class="w-7 h-7 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path></svg>
            </div>
            <h3 class="font-display text-2xl font-bold uppercase mb-3 tracking-wide">Real-time Telematics</h3>
            <p class="text-graphite-200 leading-relaxed flex-grow">Pemantauan lokasi presisi, konsumsi bahan bakar, dan perilaku operator dalam satu dashboard.</p>
          </div>

          <div v-reveal="{ anim: 'up', delay: 240 }" v-tilt class="tilt-card relative overflow-hidden rounded-2xl border border-white/10 bg-white/[0.04] backdrop-blur-sm p-8 flex flex-col h-full hover:border-steel/50 transition-colors">
            <span class="tilt-shine"></span>
            <div class="w-14 h-14 rounded-xl bg-steel flex items-center justify-center mb-6 shadow-[0_8px_20px_-8px_rgba(62,146,204,0.6)] anim-float" style="animation-delay:1s">
              <svg class="w-7 h-7 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.2" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z"></path></svg>
            </div>
            <h3 class="font-display text-2xl font-bold uppercase mb-3 tracking-wide">Automated Alerting</h3>
            <p class="text-graphite-200 leading-relaxed flex-grow">Sistem preskriptif otomatis yang merilis tiket perbaikan via Telegram & WhatsApp ke tim mekanik.</p>
          </div>
        </div>
      </div>
    </section>

    <!-- KEUNGGULAN -->
    <section id="keunggulan" class="py-24 px-6 md:px-20 bg-[color:var(--surface)] border-b border-[color:var(--border)]">
      <div class="max-w-7xl mx-auto flex flex-col lg:flex-row gap-16 items-center">
        <!-- 3D showcase -->
        <div v-reveal="'left'" class="w-full lg:w-1/2">
          <div class="viewer-3d w-full aspect-square rounded-2xl border border-[color:var(--border)] bg-steel-gradient shadow-neo relative overflow-hidden cursor-grab active:cursor-grabbing">
            <div class="absolute top-0 left-0 w-full h-10 border-b border-white/10 bg-graphite-900/60 backdrop-blur flex items-center px-4 gap-2 z-10">
              <div class="w-3 h-3 rounded-full bg-critical"></div>
              <div class="w-3 h-3 rounded-full bg-warning"></div>
              <div class="w-3 h-3 rounded-full bg-healthy"></div>
              <span class="ml-auto text-[10px] font-mono text-graphite-300">pratyaksa://3d-viewer</span>
            </div>
            <model-viewer
              v-if="isMounted"
              src="/media/models/bulldozer.glb"
              alt="Showcase 3D"
              camera-controls
              auto-rotate
              auto-rotate-delay="0"
              rotation-per-second="28deg"
              shadow-intensity="1.4"
              exposure="1.1"
              environment-image="neutral"
              interaction-prompt="none"
              class="w-full h-full pt-10 outline-none"
              style="background-color:transparent;"
            ></model-viewer>
          </div>
        </div>

        <div class="w-full lg:w-1/2">
          <span v-reveal class="badge bg-amber/10 border-amber/40 text-amber-deep mb-5">Keunggulan</span>
          <h2 v-reveal class="font-display text-5xl md:text-6xl font-bold uppercase mb-10 tracking-wide">Kenapa <span class="text-amber">Pratyaksa?</span></h2>
          <ul class="space-y-6">
            <li v-reveal="{ anim: 'left', delay: 0 }" class="flex gap-5 group">
              <div class="w-12 h-12 shrink-0 rounded-xl bg-amber/15 border border-amber/40 flex justify-center items-center text-amber group-hover:bg-amber group-hover:text-graphite-900 transition-all">
                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M5 13l4 4L19 7"/></svg>
              </div>
              <div>
                <h4 class="font-display text-2xl font-bold mb-1.5 uppercase tracking-wide">Agnostik Terhadap Merek</h4>
                <p class="text-[color:var(--text-muted)] leading-relaxed">Sensor IoT kami terintegrasi dengan berbagai merk (Komatsu, Caterpillar, Volvo) tanpa kendala.</p>
              </div>
            </li>
            <li v-reveal="{ anim: 'left', delay: 120 }" class="flex gap-5 group">
              <div class="w-12 h-12 shrink-0 rounded-xl bg-steel/15 border border-steel/40 flex justify-center items-center text-steel group-hover:bg-steel group-hover:text-white transition-all">
                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M5 13l4 4L19 7"/></svg>
              </div>
              <div>
                <h4 class="font-display text-2xl font-bold mb-1.5 uppercase tracking-wide">Infrastruktur Berbasis Lokal</h4>
                <p class="text-[color:var(--text-muted)] leading-relaxed">Skalabilitas tinggi. Data aman dan dapat diakses dari site tambang mana pun dengan latensi minimal.</p>
              </div>
            </li>
            <li v-reveal="{ anim: 'left', delay: 240 }" class="flex gap-5 group">
              <div class="w-12 h-12 shrink-0 rounded-xl bg-copper/15 border border-copper/40 flex justify-center items-center text-copper group-hover:bg-copper group-hover:text-white transition-all">
                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M5 13l4 4L19 7"/></svg>
              </div>
              <div>
                <h4 class="font-display text-2xl font-bold mb-1.5 uppercase tracking-wide">ROI Terukur Cepat</h4>
                <p class="text-[color:var(--text-muted)] leading-relaxed">Klien mencatatkan penghematan biaya <b>maintenance</b> hingga 25% pada kuartal pertama implementasi.</p>
              </div>
            </li>
          </ul>
        </div>
      </div>
    </section>

    <!-- FOOTER CTA -->
    <footer class="section-dark text-white py-28 px-6 text-center relative overflow-hidden">
      <span class="aurora aurora-soft"></span>
      <div class="absolute inset-0 bg-topo opacity-25"></div>
      <div class="absolute top-0 left-0 w-full h-1 hazard-stripe opacity-80"></div>
      <div v-reveal class="max-w-4xl mx-auto relative z-10">
        <h2 class="font-display text-4xl md:text-6xl font-bold uppercase mb-10 tracking-wide leading-tight">Tingkatkan Efektivitas Tambang Anda Sekarang</h2>
        <NuxtLink v-if="isAuthenticated" to="panel/dashboard" class="btn btn-amber !py-5 px-10 text-xl md:text-2xl">Buka Dashboard →</NuxtLink>
        <NuxtLink v-else to="account/register" class="btn btn-amber !py-5 px-10 text-xl md:text-2xl">Ayo Bergabung Bersama Kami</NuxtLink>
      </div>
    </footer>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'

// Status autentikasi — untuk menyesuaikan tampilan landing bila sudah login
const { isAuthenticated, user, initAuth, logout } = useAuth()

// Muat web component model-viewer
useHead({
  script: [
    { type: 'module', src: 'https://ajax.googleapis.com/ajax/libs/model-viewer/3.3.0/model-viewer.min.js' },
  ],
})

const isDark = ref(false)
const isMounted = ref(false)
const isMenuOpen = ref(false)

// Navbar adaptif: deteksi apakah di belakang navbar ada section gelap,
// lalu balik warna agar teks tidak bentrok dengan latar.
const navEl = ref(null)
const navOnDark = ref(true)

// Parallax shapes
const shape1 = ref(null)
const shape2 = ref(null)
const shape3 = ref(null)

// Stats counter
const stats = [
  { label: 'Physical Availability', target: 93, suffix: '%', accent: '#1FA971' },
  { label: 'Akurasi AI',            target: 95, suffix: '%', accent: '#3E92CC' },
  { label: 'Reduksi Downtime',      target: 45, suffix: '%', accent: '#F2A60C' },
  { label: 'Hemat Maintenance',     target: 30, suffix: '%', accent: '#C2703D' },
]
const statDisplay = ref(stats.map(() => 0))
const statsSection = ref(null)
let statsAnimated = false

const animateStats = () => {
  if (statsAnimated) return
  statsAnimated = true
  stats.forEach((s, i) => {
    const start = performance.now()
    const dur = 1400
    const step = (now) => {
      const t = Math.min((now - start) / dur, 1)
      const eased = 1 - Math.pow(1 - t, 3)
      statDisplay.value[i] = Math.round(s.target * eased)
      if (t < 1) requestAnimationFrame(step)
    }
    requestAnimationFrame(step)
  })
}

const updateDOM = (dark) => {
  if (import.meta.client) {
    document.documentElement.classList.toggle('dark', dark)
  }
}
const toggleTheme = () => {
  isDark.value = !isDark.value
  localStorage.setItem('theme', isDark.value ? 'dark' : 'light')
  updateDOM(isDark.value)
}
const toggleMenu = () => { isMenuOpen.value = !isMenuOpen.value }

// Cek section gelap yang sedang berada di belakang navbar
const updateNavContrast = () => {
  if (!import.meta.client) return
  const nav = navEl.value
  if (!nav) return
  // Titik sampel = tepat di bawah garis tengah navbar
  const probeY = nav.getBoundingClientRect().height / 2
  const darkEls = [
    document.querySelector('header.hero-bg'),
    document.getElementById('solusi'),
    document.querySelector('footer.section-dark'),
  ].filter(Boolean)
  let onDark = false
  for (const el of darkEls) {
    const r = el.getBoundingClientRect()
    if (r.top <= probeY && r.bottom >= probeY) { onDark = true; break }
  }
  navOnDark.value = onDark
}

let onMove = null
let statsObs = null
let onScroll = null

onMounted(() => {
  isMounted.value = true
  initAuth()
  const savedTheme = localStorage.getItem('theme')
  const osPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
  isDark.value = savedTheme === 'dark' || (!savedTheme && osPrefersDark)
  updateDOM(isDark.value)

  // mouse parallax
  onMove = (e) => {
    const cx = window.innerWidth / 2
    const cy = window.innerHeight / 2
    const dx = (e.clientX - cx) / cx
    const dy = (e.clientY - cy) / cy
    if (shape1.value) shape1.value.style.transform = `translate(${dx * 30}px, ${dy * 30}px)`
    if (shape2.value) shape2.value.style.transform = `translate(${dx * -25}px, ${dy * -25}px)`
    if (shape3.value) shape3.value.style.transform = `translate(${dx * 40}px, ${dy * 40}px)`
  }
  window.addEventListener('mousemove', onMove)

  // Navbar adaptif — pantau saat scroll & resize
  onScroll = () => updateNavContrast()
  window.addEventListener('scroll', onScroll, { passive: true })
  window.addEventListener('resize', onScroll)
  // jalankan beberapa kali untuk menangkap layout setelah model-viewer/gambar termuat
  updateNavContrast()
  requestAnimationFrame(updateNavContrast)
  setTimeout(updateNavContrast, 300)

  // stats reveal
  if (statsSection.value && 'IntersectionObserver' in window) {
    statsObs = new IntersectionObserver((entries) => {
      entries.forEach((en) => { if (en.isIntersecting) { animateStats(); statsObs.disconnect() } })
    }, { threshold: 0.3 })
    statsObs.observe(statsSection.value)
  } else {
    animateStats()
  }
})

onUnmounted(() => {
  if (onMove) window.removeEventListener('mousemove', onMove)
  if (statsObs) statsObs.disconnect()
  if (onScroll) {
    window.removeEventListener('scroll', onScroll)
    window.removeEventListener('resize', onScroll)
  }
})
</script>

<style scoped>
.parallax { transition: transform 0.15s ease-out; }

/* ============================================================
   LANDING — palet "control room" yang lebih hidup & nyaman di mata
   Mengganti charcoal datar dengan navy-teal berlapis + aurora bergerak
   ============================================================ */

/* Hero: gradien navy dalam + cahaya warna lembut (steel, hijau, amber) */
.hero-bg {
  background:
    radial-gradient(1200px 620px at 12% -15%, rgba(62,146,204,0.30), transparent 60%),
    radial-gradient(1000px 560px at 92% 8%, rgba(242,166,12,0.16), transparent 58%),
    radial-gradient(900px 700px at 70% 110%, rgba(31,169,113,0.18), transparent 60%),
    linear-gradient(150deg, #16222f 0%, #122436 42%, #0d1923 100%);
}

/* Section gelap lain (Solusi, Footer) — sedikit berbeda agar tidak monoton */
.section-dark {
  background:
    radial-gradient(900px 500px at 85% -10%, rgba(62,146,204,0.22), transparent 60%),
    radial-gradient(800px 520px at 10% 120%, rgba(194,112,61,0.16), transparent 58%),
    linear-gradient(135deg, #15212e 0%, #0f1c28 100%);
}

/* Aurora bergerak — blob warna ber-blur yang mengambang pelan */
.aurora-wrap { overflow: hidden; }
.aurora {
  position: absolute;
  border-radius: 50%;
  filter: blur(70px);
  opacity: 0.55;
  pointer-events: none;
  will-change: transform;
}
.aurora-a {
  top: -120px; left: -80px;
  width: 460px; height: 460px;
  background: radial-gradient(circle at 30% 30%, rgba(62,146,204,0.55), transparent 70%);
  animation: auroraFloatA 18s ease-in-out infinite;
}
.aurora-b {
  bottom: -140px; right: -60px;
  width: 420px; height: 420px;
  background: radial-gradient(circle at 60% 40%, rgba(242,166,12,0.40), transparent 70%);
  animation: auroraFloatB 22s ease-in-out infinite;
}
.aurora-c {
  top: 30%; left: 55%;
  width: 360px; height: 360px;
  background: radial-gradient(circle at 50% 50%, rgba(31,169,113,0.38), transparent 70%);
  animation: auroraFloatC 26s ease-in-out infinite;
}
.aurora-soft {
  top: -100px; right: 8%;
  width: 480px; height: 480px;
  opacity: 0.4;
  background: radial-gradient(circle at 50% 50%, rgba(62,146,204,0.45), transparent 70%);
  animation: auroraFloatA 24s ease-in-out infinite;
}

@keyframes auroraFloatA {
  0%, 100% { transform: translate(0, 0) scale(1); }
  50%      { transform: translate(60px, 40px) scale(1.12); }
}
@keyframes auroraFloatB {
  0%, 100% { transform: translate(0, 0) scale(1); }
  50%      { transform: translate(-50px, -30px) scale(1.18); }
}
@keyframes auroraFloatC {
  0%, 100% { transform: translate(0, 0) scale(1); }
  50%      { transform: translate(-40px, 50px) scale(0.9); }
}

/* Heading utama: teks gradien amber→jingga dengan glow halus
   (lebih lembut dari amber solid pekat yang menyilaukan) */
.text-gradient-amber {
  background: linear-gradient(110deg, #FBBF3F 0%, #F2A60C 45%, #FF8A3D 100%);
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
  color: transparent;
  filter: drop-shadow(0 3px 18px rgba(242,166,12,0.30));
}

/* ============================================================
   NAVBAR ADAPTIF — warna otomatis menyesuaikan latar di belakangnya
   ============================================================ */
.nav-bar { color: var(--text); }

/* Di atas section terang → kaca terang, teks gelap */
.nav-bar--light {
  background: color-mix(in srgb, var(--surface) 82%, transparent);
  border-color: var(--border);
  color: var(--text);
}

/* Di atas section gelap → kaca gelap, teks terang (anti-bentrok) */
.nav-bar--dark {
  background: rgba(13, 23, 33, 0.55);
  border-color: rgba(255, 255, 255, 0.12);
  color: #eef3f8;
}

/* Tombol ghost & ikon ikut menyesuaikan saat navbar gelap */
.nav-bar--dark :deep(.btn-ghost) {
  background: rgba(255, 255, 255, 0.08);
  color: #eef3f8;
  border-color: rgba(255, 255, 255, 0.18);
}
.nav-bar--dark :deep(.btn-ghost:hover) {
  background: rgba(255, 255, 255, 0.16);
  border-color: rgba(255, 255, 255, 0.35);
}

/* Hormati preferensi reduksi gerak */
@media (prefers-reduced-motion: reduce) {
  .aurora { animation: none !important; }
}
</style>
