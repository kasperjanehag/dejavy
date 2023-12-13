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
  import { ref, watch, inject } from 'vue';
  import { invoke } from '@tauri-apps/api/tauri';
  import NestedListItems from './NestedListItems.vue';
    
  interface FileTreeItem {
    id: number;
    name: string;
    icon: string;
    is_open: boolean;
    type: 'directory' | 'image';
    absolute_path?: string;
    file_format?: string;
    relative_path?: string;
    children?: FileTreeItem[];
  }

  interface Image {
    absolute_path: string;
    file_format: string;
    name: string;
    relative_path: string;
  }
  
  interface Directory {
    name: string;
    children?: FileTreeItem[];
  }

  interface UnProcessedFileTreeItem {
    Directory?: Directory;
    Image?: Image;
  }

  
  let idCounter = 0;
  
  const items = ref<FileTreeItem[]>([]);
  const selectedDirectory = inject('selectedDirectory');
  
  const getFileTreeData = async () => {
    if (selectedDirectory.value) {
      const fileTreeData = await invoke('load_image_data', { path: selectedDirectory.value });
      return fileTreeData as UnProcessedFileTreeItem[];
    } else {
      return [];
    }
  }
  

  const processFileTree = (item: UnProcessedFileTreeItem): FileTreeItem => {
    idCounter += 1;
    let processedItem: FileTreeItem = {
      id: idCounter,
      name: 'Unknown',
      icon: 'mdi-help-box',
      is_open: false,
      type: 'directory',
    }

    if (item.Directory) {
      processedItem.name = item.Directory.name;
      processedItem.children = item.Directory.children?.map(processFileTree);
      processedItem.icon = 'mdi-folder-outline';
      processedItem.type = 'directory';
    } else if (item.Image) {
      processedItem.name = item.Image.name;
      processedItem.absolute_path = item.Image.absolute_path;
      processedItem.file_format = item.Image.file_format;
      processedItem.relative_path = item.Image.relative_path;
      processedItem.icon = 'mdi-file-outline';
      processedItem.type = 'image';
    }

    return processedItem;
  }
  
  watch(selectedDirectory, async (newVal, oldVal) => {
    if (newVal !== oldVal || newVal !== undefined) {
      const fileTreeData = await getFileTreeData();
      items.value = fileTreeData.map(processFileTree);
    }
  });
  </script>