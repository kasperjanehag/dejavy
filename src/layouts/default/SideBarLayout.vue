<template>
  <v-navigation-drawer>
    <v-list-item title="Dejavy 1" subtitle="Clean up"></v-list-item>
    <v-list-item title="Dejavy 2" subtitle="Clean up"></v-list-item>
    <v-divider></v-divider>
    <v-list-item title="Dejavy 3" subtitle="Clean up"></v-list-item>
    <nested-list/>
  </v-navigation-drawer>
</template>

<script>
import NestedList from '../../components/NestedList.vue';

export default {
  components: {
    NestedList
  },
}
</script>