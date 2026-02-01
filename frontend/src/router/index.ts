import { createRouter, createWebHistory } from 'vue-router'

import { useAuthStore } from '@/stores/auth'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/login',
      name: 'login',
      component: () => import('../views/LoginView.vue'),
    },
    {
      path: '/register',
      name: 'register',
      component: () => import('../views/RegisterView.vue'),
    },
    {
      path: '/rooms',
      name: 'rooms',
      component: () => import('../views/RoomsView.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/rooms/:id/questions',
      name: 'questions',
      component: () => import('../views/QuestionsView.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/rooms/:id/access',
      name: 'access',
      component: () => import('../views/RoomAccessView.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/rooms/:id/game',
      name: 'game',
      component: () => import('../views/GameView.vue'),
      meta: { requiresAuth: true, hideLayout: true },
    },
    {
      path: '/rooms/:id/game/controller',
      name: 'game-controller',
      component: () => import('../views/GameControllerView.vue'),
      meta: { requiresAuth: true, hideLayout: true },
    },
    {
      path: '/',
      redirect: '/rooms',
    },
  ],
})

router.beforeEach((to, from, next) => {
  const auth = useAuthStore()
  if (to.meta.requiresAuth && !auth.isAuthenticated) {
    next('/login')
  } else if ((to.name === 'login' || to.name === 'register') && auth.isAuthenticated) {
    next('/rooms')
  } else {
    next()
  }
})

export default router
