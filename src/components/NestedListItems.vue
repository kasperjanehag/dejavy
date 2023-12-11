<template>

    <!-- FOLDERS -->

    <v-list-group
        v-for="(item, _) in itemsFolders"
        :key="item.id"
        :prepend-icon="item.icon"
        :value="item.name"
        v-model="item.isOpen"
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
        v-for="(item, i) in itemsFiles"
        :key="item.id"
        :prepend-icon="item.icon"
        :title="item.name"
    ></v-list-item>
  </template>

<script setup lang="ts">
import { defineProps, computed } from 'vue';
import NestedListItems from './NestedListItems.vue';

interface FileTreeItem {
    id: number;
    name?: string;
    absolute_path?: string;
    file_format?: string;
    relative_path?: string;
    children?: FileTreeItem[];
    icon?: string;
    is_open?: boolean;
    type?: 'directory' | 'image';
}

const props = defineProps<{
  items: FileTreeItem[];
}>();

const itemsFiles = computed(() => props.items.filter(item => !item.children));
const itemsFolders = computed(() => props.items.filter(item => item.children && item.children.length > 0));

</script>