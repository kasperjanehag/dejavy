import { defineStore } from 'pinia';

interface DuplicateGroup {
  [group_id: string]: string[];
}

export const useIdentifiedDuplicatesStore = defineStore({
  id: 'identifiedDuplicates',
  state: () => ({
    duplicates: {} as DuplicateGroup,
  }),
  actions: {
    addDuplicateGroup(groupId: string, images: string[]) {
      this.duplicates[groupId] = images;
    },
    removeDuplicateGroup(groupId: string) {
      delete this.duplicates[groupId];
    },
    clearDuplicates() {
      this.duplicates = {};
    },
  },
});