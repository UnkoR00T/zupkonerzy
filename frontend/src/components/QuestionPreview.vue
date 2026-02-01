<script setup lang="ts">
import { computed } from 'vue'

interface Question {
  id?: string
  question: string
  img_url: string | null
  video_url: string | null
  answers: string[]
  correct: number
  difficulty: number | null
}

const props = defineProps<{
  question: Question,
  marked?: number,
}>()

const letters = ['A', 'B', 'C', 'D']
</script>

<template>
  <div class="millionaire-preview w-full h-full flex flex-col items-center justify-center p-8 select-none font-sans">

    <div class="w-full max-w-4xl mb-8 relative">
      <div class="game-box question-box text-center p-6 min-h-[120px] flex items-center justify-center">
        <hr class="decoration-line left" />
        <hr class="decoration-line right" />
        <h2 class="text-2xl md:text-3xl font-bold text-white relative z-10">{{ question.question }}</h2>
      </div>
    </div>

    <div v-if="question.img_url || question.video_url"
      class="mb-8 max-h-[300px] max-w-[500px] overflow-hidden rounded-lg border-2 border-[#d4af37] shadow-[0_0_15px_#d4af37]">
      <img v-if="question.img_url" :src="question.img_url" class="object-contain w-full h-full" />
      <video v-if="question.video_url" :src="question.video_url" controls class="w-full h-full"></video>
    </div>

    <div class="w-full max-w-4xl grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-6">
      <div v-for="(answer, idx) in question.answers" :key="idx"
        class="game-box answer-box flex items-center p-4 relative cursor-default"
        :class="{ 'correct-answer': idx === question.correct, 'marked-answer': idx === marked }">
        <hr class="decoration-line left" />
        <hr class="decoration-line right" />
        <span class="text-[#d4af37] font-bold mr-4 text-xl">{{ letters[idx] }}:</span>
        <span class="text-white text-lg md:text-xl font-semibold">{{ answer }}</span>
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
