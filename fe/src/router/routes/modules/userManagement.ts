import { DEFAULT_LAYOUT } from '@/router/constans';

export default {
  path: '/user-management',
  name: 'userManagement',
  component: DEFAULT_LAYOUT,
  meta: {
    locale: 'menu.userManagement',
    requiresAuth: true,
    icon: 'icon-user',
    order: 0,
  },
  children: [
    {
      path: 'list',
      name: 'userManagementList',
      component: () => import('@/views/userManagement/list/index.vue'),
      meta: {
        locale: 'menu.dashboard.list',
        requiresAuth: true,
        permissions: ['*'],
      },
    },
  ],
};
