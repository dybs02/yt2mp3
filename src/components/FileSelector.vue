<template>
  <div class="flex">
    <input type="text" v-model="link" placeholder="YouTube url" class="w-full px-4 rounded-md"/>
    <button @click="download" class="ml-4">Download</button>
    <FileSelect
      v-model="file"
      text="Select File"
      accepted_types=".mp3"
      @change="openFile"
      class="ml-4"
    />
    
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import FileSelect from './input/FileInput.vue';

const link = ref('')
const file = ref<File>()

const download = () => {
  console.log('Downloading: ', link.value)
  // TODO download & open with file()
}

const openFile = () => {
  if (!file.value) {
    return
  }

  console.log('Editing file: ', file.value)
  // TODO open file
}


</script>

<style scoped>

</style>
