import { createRouter, createWebHistory } from 'vue-router'
import TeacherView from '../views/TeacherView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'teacher',
      component: TeacherView,
    },
    {
      path: '/about',
      name: 'about',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('../views/AboutView.vue'),
    },
    {
      path: '/Student',
      name: 'Student',

      component: () => import('../views/StudentView.vue'),
    },
    {
      path: '/Login',
      name: 'Login',

      component: () => import('../views/LoginView.vue'),
    },
  ],
})

export default router
