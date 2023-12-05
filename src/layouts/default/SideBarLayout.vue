<template>
  <v-navigation-drawer width="450">

    <v-list-item title="Image Formats"></v-list-item>
    <v-divider></v-divider>
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


    <v-list-item title="Image Explorer"></v-list-item>
    
    <v-container>
      <v-btn color="primary" @click="selectDirectory">Select Directory</v-btn>
      
      <v-dialog v-model="dialog" persistent max-width="600px">
        <v-card>
          <v-card-title>
            Selected Directory
          </v-card-title>
          <v-card-text>
            {{ selectedDirectory }}
          </v-card-text>
          <v-card-actions>
            <v-spacer></v-spacer>
            <v-btn color="green darken-1" text @click="dialog = false">Close</v-btn>
          </v-card-actions>
        </v-card>
      </v-dialog>
    </v-container>

    <v-divider></v-divider>
    <nested-list/>
  </v-navigation-drawer>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { open } from '@tauri-apps/api/dialog';

const extensions = ['png', 'jpeg', 'jpg', 'gif', 'bmp', 'tiff', 'ico', 'webp'];
const activeExtensions = ref<string[]>(extensions);
const dialog = ref(false);
const selectedDirectory = ref('');

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
      dialog.value = true;
    }
  } catch (error) {
    console.error('Error selecting directory:', error);
  }
};

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