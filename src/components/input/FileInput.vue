<template>
  <div>
    <button>
      <label class="whitespace-nowrap">
        <input
          ref="input"
          type="file"
          :accept=accepted_types
          @change="loadFile($event)"
          class="file-input"
        />
        {{ text }}
      </label>
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref, defineProps } from 'vue'

defineProps({
  text: {
    type: String,
    required: true
  },
  accepted_types: {
    type: String,
    required: true
  }
})

const input = ref<any>(null);
const fileModel = defineModel()

const loadFile = (e: Event) => {
  const target = e.target as HTMLInputElement;

  const file = target.files?.item(0);
  if (file) {
    fileModel.value = file;
  }
}


</script>

<style scoped>

.file-input {
  display: none;
}

</style>
