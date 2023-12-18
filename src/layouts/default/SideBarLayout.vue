<template>
  <v-navigation-drawer dark width="450">

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
      <v-row>
        <v-col cols="12">
          <v-card class="d-flex align-center" flat tile>
            <v-card-actions class="flex-grow-1 rounded-l">
              <v-btn color="secondary" @click="identifyDuplicates" block rounded="xl" class="subtitle-1">Identify Duplicates</v-btn>
            </v-card-actions>
            <v-card-text class="flex-grow-1 rounded-r d-flex align-center justify-center subtitle-1">
              <span>{{ displayedDirectory }}</span>
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>
    </v-container>

    <v-container fluid>
      <v-row>
        <v-col cols="12">
          <v-card class="d-flex align-center" flat tile>
            <v-card-actions class="flex-grow-1 rounded-l">
              <v-btn color="secondary" @click="refreshFiles" block rounded="xl" class="subtitle-1">Refresh Files</v-btn>
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
                <v-btn class="equal-width" :color="activeExtensions.includes(ext) ? 'primary' : ''" @click="toggleFileExtension(ext)">
                  <v-icon left>mdi-file-image</v-icon>
                  {{ ext }}
                </v-btn>
              </v-col>
            </v-row>
            <v-row>
              <v-col v-for="ext in ['bmp', 'tiff', 'ico', 'webp']" :key="ext">
                <v-btn class="equal-width" :color="activeExtensions.includes(ext) ? 'primary' : ''" @click="toggleFileExtension(ext)">
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
import { ref, computed, defineComponent } from 'vue';
import { open } from '@tauri-apps/api/dialog';
import { useIdentifiedDuplicatesStore } from '../../stores/identifiedDuplicatesStore';
import { useSelectedDirectoryStore } from '../../stores/selectedDirectoryStore';
import { invoke } from "@tauri-apps/api/tauri"; 
import NestedList from '../../components/NestedList.vue';


const identifiedDuplicatesStore = useIdentifiedDuplicatesStore();
const selectedDirectoryStore = useSelectedDirectoryStore();

const extensions = ['png', 'jpeg', 'jpg', 'gif', 'bmp', 'tiff', 'ico', 'webp'];
const activeExtensions = ref<string[]>(extensions);
const selectedDirectory = ref('');

const displayedDirectory = computed(() => {
  if (selectedDirectoryStore.selectedDirectory) { // use the selectedDirectory from the store
    let parts = selectedDirectoryStore.selectedDirectory.split('/');
    let lastThreeParts = parts.slice(-3).join('/');
    return lastThreeParts.length > 20 ? '...' + lastThreeParts.slice(-20) : '...' + lastThreeParts;
  } else {
    return "No path selected ...";
  }
});

const toggleFileExtension = (ext: string) => {
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
      const directory = Array.isArray(selected) ? selected[0] : selected;
      selectedDirectoryStore.setSelectedDirectory(directory); // use the store action
    }
  } catch (error) {
    console.error('Error selecting directory:', error);
  }
};

const identifyDuplicates = async () => {
  if (!selectedDirectoryStore.selectedDirectory) {
    console.log('No directory selected');
    return;
  }

  try {
    const md5Duplicates: Record<string, string[]> = await invoke('get_md5_duplicates');
    identifiedDuplicatesStore.clearDuplicates();
    
    for (const [md5Hash, duplicateSet] of Object.entries(md5Duplicates)) {
      identifiedDuplicatesStore.addDuplicateGroup(md5Hash, duplicateSet);
    }

  } catch (error) {
    console.error('Error identifying duplicates:', error);
  }
  
};

const refreshFiles = async () => {
  if (!selectedDirectoryStore.selectedDirectory) {
    console.log('No directory selected');
    return;
  }

  try {
    // Invoke the backend function to refresh the image cache
    await invoke('refresh_image_cache', { path: selectedDirectoryStore.selectedDirectory });

    // Recalculate the duplicates
    await identifyDuplicates();
  } catch (error) {
    console.error('Error refreshing files:', error);
  }
};

defineComponent({
  components: {
    NestedList
  },
});

</script>

<style scoped>
.equal-width {
  width: 80px; /* Adjust this value to your needs */
}

</style>