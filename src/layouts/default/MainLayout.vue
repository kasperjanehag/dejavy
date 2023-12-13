<template>
  <v-main>
    <v-container>
      <v-row v-for="(duplicateSet, md5Hash) in identifiedDuplicatesStore.$state.duplicates" :key="md5Hash">
        <v-col cols="3">
          <v-card-title>{{ md5Hash }}</v-card-title>
        </v-col>
        <v-col v-for="imagePath in duplicateSet" :key="imagePath" cols="auto">
          <v-card-text>{{ imagePath }}</v-card-text>
        </v-col>
      </v-row>
    </v-container>
  </v-main>
</template>

<script setup lang="ts">
import { useIdentifiedDuplicatesStore } from '../../stores/identifiedDuplicatesStore';

const identifiedDuplicatesStore = useIdentifiedDuplicatesStore();
</script>