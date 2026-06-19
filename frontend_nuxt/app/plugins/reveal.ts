/**
 * Directive v-reveal — animasi muncul saat elemen masuk viewport (scroll reveal).
 * Universal plugin: getSSRProps mencegah error SSR; logika hanya di mounted (client).
 * Pakai: v-reveal | v-reveal="'left'" | v-reveal="{ anim: 'pop', delay: 120 }"
 */
export default defineNuxtPlugin((nuxtApp) => {
  const animClass: Record<string, string> = {
    up: 'anim-up',
    pop: 'anim-pop',
    left: 'anim-left',
  }

  nuxtApp.vueApp.directive('reveal', {
    getSSRProps() {
      return {}
    },
    mounted(el: HTMLElement, binding) {
      let anim = 'up'
      let delay = 0
      if (typeof binding.value === 'string') anim = binding.value
      else if (binding.value && typeof binding.value === 'object') {
        anim = binding.value.anim || 'up'
        delay = binding.value.delay || 0
      }

      el.style.opacity = '0'

      const reveal = () => {
        el.style.opacity = ''
        if (delay) el.style.animationDelay = `${delay}ms`
        el.classList.add(animClass[anim] || 'anim-up')
      }

      if (!('IntersectionObserver' in window)) {
        reveal()
        return
      }

      const obs = new IntersectionObserver(
        (entries) => {
          entries.forEach((entry) => {
            if (entry.isIntersecting) {
              reveal()
              obs.unobserve(el)
            }
          })
        },
        { threshold: 0.15 },
      )
      obs.observe(el)
    },
  })
})
