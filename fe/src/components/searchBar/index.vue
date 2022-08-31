<template>
  <component
    :is="props.fixed ? 'a-affix' : 'div'"
    :offset-top="props.fixTop"
    @change="fixChange"
  >
    <a-row :style="fixStyle">
      <a-col :flex="1">
        <a-form
          :model="formModel"
          :label-col-props="{ span: 6 }"
          :wrapper-col-props="{ span: 18 }"
          label-align="left"
        >
          <a-row :gutter="18">
            <a-col
              v-for="item in props.searchItems"
              :key="item.name"
              :span="item.span || 8"
            >
              <a-form-item
                :field="item.name"
                :label="item.label"
                :label-col-props="{ span: item.labelSpan || 6 }"
                :wrapper-col-props="{ span: item.warpperSpan || 18 }"
              >
                <a-input
                  v-if="item.type === 'text'"
                  v-model="formModel[item.name]"
                  v-bind="item.meta"
                  @press-enter="search"
                />
                <remote-select
                  v-else-if="item.type === 'remoteSelect'"
                  v-model="formModel[item.name]"
                  :type="item.meta.remoteType"
                  :style="{ width: '100%' }"
                  v-bind="item.meta"
                  @press-enter="search"
                />
                <a-select
                  v-else-if="item.type === 'select'"
                  v-model="formModel[item.name]"
                  :style="{ width: '100%' }"
                  v-bind="item.meta"
                  @press-enter="search"
                />
                <a-input-number
                  v-else-if="item.type === 'number'"
                  v-model="formModel[item.name]"
                  v-bind="item.meta"
                />
                <a-range-picker
                  v-else-if="item.type === 'dateRange'"
                  v-model="dateRangeModel[item.name]"
                  v-bind="item.meta"
                />
              </a-form-item>
            </a-col>
          </a-row>
        </a-form>
      </a-col>
      <a-divider
        :style="{ height: layout ? '84px' : '32px' }"
        direction="vertical"
      />
      <a-col :flex="layout ? '84px' : '160px'" style="text-align: right">
        <a-space :size="18" :direction="layout ? 'vertical' : 'horizontal'">
          <a-button type="primary" @click="search">
            <template #icon>
              <icon-search />
            </template>
            查询
          </a-button>
          <a-button @click="reset">
            <template #icon>
              <icon-refresh />
            </template>
            重置
          </a-button>
        </a-space>
      </a-col>
    </a-row>
  </component>
</template>

<script lang="ts" setup>
import remoteSelect from '@/components/remoteSelect/index.vue';
import { reactive, computed, ref, watchEffect } from 'vue';
import { sum } from 'lodash-es';
import type { SearchItem } from './type';

/*
const searItems: SearchItem[] = [
  {
    type: 'text',
    name: 'name__icontain',
    label: '算法名称',
    meta: {
      placeholder: '请输入算法名称',
    },
  },
];
*/

const props = withDefaults(
  defineProps<{
    searchItems: SearchItem[];
    fixed?: boolean;
    fixTop?: number;
  }>(),
  {
    searchItems: () => [],
    fixed: true,
    fixTop: 80,
  }
);

const emit = defineEmits(['search']);

const fixStyle = ref({});

const layout = computed(() => {
  const items = props.searchItems || [];
  const totalSpan = sum(items.map((item) => item.span || 8));
  return totalSpan > 24;
});

const generateFormModel: () => { [key: string]: any } = () => {
  return props.searchItems.reduce((a: any, b: SearchItem) => {
    if (b.type === 'dateRange') {
      a[`${b.name}__gt`] = undefined;
      a[`${b.name}__lt`] = undefined;
    } else {
      a[b.name] = undefined;
    }
    return a;
  }, {});
};

const formModel = reactive(generateFormModel());

const generateDateRangeFormModel: () => { [key: string]: any } = () => {
  const needCover = props.searchItems.filter(
    (item) => item.type === 'dateRange'
  );
  const ret: any = {};
  needCover.forEach((item) => {
    ret[item.name] = [];
  });
  return ret;
};

const dateRangeModel = reactive(generateDateRangeFormModel());

const search = () => {
  emit('search', formModel);
};

const reset = () => {
  Object.assign(formModel, generateFormModel());
  Object.assign(dateRangeModel, generateDateRangeFormModel());
  emit('search', formModel);
};

const fixChange = (fixed: Event) => {
  if ((fixed as unknown as boolean) === true) {
    fixStyle.value = {
      backgroundColor: '#FFFFFF',
      boxShadow:
        'rgb(255 255 255) -10px -10px 0px 10px , rgb(255 255 255) 10px -10px 0px 10px,rgb(0 0 0 / 50%) 0px -5px 6px 5px',
    };
  } else {
    fixStyle.value = {};
  }
};

// 时间范围选择器处理
watchEffect(() => {
  // 处理 dateRange 类型数据
  Object.keys(dateRangeModel).forEach((key) => {
    // eslint-disable-next-line prefer-destructuring
    formModel[`${key}__gt`] = dateRangeModel[key][0];
    // eslint-disable-next-line prefer-destructuring
    formModel[`${key}__lt`] = dateRangeModel[key][1];
  });

  // 处理 allow clear 清理过后的 空字符串
  Object.keys(formModel).forEach((key) => {
    if (formModel[key] === '') {
      formModel[key] = undefined;
    }
  });
});

defineExpose({
  reset,
  formModel,
});
</script>

<style lang="less" scoped></style>
