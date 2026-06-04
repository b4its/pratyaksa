<template>
  <div class="min-h-screen text-black dark:text-white bg-[#fdfdfd] dark:bg-zinc-950 overflow-x-hidden selection:bg-miningYellow selection:text-black font-sans transition-colors duration-500">
    
<!-- Navbar -->
    <nav class="sticky top-0 z-50 bg-white dark:bg-zinc-950 border-b-4 border-black dark:border-zinc-700 px-6 py-4 flex justify-between items-center w-full transition-colors duration-500">
      <div class="font-black text-2xl tracking-tighter uppercase flex items-center gap-2 hover:scale-105 transition-transform cursor-pointer">
        <div class="w-6 h-6 bg-black dark:bg-white transition-colors duration-500"></div>
        <p>
Prat<span class="text-neoYellow">yaksa</span>
        </p>
      </div>
      
      <!-- Mobile Controls - MUNCUL HINGGA LAYAR xl (1280px) -->
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

      <!-- Desktop Links -  (>= 1280px) -->
      <div class="hidden xl:flex gap-8 font-bold items-center border-4 border-black dark:border-zinc-700 px-6 py-2 bg-[#f0f0f0] dark:bg-zinc-800 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] dark:shadow-[4px_4px_0px_0px_rgba(100,100,100,1)] transition-all duration-500">
        <a href="#tentang" class="hover:text-neoRed hover:underline decoration-4 underline-offset-4 transition-colors">Tentang Kami</a>
        <a href="#solusi" class="hover:text-neoBlue hover:underline decoration-4 underline-offset-4 transition-colors">Solusi Pratyaksa</a>
        <a href="#keunggulan" class="hover:text-miningYellow hover:underline decoration-4 underline-offset-4 transition-colors">Keunggulan</a>
      </div>
      
      <!-- Desktop Actions -  (>= 1280px) -->
      <div class="hidden xl:flex items-center gap-4">
        <button @click="toggleTheme" aria-label="Toggle Dark Mode" class="border-4 border-black dark:border-zinc-700 p-2 bg-white dark:bg-zinc-800 text-black dark:text-white shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] dark:shadow-[4px_4px_0px_0px_rgba(100,100,100,1)] hover:translate-x-[2px] hover:translate-y-[2px] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] dark:hover:shadow-[2px_2px_0px_0px_rgba(100,100,100,1)] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all duration-300">
          <svg v-if="!isDark" class="w-6 h-6 hover:rotate-12 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"></path></svg>
          <svg v-else class="w-6 h-6 text-miningYellow hover:rotate-12 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path></svg>
        </button>

        <button class="font-black border-4 border-black dark:border-zinc-700 px-8 py-2 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] dark:shadow-[4px_4px_0px_0px_rgba(100,100,100,1)] hover:translate-x-[2px] hover:translate-y-[2px] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] dark:hover:shadow-[2px_2px_0px_0px_rgba(100,100,100,1)] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all duration-300 uppercase">
          Masuk
        </button>
        <button class="bg-neoBlue text-white font-black border-4 border-black dark:border-zinc-700 px-8 py-2 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] dark:shadow-[4px_4px_0px_0px_rgba(100,100,100,1)] hover:translate-x-[2px] hover:translate-y-[2px] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] dark:hover:shadow-[2px_2px_0px_0px_rgba(100,100,100,1)] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all duration-300 uppercase">
          Daftar Gratis
        </button>
      </div>
    </nav>

    <!-- Mobile Slide-out Menu  -->
    <div v-show="isMenuOpen" class="xl:hidden fixed inset-0 z-40 bg-white dark:bg-zinc-950 pt-24 px-6 flex flex-col gap-6 font-bold text-xl transition-colors duration-500">
      <a href="#tentang" @click="toggleMenu" class="hover:text-neoRed hover:underline decoration-4 underline-offset-4 transition-colors block border-b-2 border-black dark:border-zinc-700 pb-2">Tentang Kami</a>
      <a href="#solusi" @click="toggleMenu" class="hover:text-neoBlue hover:underline decoration-4 underline-offset-4 transition-colors block border-b-2 border-black dark:border-zinc-700 pb-2">Solusi Pratyaksa</a>
      <a href="#keunggulan" @click="toggleMenu" class="hover:text-miningYellow hover:underline decoration-4 underline-offset-4 transition-colors block border-b-2 border-black dark:border-zinc-700 pb-2">Keunggulan</a>
      <div class="mt-8 flex flex-col gap-4">
        <button class="w-full font-black border-4 border-black dark:border-zinc-700 px-8 py-4 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] dark:shadow-[4px_4px_0px_0px_rgba(100,100,100,1)] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all duration-300 uppercase bg-[#f0f0f0] dark:bg-zinc-800 text-center">
          Masuk
        </button>
        <button class="w-full bg-neoBlue text-white font-black border-4 border-black dark:border-zinc-700 px-8 py-4 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] dark:shadow-[4px_4px_0px_0px_rgba(100,100,100,1)] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all duration-300 uppercase text-center">
          Daftar Gratis
        </button>
      </div>
    </div>

    <header class="bg-miningYellow relative border-b-4 border-black dark:border-zinc-700 p-10 md:p-32 flex flex-col items-center justify-center text-center overflow-hidden grid-pattern transition-colors duration-500">
      <div class="animate-float absolute top-20 left-10 w-24 h-24 bg-neoBlue border-4 border-black dark:border-white rounded-full hidden md:block hover:bg-neoRed transition-colors duration-500"></div>
      <div class="animate-float-delayed absolute bottom-20 right-20 w-20 h-20 bg-white dark:bg-zinc-800 border-4 border-black dark:border-white rotate-12 hidden md:block hover:rotate-45 transition-transform duration-300"></div>
      
      <div class="max-w-4xl z-10 bg-white dark:bg-zinc-900 border-4 border-black dark:border-zinc-700 p-8 md:p-14 shadow-[12px_12px_0px_0px_rgba(0,0,0,1)] dark:shadow-[12px_12px_0px_0px_rgba(50,50,50,1)] hover:shadow-[16px_16px_0px_0px_rgba(0,0,0,1)] dark:hover:shadow-[16px_16px_0px_0px_rgba(50,50,50,1)] transition-all duration-300 relative group">
        <div class="absolute -top-4 -right-4 bg-neoRed text-white px-4 py-2 font-black border-4 border-black dark:border-white uppercase text-sm transform rotate-6 group-hover:rotate-12 transition-transform">
          Mining Intelligence Platform
        </div>
        
        <h1 class="text-5xl md:text-7xl font-black uppercase leading-[1.1] tracking-tighter mb-6 mt-4 text-black dark:text-white transition-colors duration-500">
          Kendalikan Tambang Anda <br> <span class="bg-black dark:bg-white text-white dark:text-black px-2 leading-relaxed transition-colors duration-500">Dengan Data.</span>
        </h1>
        <p class="text-lg md:text-xl font-bold mb-10 max-w-2xl mx-auto text-gray-800 dark:text-gray-300 transition-colors duration-500">
          Pratyaksa mengintegrasikan Internet Of Things dan Machine Learning untuk memberikan visibilitas penuh, memprediksi kerusakan alat berat, dan mengotomatisasi keputusan operasional.
        </p>
        
        <div class="flex flex-col md:flex-row gap-6 justify-center">
          <a href="#solusi" class="bg-neoBlue text-white font-black text-center text-xl border-4 border-black dark:border-zinc-700 px-10 py-5 shadow-[6px_6px_0px_0px_rgba(0,0,0,1)] dark:shadow-[6px_6px_0px_0px_rgba(50,50,50,1)] hover:translate-x-[2px] hover:translate-y-[2px] hover:shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] dark:hover:shadow-[4px_4px_0px_0px_rgba(50,50,50,1)] active:translate-x-[6px] active:translate-y-[6px] active:shadow-none transition-all uppercase">
            Lihat Solusi Kami
          </a>
          <button class="bg-white dark:bg-zinc-800 text-black dark:text-white font-black text-xl border-4 border-black dark:border-zinc-700 px-10 py-5 shadow-[6px_6px_0px_0px_rgba(0,0,0,1)] dark:shadow-[6px_6px_0px_0px_rgba(50,50,50,1)] hover:translate-x-[2px] hover:translate-y-[2px] hover:shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] dark:hover:shadow-[4px_4px_0px_0px_rgba(50,50,50,1)] active:translate-x-[6px] active:translate-y-[6px] active:shadow-none transition-all uppercase">
            Jadwalkan Demo
          </button>
        </div>
      </div>
    </header>

    <div class="bg-black dark:bg-white text-white dark:text-black border-b-4 border-black dark:border-zinc-700 overflow-hidden flex whitespace-nowrap py-3 transition-colors duration-500">
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

    <section id="tentang" class="py-24 px-6 md:px-20 bg-white dark:bg-zinc-950 border-b-4 border-black dark:border-zinc-700 relative transition-colors duration-500">
      <div class="absolute inset-0 pattern-dots opacity-50 dark:opacity-10 transition-opacity duration-500"></div>
      <div class="max-w-4xl mx-auto text-center relative z-10 bg-white dark:bg-zinc-900 border-4 border-black dark:border-zinc-700 p-10 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] dark:shadow-[8px_8px_0px_0px_rgba(50,50,50,1)] transition-all duration-500">
        <h2 class="text-4xl md:text-6xl font-black uppercase mb-8 inline-block transition-colors duration-500">Apa itu <span class="text-neoRed">Pratyaksa?</span></h2>
        <div class="w-full h-2 bg-black dark:bg-white mb-8 transition-colors duration-500"></div>
        <p class="text-xl md:text-3xl font-bold leading-relaxed text-left text-black dark:text-gray-200 transition-colors duration-500">
          Pratyaksa adalah jembatan antara operasi alat berat dan kecerdasan buatan. Kami merancang ekosistem perangkat lunak yang mengubah data mentah dari armada tambang menjadi <span class="bg-miningYellow text-black px-2 border-2 border-black dark:border-zinc-700 inline-block transform -rotate-1 shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] dark:shadow-[2px_2px_0px_0px_rgba(50,50,50,1)] mt-2 transition-all duration-300">wawasan preskriptif</span>, memastikan alat berat Anda beroperasi pada efisiensi maksimal.
        </p>
      </div>
    </section>

    <section id="solusi" class="py-24 px-6 md:px-20 bg-neoBlue dark:bg-zinc-900 border-b-4 border-black dark:border-zinc-700 transition-colors duration-500">
      <div class="max-w-7xl mx-auto">
        <h2 class="text-5xl md:text-7xl font-black uppercase mb-16 text-white tracking-tighter drop-shadow-[4px_4px_0px_rgba(0,0,0,1)] dark:drop-shadow-[4px_4px_0px_rgba(50,50,50,1)] transition-all duration-500">Solusi End-to-End</h2>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-10">
          
          <div class="group bg-white dark:bg-zinc-800 border-4 border-black dark:border-zinc-700 p-8 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] dark:shadow-[8px_8px_0px_0px_rgba(50,50,50,1)] hover:-translate-y-4 hover:shadow-[16px_16px_0px_0px_rgba(0,0,0,1)] dark:hover:shadow-[16px_16px_0px_0px_rgba(50,50,50,1)] transition-all duration-300 flex flex-col h-full cursor-crosshair">
            <div class="w-20 h-20 bg-neoRed border-4 border-black dark:border-zinc-700 flex items-center justify-center mb-8 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] dark:shadow-[4px_4px_0px_0px_rgba(50,50,50,1)] group-hover:rotate-12 transition-all duration-300">
              <svg class="w-10 h-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
            </div>
            <h3 class="text-3xl font-black uppercase mb-4 leading-none text-black dark:text-white transition-colors duration-300">Predictive Maintenance</h3>
            <p class="font-bold text-lg text-gray-700 dark:text-gray-300 flex-grow transition-colors duration-300">Mendeteksi anomali suhu, tekanan, dan vibrasi (Engine, Transmission) sebelum kerusakan fatal terjadi via Machine Learning.</p>
          </div>

          <div class="group bg-miningYellow dark:bg-yellow-600 border-4 border-black dark:border-zinc-700 p-8 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] dark:shadow-[8px_8px_0px_0px_rgba(50,50,50,1)] hover:-translate-y-4 hover:shadow-[16px_16px_0px_0px_rgba(0,0,0,1)] dark:hover:shadow-[16px_16px_0px_0px_rgba(50,50,50,1)] transition-all duration-300 flex flex-col h-full cursor-crosshair">
            <div class="w-20 h-20 bg-white dark:bg-zinc-900 border-4 border-black dark:border-zinc-700 flex items-center justify-center mb-8 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] dark:shadow-[4px_4px_0px_0px_rgba(50,50,50,1)] group-hover:-rotate-12 transition-all duration-300">
              <svg class="w-10 h-10 text-black dark:text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path></svg>
            </div>
            <h3 class="text-3xl font-black uppercase mb-4 leading-none text-black transition-colors duration-300">Real-time Telematics</h3>
            <p class="font-bold text-lg text-gray-900 dark:text-gray-100 flex-grow transition-colors duration-300">Pemantauan lokasi presisi, konsumsi bahan bakar, dan perilaku operator dalam satu dashboard komprehensif.</p>
          </div>

          <div class="group bg-black dark:bg-zinc-950 border-4 border-black dark:border-zinc-700 p-8 shadow-[8px_8px_0px_0px_rgba(255,255,255,1)] dark:shadow-[8px_8px_0px_0px_rgba(255,73,17,1)] hover:-translate-y-4 hover:shadow-[16px_16px_0px_0px_rgba(255,73,17,1)] transition-all duration-300 flex flex-col h-full cursor-crosshair">
            <div class="w-20 h-20 bg-white dark:bg-zinc-800 border-4 border-black dark:border-zinc-700 flex items-center justify-center mb-8 shadow-[4px_4px_0px_0px_rgba(255,73,17,1)] group-hover:scale-110 transition-all duration-300">
              <svg class="w-10 h-10 text-black dark:text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z"></path></svg>
            </div>
            <h3 class="text-3xl font-black text-white uppercase mb-4 leading-none transition-colors duration-300">Automated Alerting</h3>
            <p class="font-bold text-lg text-gray-300 flex-grow transition-colors duration-300">Sistem preskriptif otomatis yang merilis tiket perbaikan via Telegram & WhatsApp langsung ke tim mekanik.</p>
          </div>

        </div>
      </div>
    </section>

    <section id="keunggulan" class="py-24 px-6 md:px-20 bg-white dark:bg-zinc-950 border-b-4 border-black dark:border-zinc-700 transition-colors duration-500">
      <div class="max-w-7xl mx-auto flex flex-col lg:flex-row gap-16 items-center">
        <div class="w-full lg:w-1/2">
          <div class="w-full aspect-square bg-[#f0f0f0] dark:bg-zinc-800 border-4 border-black dark:border-zinc-700 shadow-[16px_16px_0px_0px_rgba(0,0,0,1)] dark:shadow-[16px_16px_0px_0px_rgba(50,50,50,1)] flex flex-col justify-center items-center p-6 md:p-12 text-center relative group overflow-hidden transition-all duration-500">
            <div class="absolute top-0 left-0 w-full h-10 border-b-4 border-black dark:border-zinc-700 bg-white dark:bg-zinc-900 flex items-center px-4 gap-2 transition-colors duration-500">
              <div class="w-4 h-4 rounded-full border-2 border-black dark:border-zinc-700 bg-neoRed"></div>
              <div class="w-4 h-4 rounded-full border-2 border-black dark:border-zinc-700 bg-miningYellow"></div>
              <div class="w-4 h-4 rounded-full border-2 border-black dark:border-zinc-700 bg-green-500"></div>
            </div>
            
            <div class="w-full h-3/4 mt-8 bg-white dark:bg-zinc-950 border-4 border-black dark:border-zinc-700 p-4 flex items-end gap-4 transition-colors duration-500">
               <div class="w-1/4 bg-neoBlue border-4 border-black dark:border-zinc-700 animate-bar-1 origin-bottom"></div>
               <div class="w-1/4 bg-miningYellow border-4 border-black dark:border-zinc-700 animate-bar-2 origin-bottom"></div>
               <div class="w-1/4 bg-neoRed border-4 border-black dark:border-zinc-700 animate-bar-3 origin-bottom"></div>
               <div class="w-1/4 bg-black dark:bg-white border-4 border-black dark:border-zinc-700 animate-bar-4 origin-bottom transition-colors duration-500"></div>
            </div>
            <p class="font-black text-2xl md:text-4xl uppercase bg-black dark:bg-white text-white dark:text-black px-6 py-3 mt-8 inline-block transform -rotate-3 group-hover:rotate-0 transition-all duration-300 shadow-[6px_6px_0px_0px_rgba(255,73,17,1)]">
              Akurasi AI: 94.8%
            </p>
          </div>
        </div>

        <div class="w-full lg:w-1/2">
          <h2 class="text-5xl md:text-7xl font-black uppercase mb-12 tracking-tighter transition-colors duration-500">Kenapa <br><span class="text-neoRed">Pratyaksa?</span></h2>
          <ul class="space-y-8">
            <li class="flex gap-6 group">
              <div class="w-16 h-16 shrink-0 bg-miningYellow border-4 border-black dark:border-zinc-700 flex justify-center items-center font-black text-3xl shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] dark:shadow-[4px_4px_0px_0px_rgba(50,50,50,1)] group-hover:bg-black group-hover:text-white dark:group-hover:bg-white dark:group-hover:text-black transition-all duration-300">✓</div>
              <div>
                <h4 class="font-black text-3xl mb-2 uppercase transition-colors duration-500">Agnostik Terhadap Merek</h4>
                <p class="font-bold text-xl text-gray-700 dark:text-gray-300 transition-colors duration-500">Sensor IoT kami terintegrasi dengan berbagai merk (Komatsu, Caterpillar, Volvo) tanpa kendala.</p>
              </div>
            </li>
            <li class="flex gap-6 group">
              <div class="w-16 h-16 shrink-0 bg-neoBlue border-4 border-black dark:border-zinc-700 flex justify-center items-center font-black text-3xl text-white shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] dark:shadow-[4px_4px_0px_0px_rgba(50,50,50,1)] group-hover:bg-black dark:group-hover:bg-white dark:group-hover:text-black transition-all duration-300">✓</div>
              <div>
                <h4 class="font-black text-3xl mb-2 uppercase transition-colors duration-500">Infrastruktur Berbasis Lokal</h4>
                <p class="font-bold text-xl text-gray-700 dark:text-gray-300 transition-colors duration-500">Skalabilitas tinggi. Data Anda aman dan dapat diakses dari site tambang mana pun dengan latensi minimal.</p>
              </div>
            </li>
            <li class="flex gap-6 group">
              <div class="w-16 h-16 shrink-0 bg-neoRed border-4 border-black dark:border-zinc-700 flex justify-center items-center font-black text-3xl text-white shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] dark:shadow-[4px_4px_0px_0px_rgba(50,50,50,1)] group-hover:bg-black dark:group-hover:bg-white dark:group-hover:text-black transition-all duration-300">✓</div>
              <div>
                <h4 class="font-black text-3xl mb-2 uppercase transition-colors duration-500">ROI Terukur Cepat</h4>
                <p class="font-bold text-xl text-gray-700 dark:text-gray-300 transition-colors duration-500">Klien kami mencatatkan penghematan biaya <b>maintenance</b> hingga 25% pada kuartal pertama implementasi.</p>
              </div>
            </li>
          </ul>
        </div>
      </div>
    </section>

    <footer class="bg-black dark:bg-zinc-950 text-white py-32 px-6 text-center border-t-8 border-miningYellow relative overflow-hidden transition-colors duration-500">
      <div class="absolute -top-10 -left-10 w-40 h-40 bg-neoRed border-8 border-black dark:border-zinc-700 rounded-full opacity-50 transition-colors duration-500"></div>
      <div class="absolute -bottom-10 -right-10 w-60 h-60 bg-neoBlue border-8 border-black dark:border-zinc-700 opacity-50 rotate-45 transition-colors duration-500"></div>
      
      <div class="max-w-4xl mx-auto relative z-10">
        <h2 class="text-5xl md:text-8xl font-black uppercase mb-10 tracking-tighter leading-none drop-shadow-[4px_4px_0px_rgba(255,204,0,1)] dark:drop-shadow-[4px_4px_0px_rgba(255,204,0,0.5)] transition-all duration-500">
          Tingkatkan Efektifitas Tambang Anda Sekarang.
        </h2>
        <button class="bg-miningYellow text-black font-black text-2xl md:text-4xl border-4 border-white dark:border-zinc-700 px-12 py-6 shadow-[8px_8px_0px_0px_rgba(255,255,255,1)] dark:shadow-[8px_8px_0px_0px_rgba(100,100,100,1)] hover:translate-x-[4px] hover:translate-y-[4px] hover:shadow-[4px_4px_0px_0px_rgba(255,255,255,1)] dark:hover:shadow-[4px_4px_0px_0px_rgba(100,100,100,1)] active:translate-x-[8px] active:translate-y-[8px] active:shadow-none transition-all duration-300 uppercase mt-8">
          Ayo Bergabung Bersama Kami
        </button>
      </div>
    </footer>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'

const isDark = ref(false)
const isMounted = ref(false)
const isMenuOpen = ref(false)

const updateDOM = (dark) => {
  if (process.client) {
    if (dark) {
      document.documentElement.classList.add('dark')
    } else {
      document.documentElement.classList.remove('dark')
    }
  }
}

const toggleTheme = () => {
  isDark.value = !isDark.value
  localStorage.setItem('theme', isDark.value ? 'dark' : 'light')
  updateDOM(isDark.value)
}

const toggleMenu = () => {
  isMenuOpen.value = !isMenuOpen.value
}

onMounted(() => {
  isMounted.value = true
  const savedTheme = localStorage.getItem('theme')
  const osPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
  
  if (savedTheme === 'dark' || (!savedTheme && osPrefersDark)) {
    isDark.value = true
  } else {
    isDark.value = false
  }
  
  updateDOM(isDark.value)
})
</script>

<style>
/* Font Space Grotesk di-load global via app.vue atau nuxt.config */

/* Grid Pattern Background untuk Hero (Responsif Tema) */
.grid-pattern {
  background-image: radial-gradient(#000 2px, transparent 2px);
  background-size: 32px 32px;
}
.dark .grid-pattern {
  background-image: radial-gradient(rgba(255,255,255,0.2) 2px, transparent 2px);
}

/* Dots Pattern untuk Tentang (Responsif Tema) */
.pattern-dots {
  background-image: radial-gradient(#e5e7eb 2px, transparent 2px);
  background-size: 16px 16px;
}
.dark .pattern-dots {
  background-image: radial-gradient(rgba(255,255,255,0.1) 2px, transparent 2px);
}

/* --- ANIMASI KEYFRAMES MURNI CSS --- */
@keyframes float {
  0%, 100% { transform: translateY(0) rotate(0deg); }
  50% { transform: translateY(-20px) rotate(5deg); }
}
.animate-float {
  animation: float 6s ease-in-out infinite;
}

@keyframes float-delayed {
  0%, 100% { transform: translateY(0) rotate(12deg); }
  50% { transform: translateY(-15px) rotate(0deg); }
}
.animate-float-delayed {
  animation: float-delayed 5s ease-in-out infinite 1s;
}

@keyframes marquee {
  0% { transform: translateX(0%); }
  100% { transform: translateX(-50%); }
}
.animate-marquee {
  width: max-content;
  animation: marquee 20s linear infinite;
}
.animate-marquee:hover {
  animation-play-state: paused;
}

@keyframes barRise1 {
  0%, 100% { height: 40%; }
  50% { height: 80%; }
}
@keyframes barRise2 {
  0%, 100% { height: 70%; }
  50% { height: 30%; }
}
@keyframes barRise3 {
  0%, 100% { height: 50%; }
  50% { height: 90%; }
}
@keyframes barRise4 {
  0%, 100% { height: 20%; }
  50% { height: 60%; }
}

.animate-bar-1 { animation: barRise1 3s ease-in-out infinite; }
.animate-bar-2 { animation: barRise2 4s ease-in-out infinite 0.5s; }
.animate-bar-3 { animation: barRise3 3.5s ease-in-out infinite 1s; }
.animate-bar-4 { animation: barRise4 2.5s ease-in-out infinite 1.5s; }
</style>