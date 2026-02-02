<script setup lang="ts">
import { useWebSocketStore } from '@/stores/websocket'

const gameState = useWebSocketStore()
</script>

<template>
  <div class="millionaire-preview w-full h-full flex flex-col items-center justify-center p-8 select-none font-sans">
    <div class="w-full max-w-4xl mb-8 relative flex flex-col gap-2">
      <div class="game-box question-box text-center p-3 min-h-[20px] flex items-center justify-center"
        :class="{ 'bg-yellow-500!': gameState.gameState.question_number == 10, 'bg-green-500!': gameState.gameState.question_number > 10 }">
        <h2 class="font-bold text-lg">10 Zupek</h2>
      </div>
      <div class="game-box question-box text-center p-3 min-h-[20px] flex items-center justify-center"
        :class="{ 'bg-yellow-500!': gameState.gameState.question_number == 9, 'bg-green-500!': gameState.gameState.question_number > 9 }">
        <h2 class="font-bold text-lg">9 Zupek</h2>
      </div>
      <div class="game-box question-box text-center p-3 min-h-[20px] flex items-center justify-center"
        :class="{ 'bg-yellow-500!': gameState.gameState.question_number == 8, 'bg-green-500!': gameState.gameState.question_number > 8 }">
        <h2 class="font-bold text-lg">8 Zupek</h2>
      </div>
      <div class="game-box question-box text-center p-3 min-h-[20px] flex items-center justify-center"
        :class="{ 'bg-yellow-500!': gameState.gameState.question_number == 7, 'bg-green-500!': gameState.gameState.question_number > 7 }">
        <h2 class="font-bold text-lg">7 Zupek</h2>
      </div>
      <div class="game-box question-box text-center p-3 min-h-[20px] flex items-center justify-center"
        :class="{ 'bg-yellow-500!': gameState.gameState.question_number == 6, 'bg-green-500!': gameState.gameState.question_number > 6 }">
        <h2 class="font-bold text-lg">6 Zupek</h2>
      </div>
      <div class="game-box question-box text-center p-3 min-h-[20px] flex items-center justify-center"
        :class="{ 'bg-yellow-500!': gameState.gameState.question_number == 5, 'bg-green-500!': gameState.gameState.question_number > 5 }">
        <h2 class="font-bold text-lg">5 Zupek</h2>
      </div>
      <div class="game-box question-box text-center p-3 min-h-[20px] flex items-center justify-center"
        :class="{ 'bg-yellow-500!': gameState.gameState.question_number == 4, 'bg-green-500!': gameState.gameState.question_number > 4 }">
        <h2 class="font-bold text-lg">4 Zupki</h2>
      </div>
      <div class="game-box question-box text-center p-3 min-h-[20px] flex items-center justify-center"
        :class="{ 'bg-yellow-500!': gameState.gameState.question_number == 3, 'bg-green-500!': gameState.gameState.question_number > 3 }">
        <h2 class="font-bold text-lg">3 Zupki</h2>
      </div>
      <div class="game-box question-box text-center p-3 min-h-[20px] flex items-center justify-center"
        :class="{ 'bg-yellow-500!': gameState.gameState.question_number == 2, 'bg-green-500!': gameState.gameState.question_number > 2 }">
        <h2 class="font-bold text-lg">2 Zupki</h2>
      </div>
      <div class="game-box question-box text-center p-3 min-h-[20px] flex items-center justify-center"
        :class="{ 'bg-yellow-500!': gameState.gameState.question_number == 1, 'bg-green-800!': gameState.gameState.question_number > 1 }">
        <h2 class="font-bold text-lg">1 Zupka</h2>
      </div>
    </div>
  </div>
</template>

<style scoped>
.millionaire-preview {
  background: radial-gradient(circle at center, #0f0c29, #302b63, #24243e);
  color: white;
  border-radius: 12px;
  overflow: hidden;
}

.game-box {
  background: linear-gradient(to bottom, #000046, #1cb5e0);
  background: black;
  background: linear-gradient(90deg, transparent 0%, #000000 15%, #000000 85%, transparent 100%);
  border-top: 2px solid #666;
  border-bottom: 2px solid #666;
  position: relative;
  border-radius: 40px;
  border: 1px solid #7a7a7a;
  box-shadow: inset 0 0 20px #000000;
}

.correct-answer {
  background: linear-gradient(90deg, transparent 0%, #00600f 15%, #00600f 100%) !important;
  border-color: #0f0;
  box-shadow: 0 0 10px #0f0, inset 0 0 20px #000;
}

.marked-answer {
  background: linear-gradient(90deg, rgb(255, 166, 1) 0%, rgb(255, 166, 1) 100%) !important;
  border-color: rgb(255, 166, 1);
  box-shadow: 0 0 10px rgb(255, 166, 1), inset 0 0 20px #000;
}

.decoration-line.left {
  left: 0;
  transform-origin: left;
  transform: rotate(45deg);
  display: none;
}

.question-box {
  border: 2px solid #d4af37;
  background: rgba(0, 0, 0, 0.8);
  border-radius: 16px;
  box-shadow: 0 0 10px rgba(212, 175, 55, 0.5);
}

.answer-box {
  border: 2px solid silver;
  border-radius: 30px;
  background: linear-gradient(180deg, #1a1a40 0%, #0d0d26 50%, #1a1a40 100%);
  transition: all 0.3s ease;
}

.answer-box:hover {
  border-color: #d4af37;
  box-shadow: 0 0 8px #d4af37;
}

.correct-answer {
  background: linear-gradient(180deg, #1E3A1E 0%, #2E5A2E 50%, #1E3A1E 100%) !important;
  border-color: #4CAF50 !important;
  box-shadow: 0 0 15px #4CAF50;
}
</style>
