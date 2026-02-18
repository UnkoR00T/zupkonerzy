<script setup lang="ts">
import { useBluetoothStore } from '../stores/bluetooth';
const bluetoothStore = useBluetoothStore();
</script>

<template>
  <div class="popover absolute top-0 left-0 w-full h-full flex items-center justify-center z-100 bg-black/50" v-if="!bluetoothStore.isConnected">
    <button @click="bluetoothStore.connect()" class="bg-white text-black p-2 rounded">Connect</button>
  </div>
</template>
