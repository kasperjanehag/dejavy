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
import { ref, inject, watch } from 'vue'
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
const selectedDirectory = inject('selectedDirectory');

const getFileTreeData = async () => {
  if (selectedDirectory.value) {
    const fileTreeData = await invoke('get_file_tree_data', { path: selectedDirectory.value });
    return fileTreeData as Item[];
  } else {
    return [];
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

watch(selectedDirectory, async (newVal, oldVal) => {
  if (newVal !== oldVal) {
    const fileTreeData = await getFileTreeData();
    items.value = fileTreeData.map(processFileTree);
  }
}, { immediate: true });

</script>