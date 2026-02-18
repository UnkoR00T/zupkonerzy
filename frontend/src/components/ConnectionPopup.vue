<script setup lang="ts">
import { useWebSocketStore } from '@/stores/websocket';

const socket = useWebSocketStore();
</script>

<template>
  <div v-if="!socket.connected" class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm">
      <div class="glass-card p-8 text-center animate-pulse">
          <h2 class="text-3xl font-bold text-white mb-2">Connecting...</h2>
          <p class="text-slate-300">Please wait while we reconnect to the server</p>
      </div>
  </div>
</template>
