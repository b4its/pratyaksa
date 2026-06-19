import { ref, watch, type Ref } from 'vue'

/**
 * Animasi angka naik menuju nilai target. Mengembalikan ref yang
 * di-animasikan setiap kali `source` berubah.
 */
export function useCountUp(source: Ref<number>, duration = 700) {
  const display = ref(source.value)

  const animate = (from: number, to: number) => {
    if (!import.meta.client) { display.value = to; return }
    const start = performance.now()
    const step = (now: number) => {
      const t = Math.min((now - start) / duration, 1)
      // easeOutCubic
      const eased = 1 - Math.pow(1 - t, 3)
      display.value = from + (to - from) * eased
      if (t < 1) requestAnimationFrame(step)
      else display.value = to
    }
    requestAnimationFrame(step)
  }

  watch(source, (to, from) => animate(from ?? 0, to), { immediate: false })

  return display
}
