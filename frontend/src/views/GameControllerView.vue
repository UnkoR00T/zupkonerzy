<script setup lang="ts">
import QuestionPreview from '@/components/QuestionPreview.vue';
import { useWebSocketStore } from '@/stores/websocket';
import { onBeforeMount, onMounted, onUnmounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import Ladder from '@/components/Ladder.vue';


const route = useRoute();
const router = useRouter();
const roomId = route.params.id as string;
const socket = useWebSocketStore();

onBeforeMount(() => {
  socket.connect(roomId);
});

const startGame = () => {
  socket.sendMessage({ type: 'Start', data: {} });
};

const nextQuestion = () => {
  socket.sendMessage({ type: 'NextQuestion', data: {} });
};

const answer = (answer: number) => {
  socket.sendMessage({ type: 'AnswerQuestion', data: { answer: answer } });
};
const rerollQuestion = () => {
  socket.sendMessage({ type: 'RerollQuestion', data: {} });
};
const handleKeydown = (e: KeyboardEvent) => {
  if (e.key === 'c') {
    router.push(`/rooms/${roomId}/game`);
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
  <main class="min-h-[25dvh] w-[100dvw] flex items-center justify-center resize-y">
    <QuestionPreview :question="socket.gameState.question" :marked="socket.gameState.marked"
      :helpers="socket.gameState.helpers" class="resize-y min-h-[25dvh]" v-if="!socket.gameState.ladder" />
    <Ladder v-else />
  </main>
  <main class="mt-5 w-[100dvw] flex flex-col gap-2 items-center justify-center">
    <div class="flex gap-2">
      <button class="btn-primary" @click="startGame()" v-if="!socket.gameState.started">Start</button>
      <button class="btn-primary" @click="startGame()" v-if="socket.gameState.started">Restart</button>
      <button class="btn-primary" @click="rerollQuestion()" v-if="socket.gameState.started">Reroll</button>
      <button class="btn-primary" @click="nextQuestion()" v-if="socket.gameState.started">Next</button>
    </div>
    <div class="grid grid-cols-2 grid-rows-2 gap-2">
      <button class="btn-primary bg-green-500 w-25 h-25" @click="answer(0)">A</button>
      <button class="btn-primary bg-red-500" @click="answer(1)">B</button>
      <button class="btn-primary bg-blue-500" @click="answer(2)">C</button>
      <button class="btn-primary bg-yellow-500" @click="answer(3)">D</button>
    </div>
    <div class="flex gap-2">
      <button class="btn-primary" @click="socket.sendMessage({ type: 'UseHelper', data: { helper: 0 } })"
        :disabled="!socket.gameState.helpers[0]">50/50</button>
      <button class="btn-primary" @click="socket.sendMessage({ type: 'UseHelper', data: { helper: 1 } })"
        :disabled="!socket.gameState.helpers[1]">Phone a friend</button>
      <button class="btn-primary" @click="socket.sendMessage({ type: 'UseHelper', data: { helper: 2 } })"
        :disabled="!socket.gameState.helpers[2]">Ask audience</button>
    </div>
    <div class="flex">
      <button class="btn-primary bg-red-500" @click="socket.sendMessage({ type: 'ResetHelpers', data: {} })">Reset helpers</button>
    </div>
    <div>
      <button class="btn-primary" @click="socket.sendMessage({ type: 'SwitchLadder', data: {} })">Switch ladder</button>
    </div>
  </main>
</template>
