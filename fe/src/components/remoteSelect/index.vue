<template>
  <a-select
    v-model="value"
    v-bind="attribute"
    :disabled="mergedDisabled"
    @change="handleChange"
  >
    <a-option v-for="item in options" :key="item.value" :value="item.value">
      {{ item.label }}
    </a-option>
  </a-select>
</template>

<script lang="ts" setup>
import { watchEffect, useAttrs, computed } from 'vue';
import { useDictionaryStore } from '@/store';
import { useVModel } from '@vueuse/core';
import { useFormItem } from '@arco-design/web-vue';

import type { dictionaryKeys } from '@/store/modules/dictionary';

const props = defineProps<{
  type: dictionaryKeys;
  modelValue?: string | number;
  useCache?: boolean;
}>();
const emit = defineEmits(['update:modelValue']);
const attribute = useAttrs();
const value = useVModel(props, 'modelValue', emit);
const { mergedDisabled, eventHandlers } = useFormItem();

const dictionary = useDictionaryStore();

const options = computed(() => {
  return dictionary[props.type];
});

const handleChange = (e: any) => {
  eventHandlers.value?.onChange?.(e);
};

watchEffect(() => {
  dictionary.initOption(props.type, props.useCache);
});
</script>

<style lang="less" scoped></style>
