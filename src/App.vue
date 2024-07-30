<template>
  <FileSelector />
  <div class="mt-8 md:flex">
    <FileProperties class="md:w-2/3" />
    <CoverEditor class="md:w-1/3" img-src="./../example.jpg" />
  </div>
</template>

<script setup lang="ts">
import FileSelector from './components/FileSelector.vue'
import FileProperties from './components/FileProperties.vue';
import CoverEditor from './components/CoverEditor.vue';

</script>

<style scoped>

</style>
