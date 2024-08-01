<template>
  <div class="flex">
    <input type="text" v-model="link" placeholder="YouTube url" class="w-full px-4 rounded-md"/>
    <button @click="download" class="ml-4">Download</button>
    <button @click="file" class="ml-4">SelectFile</button>
  </div>
</template>

<script setup lang="ts">
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api';
import { ref } from 'vue';

const link = ref('')



const download = async () => {
  console.log('Downloading: ', link.value)
  invoke('download', { link: link.value })
  .then((response) => console.log(response))

  const unlisten = await listen('ytdlp', (event) => {
  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
  // event.payload is the payload object
    console.log('Received event:', event.event, event.payload)
  })
  // TODO download & open with file()
}

const file = () => {
  console.log('Editing file: ', link.value)
  // TODO open file
}


</script>

<style scoped>

</style>
