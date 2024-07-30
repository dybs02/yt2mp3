<template>
  <div class="md:mr-12">
    <h2 class="text-2xl font-semibold my-4">File Properties</h2>

    <TextInput property="title" />
    <TextInput property="artist" />
    <TextInput property="album" />
    <TextInput property="year" />

    <div class="flex">
      <div class="flex-auto">
        <TextInput property="start time" />
      </div>
      <div class="flex-auto">
        <TextInput property="end time" />
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
import TextInput from './TextInput.vue';

</script>

<style scoped>

</style>
