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
import { ref } from 'vue';
import FileSelect from './input/FileInput.vue';

const link = ref('')
const filename = ref<string>()

const download = () => {
  console.log('Downloading: ', link.value)
  // TODO download & open with file()
}

const openFile = async (fn: string) => {
  filename.value = fn;
  console.log('Editing file: ', filename.value)
}


</script>

<style scoped>

</style>
