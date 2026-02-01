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



</script>

<template>
  <main class="min-h-[25dvh] w-[100dvw] flex items-center justify-center resize-y">
    <QuestionPreview :question="socket.gameState.question" :marked="socket.gameState.marked"
      class="resize-y min-h-[25dvh]" />
  </main>
  <main class="mt-5 w-[100dvw] flex flex-col gap-2 items-center justify-center">
    <div class="flex gap-2">
      <button class="btn-primary bg-green-800" @click="startGame()">Start</button>
      <button class="btn-primary bg-red-800" @click="rerollQuestion()">Reroll</button>
      <button class="btn-primary bg-blue-800" @click="nextQuestion()">Next</button>
    </div>
    <div class="grid grid-cols-2 grid-rows-2 gap-2">
      <button class="btn-primary bg-green-500 w-25 h-25" @click="answer(0)">A</button>
      <button class="btn-primary bg-red-500" @click="answer(1)">B</button>
      <button class="btn-primary bg-blue-500" @click="answer(2)">C</button>
      <button class="btn-primary bg-yellow-500" @click="answer(3)">D</button>
    </div>
  </main>
</template>
