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
      link: [
        { rel: 'stylesheet', href: 'https://unpkg.com/leaflet@1.9.4/dist/leaflet.css' },
      ],
    },
    // Transisi antar halaman
    pageTransition: { name: 'page', mode: 'out-in' },
  },

  // 2. ATUR HOST KE '0.0.0.0' DISINI SETELAH SRCDIR DITEGASKAN
  devServer: {
    host: '0.0.0.0',
    port: 3000
  },

  // ---  KONFIGURASI TAILWIND — MINING / INDUSTRIAL DESIGN SYSTEM ---
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
            // Graphite menggantikan "black" → melembutkan seluruh border/teks/aksen lama
            black: '#171d24',

            // === Token aksen utama (memetakan token lama agar markup re-skinned otomatis) ===
            miningYellow: '#F2A60C', // hi-vis safety amber
            neoYellow:    '#C2703D', // copper / ore
            neoRed:       '#E0413E', // hazard red
            neoBlue:      '#3E92CC', // steel blue (data)

            // === Palet industrial / pertambangan ===
            graphite: {
              50:  '#f3f5f7',
              100: '#e5e8ec',
              200: '#cbd2da',
              300: '#a4afbc',
              400: '#76859a',
              500: '#566479',
              600: '#3f4b5c',
              700: '#2c3643',
              800: '#1d242e',
              900: '#141a21',
              950: '#0c1014',
            },
            amber: {
              DEFAULT: '#F2A60C',
              soft: '#FBBF3F',
              deep: '#D9930A',
            },
            copper: '#C2703D',
            steel:  '#3E92CC',
            ore:    '#B65A2C',
            sand:   '#E7DECF',

            // === Status armada ===
            healthy:  '#1FA971',
            warning:  '#E0A106',
            critical: '#E0413E',
            rusak:    '#7A848E',
          },
          fontFamily: {
            sans:    ['Inter', 'ui-sans-serif', 'system-ui', 'sans-serif'],
            display: ['"Saira Condensed"', 'Inter', 'sans-serif'],
            mono:    ['"JetBrains Mono"', 'ui-monospace', 'monospace'],
          },
          boxShadow: {
            // Elevasi lembut menggantikan hard-offset brutalist
            neo:       '0 18px 44px -20px rgba(12,16,20,0.55)',
            neoHover:  '0 10px 24px -14px rgba(12,16,20,0.45)',
            'elev-sm': '0 4px 14px -8px rgba(12,16,20,0.35)',
            'elev-lg': '0 28px 60px -24px rgba(12,16,20,0.6)',
            'inset-line': 'inset 0 1px 0 0 rgba(255,255,255,0.05)',
            'glow-amber': '0 0 0 1px rgba(242,166,12,0.4), 0 8px 30px -8px rgba(242,166,12,0.35)',
          },
          borderRadius: {
            xl2: '1.125rem',
            '3xl': '1.75rem',
          },
          backgroundImage: {
            'amber-gradient': 'linear-gradient(135deg, #F2A60C 0%, #D9930A 100%)',
            'steel-gradient': 'linear-gradient(135deg, #2c3643 0%, #141a21 100%)',
            'copper-gradient': 'linear-gradient(135deg, #D98049 0%, #B65A2C 100%)',
          },
        }
      }
    }
  }
})
