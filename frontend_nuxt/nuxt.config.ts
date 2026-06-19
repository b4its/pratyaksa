// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  // 1. TEGASKAN KEMBALI folder source kamu ada di 'app' agar Nuxt tidak bingung
  srcDir: 'app',

  dir: {
    pages: 'pages',
  },

  modules: [
    '@nuxtjs/tailwindcss',
    'shadcn-nuxt'
  ],

  // Daftarkan <model-viewer> (Google) sebagai custom element agar Vue tidak warning
  vue: {
    compilerOptions: {
      isCustomElement: (tag) => tag === 'model-viewer',
    },
  },

  // Runtime config — NUXT_PUBLIC_API_BASE bisa di-override via env var
  // Di production (Docker): /api/v1  — request melewati Nginx → backend
  // Di development lokal: http://localhost:8080/api/v1
  runtimeConfig: {
    public: {
      apiBase: process.env.NUXT_PUBLIC_API_BASE || 'http://localhost:8080/api/v1',
    },
  },

  shadcn: {
    /** Prefix untuk semua komponen shadcn */
    prefix: '',
    componentDir: './components/ui' 
  },

  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },

  // Global animation layer
  css: ['~/assets/css/animations.css'],

  app: {
    head: {
      title: 'Pratyaksa',
    },
    // Transisi antar halaman
    pageTransition: { name: 'page', mode: 'out-in' },
  },

  // 2. ATUR HOST KE '0.0.0.0' DISINI SETELAH SRCDIR DITEGASKAN
  devServer: {
    host: '0.0.0.0',
    port: 3000
  },

  // ---  KONFIGURASI TAILWIND ---
  tailwindcss: {
    config: {
      darkMode: 'class',
      content: [
        "./app/**/*.{js,vue,ts}",
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