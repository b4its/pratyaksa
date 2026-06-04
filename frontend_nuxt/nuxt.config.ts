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

  shadcn: {
    /** Prefix untuk semua komponen shadcn */
    prefix: '',
    componentDir: './components/ui' 
  },

  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  app: {
    head: {
      title: 'Pratyaksa',
    }
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