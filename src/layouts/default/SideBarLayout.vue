<template>
  <v-navigation-drawer width="450">

    
    
    <v-container fluid>
      <v-row>
        <v-col cols="12">
          <v-card class="d-flex align-center" flat tile>
            <v-card-actions class="flex-grow-1 rounded-l">
              <v-btn color="primary" @click="selectDirectory" block rounded="xl" class="subtitle-1">Select Directory</v-btn>
            </v-card-actions>
            <v-card-text class="flex-grow-1 rounded-r d-flex align-center justify-center subtitle-1">
              <span>{{ displayedDirectory }}</span>
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
    
    <v-container fluid>
      <v-row justify="center" align="center">
        <v-col cols="auto">
          <div>
            <v-row>
              <v-col v-for="ext in ['png', 'jpeg', 'jpg', 'gif']" :key="ext">
                <v-btn class="equal-width" :color="activeExtensions.includes(ext) ? 'primary' : ''" @click="toggleExtension(ext)">
                  <v-icon left>mdi-file-image</v-icon>
                  {{ ext }}
                </v-btn>
              </v-col>
            </v-row>
            <v-row>
              <v-col v-for="ext in ['bmp', 'tiff', 'ico', 'webp']" :key="ext">
                <v-btn class="equal-width" :color="activeExtensions.includes(ext) ? 'primary' : ''" @click="toggleExtension(ext)">
                  <v-icon left>mdi-file-image</v-icon>
                  {{ ext }}
                </v-btn>
              </v-col>
            </v-row>
          </div>
        </v-col>
      </v-row>
    </v-container>
    
    <v-divider></v-divider>
    <nested-list/>
  </v-navigation-drawer>
</template>

<script setup lang="ts">
import { ref, computed, provide } from 'vue';
import { open } from '@tauri-apps/api/dialog';

const extensions = ['png', 'jpeg', 'jpg', 'gif', 'bmp', 'tiff', 'ico', 'webp'];
const activeExtensions = ref<string[]>(extensions);
const selectedDirectory = ref('');

const displayedDirectory = computed(() => {
  if (selectedDirectory.value) {
    let parts = selectedDirectory.value.split('/');
    let lastThreeParts = parts.slice(-3).join('/');
    return lastThreeParts.length > 20 ? '...' + lastThreeParts.slice(-20) : '...' + lastThreeParts;
  } else {
    return "No path selected ...";
  }
});

const toggleExtension = (ext: string) => {
  const index = activeExtensions.value.indexOf(ext);
  if (index >= 0) {
    activeExtensions.value.splice(index, 1);
  } else {
    activeExtensions.value.push(ext);
  }
};

const selectDirectory = async () => {
  try {
    const selected = await open({ directory: true, multiple: false });
    if (selected) {
      selectedDirectory.value = Array.isArray(selected) ? selected[0] : selected;
    }
  } catch (error) {
    console.error('Error selecting directory:', error);
  }
};

provide('selectedDirectory', selectedDirectory);

</script>

<script lang="ts">
import NestedList from '../../components/NestedList.vue';

export default {
  components: {
    NestedList
  },
};
</script>

<style scoped>
.equal-width {
  width: 80px; /* Adjust this value to your needs */
}


</style>