/**
 * Composable untuk state autentikasi
 * Token disimpan di localStorage
 */

interface AuthUser {
  id: string
  name: string
  email: string
  role: string
  created_at: string
}

const authUser = ref<AuthUser | null>(null)
const authToken = ref<string | null>(null)

export const useAuth = () => {
  const router = useRouter()
  const api = useApi()

  // Initialize from localStorage on client
  const initAuth = () => {
    if (import.meta.client) {
      const token = localStorage.getItem('auth_token')
      const userStr = localStorage.getItem('auth_user')
      if (token && userStr) {
        authToken.value = token
        try {
          authUser.value = JSON.parse(userStr)
        } catch {
          clearAuth()
        }
      }
    }
  }

  const setAuth = (token: string, user: AuthUser) => {
    authToken.value = token
    authUser.value = user
    if (import.meta.client) {
      localStorage.setItem('auth_token', token)
      localStorage.setItem('auth_user', JSON.stringify(user))
    }
  }

  const clearAuth = () => {
    authToken.value = null
    authUser.value = null
    if (import.meta.client) {
      localStorage.removeItem('auth_token')
      localStorage.removeItem('auth_user')
    }
  }

  const login = async (email: string, password: string) => {
    const res = await api.login(email, password) as any
    setAuth(res.data.token, res.data.user)
    return res
  }

  const register = async (name: string, email: string, password: string) => {
    const res = await api.register(name, email, password) as any
    setAuth(res.data.token, res.data.user)
    return res
  }

  const logout = () => {
    clearAuth()
    router.push('/account/login')
  }

  const isAuthenticated = computed(() => !!authToken.value)

  return {
    user: authUser,
    token: authToken,
    isAuthenticated,
    initAuth,
    login,
    register,
    logout,
  }
}
