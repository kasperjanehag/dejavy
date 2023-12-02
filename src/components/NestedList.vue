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

const items = ref([])

onMounted(async () => {
  items.value = await invoke('get_data')
})
</script>