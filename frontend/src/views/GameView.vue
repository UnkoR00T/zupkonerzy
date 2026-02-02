<script setup lang="ts">
import Ladder from '@/components/Ladder.vue';
import QuestionPreview from '@/components/QuestionPreview.vue';
import { useWebSocketStore } from '@/stores/websocket';
import { onBeforeMount, onMounted, onUnmounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';


const route = useRoute();
const router = useRouter();
const roomId = route.params.id as string;
const socket = useWebSocketStore();

onBeforeMount(() => {
  socket.connect(roomId);
});

const handleKeydown = (e: KeyboardEvent) => {
  if (e.key === 'c') {
    router.push(`/rooms/${roomId}/game/controller`);
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})


</script>

<template>
  <main class="h-[100dvh] w-[100dvw] flex items-center justify-center" v-if="socket.gameState.started">
    <QuestionPreview :question="socket.gameState.question" :marked="socket.gameState.marked"
      v-if="!socket.gameState.ladder" />
    <Ladder v-else />
  </main>
  <main v-else class="h-[100dvh] w-[100dvw] flex flex-col gap-2 items-center justify-center">
    <img src="/logo_blue.png" alt="Logo" class="w-32">
    <h1 class="text-6xl font-bold">Zupkonerzy</h1>
    <p class="text-lg text-gray-600">Powered by UnkoR00T.com</p>
  </main>
</template>
