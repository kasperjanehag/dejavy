<template>
  <v-layout class="rounded rounded-md">

    <v-app-bar>
      <v-img src="src/assets/logo.png"></v-img>
    </v-app-bar>
    <SideBarLayout />
    <MainLayout />

  </v-layout>
  
</template>

<script setup lang="ts">
import SideBarLayout from './SideBarLayout.vue'
import MainLayout from './MainLayout.vue'
</script>
