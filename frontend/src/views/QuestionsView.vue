<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { request } from '@/api/api'
import QuestionPreview from '@/components/QuestionPreview.vue'

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
  difficulty: number | null
  fun_fact?: string
}

interface QuestionsResponse {
  questions: Question[]
  total: number
  page: number
  limit: number
}

const questions = ref<Question[]>([])
const loading = ref(true)
const isEditing = ref(false)
const previewQuestion = ref<Question | null>(null)
const editedQuestion = ref<Question>({
  question: '',
  img_url: null,
  video_url: null,
  answers: ['', '', '', ''],
  correct: 0,
  difficulty: null,
  fun_fact: ''
})

// Filters and Pagination
const page = ref(1)
const limit = ref(50)
const search = ref('')
const difficulty = ref<number | ''>('')
const total = ref(0)
let searchTimeout: ReturnType<typeof setTimeout>

async function fetchQuestions() {
  loading.value = true
  try {
    const queryParams = new URLSearchParams()
    queryParams.append('page', page.value.toString())
    queryParams.append('limit', limit.value.toString())
    if (search.value) queryParams.append('search', search.value)
    if (difficulty.value !== '') queryParams.append('difficulty', difficulty.value.toString())

    const data = await request<QuestionsResponse>(`/rooms/${roomId}/questions?${queryParams.toString()}`, 'GET', null, auth.token!)
    // Handle both old and new response structure implicitly during migration if needed, but we expect new structure
    if ('questions' in data) {
      questions.value = data.questions
      total.value = data.total
    } else {
      // Fallback for safety if backend wasn't ready
      questions.value = data as unknown as Question[]
      total.value = questions.value.length
    }
  } catch (e: any) {
    console.error(e)
  } finally {
    loading.value = false
  }
}

// Debounced search
watch(search, () => {
  clearTimeout(searchTimeout)
  searchTimeout = setTimeout(() => {
    page.value = 1
    fetchQuestions()
  }, 300)
})

watch([difficulty, limit], () => {
  page.value = 1
  fetchQuestions()
})

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
    total.value -= 1
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
    correct: 0,
    difficulty: null,
    fun_fact: ''
  }
  isEditing.value = false
}

function nextPage() {
  if (page.value * limit.value < total.value) {
    page.value++
    fetchQuestions()
  }
}

function prevPage() {
  if (page.value > 1) {
    page.value--
    fetchQuestions()
  }
}

onMounted(fetchQuestions)
</script>

<template>
  <div class="max-w-4xl mx-auto p-6 space-y-8 animate-in fade-in slide-in-from-bottom-4 duration-700">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-4">
        <button @click="router.push('/rooms')"
          class="p-2 hover:bg-white/5 rounded-full transition-colors text-slate-400 hover:text-white">
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

    <!-- Filters -->
    <div class="flex flex-col md:flex-row gap-4">
      <div class="flex-1">
        <input v-model="search" class="input-field w-full" placeholder="Search questions..." />
      </div>
      <div class="w-full md:w-48">
        <select v-model="difficulty" class="input-field w-full">
          <option value="" class="text-black">All Difficulties</option>
          <option v-for="i in 10" :key="i" :value="i" class="text-black">Level {{ i }}</option>
        </select>
      </div>
    </div>

    <!-- Edit Modal / Form -->
    <div v-if="isEditing"
      class="fixed inset-0 z-50 flex items-center justify-center p-6 bg-slate-950/80 backdrop-blur-sm">
      <div class="glass-card w-full max-w-2xl p-8 max-h-[90vh] overflow-y-auto">
        <h2 class="text-2xl font-bold text-white mb-6">{{ editedQuestion.id ? 'Edit Question' : 'New Question' }}</h2>
        <form @submit.prevent="saveQuestion" class="space-y-6">
          <div class="space-y-2">
            <label class="text-sm font-medium text-slate-300">Question Text</label>
            <input v-model="editedQuestion.question" required class="input-field"
              placeholder="What is the capital of France?" />
          </div>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="space-y-2">
              <label class="text-sm font-medium text-slate-300">Difficulty</label>
              <input type="number" v-model.number="editedQuestion.difficulty" class="input-field" placeholder="1-10" />
            </div>
            <div class="space-y-2">
              <label class="text-sm font-medium text-slate-300">Image URL (Optional)</label>
              <input v-model="editedQuestion.img_url" class="input-field" placeholder="https://..." />
            </div>
            <div class="space-y-2">
              <label class="text-sm font-medium text-slate-300">Video URL (Optional)</label>
              <input v-model="editedQuestion.video_url" class="input-field" placeholder="https://..." />
            </div>
          </div>

          <div class="space-y-2">
            <label class="text-sm font-medium text-slate-300">Fun Fact (Optional)</label>
            <textarea v-model="editedQuestion.fun_fact" class="input-field min-h-[80px]"
              placeholder="Did you know..."></textarea>
          </div>

          <div class="space-y-4">
            <label class="text-sm font-medium text-slate-300">Answers</label>
            <div v-for="(answer, index) in editedQuestion.answers" :key="index" class="flex items-center gap-4">
              <input type="radio" :value="index" v-model="editedQuestion.correct" name="correct-answer"
                class="w-5 h-5 text-brand bg-white/5 border-white/10" />
              <input v-model="editedQuestion.answers[index]" required class="input-field"
                :placeholder="'Answer ' + (index + 1)" />
            </div>
          </div>

          <div class="flex justify-end gap-4 pt-4">
            <button type="button" @click="resetForm()"
              class="px-6 py-2 text-slate-400 hover:text-white transition-colors">Cancel</button>
            <button type="submit" class="btn-primary">Save Question</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Preview Modal -->
    <div v-if="previewQuestion"
      class="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/90 backdrop-blur-sm"
      @click.self="previewQuestion = null">
      <div class="w-full h-full max-w-7xl max-h-[90vh] p-4 flex flex-col">
        <div class="flex justify-end mb-4">
          <button @click="previewQuestion = null" class="text-white hover:text-brand transition-colors">
            <svg class="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
        <div class="flex-1 overflow-auto rounded-2xl relative">
          <QuestionPreview :question="previewQuestion" :helpers="[true, true, true]" />
        </div>
      </div>
    </div>

    <div v-if="loading" class="flex justify-center py-20">
      <div class="animate-pulse flex flex-col items-center">
        <div class="h-12 w-12 bg-brand/20 rounded-full mb-4"></div>
        <p class="text-slate-500">Loading questions...</p>
      </div>
    </div>

    <div v-else class="space-y-4">
      <div v-if="questions.length === 0" class="text-center py-10 text-slate-400">
        No questions found matching your criteria.
      </div>
      <div v-for="(q, idx) in questions" :key="q.id" class="glass-card p-6">
        <div class=" flex items-start justify-between gap-4 group mb-2">
          <div class="space-y-4 flex-1">
            <div class="flex items-center gap-2">
              <span class="text-brand font-bold">#{{ (page - 1) * limit + idx + 1 }}</span>
              <div class="flex-1">
                <div class="flex items-center gap-2 mb-1">
                  <span v-if="q.difficulty === null || q.difficulty === undefined"
                    class="text-xs font-medium px-2 py-0.5 rounded bg-yellow-500/10 text-yellow-500 border border-yellow-500/20">
                    Missing Difficulty
                  </span>
                  <span v-else class="text-xs font-medium px-2 py-0.5 rounded bg-slate-700 text-slate-300">
                    Lvl {{ q.difficulty }}
                  </span>
                </div>
                <h3 class="text-lg font-medium text-white">{{ q.question }}</h3>
              </div>
            </div>

          </div>

          <div class="flex gap-2 transition-opacity">
            <button @click="previewQuestion = q"
              class="p-2 hover:bg-blue-500/10 text-blue-400 rounded-lg transition-all" title="Preview">
              <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
              </svg>
            </button>
            <button @click="openEdit(q)" class="p-2 hover:bg-brand/10 text-brand rounded-lg transition-all">
              <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
              </svg>
            </button>
            <button @click="deleteQuestion(q.id!)"
              class="p-2 hover:bg-red-500/10 text-red-500 rounded-lg transition-all">
              <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
            </button>
          </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
          <div v-for="(ans, aIdx) in q.answers" :key="aIdx" class="px-4 py-2 rounded-lg text-sm border"
            :class="aIdx === q.correct ? 'bg-green-500/10 border-green-500/50 text-green-400' : 'bg-white/5 border-white/10 text-slate-400'">
            {{ ans }}
          </div>
        </div>
      </div>

      <!-- Pagination -->
      <div v-if="total > limit" class="flex items-center justify-between pt-4 border-t border-white/10">
        <div class="text-sm text-slate-400">
          Showing {{ (page - 1) * limit + 1 }} to {{ Math.min(page * limit, total) }} of {{ total }} results
        </div>
        <div class="flex gap-2">
          <button @click="prevPage" :disabled="page <= 1"
            class="px-4 py-2 rounded-lg bg-white/5 hover:bg-white/10 disabled:opacity-50 disabled:cursor-not-allowed transition-colors text-white">
            Previous
          </button>
          <button @click="nextPage" :disabled="page * limit >= total"
            class="px-4 py-2 rounded-lg bg-white/5 hover:bg-white/10 disabled:opacity-50 disabled:cursor-not-allowed transition-colors text-white">
            Next
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
