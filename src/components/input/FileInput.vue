<template>
  <div>
    <button @click="select" class="whitespace-nowrap">
      {{ text }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue'
import { open } from '@tauri-apps/api/dialog';

const props = defineProps({
  text: {
    type: String,
    required: true
  },
  filterName: {
    type: String,
    required: true
  },
  extensions: {
    type: Array<string>,
    required: true
  }
})

const emit = defineEmits<{
  (e: 'file-selected', filename: string): void
}>()

const select = async () => {
  const selected = await open({
    multiple: false,
    filters: [{
      name: props.filterName,
      extensions: props.extensions
    }]
  });

  if (selected === null || Array.isArray(selected)) {
    return;
  } else {
    emit('file-selected', selected);
  }
}


</script>

<style scoped>

</style>
