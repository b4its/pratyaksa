<script setup lang="ts">
/**
 * Logo PRATYAKSA (PNG) — dipakai di landing page, panel, dan halaman auth.
 * Logo sudah mencakup emblem + wordmark.
 *
 * Karena logo asli berwarna navy gelap (~66% area), di DARK MODE dipakai varian
 * terang (pratyaksa_logo_dark.png) agar tetap terlihat. Pergantian via CSS class
 * `html.dark` (bukan JS) supaya aman saat SSR / hydration.
 */
const props = withDefaults(defineProps<{ height?: string; onDark?: boolean }>(), {
  height: '2.25rem',
  onDark: false,
})
</script>

<template>
  <span class="pratyaksa-logo-wrap" :class="{ 'force-on-dark': props.onDark }" :style="{ height: props.height }">
    <img
      src="/assets/pratyaksa_logo.png"
      alt="PRATYAKSA"
      class="pratyaksa-logo logo-light"
      :style="{ height: props.height }"
      draggable="false"
    />
    <img
      src="/assets/pratyaksa_logo_dark.png"
      alt="PRATYAKSA"
      class="pratyaksa-logo logo-dark"
      :style="{ height: props.height }"
      draggable="false"
      aria-hidden="true"
    />
  </span>
</template>

<style>
.pratyaksa-logo-wrap { display: inline-flex; align-items: center; }
.pratyaksa-logo {
  width: auto;
  object-fit: contain;
  display: block;
  user-select: none;
}
/* Default (light theme): tampilkan logo asli, sembunyikan varian dark */
.pratyaksa-logo.logo-dark { display: none; }
/* Dark theme: tampilkan varian terang */
html.dark .pratyaksa-logo.logo-light { display: none; }
html.dark .pratyaksa-logo.logo-dark { display: block; }
/* Override manual: paksa varian terang saat berada di atas latar gelap
   (mis. navbar adaptif di atas hero), terlepas dari tema aktif. */
.pratyaksa-logo-wrap.force-on-dark .logo-light { display: none; }
.pratyaksa-logo-wrap.force-on-dark .logo-dark { display: block; }
</style>
