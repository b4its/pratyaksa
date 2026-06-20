<script setup lang="ts">
/**
 * Target link dari bot Telegram: /wo/generate/{asset_id}
 * Mengarahkan ke halaman Work Order di panel dengan unit yang sudah dipilih.
 */
const route = useRoute()
const asset = String(route.params.asset || '')

await navigateTo({ path: '/panel/work_order', query: asset ? { asset } : {} }, { redirectCode: 302 })
</script>

<template>
  <div class="flex items-center justify-center h-screen text-[color:var(--text-muted)] font-semibold uppercase tracking-widest">
    Mengarahkan ke Work Order…
  </div>
</template>
