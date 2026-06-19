import { ref } from 'vue'

// Singleton state — dibagikan ke seluruh halaman
const isDark = ref(false)

/**
 * useTheme — manajemen dark / light mode.
 * Disinkronkan dengan localStorage('theme') agar konsisten di semua halaman
 * (landing, auth, panel) dan mengikuti preferensi OS bila belum diset.
 */
export const useTheme = () => {
  const apply = (dark: boolean) => {
    isDark.value = dark
    if (import.meta.client) {
      document.documentElement.classList.toggle('dark', dark)
    }
  }

  const initTheme = () => {
    if (!import.meta.client) return
    const saved = localStorage.getItem('theme')
    const osPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    apply(saved === 'dark' || (!saved && osPrefersDark))
  }

  const toggleTheme = () => {
    const next = !isDark.value
    apply(next)
    if (import.meta.client) localStorage.setItem('theme', next ? 'dark' : 'light')
  }

  const setTheme = (dark: boolean) => {
    apply(dark)
    if (import.meta.client) localStorage.setItem('theme', dark ? 'dark' : 'light')
  }

  return { isDark, initTheme, toggleTheme, setTheme }
}
