<template>
    <v-card
      class="mx-auto"
    >
      <v-list
        density="compact"
      >

        <!-- Temporary button to list files -->
        <v-list-group value="Paths">
        <template v-slot:activator="{ props }">
          <v-list-item
            v-bind="props"
            prepend-icon="mdi-folder"
            title="Paths"
            @click="showPaths = !showPaths; listPaths()"
          ></v-list-item>
        </template>

        <v-list-item
          v-for="path in paths"
          :key="path"
          :value="path"
          :title="path"
        ></v-list-item>
      </v-list-group>
        
        
        
        <NestedListItems :items="items"></NestedListItems>
  
      </v-list>
    </v-card>
  </template>
  
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import NestedListItems from './NestedListItems.vue'

interface Item {
  id: number;
  name: string;
  children?: Item[];
  icon?: string;
  is_open?: boolean;
}

const items = ref<Item[]>([])
const paths = ref<string[]>([])
const showPaths = ref(false)


const listPaths = async () => {
  console.log('listPaths called');
  if (showPaths.value) {
    const result = await invoke('list_paths');
    console.log('list_paths result:', result);
    paths.value = result as string[];
  } else {
    paths.value = [];
  }
}

const processFileTree = (item: Item): Item => {
  const processedItem: Item = {
    ...item,
    icon: item.children ? 'mdi-folder-outline' : 'mdi-file-outline',
    ...(item.children && { is_open: false }),
  }

  if (item.children) {
    processedItem.children = item.children.map(processFileTree)
  }

  return processedItem
}

onMounted(async () => {
  // Get file tree from backend
  const fileTreeData = (await invoke('get_data')) as Item[];
  
  // Add front-end specific properties to render tree
  items.value = fileTreeData.map(processFileTree)

  // Call listPaths function
  await listPaths()
})
</script>