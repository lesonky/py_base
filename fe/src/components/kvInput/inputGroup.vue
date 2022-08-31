<template>
  <div class="w-full">
    <kv-input
      v-for="(item, index) in data"
      :key="item.id"
      v-model="data[index]"
      :show-plus="index === data.length - 1"
      :show-minus="data.length > 1"
      @add="handleAdd"
      @delete="handleDelete(index)"
    ></kv-input>
  </div>
</template>

<script lang="ts" setup>
import { watchEffect } from 'vue';
import { useVModel } from '@vueuse/core';
import { nanoid } from 'nanoid';
import kvInput from './input.vue';

const props = defineProps<{
  modelValue: { key: string; value: string; id: string }[];
}>();

const emit = defineEmits(['update:modelValue']);

const data = useVModel(props, 'modelValue', emit);

const handleDelete = (index: number) => {
  data.value.splice(index, 1);
};

const handleAdd = () => {
  data.value.push({
    id: nanoid(10),
    key: '',
    value: '',
  });
};

watchEffect(() => {
  if (data.value.length === 0) {
    handleAdd();
  }
});
</script>

<style lang="less" scoped></style>
