<template>
  <a-modal v-model:visible="$visible" width="640px">
    <template #title> {{ $title }} </template>
    <a-form
      ref="formRef"
      :model="localUserState"
      :style="{ width: '600px' }"
      :rules="computedRules"
    >
      <a-form-item field="name" label="用户名">
        <a-input
          v-model="localUserState.name"
          :disabled="!isCreate"
          placeholder="请输入用户名"
        />
      </a-form-item>
      <a-form-item field="role.name" label="角色">
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
      <a-divider style="margin-top: 0" />
      <a-alert
        v-if="localUserState.password && isSelf"
        closable
        type="warning"
        style="margin-bottom: 16px"
      >
        修改当前登录用户密码后需要重新登录
      </a-alert>
      <a-form-item field="password" label="密码">
        <a-input v-model="localUserState.password" placeholder="请初始密码" />
      </a-form-item>
      <a-form-item field="passwordConfirm" label="请确认密码">
        <a-input
          v-model="localUserState.passwordConfirm"
          placeholder="请确认密码"
        />
      </a-form-item>
    </a-form>
    <template #footer>
      <a-button type="outline" @click="handleCancel">取消</a-button
      ><a-button type="primary" @click="handleOk">确认</a-button>
    </template>
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
const isCreate = computed<boolean>(() => {
  return !props.userState?.accountId;
});

const $title = computed(() => {
  return props.title || (isCreate.value ? '新增用户' : '修改用户');
});

// Prop的值来初始化本地模型
const localUserState = ref<CUserState>(createEmptyUser());
watch(
  () => props.userState,
  () => {
    localUserState.value = { ...createEmptyUser(), ...props.userState };
  }
);

const formRef = ref();
const rules = {
  'name': [
    {
      required: true,
      message: '请输入用户名',
    },
    {
      match: /^[a-z][a-z0-9_]+$/gi,
      message: '格式:字母开头,字母数字下划线组成',
    },
    {
      maxLength: 16,
      message: '长度不超过16个字符',
    },
  ],
  'role.name': [
    {
      required: true,
      message: '请选择用户角色',
    },
  ],
  'nickName': [
    {
      maxLength: 16,
      message: '长度不超过16个字符',
    },
  ],
  'email': [
    {
      match: /^[^@ \t\r\n]+@[^@ \t\r\n]+\.[^@ \t\r\n]+$/i,
      message: '邮箱格式不正确',
    },
  ],
  'phone': [
    {
      match: /^1[0-9]{10}$/i,
      message: '手机号码格式不正确',
    },
  ],
};

// 只有新增的时候才检查密码
const computedRules = computed(() => {
  const passwordRules = {
    password: [
      {
        required: isCreate.value,
        minLength: 8,
        maxLength: 16,
        message: '密码为8到16位',
      },
    ],
    passwordConfirm: [
      {
        required: isCreate.value,
        validator: (value: string, cb: (msg: string) => void) => {
          if (
            localUserState.value.password &&
            value !== localUserState.value.password
          ) {
            cb('确认密码不一致');
          }
          return true;
        },
      },
    ],
  };

  return { ...rules, ...passwordRules };
});

const closeDialog = () => {
  formRef.value.clearValidate();
  emit('update:visible', false);
};

const handleOk = async () => {
  try {
    // 检查表单,如果有错误则不关闭
    const ret = await formRef.value.validate();
    if (!ret) {
      emit('ok', omit(localUserState.value, 'passwordConfirm'));
      closeDialog();
    }
  } catch (err) {
    console.error(err);
  }
};
const handleCancel = () => {
  emit('cancel');
  closeDialog();
};
</script>

<style lang="less" scoped></style>
