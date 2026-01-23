<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { request } from '@/api/api'

const router = useRouter()
const auth = useAuthStore()

const email = ref('')
const password = ref('')
const error = ref('')
const loading = ref(false)

async function handleLogin() {
  loading.value = true
  error.value = ''
  try {
    const data = await request<{ token: string; account_details: { username: string; email: string } }>(
      '/clients/login',
      'POST',
      { email: email.value, password: password.value }
    )
    auth.setAuth(data.token, data.account_details)
    router.push('/rooms')
  } catch (e: any) {
    error.value = e.message
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center p-6">
    <div class="glass-card w-full max-w-md p-8 space-y-8 animate-in fade-in zoom-in duration-500">
      <div class="text-center">
        <h1 class="text-4xl font-bold text-white mb-2">Welcome Back</h1>
        <p class="text-slate-400">Login to manage your quiz rooms</p>
      </div>

      <form @submit.prevent="handleLogin" class="space-y-6">
        <div v-if="error" class="bg-red-500/10 border border-red-500/50 text-red-500 p-3 rounded-xl text-sm">
          {{ error }}
        </div>

        <div class="space-y-2">
          <label class="text-sm font-medium text-slate-300 ml-1">Email Address</label>
          <input
            v-model="email"
            type="email"
            required
            class="input-field"
            placeholder="you@example.com"
          />
        </div>

        <div class="space-y-2">
          <label class="text-sm font-medium text-slate-300 ml-1">Password</label>
          <input
            v-model="password"
            type="password"
            required
            class="input-field"
            placeholder="••••••••"
          />
        </div>

        <button :disabled="loading" type="submit" class="btn-primary w-full py-3">
          <span v-if="loading" class="flex items-center justify-center">
            <svg class="animate-spin h-5 w-5 mr-3 text-white" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Logging in...
          </span>
          <span v-else>Login</span>
        </button>
      </form>

      <p class="text-center text-slate-400">
        Don't have an account?
        <router-link to="/register" class="text-brand hover:text-brand-light font-medium transition-colors">Register</router-link>
      </p>
    </div>
  </div>
</template>
