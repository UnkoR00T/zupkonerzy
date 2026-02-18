import { defineStore } from 'pinia'

export const useSoundStore = defineStore('sound', () => {
  const correctSound = new Audio('/sounds/correct.mp3');
  const wrongSound = new Audio('/sounds/wrong.mp3');

  const playCorrect = () => {
    correctSound.currentTime = 0;
    correctSound.play().catch(e => console.error("Error playing correct sound:", e));
  }

  const playWrong = () => {
    wrongSound.currentTime = 0;
    wrongSound.play().catch(e => console.error("Error playing wrong sound:", e));
  }

  return {
    playCorrect,
    playWrong
  }
})
