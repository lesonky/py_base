<template>
  <div class="container">
    <Breadcrumb :items="['用户管理', '用户列表']" :use-i18n="false" />
    <a-card class="general-card" title="查询用户">
      <a-row>
        <a-col :flex="1">
          <a-form
            :model="formModel"
            :label-col-props="{ span: 6 }"
            :wrapper-col-props="{ span: 18 }"
            label-align="left"
          >
            <a-row :gutter="18">
              <a-col :span="8">
                <a-form-item field="string" label="用户名">
                  <a-input v-model="formModel.name" placeholder="用户名" />
                </a-form-item>
              </a-col>
              <a-col :span="8">
                <a-form-item field="string" label="昵称">
                  <a-input v-model="formModel.nickName" placeholder="昵称" />
                </a-form-item>
              </a-col>
              <a-col :span="8">
                <a-form-item field="string" label="E-Mail">
                  <a-input v-model="formModel.email" placeholder="E-Mail" />
                </a-form-item>
              </a-col>
              <a-col :span="8">
                <a-form-item field="string" label="电话号码">
                  <a-input v-model="formModel.phone" placeholder="电话号码" />
                </a-form-item>
              </a-col>
              <a-col :span="8">
                <a-form-item field="string" label="角色">
                  <a-select
                    v-model="formModel.role"
                    :style="{ width: '100%' }"
                    placeholder="角色"
                    allow-clear
                  >
                    <a-option
                      v-for="(item, index) in userStore.roleList"
                      :key="index"
                      >{{ item.name }}</a-option
                    >
                  </a-select>
                </a-form-item>
              </a-col>
            </a-row>
          </a-form>
        </a-col>
        <a-divider style="height: 84px" direction="vertical" />
        <a-col :flex="'86px'" style="text-align: right">
          <a-space direction="vertical" :size="18">
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
      <a-divider style="margin-top: 0" />
      <a-row style="margin-bottom: 16px">
        <a-col :span="24" style="text-align: right">
          <a-button type="primary" @click="createUser">
            <template #icon>
              <icon-plus />
            </template>
            新增用户
          </a-button>
        </a-col>
      </a-row>
      <a-table
        :loading="loading"
        :pagination="pagination"
        :data="userList"
        :bordered="false"
        @page-change="onPageChange"
      >
        <template #columns>
          <a-table-column title="用户名">
            <template #cell="{ record }">
              <a-avatar
                v-if="record.name"
                :size="16"
                shape="square"
                :style="{ backgroundColor: '#00d0b6' }"
              >
                <img v-if="record.avatar" alt="avatar" :src="record.avatar" />
                <template v-else>
                  {{ record.name }}
                </template>
              </a-avatar>
              {{ record.name }}
            </template>
          </a-table-column>
          <a-table-column title="昵称">
            <template #cell="{ record }">
              <a-avatar
                v-if="record.nickName"
                :size="16"
                shape="square"
                :style="{ backgroundColor: '#00d0b6' }"
              >
                <img v-if="record.avatar" alt="avatar" :src="record.avatar" />
                <template v-else>
                  {{ record.nickName }}
                </template>
              </a-avatar>
              {{ record.nickName }}
            </template>
          </a-table-column>
          <a-table-column title="E-Mail">
            <template #cell="{ record }">
              <a :href="`mailto:${record.email}`">{{ record.email }}</a>
            </template>
          </a-table-column>
          <a-table-column title="电话号码" data-index="phone" />
          <a-table-column title="注册时间">
            <template #cell="{ record }">
              {{ dayjs(record.registrationDate).format('YYYY-MM-DD HH:mm:ss') }}
            </template>
          </a-table-column>
          <a-table-column title="角色">
            <template #cell="{ record }">
              <a-tag
                :color="record.role.name === 'admin' ? 'blue' : 'gray'"
                style="margin-right: 8px"
              >
                {{ record.role.name }}
              </a-tag>
            </template>
          </a-table-column>
          <a-table-column title="是否激活">
            <template #cell="{ record }">
              <a-popconfirm
                v-if="!isSelf(record)"
                :content="`是否 ${record.isActive ? '停用' : '激活'} 用户 ${
                  record.nickName || record.name
                }?`"
                @ok="changeUserActive(record)"
              >
                <a-switch :model-value="record.isActive" />
              </a-popconfirm>
              <a-tooltip v-else content="不能停用自己.">
                <a-switch disable :model-value="record.isActive"
              /></a-tooltip>
            </template>
          </a-table-column>
          <a-table-column title="操作">
            <template #cell="{ record }">
              <a-popconfirm
                v-if="!isSelf(record)"
                :content="`是否删除用户 ${record.nickName || record.name}?`"
                @ok="deleteUser(record)"
              >
                <a-button type="text" size="small"> 删除 </a-button>
              </a-popconfirm>
              <a-button type="text" size="small" @click="editUser(record)">
                编辑
              </a-button>
              <a-popconfirm
                :on-before-ok="passwordResetCheck"
                @ok="resetUserPassword(record)"
                @cancel="formRef.resetFields()"
              >
                <template #content>
                  {{
                    `为 ${
                      isSelf(record) ? '自己' : record.nickName || record.name
                    } 重置密码${isSelf(record) ? ' , 修改后需要重新登录' : ''}`
                  }}
                  <a-divider style="margin: 8px 0" />
                  <a-form
                    ref="formRef"
                    size="small"
                    :model="passwordConfirm"
                    :style="{ width: '400px' }"
                  >
                    <a-form-item
                      field="password"
                      label="新密码"
                      :rules="[
                        { required: true, message: '请填写新密码' },
                        {
                          minLength: 8,
                          maxLength: 16,
                          message: '密码为8到16位',
                        },
                      ]"
                      :validate-trigger="['change', 'input']"
                    >
                      <a-input
                        v-model="passwordConfirm.password"
                        placeholder="请输入新密码"
                      />
                    </a-form-item>
                    <a-form-item
                      field="passwordAgain"
                      label="确认密码"
                      :rules="passwordAgainRule"
                      :validate-trigger="['change', 'input']"
                    >
                      <a-input
                        v-model="passwordConfirm.passwordAgain"
                        placeholder="请确认密码"
                      />
                    </a-form-item>
                  </a-form>
                </template>
                <a-button type="text" size="small"> 重置密码 </a-button>
              </a-popconfirm>
            </template>
          </a-table-column>
        </template>
      </a-table>
    </a-card>
    <UserDialog
      v-model:visible="userDialogVisible"
      :user-state="currentEditUser"
      @ok="userDialogOk"
    ></UserDialog>
  </div>
</template>

<script lang="ts" setup>
import { onBeforeMount, ref, reactive } from 'vue';
import dayjs from 'dayjs';

import { getUserList, upsertUser, QueryUserData } from '@/api/user';
import useLoading from '@/hooks/loading';
import { useUserStore } from '@/store';

import { UserState } from '@/store/modules/user/types';
import { Pagination } from '#/global';
import { omit } from 'lodash';
import UserDialog from '../components/UserDialog.vue';

const generateFormModel: () => QueryUserData = () => {
  return {
    name: '',
    nickName: '',
    email: '',
    phone: '',
    role: '',
  };
};

const { loading, setLoading } = useLoading(true);
const userStore = useUserStore();
const userList = ref<UserState[]>([]);

const formModel = ref(generateFormModel());
const formRef = ref();

const userDialogVisible = ref(false);
// 记录当前正在修改的用户
const currentEditUser = ref<UserState>({});

const basePagination: Pagination = {
  pageSize: 10,
  pageNum: 1,
};
const pagination = reactive({ ...basePagination });

const passwordConfirm = reactive({
  password: '',
  passwordAgain: '',
  confirm: false,
});

const passwordAgainRule = [
  {
    validator: (value: string, cb: (msg: string) => void) => {
      if (value !== passwordConfirm.password) {
        cb('确认密码不一致');
      }
      return true;
    },
  },
];

const fetchData = async (params: QueryUserData) => {
  setLoading(true);
  try {
    const { data } = await getUserList(params);
    userList.value = data.items;
    pagination.pageNum = params.pageNum || 0;
    pagination.total = data.total;
  } catch (err) {
    console.error(err);
  } finally {
    setLoading(false);
  }
};

const search = () => {
  fetchData({ ...basePagination, ...formModel.value });
};

const reset = () => {
  formModel.value = generateFormModel();
  search();
};

const onPageChange = (pageNum: number) => {
  fetchData({ ...formModel.value, ...basePagination, pageNum });
};

const isSelf = (user: UserState) => userStore.accountId === user.accountId;

const upsertUserInfo = async (user: UserState & { password?: string }) => {
  setLoading(true);
  try {
    const { data } = await upsertUser({ ...user });
    if (isSelf(user)) {
      userStore.setInfo(user);
    }
    if (user.accountId) {
      const userIndex = userList.value.findIndex(
        (item) => item.accountId === data.accountId
      );
      userList.value[userIndex] = data;
    } else {
      reset();
    }
  } catch (err) {
    console.error(err);
  } finally {
    setLoading(false);
  }
};

const changeUserActive = (user: UserState) => {
  upsertUserInfo({ ...user, isActive: !user.isActive });
};

const deleteUser = (user: UserState) => {
  upsertUserInfo({ ...user, isDeleted: true });
};

const resetUserPassword = async (user: UserState) => {
  try {
    await upsertUserInfo({ ...user, password: passwordConfirm.password });
    // 为自己重置密码后需要重新登录
    if (isSelf(user)) {
      userStore.logout();
    }
  } catch (err) {
    console.error(err);
  } finally {
    formRef.value?.resetFields();
  }
};

const passwordResetCheck = (
  done: (closed: boolean) => void
): void | boolean => {
  // 检查,做提示
  formRef.value.validate();

  // 手动检查,判断是否关闭
  if (
    passwordConfirm.passwordAgain === passwordConfirm.password &&
    passwordConfirm.password.length >= 8 &&
    passwordConfirm.password.length <= 16
  ) {
    done(true);
  }
  return false;
};

const createUser = () => {
  currentEditUser.value = {};
  userDialogVisible.value = true;
};

const editUser = (user: UserState) => {
  currentEditUser.value = { ...user };
  userDialogVisible.value = true;
};

const userDialogOk = (user: UserState) => {
  if (user.accountId) {
    upsertUserInfo(omit(user, 'password'));
  } else {
    upsertUserInfo(user);
  }
};

onBeforeMount(() => {
  fetchData({ ...formModel.value, ...pagination });
});
</script>

<style lang="less" scoped>
.container {
  height: 100%;
  padding: 0 20px;
  background-color: var(--color-fill-2);
}
</style>
