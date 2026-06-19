/**
 * Directive v-tilt — efek kartu 3D interaktif mengikuti pergerakan mouse.
 * Universal plugin: getSSRProps mencegah error saat SSR; logika DOM hanya
 * dijalankan pada hook mounted (client-side).
 */
export default defineNuxtPlugin((nuxtApp) => {
  interface TiltOpts { max?: number; scale?: number }
  const handlers = new WeakMap<HTMLElement, { move: (e: MouseEvent) => void; leave: () => void }>()

  nuxtApp.vueApp.directive('tilt', {
    getSSRProps() {
      return {}
    },
    mounted(el: HTMLElement, binding) {
      const opts: TiltOpts = binding.value || {}
      const max = opts.max ?? 10
      const scale = opts.scale ?? 1.02
      el.classList.add('tilt-card')

      const move = (e: MouseEvent) => {
        const rect = el.getBoundingClientRect()
        const px = (e.clientX - rect.left) / rect.width
        const py = (e.clientY - rect.top) / rect.height
        const rx = (0.5 - py) * max * 2
        const ry = (px - 0.5) * max * 2
        el.style.transform = `perspective(900px) rotateX(${rx.toFixed(2)}deg) rotateY(${ry.toFixed(2)}deg) scale(${scale})`
        el.style.setProperty('--mx', `${(px * 100).toFixed(1)}%`)
        el.style.setProperty('--my', `${(py * 100).toFixed(1)}%`)
      }
      const leave = () => {
        el.style.transform = 'perspective(900px) rotateX(0) rotateY(0) scale(1)'
      }

      el.addEventListener('mousemove', move)
      el.addEventListener('mouseleave', leave)
      handlers.set(el, { move, leave })
    },
    unmounted(el: HTMLElement) {
      const h = handlers.get(el)
      if (h) {
        el.removeEventListener('mousemove', h.move)
        el.removeEventListener('mouseleave', h.leave)
        handlers.delete(el)
      }
    },
  })
})
