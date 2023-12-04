<template>
  <v-navigation-drawer width="400">
    <v-list-item title="Image Explorer"></v-list-item>
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