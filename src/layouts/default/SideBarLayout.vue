<template>
  <v-navigation-drawer width="350">
    <v-list-item title="Dejavy" subtitle="Clean up"></v-list-item>
    <v-divider></v-divider>
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