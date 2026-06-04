<template>
  <div class="min-h-screen bg-[#bg-color] bg-[#fdf5e6] flex items-center justify-center p-4 selection:bg-[#ff90e8] selection:text-black font-sans">
    
    <div class="absolute inset-0 z-0 pointer-events-none overflow-hidden flex justify-between items-center opacity-30">
       <div class="w-64 h-64 bg-[#ffc900] border-4 border-black rounded-full -ml-32 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)]"></div>
       <div class="w-96 h-96 bg-[#ff90e8] border-4 border-black -mr-20 rotate-12 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)]"></div>
    </div>

    <main class="relative z-10 w-full max-w-md bg-white border-4 border-black p-8 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] rounded-none">
      
      <header class="mb-8 border-b-4 border-black pb-4">
        <h1 class="text-4xl font-black uppercase tracking-tight text-black mb-2">
          Masuk
        </h1>
        <p class="text-black font-bold text-sm">
          Akses panel kontrol sistem Pratyaksa.
        </p>
      </header>

      <form @submit.prevent="handleLogin" class="space-y-6">
        
        <div class="space-y-2 relative">
          <label for="email" class="block font-bold text-black uppercase text-sm">Email Akses</label>
          <input 
            id="email"
            v-model="form.email"
            type="email" 
            placeholder="engineer@pratyaksa.id"
            class="w-full px-4 py-3 bg-white text-black border-4 border-black focus:outline-none shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] focus:shadow-none focus:translate-x-[4px] focus:translate-y-[4px] transition-all duration-200 font-bold placeholder:text-gray-400 placeholder:font-medium"
            required
          />
        </div>

        <div class="space-y-2 relative">
          <div class="flex justify-between items-center">
            <label for="password" class="block font-bold text-black uppercase text-sm">Kata Sandi</label>
            <a href="#" class="text-xs font-bold underline hover:text-[#ff90e8] transition-colors">Lupa sandi?</a>
          </div>
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
              @click="togglePassword"
              class="absolute right-3 top-1/2 -translate-y-1/2 p-1 border-2 border-transparent hover:border-black hover:bg-[#ffc900] transition-colors focus:outline-none"
              aria-label="Toggle password visibility"
            >
              <svg v-if="!showPassword" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square" stroke-linejoin="miter"><path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/></svg>
              <svg v-else xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="square" stroke-linejoin="miter"><path d="M9.88 9.88a3 3 0 1 0 4.24 4.24"/><path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"/><path d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"/><line x1="2" x2="22" y1="2" y2="22"/></svg>
            </button>
          </div>
        </div>

        <button 
          type="submit" 
          :disabled="isLoading"
          class="w-full mt-6 px-6 py-4 bg-[#a6fa86] text-black font-black uppercase tracking-wider border-4 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:shadow-none hover:translate-x-[4px] hover:translate-y-[4px] transition-all duration-200 disabled:opacity-70 disabled:cursor-not-allowed flex items-center justify-center gap-2"
        >
          <span v-if="isLoading">Memproses...</span>
          <span v-else>Masuk ke Sistem</span>
          <svg v-if="!isLoading" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="square" stroke-linejoin="miter"><path d="M5 12h14"/><path d="m12 5 7 7-7 7"/></svg>
          <svg v-else class="animate-spin h-5 w-5 text-black" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
        </button>
      </form>

      <div class="mt-8 pt-6 border-t-4 border-black text-center">
        <p class="text-black font-bold">
          Belum punya akses? 
          <NuxtLink 
            to="/account/register" 
            class="bg-[#ff90e8] px-2 py-1 ml-1 border-2 border-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:shadow-none hover:translate-x-[2px] hover:translate-y-[2px] transition-all duration-200 inline-block uppercase text-xs"
          >
            Daftar
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
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

// Reactive state
const form = reactive({
  email: '',
  password: ''
})

const isLoading = ref(false)
const showPassword = ref(false)

// Methods
const togglePassword = () => {
  showPassword.value = !showPassword.value
}

const handleLogin = async () => {
  isLoading.value = true
  
  // Simulasi API call delay untuk mendemonstrasikan state interaktif UX Neobrutalism
  try {
    await new Promise(resolve => setTimeout(resolve, 1500))
    
    // Logika otentikasi aktual Anda akan berada di sini (contoh: memanggil Nuxt/Auth atau store Pinia)
    console.log('Login attempt with:', form.email)
    
    // Redirect ke panel setelah login sukses
    await router.push('/panel/dashboard')
    
  } catch (error) {
    console.error('Login failed', error)
    // Terapkan toast notification bergaya Neobrutalism di sini jika gagal
  } finally {
    isLoading.value = false
  }
}
</script>