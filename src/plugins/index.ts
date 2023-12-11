/**
 * plugins/index.ts
 *
 * Automatically included in `./src/main.js`
 */

// Plugins
import vuetify from './vuetify'
import router from '../router'
import { useStore } from './store';
import { App } from 'vue'

export function registerPlugins (app: App<Element>): void {
  app
    .use(vuetify)
    .use(router)
    .use(useStore)
}
