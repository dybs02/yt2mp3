<template>
  <div class="mt-4 md:mt-0">
    <h2 class="text-2xl font-semibold my-4">Cover Image</h2>
    <div>
      <input
        ref="input"
        type="file"
        accept="image/*"
        @change="loadFile($event)"
        class="w-full mb-2"
      />
    </div>

    <div v-if="isFileSelected">
      <div class="p-2 rounded-md bg-color">
        <vue-cropper
          ref="cropper"
          :src="null"
          :aspect-ratio="aspectRatio ? 1 : NaN"
          alt="Cover Image"
          @ready="ready"
        >
        </vue-cropper>
      </div>

      <div class="bg-color w-fit my-2 px-2 rounded-md">
        <input type="checkbox" id="aspect-ratio" v-model="aspectRatio" @change="updateAspectRatio">
        <label for="aspect-ratio"> 1:1 Aspect Ratio</label>
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
import 'cropperjs/dist/cropper.css';
import { nextTick, ref } from 'vue';
import VueCropper from 'vue-cropperjs';


const cropper = ref<any>(null);
const isFileSelected = ref(false);
const aspectRatio = ref(true);


const updateAspectRatio = () => {
  cropper.value.setAspectRatio(aspectRatio.value ? 1 : NaN);
}

const ready = () => {
  console.log('ready');
}

const loadFile = (e: Event) => {
  const target = e.target as HTMLInputElement;

  const file = target.files?.item(0);
  if (file) {
    const reader = new FileReader();
    reader.onload = async (e) => {
      isFileSelected.value = true;
      await nextTick()
      cropper.value.replace(e.target?.result as string);
    }
    reader.readAsDataURL(file);
  }
}


</script>

<style scoped>

</style>
