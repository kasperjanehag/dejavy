// Composables
import { createRouter, createWebHashHistory } from 'vue-router'
import DefaultLayout from '../layouts/default/Default.vue'
import HomeView from '../views/Home.vue'

const routes = [
  {
    path: '/',
    component: HomeView,
    // children: [
    //   {
    //     path: '',
    //     name: 'Home',
    //     // route level code-splitting
    //     // this generates a separate chunk (Home-[hash].js) for this route
    //     // which is lazy-loaded when the route is visited.
    //     component: HomeView,
    //   },
    // ],
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router
