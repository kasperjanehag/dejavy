<template>
  <v-main>
    <v-pagination v-model="currentPage" :length="totalPages"></v-pagination>
    <v-container>
      <v-row v-for="(duplicateSet, index) in paginatedDuplicates" :key="index" :class="index % 2 === 0 ? 'bg-grey darken-1' : 'bg-grey lighten-1'">
        <v-col v-for="image in duplicateSet" :key="image" cols="3" class="d-flex child-flex">
          <v-img :src="imageData[image]" aspect-ratio="1" cover class="bg-grey-lighten-2">
            <template v-slot:placeholder>
              <v-row class="fill-height ma-0" align="center" justify="center">
                <v-progress-circular indeterminate color="grey-lighten-5"></v-progress-circular>
              </v-row>
            </template>
          </v-img>
        </v-col>
      </v-row>
    </v-container>
  </v-main>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { useIdentifiedDuplicatesStore } from '../../stores/identifiedDuplicatesStore';
import { invoke } from '@tauri-apps/api/tauri';

const identifiedDuplicatesStore = useIdentifiedDuplicatesStore();
const imageData = ref<Record<string, string>>({});
const itemsPerPage = ref(3);
const currentPage = ref(1);

const paginatedDuplicates = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage.value;
  const end = start + itemsPerPage.value;
  return Object.values(identifiedDuplicatesStore.$state.duplicates).slice(start, end);
});

const totalPages = computed(() => Math.ceil(Object.keys(identifiedDuplicatesStore.$state.duplicates).length / itemsPerPage.value));

watch([() => currentPage.value, () => identifiedDuplicatesStore.$state.duplicates], async (_) => {
  const imagePromises = [];
  for (const duplicateSet of paginatedDuplicates.value) {
    for (const image of duplicateSet) {
      if (!imageData.value[image]) {
        imagePromises.push(loadImage(image));
      }
    }
  }
  const results = await Promise.all(imagePromises);
  results.forEach(({image, data}) => {
    imageData.value[image] = data;
  });
}, { immediate: true });

async function loadImage(imagePath: string) {
  const data = await invoke('get_image_data', { absolutePath: imagePath });
  return { image: imagePath, data: `data:image/png;base64,${data}` };
}

</script>