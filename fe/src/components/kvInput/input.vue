<template>
  <a-row class="mb-4" :gutter="32">
    <a-col :span="12" :flex="1">
      <a-row class="items-center">
        <a-col :span="2">
          <span>Key:</span>
        </a-col>
        <a-col :span="22">
          <a-input v-model.trim="data.key" placeholder="KEY"></a-input>
        </a-col>
      </a-row>
    </a-col>
    <a-col :span="12" :flex="1">
      <a-row class="items-center">
        <a-col :span="2">Value:</a-col>
        <a-col :span="22">
          <a-input v-model.trim="data.value" placeholder="VALUE"></a-input>
        </a-col>
      </a-row>
    </a-col>
    <a-col
      v-if="props.showMinus || props.showPlus"
      flex="100px"
      class="flex justify-between items-center"
    >
      <a-button
        v-if="props.showMinus"
        type="outline"
        shape="circle"
        size="small"
        @click="$emit('delete')"
      >
        <icon-close />
      </a-button>
      <a-button
        v-if="props.showPlus"
        type="outline"
        shape="circle"
        size="small"
        @click="$emit('add')"
      >
        <icon-plus />
      </a-button>
    </a-col>
  </a-row>
</template>

<script lang="ts" setup>
import { useVModel } from '@vueuse/core';

const props = defineProps<{
  modelValue: { key: string; value: string };
  showPlus?: boolean;
  showMinus?: boolean;
}>();
const emit = defineEmits(['update:modelValue', 'delete', 'add']);
const data = useVModel(props, 'modelValue', emit);
</script>

<style lang="less" scoped></style>
