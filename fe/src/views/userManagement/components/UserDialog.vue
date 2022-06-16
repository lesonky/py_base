<template>
  <a-modal v-model:visible="$visible" @ok="handleOk" @cancel="handleCancel">
    <template #title> {{ $title }} </template>
  </a-modal>
</template>

<script lang="ts" setup>
import { UserState } from '@/store/modules/user/types';
import { useUserStore } from '@/store';
import { useVModel } from '@vueuse/core';
import { reactive, computed } from 'vue';

const props = withDefaults(
  defineProps<{
    visible: boolean;
    userState?: UserState;
    title: string;
  }>(),
  {
    visible: false,
    title: '编辑用户',
  }
);

const emit = defineEmits(['update:visible', 'update:userState']);

const $visible = useVModel(props, 'visible', emit);

const userStore = useUserStore();

// 是否是修改自己
const isSelf = computed(() => {
  return props.userState && props.userState.accountId === userStore.accountId;
});

// 是否是新建角色
const isCreate = computed(() => {
  return !props.userState?.accountId;
});

const $title = computed(() => {
  return props.title || isCreate ? '新增用户' : '修改用户';
});

const localUserStore = reactive<UserState>({});

const handleOk = () => {};
const handleCancel = () => {};
</script>

<style lang="less" scoped></style>
