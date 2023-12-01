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
  
<script>

import NestedListItems from './NestedListItems.vue';

export default {
    components: {
        NestedListItems
    },
    data: function() {
        return {
            items: [],
        };
    },
    mounted() {
      eel.list_images()((result) => {
          console.log(result);
          this.items = result;
      });
    },
}
</script>
