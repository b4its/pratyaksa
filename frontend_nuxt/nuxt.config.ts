// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  // Kasih tahu Nuxt kalau semua file source ada di dalam folder 'app'
  srcDir: 'app',
  dir: {
    pages: 'pages',
  },

  modules: [
    '@nuxtjs/tailwindcss',
    'shadcn-nuxt'
  ],

  
  shadcn: {
    /** Prefix untuk semua komponen shadcn kamu */
    prefix: '',
    /** * Karena srcDir sudah diarahkan ke 'app/', 
     * path di bawah ini otomatis relatif dari dalam folder 'app/' 
     */
    componentDir: './components/ui' 
  },

  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  app: {
    head: {
      title: 'Pratyaksa',
    }
  },

  

  // --- MASUKKAN KONFIGURASI TAILWIND DI SINI ---
  tailwindcss: {
    config: {
      darkMode: 'class',
      content: [
        "./app/**/*.{js,vue,ts}",
        // Memastikan app.vue dan index.vue di dalam folder app terbaca
        "./app/**/*.vue" 
      ],
      theme: {
        extend: {
          colors: {
            miningYellow: '#FFCC00',
            neoYellow: '#c5a218',
            neoRed: '#FF4911',
            neoBlue: '#33A8FF',
          },
          boxShadow: {
            neo: '8px 8px 0px 0px rgba(0,0,0,1)',
            neoHover: '4px 4px 0px 0px rgba(0,0,0,1)',
          }
        }
      }
    }
  }
  
})