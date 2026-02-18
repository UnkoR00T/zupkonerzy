import { defineStore } from 'pinia'
import { useAuthStore } from './auth'
import { ref } from 'vue'
import { useBluetoothStore } from './bluetooth'

export const useWebSocketStore = defineStore('websocket', () => {
  const auth = useAuthStore();
  const bluetooth = useBluetoothStore();

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
    ladder: boolean
    question_number: number
    helpers: boolean[]
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
    ladder: true,
    question_number: 1,
    helpers: [true, true, true],
  })

  const connect = (roomId: string) => {
    if (
      currentRoomId.value === roomId &&
      socket.value &&
      (socket.value.readyState === WebSocket.OPEN ||
        socket.value.readyState === WebSocket.CONNECTING)
    ) {
      return
    }

    if (socket.value) {
      socket.value.close()
    }

    currentRoomId.value = roomId
    const url = import.meta.env.DEV
      ? `ws://localhost:8000/?token=${auth.token}&room=${roomId}`
      : `wss://zupkonerzy.unkor00t.com/wss?token=${auth.token}&room=${roomId}`
    socket.value = new WebSocket(url)

    socket.value.onmessage = (event) => {
      console.log(event.data)
      try {
        const message: { type: string; data: Record<string, string> } = JSON.parse(event.data)
        if (message.type == 'GameStarted') {
          gameState.value.started = true
        } else if (message.type == 'Question') {
          const data = message.data.question as unknown as {
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
          gameState.value.marked = -1
          gameState.value.question_number = message.data.question_number as unknown as number
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
          if(data.marked == data.correct) {
            bluetooth.pulse({r: 0, g: 255, b: 0}, 3000);
          } else {
            bluetooth.pulse({r: 255, g: 0, b: 0}, 3000);
          }
        } else if (message.type == 'SwitchLadder') {
          const data = message.data as unknown as boolean
          gameState.value.ladder = data
        } else if (message.type == 'HelperUsed') {
          const data = message.data as unknown as {
            remove: number[]
            helper: number
          }
          gameState.value.helpers[data.helper] = false
          data.remove.forEach((index) => {
            gameState.value.question.answers[index] = ''
          })
          bluetooth.pulse({r: 255, g: 255, b: 0}, 3000); //zupkonerzy = good jiggle physics = 100%
        } else if (message.type == 'Helpers') {
          const data = message.data as unknown as boolean[]
          gameState.value.helpers = data
        }
      } catch (e) {
        console.error('Failed to parse websocket message', e)
      }
    }

    socket.value.onclose = () => {
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
