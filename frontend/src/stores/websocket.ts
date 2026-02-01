import { defineStore } from 'pinia'
import { useAuthStore } from './auth'
import { ref } from 'vue'

export const useWebSocketStore = defineStore('websocket', () => {
  const auth = useAuthStore()

  const socket = ref<WebSocket | null>(null)
  const currentRoomId = ref<string | null>(null)
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
    marked: number
  }>({
    started: false,
    question: {
      id: '',
      question: '',
      img_url: '',
      video_url: '',
      answers: [],
      correct: -1,
      difficulty: 0,
    },
    marked: -1,
  })

  const connect = (roomId: string) => {
    // If already connected to this room and socket is open/connecting, do nothing
    if (
      currentRoomId.value === roomId &&
      socket.value &&
      (socket.value.readyState === WebSocket.OPEN ||
        socket.value.readyState === WebSocket.CONNECTING)
    ) {
      return
    }

    // Close existing connection if any
    if (socket.value) {
      socket.value.close()
    }

    currentRoomId.value = roomId
    const url = `ws://localhost:8000/?token=${auth.token}&room=${roomId}`
    socket.value = new WebSocket(url)

    socket.value.onmessage = (event) => {
      console.log(event.data)
      try {
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
        } else if (message.type == 'MarkQuestion') {
          const data = message.data as unknown as number
          gameState.value.marked = data
        } else if (message.type == 'FinalAnswer') {
          const data = message.data as unknown as {
            correct: number
            marked: number
          }
          gameState.value.question.correct = data.correct
          gameState.value.marked = data.marked
        }
      } catch (e) {
        console.error('Failed to parse websocket message', e)
      }
    }

    socket.value.onclose = () => {
      // Only clear if this was the socket we were tracking
      // (Is this check needed? socket.value might have changed if we reconnected fast)
      // For simplicity, maybe just leave it provided we handle start cleanly
    }
  }

  const sendMessage = (message: { type: string; data: Record<string, unknown> }) => {
    if (socket.value && socket.value.readyState === WebSocket.OPEN) {
      socket.value.send(JSON.stringify(message))
    } else {
      console.warn('WebSocket is not open. Cannot send message:', message)
    }
  }

  return {
    socket,
    gameState,
    connect,
    sendMessage,
  }
})
