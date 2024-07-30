<template>
  <div class="mt-4 md:mt-0">
    <h2 class="text-2xl font-semibold my-4">Cover Image</h2>

    <div class="p-2 rounded-md bg-color">
      <vue-cropper
        ref="cropper"
        :key="time"
        :src="imgSrc"
        :aspect-ratio="aspectRatio ? 1 : NaN"
        alt="Source Image"
        @ready="ready"
      >
      </vue-cropper>
    </div>

    <div class="bg-color w-fit my-2 px-2 rounded-md">
      <input type="checkbox" id="aspect-ratio" v-model="aspectRatio" @change="refreshCropper">
      <label for="aspect-ratio"> 1:1 Aspect Ratio</label>
    </div>

  </div>
</template>

<script setup lang="ts">
import 'cropperjs/dist/cropper.css';
import { defineProps, ref } from 'vue';
import VueCropper from 'vue-cropperjs';


const time = ref(Date.now());
const aspectRatio = ref(true);

defineProps({
  imgSrc: {
    type: String,
    required: true
  }
})


const refreshCropper = () => {
  console.log(aspectRatio);
  time.value = Date.now();
}

const ready = () => {
  console.log('ready');
}


</script>

<style scoped>

</style>
