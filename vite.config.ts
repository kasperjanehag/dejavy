import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";;

import vuetify, { transformAssetUrls } from 'vite-plugin-vuetify'
import ViteFonts from 'unplugin-fonts/vite'

// import { fileURLToPath, URL } from 'node:url'

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    vue(
      // {template: { transformAssetUrls }}
    ),
    
    // https://github.com/vuetifyjs/vuetify-loader/tree/master/packages/vite-plugin#readme
    vuetify({
      autoImport: true,
      styles: {
        configFile: 'src/styles/settings.scss',
      },
    }),

    ViteFonts({
      google: {
        families: [{
          name: 'Roboto',
          styles: 'wght@100;300;400;500;700;900',
        }],
      },
    }),

  ],

  // resolve: {
  //   alias: {
  //     '@': fileURLToPath(new URL('./src', import.meta.url))
  //   },
  //   extensions: [
  //     '.js',
  //     '.json',
  //     '.jsx',
  //     '.mjs',
  //     '.ts',
  //     '.tsx',
  //     '.vue',
  //   ],
  // },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
  }
}));
