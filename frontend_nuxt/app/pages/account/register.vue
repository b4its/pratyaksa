<template>
  <div class="min-h-screen bg-[#fdf5e6] flex items-center justify-center p-4 selection:bg-[#ffc900] selection:text-black font-sans py-12">
    
    <!-- Background Decoration -->
    <div class="absolute inset-0 z-0 pointer-events-none overflow-hidden flex flex-col justify-between items-end opacity-20">
       <div class="w-72 h-72 bg-[#88aaff] border-4 border-black rotate-45 -mt-20 -mr-20 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)]"></div>
       <div class="w-48 h-48 bg-[#a6fa86] border-4 border-black rounded-full mb-10 ml-10 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)]"></div>
    </div>

    <!-- Main Register Card -->
    <main class="relative z-10 w-full max-w-lg bg-white border-4 border-black p-8 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] rounded-none">
      
      <!-- Header -->
      <header class="mb-8 border-b-4 border-black pb-4 flex justify-between items-end">
        <div>
          <h1 class="text-4xl font-black uppercase tracking-tight text-black mb-2">
            Daftar
          </h1>
          <p class="text-black font-bold text-sm">
            Buat akun baru sistem Pratyaksa.
          </p>
        </div>
        <!-- Ornamental Badge -->
        <div class="bg-[#ffc900] px-3 py-1 border-4 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] -rotate-3 hidden sm:block">
          <span class="font-black uppercase text-xs">Dapatkan Akses Baru</span>
        </div>
      </header>

      <!-- Register Form -->
      <form @submit.prevent="handleRegister" class="space-y-5">
        
        <!-- Name Field -->
        <div class="space-y-2">
          <label for="name" class="block font-bold text-black uppercase text-sm">Nama Lengkap</label>
          <input 
            id="name"
            v-model="form.name"
            type="text" 
            placeholder="John Doe"
            class="w-full px-4 py-3 bg-white text-black border-4 border-black focus:outline-none shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] focus:shadow-none focus:translate-x-[4px] focus:translate-y-[4px] transition-all duration-200 font-bold placeholder:text-gray-400 placeholder:font-medium"
            required
          />
        </div>

        <!-- Email Field -->
        <div class="space-y-2">
          <label for="email" class="block font-bold text-black uppercase text-sm">Email</label>
          <input 
            id="email"
            v-model="form.email"
            type="email" 
            placeholder="engineer@pratyaksa.id"
            class="w-full px-4 py-3 bg-white text-black border-4 border-black focus:outline-none shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] focus:shadow-none focus:translate-x-[4px] focus:translate-y-[4px] transition-all duration-200 font-bold placeholder:text-gray-400 placeholder:font-medium"
            required
          />
        </div>

        <!-- Password Grid Layout for larger screens -->
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-5">
          <!-- Password Field -->
          <div class="space-y-2 relative">
            <label for="password" class="block font-bold text-black uppercase text-sm">Kata Sandi</label>
            <div class="relative">
              <input 
                id="password"
                v-model="form.password"
                :type="showPassword ? 'text' : 'password'" 
                placeholder="••••••••"
                class="w-full px-4 py-3 bg-white text-black border-4 border-black focus:outline-none shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] focus:shadow-none focus:translate-x-[4px] focus:translate-y-[4px] transition-all duration-200 font-bold placeholder:text-gray-400"
                required
              />
              <button 
                type="button" 
                @click="showPassword = !showPassword"
                class="absolute right-3 top-1/2 -translate-y-1/2 p-1 border-2 border-transparent hover:border-black hover:bg-[#ff90e8] transition-colors focus:outline-none"
              >
                <svg v-if="!showPassword" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/></svg>
                <svg v-else xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><path d="M9.88 9.88a3 3 0 1 0 4.24 4.24"/><path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"/><path d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"/><line x1="2" x2="22" y1="2" y2="22"/></svg>
              </button>
            </div>
          </div>

          <!-- Confirm Password Field -->
          <div class="space-y-2 relative">
            <label for="confirmPassword" class="block font-bold text-black uppercase text-sm">Konfirmasi Sandi</label>
            <div class="relative">
              <input 
                id="confirmPassword"
                v-model="form.confirmPassword"
                :type="showConfirmPassword ? 'text' : 'password'" 
                placeholder="••••••••"
                class="w-full px-4 py-3 bg-white text-black border-4 border-black focus:outline-none shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] focus:shadow-none focus:translate-x-[4px] focus:translate-y-[4px] transition-all duration-200 font-bold placeholder:text-gray-400"
                :class="{'border-red-500 shadow-[4px_4px_0px_0px_rgba(239,68,68,1)]': passwordMismatch}"
                required
              />
              <button 
                type="button" 
                @click="showConfirmPassword = !showConfirmPassword"
                class="absolute right-3 top-1/2 -translate-y-1/2 p-1 border-2 border-transparent hover:border-black hover:bg-[#ff90e8] transition-colors focus:outline-none"
              >
                <svg v-if="!showConfirmPassword" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/></svg>
                <svg v-else xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square"><path d="M9.88 9.88a3 3 0 1 0 4.24 4.24"/><path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"/><path d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"/><line x1="2" x2="22" y1="2" y2="22"/></svg>
              </button>
            </div>
          </div>
        </div>

        <!-- Mismatch Error Banner -->
        <div v-if="passwordMismatch" class="bg-red-400 border-4 border-black px-4 py-2 flex items-center gap-2">
          <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="3" stroke-linecap="square"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
          <span class="font-bold text-black uppercase text-xs tracking-wider">Kata sandi tidak cocok!</span>
        </div>

        <!-- Submit Button -->
        <button 
          type="submit" 
          :disabled="isLoading || passwordMismatch"
          class="w-full mt-6 px-6 py-4 bg-[#ff90e8] text-black font-black uppercase tracking-wider border-4 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:shadow-none hover:translate-x-[4px] hover:translate-y-[4px] transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
        >
          <span v-if="isLoading">Mendaftarkan...</span>
          <span v-else>Buat Akun</span>
          <svg v-if="!isLoading" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="square" stroke-linejoin="miter"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><line x1="19" x2="19" y1="8" y2="14"/><line x1="22" x2="16" y1="11" y2="11"/></svg>
          <svg v-else class="animate-spin h-5 w-5 text-black" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
        </button>
      </form>

      <!-- Footer Action -->
      <div class="mt-8 pt-6 border-t-4 border-black text-center">
        <p class="text-black font-bold">
          Sudah punya akses? 
          <NuxtLink 
            to="/account/login" 
            class="bg-[#ffc900] px-2 py-1 ml-1 border-2 border-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:shadow-none hover:translate-x-[2px] hover:translate-y-[2px] transition-all duration-200 inline-block uppercase text-xs"
          >
            Masuk
          </NuxtLink>
        </p>
        <p>atau</p>
        <p class="text-black font-bold mt-4">
             Kembali ke
          <NuxtLink 
            to="../" 
            class="bg-neoBlue px-2 py-1 ml-1 border-2 border-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:shadow-none hover:translate-x-[2px] hover:translate-y-[2px] transition-all duration-200 inline-block uppercase text-xs"
          >
            Landing Page
          </NuxtLink>
        </p>
      </div>

    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
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

// Validation Logic
const passwordMismatch = computed(() => {
  if (form.confirmPassword === '') return false
  return form.password !== form.confirmPassword
})

// Submit Handler
const handleRegister = async () => {
  // Guard check preventif jika submit ter-trigger saat form tidak valid
  if (passwordMismatch.value || !form.password || !form.name || !form.email) return

  isLoading.value = true
  
  try {
    // Simulasi POST request ke backend
    await new Promise(resolve => setTimeout(resolve, 1500))
    
    console.log('Register success for:', form.email)
    
    // Redirect ke login atau langsung dashboard setelah register
    await router.push('/account/login')
    
  } catch (error) {
    console.error('Registration failed', error)
  } finally {
    isLoading.value = false
  }
}
</script>