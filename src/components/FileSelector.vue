<template>
  <div class="flex">
    <input type="text" v-model="link" placeholder="YouTube url" class="w-full px-4 rounded-md"/>
    <button @click="download" class="ml-4">Download</button>
    <FileSelect
      v-model="filename"
      text="Select File"
      filterName="MP3 files"
      :extensions="['mp3']"
      @file-selected="openFile"
      class="ml-4"
    />
    
  </div>
</template>

<script setup lang="ts">
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api';
import { ref } from 'vue';;
import FileSelect from './input/FileInput.vue';

const link = ref('')
const filename = ref<string>()


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

const openFile = async (fn: string) => {
  filename.value = fn;
  console.log('Editing file: ', filename.value)
}


</script>

<style scoped>

</style>
