<template>
  <a-modal
    v-model:visible="$visible"
    width="640px"
    @ok="handleOk"
    @cancel="handleCancel"
  >
    <template #title> {{ $title }} </template>
    <a-form :model="localUserState" :style="{ width: '600px' }">
      <a-form-item field="name" label="用户名">
        <a-input
          v-model="localUserState.name"
          :disabled="!isCreate"
          placeholder="请输入用户名"
        />
      </a-form-item>
      <a-form-item field="string" label="角色">
        <a-select
          v-model="localUserState.role!.name"
          :disabled="isSelf"
          :style="{ width: '100%' }"
          placeholder="请选择角色"
          allow-clear
        >
          <a-option v-for="(item, index) in userStore.roleList" :key="index">{{
            item.name
          }}</a-option>
        </a-select>
      </a-form-item>
      <a-form-item field="nickName" label="昵称">
        <a-input v-model="localUserState.nickName" placeholder="请输入昵称" />
      </a-form-item>
      <a-form-item field="email" label="Email">
        <a-input v-model="localUserState.email" placeholder="请输入Email" />
      </a-form-item>
      <a-form-item field="phone" label="电话">
        <a-input v-model="localUserState.phone" placeholder="请输入电话" />
      </a-form-item>
      <a-form-item v-if="isCreate" field="password" label="密码">
        <a-input v-model="localUserState.password" placeholder="请初始密码" />
      </a-form-item>
      <a-form-item v-if="isCreate" field="passwordConfirm" label="请确认密码">
        <a-input
          v-model="localUserState.passwordConfirm"
          placeholder="请确认密码"
        />
      </a-form-item>
    </a-form>
  </a-modal>
</template>

<script lang="ts" setup>
import { UserState } from '@/store/modules/user/types';
import { useUserStore } from '@/store';
import { useVModel } from '@vueuse/core';
import { ref, computed, watch } from 'vue';
import { omit } from 'lodash-es';

type CUserState = UserState & { passwordConfirm?: string };

const createEmptyUser = (): CUserState => ({
  name: '',
  nickName: '',
  avatar: '',
  isActive: true,
  email: '',
  introduction: '',
  phone: '',
  role: {
    name: '',
  },
  password: '',
  passwordConfirm: '',
  isDeleted: false,
});

const props = withDefaults(
  defineProps<{
    visible?: boolean;
    userState?: CUserState;
    title?: string;
  }>(),
  {
    visible: false,
    title: '编辑用户',
  }
);

const emit = defineEmits(['update:visible', 'ok', 'cancel']);

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

// Prop的值来初始化本地模型
const localUserState = ref<CUserState>(createEmptyUser());
watch(
  () => props.userState,
  () => {
    localUserState.value = { ...createEmptyUser(), ...props.userState };
  }
);

const handleOk = () => {
  emit('ok', omit(localUserState.value, 'passwordConfirm'));
};
const handleCancel = () => {
  emit('cancel');
};
</script>

<style lang="less" scoped></style>
