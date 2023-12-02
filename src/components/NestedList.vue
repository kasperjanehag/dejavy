<template>
    <v-card
      class="mx-auto"
    >
      <v-list
        density="compact"
      >
        
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
})
</script>