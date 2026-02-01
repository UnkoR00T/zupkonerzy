<script setup lang="ts">
import QuestionPreview from '@/components/QuestionPreview.vue';
import { useWebSocketStore } from '@/stores/websocket';
import { onBeforeMount } from 'vue';
import { useRoute } from 'vue-router';


const route = useRoute();
const roomId = route.params.id as string;
const socket = useWebSocketStore();

onBeforeMount(() => {
  socket.connect(roomId);
});


</script>

<template>
  <main class="h-[100dvh] w-[100dvw] flex items-center justify-center" v-if="socket.gameState.started">
    <QuestionPreview :question="socket.gameState.question" />
  </main>
  <main v-else class="h-[100dvh] w-[100dvw] flex flex-col gap-2 items-center justify-center">
    <h1 class="text-6xl font-bold">Zupkonerzy</h1>
    <p class="text-lg text-gray-600">Powered by UnkoR00T.com</p>
  </main>
</template>
