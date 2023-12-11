<template>

    <!-- FOLDERS -->

    <v-list-group
        v-for="(item, _) in itemsFolders"
        :key="item.id"
        :prepend-icon="item.icon"
        :value="item.name"
        v-model="item.is_open"
    >
        <template v-slot:activator="{ props }">
            <v-list-item
            :title="item.name"
            v-bind="props"
            ></v-list-item>
        </template>

        <NestedListItems
            :items="item.children"
        />

    </v-list-group>

    <!-- FILES  -->
    <v-list-item 
        v-for="(item, _) in itemsFiles"
        :key="item.id"
        :prepend-icon="item.icon"
        :title="item.name"
        :class="{ 'selected': selectedImagesStore.images.find(image => image.absolute_path === item.absolute_path) }"
        @click="selectImage(item)"
    ></v-list-item>

</template>

<script setup lang="ts">

import { computed, defineProps } from 'vue'
import NestedListItems from './NestedListItems.vue';
import { useSelectedImagesStore } from '../stores/selectedImagesStore';

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
};

const selectedImagesStore = useSelectedImagesStore();

const props = defineProps<{
  items: FileTreeItem[];
}>();

const selectImage = (item: FileTreeItem) => {
  if (item.type === 'image' && item.absolute_path) {
    if (selectedImagesStore.images.find(image => image.absolute_path === item.absolute_path)) {
      selectedImagesStore.removeImagePath({ absolute_path: item.absolute_path });
    } else {
      selectedImagesStore.addImagePath({ absolute_path: item.absolute_path });
    }
  }
}

const itemsFiles = computed(() => props.items.filter(item => !item.children));
const itemsFolders = computed(() => props.items.filter(item => item.children && item.children.length > 0));

</script>

<style scoped>
.selected {
  background-color: #f0f0f0; /* Change this to your preferred color */
}
</style>