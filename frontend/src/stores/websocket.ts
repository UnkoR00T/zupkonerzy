import { defineStore } from 'pinia'
import { useAuthStore } from './auth'
import { ref } from 'vue'

export const useWebSocketStore = defineStore('websocket', () => {
  const auth = useAuthStore()

  const socket = ref<WebSocket | null>(null)
  const gameState = ref<{
    started: boolean
    question: {
      id: string
      question: string
      img_url: string
      video_url: string
      answers: string[]
      correct: number
      difficulty: number
    }
  }>({
    started: false,
    question: {
      id: '',
      question: '',
      img_url: '',
      video_url: '',
      answers: [],
      correct: 0,
      difficulty: 0,
    },
  })

  const connect = (roomId: string) => {
    const url = `ws://localhost:8000/?token=${auth.token}&room=${roomId}`
    socket.value = new WebSocket(url)
    socket.value.onmessage = (event) => {
      console.log(event.data)
      const message: { type: string; data: Record<string, string> } = JSON.parse(event.data)
      if (message.type == 'GameStarted') {
        gameState.value.started = true
      } else if (message.type == 'Question') {
        const data = message.data as unknown as {
          id: string
          question: string
          img_url: string
          video_url: string
          answers: string[]
          correct: number
          difficulty: number
        }
        gameState.value.question = {
          id: data.id,
          question: data.question,
          img_url: data.img_url,
          video_url: data.video_url,
          answers: data.answers,
          correct: data.correct,
          difficulty: data.difficulty,
        }
      }
    }
  }

  return {
    socket,
    gameState,
    connect,
  }
})
