<template>
  <v-main class="d-flex align-center justify-center" style="min-height: 300px;">
    <v-img
      v-for="(image, index) in selectedImagesStore.images"
      :key="image.absolute_path"
      :src="imageData[index]"
    ></v-img>
  </v-main>
</template>

<script setup lang="ts">
import { ref, watchEffect } from 'vue';
import { useSelectedImagesStore } from '../../stores/selectedImagesStore';
import { invoke } from '@tauri-apps/api/tauri';

const selectedImagesStore = useSelectedImagesStore();
const imageData = ref<string[]>([]);

watchEffect(async () => {
  imageData.value = await Promise.all(
    selectedImagesStore.images.map(image =>
      invoke('get_image_data', { absolutePath: image.absolute_path })
        .then(data => `data:image/png;base64,${data}`)
    )
  );
});
</script>