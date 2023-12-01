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
            items: [
              {
                id: 1,
                icon: "mdi-plus-outline",
                name: "File 1"
              },
              {
                id: 2,
                icon: "mdi-plus-outline",
                name: "Folder 1",
                isOpen: false,
                children: [
                  {
                    id: 3,
                    icon: "mdi-plus-outline",
                    name: "File 2"
                  }
                ]
              }
            ],
        };
    },
}
</script>
