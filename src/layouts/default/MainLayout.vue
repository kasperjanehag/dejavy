<template>
  <v-main>
    <v-container>
      <v-row v-for="(duplicateSet, md5Hash) in identifiedDuplicatesStore.$state.duplicates" :key="md5Hash">
        <v-col cols="3">
          <v-card-title>{{ md5Hash }}</v-card-title>
        </v-col>
        <v-col cols="9">
          <v-row>
            <v-col v-for="image in duplicateSet" :key="image" cols="4" class="d-flex child-flex">
              <v-img :src="imageData[image]" aspect-ratio="1" cover class="bg-grey-lighten-2">
                <template v-slot:placeholder>
                  <v-row class="fill-height ma-0" align="center" justify="center">
                    <v-progress-circular indeterminate color="grey-lighten-5"></v-progress-circular>
                  </v-row>
                </template>
              </v-img>
            </v-col>
          </v-row>
        </v-col>
      </v-row>
    </v-container>
  </v-main>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useIdentifiedDuplicatesStore } from '../../stores/identifiedDuplicatesStore';
import { invoke } from '@tauri-apps/api/tauri';

const identifiedDuplicatesStore = useIdentifiedDuplicatesStore();
const imageData = ref<Record<string, string>>({});

watch(() => identifiedDuplicatesStore.$state.duplicates, async (newDuplicates) => {
  for (const duplicateSet of Object.values(newDuplicates)) {
    for (const image of duplicateSet) {
      imageData.value[image] = await loadImage(image);
    }
  }
}, { immediate: true });

async function loadImage(imagePath: string) {
  const data = await invoke('get_image_data', { absolutePath: imagePath });
  return `data:image/png;base64,${data}`;
}
</script>