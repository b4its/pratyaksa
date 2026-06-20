<template>
  <div class="min-h-screen bg-topo relative flex items-center justify-center p-4 py-12 overflow-hidden text-[color:var(--text)]">

    <!-- Ambient industrial glow -->
    <div class="absolute inset-0 z-0 pointer-events-none overflow-hidden">
      <div class="absolute -top-32 -right-24 w-[28rem] h-[28rem] rounded-full bg-steel/20 blur-3xl"></div>
      <div class="absolute -bottom-40 -left-24 w-96 h-96 rounded-full bg-copper/20 blur-3xl"></div>
      <div class="absolute inset-0 bg-mesh opacity-60"></div>
    </div>

    <main class="relative z-10 w-full max-w-lg panel-raised overflow-hidden anim-pop">

      <div class="h-1.5 hazard-stripe opacity-90"></div>

      <div class="p-8">
        <header class="mb-8 flex justify-between items-start gap-4">
          <div>
            <div class="mb-5">
              <AppLogo height="2.75rem" />
              <p class="text-[11px] font-semibold uppercase tracking-[0.2em] text-[color:var(--text-faint)] mt-2">Mining Intelligence</p>
            </div>
            <h1 class="font-display text-4xl font-bold uppercase tracking-wide">Daftar</h1>
            <p class="text-[color:var(--text-muted)] text-sm mt-1">Buat akun baru untuk mengakses sistem.</p>
          </div>
          <span class="badge bg-amber/15 border-amber/40 text-amber-deep hidden sm:inline-flex">Akses Baru</span>
        </header>

        <form @submit.prevent="handleRegister" class="space-y-5">

          <div>
            <label for="name" class="label">Nama Lengkap</label>
            <input id="name" v-model="form.name" type="text" placeholder="John Doe" class="field" required />
          </div>

          <div>
            <label for="email" class="label">Email</label>
            <input id="email" v-model="form.email" type="email" placeholder="engineer@pratyaksa.id" class="field" required />
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-5">
            <div>
              <label for="password" class="label">Kata Sandi</label>
              <div class="relative">
                <input
                  id="password"
                  v-model="form.password"
                  :type="showPassword ? 'text' : 'password'"
                  placeholder="••••••••"
                  class="field pr-12"
                  required
                />
                <button type="button" @click="showPassword = !showPassword"
                  class="absolute right-2 top-1/2 -translate-y-1/2 p-2 rounded-lg text-[color:var(--text-muted)] hover:text-amber hover:bg-[color:var(--surface-2)] transition-colors focus:outline-none">
                  <svg v-if="!showPassword" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/></svg>
                  <svg v-else xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9.88 9.88a3 3 0 1 0 4.24 4.24"/><path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"/><path d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"/><line x1="2" x2="22" y1="2" y2="22"/></svg>
                </button>
              </div>
            </div>

            <div>
              <label for="confirmPassword" class="label">Konfirmasi</label>
              <div class="relative">
                <input
                  id="confirmPassword"
                  v-model="form.confirmPassword"
                  :type="showConfirmPassword ? 'text' : 'password'"
                  placeholder="••••••••"
                  class="field pr-12"
                  :class="{ '!border-critical focus:!shadow-[0_0_0_3px_rgba(224,65,62,0.25)]': passwordMismatch }"
                  required
                />
                <button type="button" @click="showConfirmPassword = !showConfirmPassword"
                  class="absolute right-2 top-1/2 -translate-y-1/2 p-2 rounded-lg text-[color:var(--text-muted)] hover:text-amber hover:bg-[color:var(--surface-2)] transition-colors focus:outline-none">
                  <svg v-if="!showConfirmPassword" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/></svg>
                  <svg v-else xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9.88 9.88a3 3 0 1 0 4.24 4.24"/><path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"/><path d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"/><line x1="2" x2="22" y1="2" y2="22"/></svg>
                </button>
              </div>
            </div>
          </div>

          <div v-if="passwordMismatch" class="flex items-center gap-2 px-4 py-2.5 rounded-lg bg-critical/10 border border-critical/40 text-critical">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
            <span class="font-semibold text-xs uppercase tracking-wide">Kata sandi tidak cocok</span>
          </div>

          <div v-if="errorMessage" class="flex items-center gap-2 px-4 py-3 rounded-lg bg-critical/10 border border-critical/40 text-critical">
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
            <span class="font-semibold text-sm">{{ errorMessage }}</span>
          </div>

          <button
            type="submit"
            :disabled="isLoading || passwordMismatch"
            class="btn btn-amber w-full !py-3.5 text-base disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span v-if="isLoading">Mendaftarkan…</span>
            <span v-else>Buat Akun</span>
            <svg v-if="!isLoading" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><line x1="19" x2="19" y1="8" y2="14"/><line x1="22" x2="16" y1="11" y2="11"/></svg>
            <svg v-else class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path></svg>
          </button>
        </form>

        <div class="mt-8 pt-6 border-t border-[color:var(--border)] text-center space-y-3">
          <p class="text-sm text-[color:var(--text-muted)]">
            Sudah punya akses?
            <NuxtLink to="/account/login" class="font-semibold text-amber hover:underline underline-offset-4 ml-1">Masuk</NuxtLink>
          </p>
          <NuxtLink to="../" class="inline-flex items-center gap-1.5 text-xs font-semibold text-[color:var(--text-faint)] hover:text-steel transition-colors">
            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="m12 19-7-7 7-7"/><path d="M19 12H5"/></svg>
            Kembali ke Landing Page
          </NuxtLink>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

// Form State
const form = reactive({
  name: '',
  email: '',
  password: '',
  confirmPassword: ''
})

// UI State
const isLoading = ref(false)
const showPassword = ref(false)
const showConfirmPassword = ref(false)
const errorMessage = ref('')

// Sinkronisasi Tema (Fix TypeScript via import.meta.client)
onMounted(() => {
  if (import.meta.client) {
    const savedTheme = localStorage.getItem('theme')
    const osPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches

    if (savedTheme === 'dark' || (!savedTheme && osPrefersDark)) {
      document.documentElement.classList.add('dark')
    } else {
      document.documentElement.classList.remove('dark')
    }
  }
})

// Validation Logic
const passwordMismatch = computed(() => {
  if (form.confirmPassword === '') return false
  return form.password !== form.confirmPassword
})

// Submit Handler
const handleRegister = async () => {
  if (passwordMismatch.value || !form.password || !form.name || !form.email) return

  isLoading.value = true
  errorMessage.value = ''

  try {
    const auth = useAuth()
    await auth.register(form.name, form.email, form.password)
    await router.push('/panel/dashboard')
  } catch (error: any) {
    const msg = error?.data?.message || error?.message || 'Pendaftaran gagal. Coba lagi.'
    errorMessage.value = msg
    console.error('Registration failed', error)
  } finally {
    isLoading.value = false
  }
}
</script>
