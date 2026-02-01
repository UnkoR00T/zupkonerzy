<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { request } from '@/api/api'

const route = useRoute()
const router = useRouter()
const auth = useAuthStore()
const roomId = route.params.id as string

interface AccessedUser {
  id: string
  name: string
  email: string
}

const users = ref<AccessedUser[]>([])
const loading = ref(true)
const error = ref('')
const newUserEmail = ref('')
const isAdding = ref(false)

async function fetchAccess() {
  loading.value = true
  try {
    const data = await request<AccessedUser[]>(`/rooms/${roomId}/access`, 'GET', null, auth.token!)
    users.value = data
  } catch (e: unknown) {
    if (e instanceof Error) {
      error.value = e.message
    } else {
      error.value = 'An unknown error occurred'
    }
  } finally {
    loading.value = false
  }
}

async function addAccess() {
  if (!newUserEmail.value) return
  isAdding.value = true
  try {
    await request(`/rooms/${roomId}/access`, 'POST', { email: newUserEmail.value }, auth.token!)
    newUserEmail.value = ''
    fetchAccess()
  } catch (e: any) {
    alert(e.message)
  } finally {
    isAdding.value = false
  }
}

async function removeAccess(email: string) {
  if (!confirm(`Are you sure you want to remove access for ${email}?`)) return
  try {
    await request(`/rooms/${roomId}/access`, 'DELETE', { email }, auth.token!)
    fetchAccess()
  } catch (e: any) {
    alert(e.message)
  }
}

onMounted(fetchAccess)
</script>

<template>
  <div class="max-w-4xl mx-auto p-6 space-y-8 animate-in fade-in slide-in-from-bottom-4 duration-700">
    <div class="flex items-center gap-4">
      <button @click="router.push('/rooms')" class="text-slate-400 hover:text-white transition-colors">
        <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
        </svg>
      </button>
      <div>
        <h1 class="text-3xl font-bold text-white">Room Access Management</h1>
        <p class="text-slate-400">Manage who can access this room</p>
      </div>
    </div>

    <!-- Add User Form -->
    <div class="glass-card p-6">
      <h2 class="text-xl font-bold text-white mb-4">Grant Access</h2>
      <div class="flex gap-2">
        <input v-model="newUserEmail" @keyup.enter="addAccess" type="email" class="input-field max-w-md w-full"
          placeholder="User Email" />
        <button @click="addAccess" :disabled="isAdding" class="btn-primary whitespace-nowrap">
          {{ isAdding ? 'Adding...' : 'Add User' }}
        </button>
      </div>
    </div>

    <!-- Users List -->
    <div v-if="loading" class="flex justify-center py-10">
      <div class="animate-pulse flex flex-col items-center">
        <div class="h-8 w-8 bg-brand/20 rounded-full mb-3"></div>
        <p class="text-slate-500">Loading users...</p>
      </div>
    </div>

    <div v-else-if="users.length === 0" class="glass-card p-8 text-center">
      <p class="text-slate-400">No other users have access to this room.</p>
    </div>

    <div v-else class="space-y-4">
      <h3 class="text-lg font-semibold text-white">Users with Access</h3>
      <div class="grid gap-3">
        <div v-for="user in users" :key="user.id"
          class="glass-card p-4 flex items-center justify-between group hover:bg-white/5 transition-all">
          <div>
            <p class="text-white font-medium">{{ user.name || 'Unnamed User' }}</p>
            <p class="text-sm text-slate-500">{{ user.email }}</p>
          </div>
          <button @click="removeAccess(user.email!)"
            class="text-slate-500 hover:text-red-500 p-2 rounded-lg hover:bg-red-500/10 transition-all"
            title="Remove Access">
            <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
