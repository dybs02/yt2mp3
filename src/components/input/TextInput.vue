<template>
  <div class="my-2">
    <label :for=property class="px-3 pb-2 rounded-md capitalize bg-color">{{ property }}</label>
    <div>
      <input type="text" :id=property class="w-full"/>
    </div>
  </div>
</template>

<script setup lang="ts">

defineProps({
  property: {
    type: String,
    required: true
  }
})


</script>

<style scoped>

</style>
