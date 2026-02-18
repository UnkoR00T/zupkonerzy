import { defineStore } from 'pinia'
import { useBluetooth } from '@vueuse/core'

const SERVICE_UUID = '4fafc201-1fb5-459e-8fcc-c5c9c331914b'
const CHARACTERISTIC_UUID = 'beb5483e-36e1-4688-b7f5-ea07361b26a8'

export const useBluetoothStore = defineStore('bluetooth', () => {
  const {
    isSupported,
    isConnected,
    device,
    requestDevice,
    server,
  } = useBluetooth({
    acceptAllDevices: true,
    optionalServices: [SERVICE_UUID],
  })

  async function connect() {
    try {
      await requestDevice();
    } catch (error) {
      console.error('Bluetooth connection failed:', error)
    }
  }
  async function sendColor(r: number, g: number, b: number) {

    const payload = JSON.stringify({
      type: 'SetColor',
      data: { r:r, g:g, b:b },
    })

    try {
      const service = await server.value.getPrimaryService(SERVICE_UUID)
      const characteristic = await service.getCharacteristic(CHARACTERISTIC_UUID)
      await characteristic.writeValue(new TextEncoder().encode(payload))
    } catch (error) {
      console.error('Failed to send color:', error)
    }
  }
  async function pulse(color: {r: number, g: number, b: number}, timeout: number) {
    sendColor(color.r, color.g, color.b);
    setTimeout(() => {
      sendColor(0, 0, 0);
    }, timeout);
  }

  return {
    isSupported,
    isConnected,
    device,
    server,
    connect,
    sendColor,
    pulse,
  }
})
