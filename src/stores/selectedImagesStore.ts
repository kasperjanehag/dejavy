import { defineStore } from 'pinia';

interface ImagePaths {
    absolute_path: string;
}

export const useSelectedImagesStore = defineStore({
  id: 'selectedImages',
  state: () => ({
    images: [] as ImagePaths[],
  }),
  actions: {
    addImagePath(newImagePath: ImagePaths) {
        if (!this.images.find(image_path => image_path.absolute_path === newImagePath.absolute_path)) {
          this.images.push(newImagePath);
        }
      },
      removeImagePath(imageToRemove: ImagePaths) {
        const index = this.images.findIndex(image_path => image_path.absolute_path === imageToRemove.absolute_path);
        if (index !== -1) {
          this.images.splice(index, 1);
        }
      },
  },
});