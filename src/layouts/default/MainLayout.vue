<template>
  <v-main class="d-flex align-center justify-center" style="min-height: 300px;">
    <v-list>
      <v-list-item 
        v-for="image in selectedImagesStore.images" 
        :key="image.absolute_path"
      >
        <v-list-item-title>{{ image.absolute_path }}</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-main>
</template>

<script setup lang="ts">
import { useSelectedImagesStore } from '../../stores/selectedImagesStore';

const selectedImagesStore = useSelectedImagesStore();
</script>