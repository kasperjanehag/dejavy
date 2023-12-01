<template>

    <!-- FOLDERS -->

    <v-list-group
        v-for="(item, _) in itemsFolders"
        :key="item.id"
        :prepend-icon="item.icon"
        :value="item.name"
        v-model="item.isOpen"
    >
        <template v-slot:activator="{ props }">
            <v-list-item
            :title="item.name"
            v-bind="props"
            ></v-list-item>
        </template>

        <NestedListItems
            :items="item.children"
        />

    </v-list-group>

    <!-- FILES  -->
    <v-list-item 
        v-for="(item, i) in itemsFiles"
        :key="item.id"
        :prepend-icon="item.icon"
        :title="item.name"
    ></v-list-item>
  </template>

<script>
import NestedListItems from './NestedListItems.vue';

export default {
    name: 'NestedListItems',
    props: {
        items: {
            type: Object,
            required: true
        }
    },
    computed: {
            itemsFiles() {
                return this.items.filter(item => !item.children);
            },
            itemsFolders() {
                return this.items.filter(item => item.children && item.children.length > 0);
            }
        },
}
</script>