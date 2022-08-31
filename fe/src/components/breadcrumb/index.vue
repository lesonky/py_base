<template>
  <a-breadcrumb class="container-breadcrumb">
    <a-breadcrumb-item>
      <icon-apps />
    </a-breadcrumb-item>
    <template v-for="(item, index) in computedItems" :key="item.title || item">
      <a-breadcrumb-item
        :class="{ link: !!(item as BreadcrumbItem).to }"
        @click="handleBreadcrumbClick(item, index)"
      >
        {{
          useI18n
            ? $t((item as BreadcrumbItem).title || (item as string))
            : (item as BreadcrumbItem).title || item
        }}
      </a-breadcrumb-item>
    </template>
  </a-breadcrumb>
</template>

<script lang="ts" setup>
import { PropType, computed } from 'vue';
import { useRouter, useRoute } from 'vue-router';

/*
参数说明:
useI18n: 是否使用国际化
items:
  字符串: 直接渲染,或者通过国际化渲染
  对象:
    title: 直接渲染,或者通过国际化渲染
    to: 点击跳转地址
useRoute: 是否使用 route 自动计算项目
lastTitle: 在 useRoute 模式下,替换最后一项的显示,最后一项可能是同一个路由,兼具新增,编辑等功能
*/

type BreadcrumbItem = { title: string; to: string };

const props = defineProps({
  items: {
    type: Array as PropType<(string | BreadcrumbItem)[]>,
    default() {
      return [];
    },
  },
  useI18n: {
    type: Boolean,
    default: true,
  },
  useRoute: {
    type: Boolean,
    default: false,
  },
  lastTitle: {
    type: String,
    default: '',
  },
});

const router = useRouter();
const route = useRoute();

const buildItem = (r: any) => {
  return {
    title: r.meta.title || '',
    to: router.resolve({
      name: r.name,
      params: r.meta.params,
      query: r.meta.query,
    }).path,
  };
};

const computedItems = computed(() => {
  if (!props.useRoute) {
    return props.items;
  }

  // 根据当前路由,计算项目
  return route.matched
    .map((item, index) => {
      if (item.children.length !== 0) {
        return item.meta.title || '';
      }
      // 最后一个路由需要特殊处理
      if (index === route.matched.length - 1) {
        // 如果有 activeMenu 则表示是二级页面,需要通过 activeMenu 找到父级路由
        if (item.meta.activeMenu) {
          const parentRoute = router
            .getRoutes()
            .find((r: any) => r.name === item.meta.activeMenu) as any;
          return [
            buildItem(parentRoute),
            props.lastTitle || item.meta.title || '',
          ];
        }
        return props.lastTitle || item.meta.title || '';
      }
      return buildItem(item);
    })
    .flat(2);
});

const handleBreadcrumbClick = (
  item: string | BreadcrumbItem,
  index: number
): void => {
  if (typeof item !== 'string' && item.to && index !== props.items.length - 1) {
    router.push({ path: item.to });
  }
};
</script>

<style scoped lang="less">
.container-breadcrumb {
  margin: 16px 0;

  :deep(.arco-breadcrumb-item) {
    color: rgb(var(--gray-6));

    &:last-child {
      color: rgb(var(--gray-8));
    }

    &.link {
      cursor: pointer;

      &:hover {
        color: rgb(var(--primary-6));
        font-weight: 500;
      }
    }
  }
}
</style>
