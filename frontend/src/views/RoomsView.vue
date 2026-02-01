<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { request } from '@/api/api'

const router = useRouter()
const auth = useAuthStore()

interface Room {
  id: string
  name: string
  owner: string
}

const rooms = ref<Room[]>([])
const loading = ref(true)
const error = ref('')
const newRoomName = ref('')
const isCreating = ref(false)

async function fetchRooms() {
  loading.value = true
  try {
    const data = await request<Room[]>('/rooms', 'GET', null, auth.token!)
    rooms.value = data
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

async function createRoom() {
  if (!newRoomName.value) return
  isCreating.value = true
  try {
    const data = await request<{ id: string }>('/rooms', 'POST', { name: newRoomName.value }, auth.token!)
    rooms.value.push({ id: data.id, name: newRoomName.value, owner: '' }) // owner ID is usually not needed immediately
    newRoomName.value = ''
  } catch (e: any) {
    alert(e.message)
  } finally {
    isCreating.value = false
  }
}

async function deleteRoom(id: string) {
  if (!confirm('Are you sure you want to delete this room?')) return
  try {
    await request(`/rooms/${id}`, 'DELETE', null, auth.token!)
    rooms.value = rooms.value.filter(r => r.id !== id)
  } catch (e: any) {
    alert(e.message)
  }
}

onMounted(fetchRooms)
</script>

<template>
  <div class="max-w-6xl mx-auto p-6 space-y-8 animate-in fade-in slide-in-from-bottom-4 duration-700">
    <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
      <div>
        <h1 class="text-3xl font-bold text-white">Your Quiz Rooms</h1>
        <p class="text-slate-400">Manage and create new interactive experiences</p>
      </div>

      <div class="flex gap-2">
        <input v-model="newRoomName" @keyup.enter="createRoom" type="text" class="input-field max-w-xs"
          placeholder="Room Name" />
        <button @click="createRoom" :disabled="isCreating" class="btn-primary whitespace-nowrap">
          Create Room
        </button>
      </div>
    </div>

    <div v-if="loading" class="flex justify-center py-20">
      <div class="animate-pulse flex flex-col items-center">
        <div class="h-12 w-12 bg-brand/20 rounded-full mb-4"></div>
        <p class="text-slate-500">Loading rooms...</p>
      </div>
    </div>

    <div v-else-if="rooms.length === 0" class="glass-card p-12 text-center">
      <div class="mb-4">
        <svg class="h-16 w-16 mx-auto text-slate-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1"
            d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
        </svg>
      </div>
      <h3 class="text-xl font-medium text-slate-300">No rooms yet</h3>
      <p class="text-slate-500 mt-2">Create your first room using the form above.</p>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div v-for="room in rooms" :key="room.id"
        class="glass-card group hover:scale-[1.02] transition-all duration-300 p-6 flex flex-col justify-between">
        <div>
          <div class="flex justify-between items-start mb-4">
            <h2 class="text-xl font-bold text-white group-hover:text-brand transition-colors">{{ room.name }}</h2>
            <button @click.stop="deleteRoom(room.id)"
              class="text-slate-500 hover:text-red-500 p-1 rounded-lg hover:bg-red-500/10 transition-all">
              <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
            </button>
          </div>
          <p class="text-sm text-slate-500 mb-6">ID: {{ room.id }}</p>
        </div>

        <div class="flex gap-2 w-full">
          <button @click="router.push(`/rooms/${room.id}/access`)"
            class="flex-1 py-2 bg-white/5 hover:bg-brand text-slate-300 hover:text-white rounded-xl transition-all border border-white/10 flex items-center justify-center gap-2 group/btn">
            <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
            </svg>
            Access
          </button>
          <button @click="router.push(`/rooms/${room.id}/questions`)"
            class="flex-1 py-2 bg-white/5 hover:bg-brand text-slate-300 hover:text-white rounded-xl transition-all border border-white/10 flex items-center justify-center gap-2 group/btn">
            Questions
            <svg class="h-4 w-4 transform group-hover/btn:translate-x-1 transition-transform" fill="none"
              viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
            </svg>
          </button>
        </div>
        <div class="flex w-full mt-2">
          <button @click="router.push(`/rooms/${room.id}/game`)"
            class="flex-1 py-2 bg-green-700 hover:bg-green-500 text-slate-300 hover:text-white rounded-xl transition-all border border-white/10 flex items-center justify-center gap-2 group/btn">
            Play
            <svg class="h-4 w-4 transform group-hover/btn:translate-x-1 transition-transform" fill="none"
              viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
            </svg>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
