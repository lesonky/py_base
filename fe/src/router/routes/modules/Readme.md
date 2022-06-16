# 菜单代码样例

[参考文档](https://arco.design/docs/pro/routes-and-menu)

```ts
import { DEFAULT_LAYOUT } from '@/router/constans';

export default {
  path: '/dashboard',
  name: 'dashboard',
  component: DEFAULT_LAYOUT,
  meta: {
    locale: 'menu.dashboard',
    requiresAuth: true,
    icon: 'icon-user',
    order: 0,
  },
  children: [
    {
      path: 'workplace',
      name: 'Workplace',
      component: () => import('@/views/dashboard/workplace/index.vue'),
      meta: {
        locale: 'menu.dashboard.workplace',
        requiresAuth: true,
        permissions: ['*'],
      },
    },
  ],
};
```