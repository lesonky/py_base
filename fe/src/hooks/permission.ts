import { RouteLocationNormalized, RouteRecordRaw } from 'vue-router';
import { intersection } from 'lodash';
import { useUserStore } from '@/store';

export default function usePermission() {
  const userStore = useUserStore();
  return {
    accessRouter(route: RouteLocationNormalized | RouteRecordRaw) {
      return (
        !route.meta?.requiresAuth ||
        !route.meta?.permissions ||
        route.meta?.permissions?.includes('*') ||
        intersection(route.meta?.permissions, userStore.userInfo.permissions)
          .length > 0
      );
    },
    findFirstPermissionRoute(_routers: any, permissions = ['all']) {
      const cloneRouters = [..._routers];
      while (cloneRouters.length) {
        const firstElement = cloneRouters.shift();
        if (
          firstElement?.meta?.roles?.find((el: string[]) => {
            return (
              el.includes('*') ||
              permissions.includes('all') ||
              intersection(el, permissions).length > 0
            );
          })
        )
          return { name: firstElement.name };
        if (firstElement?.children) {
          cloneRouters.push(...firstElement.children);
        }
      }
      return null;
    },
    // You can add any rules you want
  };
}
