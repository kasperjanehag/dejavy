<template>
  <!-- <v-app> -->
  <v-layout class="rounded rounded-md">

    <v-app-bar title="Application bar"></v-app-bar>
    <SideBarLayout />
    <MainLayout />

  </v-layout>
      <!-- </v-layout> -->
  <!-- </v-app> -->
</template>

<script>
import { defineComponent } from 'vue'
import SideBarLayout from './SideBarLayout.vue'
import MainLayout from './MainLayout.vue'

export default {
  name: 'DefaultLayout',
  components: {
    SideBarLayout,
    MainLayout
  }
}
</script>
