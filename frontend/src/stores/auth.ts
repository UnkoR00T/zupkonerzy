import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useAuthStore = defineStore('auth', () => {
  const token = ref<string | null>(localStorage.getItem('token'))
  const user = ref<{ username: string; email: string } | null>(
    JSON.parse(localStorage.getItem('user') || 'null')
  )

  const isAuthenticated = computed(() => !!token.value)

  function setAuth(newToken: string, userData?: { username: string; email: string }) {
    token.value = newToken
    localStorage.setItem('token', newToken)
    if (userData) {
      user.value = userData
      localStorage.setItem('user', JSON.stringify(userData))
    }
  }

  function logout() {
    token.value = null
    user.value = null
    localStorage.removeItem('token')
    localStorage.removeItem('user')
  }

  return {
    token,
    user,
    isAuthenticated,
    setAuth,
    logout,
  }
})
