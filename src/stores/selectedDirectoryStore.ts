// directoryStore.ts
import { defineStore } from 'pinia';

export const useSelectedDirectoryStore = defineStore({
  id: 'directory',
  state: () => ({
    selectedDirectory: '',
  }),
  actions: {
    setSelectedDirectory(directory: string) {
      this.selectedDirectory = directory;
    },
  },
});