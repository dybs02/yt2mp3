<template>
  <div class="flex">
    <input type="text" v-model="link" placeholder="YouTube url" class="w-full px-4 rounded-md"/>
    <button @click="download" class="ml-4">Download</button>
    <button @click="file" class="ml-4">SelectFile</button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const link = ref('')

const download = () => {
  console.log('Downloading: ', link.value)
  // TODO download & open with file()
}

const file = () => {
  console.log('Editing file: ', link.value)
  // TODO open file
}


</script>

<style scoped>

</style>
