<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { request } from '@/api/api'

const route = useRoute()
const router = useRouter()
const auth = useAuthStore()

const roomId = route.params.id as string

interface Question {
  id?: string
  question: string
  img_url: string | null
  video_url: string | null
  answers: string[]
  correct: number
}

const questions = ref<Question[]>([])
const loading = ref(true)
const isEditing = ref(false)
const editedQuestion = ref<Question>({
  question: '',
  img_url: null,
  video_url: null,
  answers: ['', '', '', ''],
  correct: 0
})

async function fetchQuestions() {
  loading.value = true
  try {
    const data = await request<Question[]>(`/rooms/${roomId}/questions`, 'GET', null, auth.token!)
    questions.value = data
  } catch (e: any) {
    console.error(e)
  } finally {
    loading.value = false
  }
}

async function saveQuestion() {
  try {
    if (editedQuestion.value.id) {
      await request(`/rooms/${roomId}/questions/${editedQuestion.value.id}`, 'PUT', editedQuestion.value, auth.token!)
    } else {
      await request(`/rooms/${roomId}/questions`, 'POST', editedQuestion.value, auth.token!)
    }
    fetchQuestions()
    resetForm()
  } catch (e: any) {
    alert(e.message)
  }
}

async function deleteQuestion(id: string) {
  if (!confirm('Delete this question?')) return
  try {
    await request(`/rooms/${roomId}/questions/${id}`, 'DELETE', null, auth.token!)
    questions.value = questions.value.filter(q => q.id !== id)
  } catch (e: any) {
    alert(e.message)
  }
}

function openEdit(q?: Question) {
  if (q) {
    editedQuestion.value = JSON.parse(JSON.stringify(q))
  } else {
    resetForm()
  }
  isEditing.value = true
}

function resetForm() {
  editedQuestion.value = {
    question: '',
    img_url: null,
    video_url: null,
    answers: ['', '', '', ''],
    correct: 0
  }
  isEditing.value = false
}

onMounted(fetchQuestions)
</script>

<template>
  <div class="max-w-4xl mx-auto p-6 space-y-8 animate-in fade-in slide-in-from-bottom-4 duration-700">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-4">
        <button @click="router.push('/rooms')" class="p-2 hover:bg-white/5 rounded-full transition-colors text-slate-400 hover:text-white">
          <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
          </svg>
        </button>
        <div>
          <h1 class="text-3xl font-bold text-white">Manage Questions</h1>
          <p class="text-slate-400">Add or edit questions for this room</p>
        </div>
      </div>
      <button @click="openEdit()" class="btn-primary">Add Question</button>
    </div>

    <!-- Edit Modal / Form -->
    <div v-if="isEditing" class="fixed inset-0 z-50 flex items-center justify-center p-6 bg-slate-950/80 backdrop-blur-sm">
      <div class="glass-card w-full max-w-2xl p-8 max-h-[90vh] overflow-y-auto">
        <h2 class="text-2xl font-bold text-white mb-6">{{ editedQuestion.id ? 'Edit Question' : 'New Question' }}</h2>
        <form @submit.prevent="saveQuestion" class="space-y-6">
          <div class="space-y-2">
            <label class="text-sm font-medium text-slate-300">Question Text</label>
            <input v-model="editedQuestion.question" required class="input-field" placeholder="What is the capital of France?" />
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="space-y-2">
              <label class="text-sm font-medium text-slate-300">Image URL (Optional)</label>
              <input v-model="editedQuestion.img_url" class="input-field" placeholder="https://..." />
            </div>
            <div class="space-y-2">
              <label class="text-sm font-medium text-slate-300">Video URL (Optional)</label>
              <input v-model="editedQuestion.video_url" class="input-field" placeholder="https://..." />
            </div>
          </div>

          <div class="space-y-4">
            <label class="text-sm font-medium text-slate-300">Answers</label>
            <div v-for="(answer, index) in editedQuestion.answers" :key="index" class="flex items-center gap-4">
              <input
                type="radio"
                :value="index"
                v-model="editedQuestion.correct"
                name="correct-answer"
                class="w-5 h-5 text-brand bg-white/5 border-white/10"
              />
              <input
                v-model="editedQuestion.answers[index]"
                required
                class="input-field"
                :placeholder="'Answer ' + (index + 1)"
              />
            </div>
          </div>

          <div class="flex justify-end gap-4 pt-4">
            <button type="button" @click="resetForm()" class="px-6 py-2 text-slate-400 hover:text-white transition-colors">Cancel</button>
            <button type="submit" class="btn-primary">Save Question</button>
          </div>
        </form>
      </div>
    </div>

    <div v-if="loading" class="flex justify-center py-20">
      <div class="animate-pulse flex flex-col items-center">
        <div class="h-12 w-12 bg-brand/20 rounded-full mb-4"></div>
        <p class="text-slate-500">Loading questions...</p>
      </div>
    </div>

    <div v-else class="space-y-4">
      <div
        v-for="(q, idx) in questions"
        :key="q.id"
        class="glass-card p-6 flex items-start justify-between gap-4 group"
      >
        <div class="space-y-4 flex-1">
          <div class="flex items-center gap-2">
            <span class="text-brand font-bold">#{{ idx + 1 }}</span>
            <h3 class="text-lg font-medium text-white">{{ q.question }}</h3>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
            <div
              v-for="(ans, aIdx) in q.answers"
              :key="aIdx"
              class="px-4 py-2 rounded-lg text-sm border"
              :class="aIdx === q.correct ? 'bg-green-500/10 border-green-500/50 text-green-400' : 'bg-white/5 border-white/10 text-slate-400'"
            >
              {{ ans }}
            </div>
          </div>
        </div>

        <div class="flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
          <button @click="openEdit(q)" class="p-2 hover:bg-brand/10 text-brand rounded-lg transition-all">
            <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
            </svg>
          </button>
          <button @click="deleteQuestion(q.id!)" class="p-2 hover:bg-red-500/10 text-red-500 rounded-lg transition-all">
            <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
