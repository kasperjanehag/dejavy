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
    <v-divider></v-divider>
    <nested-list/>
  </v-navigation-drawer>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import NestedList from '../../components/NestedList.vue';

const extensions = ['png', 'jpeg', 'jpg', 'gif', 'bmp', 'tiff', 'ico', 'webp'];
const activeExtensions = ref<string[]>(extensions);

const toggleExtension = (ext: string) => {
  const index = activeExtensions.value.indexOf(ext);
  if (index >= 0) {
    activeExtensions.value.splice(index, 1);
  } else {
    activeExtensions.value.push(ext);
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