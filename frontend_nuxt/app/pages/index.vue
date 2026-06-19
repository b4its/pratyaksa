<template>
  <div class="min-h-screen text-black dark:text-white bg-[#fdfdfd] dark:bg-zinc-950 selection:bg-miningYellow selection:text-black font-sans transition-colors duration-500 overflow-x-hidden">

    <!-- NAV -->
    <nav class="sticky top-0 z-50 bg-white/90 dark:bg-zinc-950/90 backdrop-blur border-b-4 border-black dark:border-zinc-700 px-6 py-4 flex justify-between items-center w-full transition-colors duration-500">
      <div class="font-black text-2xl tracking-tighter uppercase flex items-center gap-2 hover:scale-105 transition-transform cursor-pointer">
        <div class="w-6 h-6 bg-black dark:bg-white anim-spin-slow"></div>
        <p>Prat<span class="text-neoYellow">yaksa</span></p>
      </div>

      <div class="xl:hidden flex gap-4">
        <button @click="toggleTheme" aria-label="Toggle Dark Mode" class="border-4 border-black dark:border-white p-2 bg-neoBlue dark:bg-zinc-800 text-white active:translate-y-1 active:shadow-none shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] dark:shadow-[2px_2px_0px_0px_rgba(255,255,255,1)] transition-all">
          <svg v-if="!isDark" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"></path></svg>
          <svg v-else class="w-6 h-6 text-miningYellow" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path></svg>
        </button>
        <button @click="toggleMenu" aria-label="Toggle Navigation Menu" class="border-4 border-black dark:border-white p-2 bg-miningYellow text-black active:translate-y-1 active:shadow-none shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] dark:shadow-[2px_2px_0px_0px_rgba(255,255,255,1)] transition-all">
          <svg v-if="!isMenuOpen" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M4 6h16M4 12h16M4 18h16"></path></svg>
          <svg v-else class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>

      <div class="hidden xl:flex gap-8 font-bold items-center border-4 border-black dark:border-zinc-700 px-6 py-2 bg-[#f0f0f0] dark:bg-zinc-800 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] dark:shadow-[4px_4px_0px_0px_rgba(100,100,100,1)] transition-all duration-500">
        <a href="#tentang" class="hover:text-neoRed hover:underline decoration-4 underline-offset-4 transition-colors">Tentang Kami</a>
        <a href="#solusi" class="hover:text-neoBlue hover:underline decoration-4 underline-offset-4 transition-colors">Solusi Pratyaksa</a>
        <a href="#keunggulan" class="hover:text-miningYellow hover:underline decoration-4 underline-offset-4 transition-colors">Keunggulan</a>
      </div>

      <div class="hidden xl:flex items-center gap-4">
        <button @click="toggleTheme" aria-label="Toggle Dark Mode" class="border-4 border-black dark:border-zinc-700 p-2 bg-white dark:bg-zinc-800 text-black dark:text-white shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] dark:shadow-[4px_4px_0px_0px_rgba(100,100,100,1)] lift">
          <svg v-if="!isDark" class="w-6 h-6 hover:rotate-12 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"></path></svg>
          <svg v-else class="w-6 h-6 text-miningYellow hover:rotate-12 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path></svg>
        </button>
        <NuxtLink to="panel/dashboard" class="inline-block font-black border-4 border-black dark:border-zinc-700 px-8 py-2 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] dark:shadow-[4px_4px_0px_0px_rgba(100,100,100,1)] lift uppercase text-center">Masuk</NuxtLink>
        <NuxtLink to="account/register" class="inline-block bg-neoBlue text-white font-black border-4 border-black dark:border-zinc-700 px-8 py-2 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] dark:shadow-[4px_4px_0px_0px_rgba(100,100,100,1)] lift uppercase text-center">Daftar Gratis</NuxtLink>
      </div>
    </nav>

    <!-- MOBILE MENU -->
    <div v-show="isMenuOpen" class="xl:hidden fixed inset-0 z-40 bg-white dark:bg-zinc-950 pt-24 px-6 flex flex-col gap-6 font-bold text-xl transition-colors duration-500">
      <a href="#tentang" @click="toggleMenu" class="hover:text-neoRed hover:underline decoration-4 underline-offset-4 transition-colors block border-b-2 border-black dark:border-zinc-700 pb-2">Tentang Kami</a>
      <a href="#solusi" @click="toggleMenu" class="hover:text-neoBlue hover:underline decoration-4 underline-offset-4 transition-colors block border-b-2 border-black dark:border-zinc-700 pb-2">Solusi Pratyaksa</a>
      <a href="#keunggulan" @click="toggleMenu" class="hover:text-miningYellow hover:underline decoration-4 underline-offset-4 transition-colors block border-b-2 border-black dark:border-zinc-700 pb-2">Keunggulan</a>
      <div class="mt-8 flex flex-col gap-4">
        <NuxtLink to="account/login" class="w-full font-black border-4 border-black dark:border-zinc-700 px-8 py-4 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all duration-300 uppercase bg-[#f0f0f0] dark:bg-zinc-800 text-center">Masuk</NuxtLink>
        <NuxtLink to="account/register" class="w-full bg-neoBlue text-white font-black border-4 border-black dark:border-zinc-700 px-8 py-4 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all duration-300 uppercase text-center">Daftar Gratis</NuxtLink>
      </div>
    </div>

    <!-- HERO -->
    <header class="relative border-b-4 border-black dark:border-zinc-700 overflow-hidden bg-miningYellow dark:bg-zinc-900 grid-pattern transition-colors duration-500">
      <!-- floating parallax shapes -->
      <div ref="shape1" class="parallax absolute top-16 left-8 w-28 h-28 bg-neoBlue border-4 border-black dark:border-white rounded-full hidden md:block anim-float"></div>
      <div ref="shape2" class="parallax absolute bottom-24 right-16 w-24 h-24 bg-white dark:bg-zinc-800 border-4 border-black dark:border-white rotate-12 hidden md:block anim-float" style="animation-delay:1s"></div>
      <div ref="shape3" class="parallax absolute top-1/3 right-1/3 w-16 h-16 bg-neoRed border-4 border-black dark:border-white hidden lg:block anim-float" style="animation-delay:.5s"></div>

      <div class="relative z-10 max-w-7xl mx-auto px-6 py-16 md:py-24 grid lg:grid-cols-2 gap-12 items-center">
        <!-- left: copy -->
        <div class="anim-left">
          <div class="inline-flex items-center gap-2 bg-neoRed text-white px-4 py-2 font-black border-4 border-black dark:border-white uppercase text-xs -rotate-2 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] mb-6">
            <span class="w-2 h-2 rounded-full bg-white anim-live"></span> Mining Intelligence Platform
          </div>
          <h1 class="text-5xl md:text-7xl font-black uppercase leading-[1.05] tracking-tighter mb-6">
            Kendalikan<br>Tambang Anda<br>
            <span class="bg-black dark:bg-white text-white dark:text-black px-3 inline-block mt-2 anim-gradient" style="background-image:linear-gradient(90deg,#000,#333,#000)">Dengan Data</span>
          </h1>
          <p class="text-lg md:text-xl font-bold mb-8 max-w-xl text-gray-800 dark:text-gray-300">
            Pratyaksa mengintegrasikan IoT dan Machine Learning untuk memprediksi kerusakan alat berat dan mengotomatisasi keputusan operasional secara real-time.
          </p>
          <div class="flex flex-col sm:flex-row gap-4">
            <NuxtLink to="account/register" class="bg-neoBlue text-white font-black text-center text-lg border-4 border-black dark:border-zinc-700 px-8 py-4 shadow-[6px_6px_0px_0px_rgba(0,0,0,1)] hover:translate-x-[3px] hover:translate-y-[3px] hover:shadow-[3px_3px_0px_0px_rgba(0,0,0,1)] active:translate-x-[6px] active:translate-y-[6px] active:shadow-none transition-all uppercase">Mulai Sekarang →</NuxtLink>
            <a href="#solusi" class="bg-white dark:bg-zinc-800 text-black dark:text-white font-black text-lg border-4 border-black dark:border-zinc-700 px-8 py-4 shadow-[6px_6px_0px_0px_rgba(0,0,0,1)] hover:translate-x-[3px] hover:translate-y-[3px] hover:shadow-[3px_3px_0px_0px_rgba(0,0,0,1)] active:translate-x-[6px] active:translate-y-[6px] active:shadow-none transition-all uppercase text-center">Lihat Solusi</a>
          </div>
        </div>

        <!-- right: interactive 3D model -->
        <div class="anim-pop">
          <div class="viewer-3d border-4 border-black dark:border-zinc-700 bg-gradient-to-br from-white to-gray-200 dark:from-zinc-800 dark:to-zinc-950 shadow-[12px_12px_0px_0px_rgba(0,0,0,1)] dark:shadow-[12px_12px_0px_0px_rgba(50,50,50,1)] relative overflow-hidden h-[340px] md:h-[460px] cursor-grab active:cursor-grabbing">
            <model-viewer
              v-if="isMounted"
              src="https://modelviewer.dev/shared-assets/models/RobotExpressive.glb"
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
            <div class="absolute top-3 left-3 bg-neoBlue text-white text-[10px] font-black px-2 py-1 border-2 border-black pointer-events-none flex items-center gap-1">
              <span class="w-2 h-2 rounded-full bg-white anim-live"></span> LIVE 3D VISUAL
            </div>
            <div class="absolute bottom-3 right-3 bg-black text-white text-[10px] font-black px-2 py-1 pointer-events-none">DRAG UNTUK PUTAR 360°</div>
          </div>
        </div>
      </div>
    </header>

    <!-- MARQUEE -->
    <div class="bg-black dark:bg-white text-white dark:text-black border-b-4 border-black dark:border-zinc-700 overflow-hidden flex whitespace-nowrap py-3">
      <div class="animate-marquee font-black text-xl uppercase tracking-widest flex gap-8 items-center">
        <span>⚡ IoT Sensor Integration</span><span>•</span>
        <span>🚧 Predictive Maintenance</span><span>•</span>
        <span>📊 Machine Learning Analysis</span><span>•</span>
        <span>⚙️ Automated Alerting</span><span>•</span>
        <span>⚡ IoT Sensor Integration</span><span>•</span>
        <span>🚧 Predictive Maintenance</span><span>•</span>
        <span>📊 Machine Learning Analysis</span><span>•</span>
        <span>⚙️ Automated Alerting</span>
      </div>
    </div>

    <!-- STATS COUNTER -->
    <section ref="statsSection" class="py-16 px-6 md:px-20 bg-white dark:bg-zinc-950 border-b-4 border-black dark:border-zinc-700">
      <div class="max-w-6xl mx-auto grid grid-cols-2 lg:grid-cols-4 gap-6">
        <div v-for="(s, i) in stats" :key="s.label" v-tilt class="tilt-card relative overflow-hidden border-4 border-black dark:border-zinc-700 p-6 text-center shadow-[6px_6px_0px_0px_rgba(0,0,0,1)] dark:shadow-[6px_6px_0px_0px_rgba(50,50,50,1)]" :class="s.bg">
          <span class="tilt-shine"></span>
          <p class="text-4xl md:text-5xl font-black" :class="s.text">{{ statDisplay[i] }}{{ s.suffix }}</p>
          <p class="text-xs font-black uppercase mt-2" :class="s.text">{{ s.label }}</p>
        </div>
      </div>
    </section>

    <!-- TENTANG -->
    <section id="tentang" class="py-24 px-6 md:px-20 bg-white dark:bg-zinc-950 border-b-4 border-black dark:border-zinc-700 relative">
      <div class="absolute inset-0 pattern-dots opacity-50 dark:opacity-10"></div>
      <div v-reveal class="max-w-4xl mx-auto text-center relative z-10 bg-white dark:bg-zinc-900 border-4 border-black dark:border-zinc-700 p-10 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] dark:shadow-[8px_8px_0px_0px_rgba(50,50,50,1)]">
        <h2 class="text-4xl md:text-6xl font-black uppercase mb-8 inline-block">Apa itu <span class="text-neoRed">Pratyaksa?</span></h2>
        <div class="w-full h-2 bg-black dark:bg-white mb-8"></div>
        <p class="text-xl md:text-3xl font-bold leading-relaxed text-left text-black dark:text-gray-200">
          Pratyaksa adalah jembatan antara operasi alat berat dan kecerdasan buatan. Kami mengubah data mentah dari armada tambang menjadi <span class="bg-miningYellow text-black px-2 border-2 border-black inline-block -rotate-1 shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] mt-2">wawasan preskriptif</span>, memastikan alat berat Anda beroperasi pada efisiensi maksimal.
        </p>
      </div>
    </section>

    <!-- SOLUSI -->
    <section id="solusi" class="py-24 px-6 md:px-20 bg-neoBlue dark:bg-zinc-900 border-b-4 border-black dark:border-zinc-700">
      <div class="max-w-7xl mx-auto">
        <h2 v-reveal class="text-5xl md:text-7xl font-black uppercase mb-16 text-white tracking-tighter drop-shadow-[4px_4px_0px_rgba(0,0,0,1)]">Solusi End-to-End</h2>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-10">
          <div v-reveal="{ anim: 'up', delay: 0 }" v-tilt class="tilt-card relative overflow-hidden bg-white dark:bg-zinc-800 border-4 border-black dark:border-zinc-700 p-8 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] dark:shadow-[8px_8px_0px_0px_rgba(50,50,50,1)] flex flex-col h-full">
            <span class="tilt-shine"></span>
            <div class="w-20 h-20 bg-neoRed border-4 border-black flex items-center justify-center mb-8 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] anim-float">
              <svg class="w-10 h-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
            </div>
            <h3 class="text-3xl font-black uppercase mb-4 leading-none text-black dark:text-white">Predictive Maintenance</h3>
            <p class="font-bold text-lg text-gray-700 dark:text-gray-300 flex-grow">Mendeteksi anomali suhu, tekanan, dan vibrasi sebelum kerusakan fatal terjadi via Machine Learning.</p>
          </div>

          <div v-reveal="{ anim: 'up', delay: 120 }" v-tilt class="tilt-card relative overflow-hidden bg-miningYellow dark:bg-yellow-600 border-4 border-black dark:border-zinc-700 p-8 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] dark:shadow-[8px_8px_0px_0px_rgba(50,50,50,1)] flex flex-col h-full">
            <span class="tilt-shine"></span>
            <div class="w-20 h-20 bg-white dark:bg-zinc-900 border-4 border-black flex items-center justify-center mb-8 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] anim-float" style="animation-delay:.5s">
              <svg class="w-10 h-10 text-black dark:text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path></svg>
            </div>
            <h3 class="text-3xl font-black uppercase mb-4 leading-none text-black">Real-time Telematics</h3>
            <p class="font-bold text-lg text-gray-900 dark:text-gray-100 flex-grow">Pemantauan lokasi presisi, konsumsi bahan bakar, dan perilaku operator dalam satu dashboard.</p>
          </div>

          <div v-reveal="{ anim: 'up', delay: 240 }" v-tilt class="tilt-card relative overflow-hidden bg-black dark:bg-zinc-950 border-4 border-black dark:border-zinc-700 p-8 shadow-[8px_8px_0px_0px_rgba(255,255,255,1)] dark:shadow-[8px_8px_0px_0px_rgba(255,73,17,1)] flex flex-col h-full">
            <span class="tilt-shine"></span>
            <div class="w-20 h-20 bg-white dark:bg-zinc-800 border-4 border-black flex items-center justify-center mb-8 shadow-[4px_4px_0px_0px_rgba(255,73,17,1)] anim-float" style="animation-delay:1s">
              <svg class="w-10 h-10 text-black dark:text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z"></path></svg>
            </div>
            <h3 class="text-3xl font-black text-white uppercase mb-4 leading-none">Automated Alerting</h3>
            <p class="font-bold text-lg text-gray-300 flex-grow">Sistem preskriptif otomatis yang merilis tiket perbaikan via Telegram & WhatsApp ke tim mekanik.</p>
          </div>
        </div>
      </div>
    </section>

    <!-- KEUNGGULAN -->
    <section id="keunggulan" class="py-24 px-6 md:px-20 bg-white dark:bg-zinc-950 border-b-4 border-black dark:border-zinc-700">
      <div class="max-w-7xl mx-auto flex flex-col lg:flex-row gap-16 items-center">
        <!-- 3D showcase -->
        <div v-reveal="'left'" class="w-full lg:w-1/2">
          <div class="viewer-3d w-full aspect-square bg-gradient-to-br from-gray-100 to-gray-300 dark:from-zinc-800 dark:to-zinc-950 border-4 border-black dark:border-zinc-700 shadow-[16px_16px_0px_0px_rgba(0,0,0,1)] dark:shadow-[16px_16px_0px_0px_rgba(50,50,50,1)] relative overflow-hidden cursor-grab active:cursor-grabbing">
            <div class="absolute top-0 left-0 w-full h-10 border-b-4 border-black dark:border-zinc-700 bg-white dark:bg-zinc-900 flex items-center px-4 gap-2 z-10">
              <div class="w-4 h-4 rounded-full border-2 border-black bg-neoRed"></div>
              <div class="w-4 h-4 rounded-full border-2 border-black bg-miningYellow"></div>
              <div class="w-4 h-4 rounded-full border-2 border-black bg-green-500"></div>
              <span class="ml-auto text-[10px] font-black uppercase">pratyaksa://3d-viewer</span>
            </div>
            <model-viewer
              v-if="isMounted"
              src="https://modelviewer.dev/shared-assets/models/RobotExpressive.glb"
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
          <h2 v-reveal class="text-5xl md:text-7xl font-black uppercase mb-12 tracking-tighter">Kenapa <br><span class="text-neoRed">Pratyaksa?</span></h2>
          <ul class="space-y-8">
            <li v-reveal="{ anim: 'left', delay: 0 }" class="flex gap-6 group">
              <div class="w-16 h-16 shrink-0 bg-miningYellow border-4 border-black flex justify-center items-center font-black text-3xl shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] group-hover:bg-black group-hover:text-white transition-all duration-300 group-hover:rotate-6">✓</div>
              <div>
                <h4 class="font-black text-3xl mb-2 uppercase">Agnostik Terhadap Merek</h4>
                <p class="font-bold text-xl text-gray-700 dark:text-gray-300">Sensor IoT kami terintegrasi dengan berbagai merk (Komatsu, Caterpillar, Volvo) tanpa kendala.</p>
              </div>
            </li>
            <li v-reveal="{ anim: 'left', delay: 120 }" class="flex gap-6 group">
              <div class="w-16 h-16 shrink-0 bg-neoBlue border-4 border-black flex justify-center items-center font-black text-3xl text-white shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] group-hover:bg-black transition-all duration-300 group-hover:rotate-6">✓</div>
              <div>
                <h4 class="font-black text-3xl mb-2 uppercase">Infrastruktur Berbasis Lokal</h4>
                <p class="font-bold text-xl text-gray-700 dark:text-gray-300">Skalabilitas tinggi. Data aman dan dapat diakses dari site tambang mana pun dengan latensi minimal.</p>
              </div>
            </li>
            <li v-reveal="{ anim: 'left', delay: 240 }" class="flex gap-6 group">
              <div class="w-16 h-16 shrink-0 bg-neoRed border-4 border-black flex justify-center items-center font-black text-3xl text-white shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] group-hover:bg-black transition-all duration-300 group-hover:rotate-6">✓</div>
              <div>
                <h4 class="font-black text-3xl mb-2 uppercase">ROI Terukur Cepat</h4>
                <p class="font-bold text-xl text-gray-700 dark:text-gray-300">Klien mencatatkan penghematan biaya <b>maintenance</b> hingga 25% pada kuartal pertama implementasi.</p>
              </div>
            </li>
          </ul>
        </div>
      </div>
    </section>

    <!-- FOOTER CTA -->
    <footer class="bg-black dark:bg-zinc-950 text-white py-32 px-6 text-center border-t-8 border-miningYellow relative overflow-hidden">
      <div class="absolute -top-10 -left-10 w-40 h-40 bg-neoRed border-8 border-black rounded-full opacity-50 anim-float"></div>
      <div class="absolute -bottom-10 -right-10 w-60 h-60 bg-neoBlue border-8 border-black opacity-50 rotate-45 anim-float" style="animation-delay:1s"></div>
      <div v-reveal class="max-w-4xl mx-auto relative z-10">
        <h2 class="text-5xl md:text-8xl font-black uppercase mb-10 tracking-tighter leading-none drop-shadow-[4px_4px_0px_rgba(255,204,0,1)]">Tingkatkan Efektifitas Tambang Anda Sekarang.</h2>
        <NuxtLink to="account/register" class="inline-block bg-miningYellow text-black font-black text-2xl md:text-4xl border-4 border-white px-12 py-6 shadow-[8px_8px_0px_0px_rgba(255,255,255,1)] hover:translate-x-[4px] hover:translate-y-[4px] hover:shadow-[4px_4px_0px_0px_rgba(255,255,255,1)] active:translate-x-[8px] active:translate-y-[8px] active:shadow-none transition-all duration-300 uppercase mt-8">Ayo Bergabung Bersama Kami</NuxtLink>
      </div>
    </footer>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'

// Muat web component model-viewer
useHead({
  script: [
    { type: 'module', src: 'https://ajax.googleapis.com/ajax/libs/model-viewer/3.3.0/model-viewer.min.js' },
  ],
})

const isDark = ref(false)
const isMounted = ref(false)
const isMenuOpen = ref(false)

// Parallax shapes
const shape1 = ref(null)
const shape2 = ref(null)
const shape3 = ref(null)

// Stats counter
const stats = [
  { label: 'Physical Availability', target: 93, suffix: '%', bg: 'bg-emerald-400', text: 'text-black' },
  { label: 'Akurasi AI', target: 95, suffix: '%', bg: 'bg-white dark:bg-zinc-800', text: 'text-black dark:text-white' },
  { label: 'Reduksi Downtime', target: 45, suffix: '%', bg: 'bg-miningYellow', text: 'text-black' },
  { label: 'Hemat Maintenance', target: 30, suffix: '%', bg: 'bg-neoRed', text: 'text-white' },
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

let onMove = null
let statsObs = null

onMounted(() => {
  isMounted.value = true
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
    if (shape2.value) shape2.value.style.transform = `translate(${dx * -25}px, ${dy * -25}px) rotate(12deg)`
    if (shape3.value) shape3.value.style.transform = `translate(${dx * 40}px, ${dy * 40}px)`
  }
  window.addEventListener('mousemove', onMove)

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
})
</script>

<style>
.grid-pattern { background-image: radial-gradient(#000 2px, transparent 2px); background-size: 32px 32px; }
.dark .grid-pattern { background-image: radial-gradient(rgba(255,255,255,0.2) 2px, transparent 2px); }
.pattern-dots { background-image: radial-gradient(#e5e7eb 2px, transparent 2px); background-size: 16px 16px; }
.dark .pattern-dots { background-image: radial-gradient(rgba(255,255,255,0.1) 2px, transparent 2px); }

.parallax { transition: transform 0.15s ease-out; }

@keyframes marquee { 0% { transform: translateX(0%); } 100% { transform: translateX(-50%); } }
.animate-marquee { width: max-content; animation: marquee 20s linear infinite; }
.animate-marquee:hover { animation-play-state: paused; }
</style>
