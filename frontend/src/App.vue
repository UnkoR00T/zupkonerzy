<script setup lang="ts">
import { RouterView, useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const route = useRoute()
const auth = useAuthStore()

function handleLogout() {
  auth.logout()
  router.push('/login')
}
</script>

<template>
  <div class="min-h-screen bg-slate-950 text-slate-100 flex flex-col">
    <!-- Navigation -->
    <nav v-if="auth.isAuthenticated && !route.meta.hideLayout"
      class="glass-card !rounded-none !border-x-0 !border-t-0 p-4 sticky top-0 z-40">
      <div class="max-w-6xl mx-auto flex items-center justify-between">
        <div class="flex items-center gap-2 cursor-pointer" @click="router.push('/rooms')">
          <div class="h-8 w-8 rounded-lg flex items-center justify-center font-bold">
            <img src="/logo_blue.png" alt="Logo">
          </div>
          <span class="text-xl font-bold tracking-tight">Zupkonerzy</span>
        </div>

        <div class="flex items-center gap-6">
          <div class="hidden md:flex flex-col items-end">
            <span class="text-sm font-medium">{{ auth.user?.username }}</span>
            <span class="text-xs text-slate-500">{{ auth.user?.email }}</span>
          </div>

          <button @click="handleLogout"
            class="px-4 py-2 text-sm font-medium text-slate-400 hover:text-white hover:bg-white/5 rounded-lg transition-all">
            Logout
          </button>
        </div>
      </div>
    </nav>

    <!-- Main Content -->
    <main class="flex-1">
      <RouterView />
    </main>

    <!-- Footer -->
    <footer v-if="!route.meta.hideLayout" class="p-8 text-center text-slate-600 text-sm">
      &copy; 2026 Zupkonerzy; Powered by UnkoR00T.com
    </footer>
  </div>
</template>

<style>
/* Global transitions */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
