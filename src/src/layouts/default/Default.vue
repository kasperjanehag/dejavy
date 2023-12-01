<template>
  <v-layout class="rounded rounded-md">
    <v-app-bar title="Application bar"></v-app-bar>
    <SideBarLayout />
    <MainLayout />
  </v-layout>
</template>

<script>
import SideBarLayout from './SideBarLayout.vue'
import MainLayout from './MainLayout.vue'

export default {
  components: {
    SideBarLayout,
    MainLayout
  }
}
</script>