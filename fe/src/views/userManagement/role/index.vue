<template>
  <div class="container">
    <Breadcrumb :items="['用户管理', '角色列表']" :use-i18n="false" />
    <a-card class="general-card" title="角色列表">
      <a-form
        :model="formModel"
        :label-col-props="{ span: 6 }"
        :wrapper-col-props="{ span: 18 }"
        label-align="left"
      >
        <a-row :gutter="18">
          <a-col :span="8">
            <a-form-item field="name" label="角色名">
              <a-input v-model="formModel.name" placeholder="角色名" />
            </a-form-item>
          </a-col>
          <a-col :span="16" style="text-align: right">
            <a-button type="primary" @click="addRole">
              <template #icon>
                <icon-plus />
              </template>
              新增角色
            </a-button>
          </a-col>
        </a-row>
      </a-form>
      <a-divider style="margin-top: 0" />
      <a-table
        :loading="loading"
        :data="computedRoleList"
        :bordered="false"
        :pagination="false"
      >
        <template #columns>
          <a-table-column title="ID" data-index="id" :width="36" />
          <a-table-column title="角色名称" data-index="name" :width="200" />
          <a-table-column title="权限">
            <template #cell="{ record }">
              <a-tag
                v-for="item in record.permissions"
                :key="item"
                :color="item === 'All' ? 'yellow' : 'blue'"
                style="margin: 0 8px 8px 0"
                >{{ item }}</a-tag
              >
            </template>
          </a-table-column>
          <a-table-column title="操作" :width="200">
            <template #cell="{ record }">
              <a-popconfirm
                v-if="record.name !== 'admin' && !isCurrentRole(record)"
                content="是否删该角色?"
                @ok="deleteRole(record)"
              >
                <a-button type="text" size="small"> 删除 </a-button>
              </a-popconfirm>
              <a-button
                v-if="record.name !== 'admin' && !isCurrentRole(record)"
                type="text"
                size="small"
                @click="editRole(record)"
              >
                编辑
              </a-button>
            </template>
          </a-table-column>
        </template>
      </a-table>
    </a-card>
    <a-modal v-model:visible="editRoleDialogVisible" width="640px">
      <template #title>
        {{ currentRole.id ? `编辑角色 ${currentRole.name}` : '新增角色' }}
      </template>
      <a-form
        ref="formRef"
        :model="currentRole"
        :style="{ width: '600px' }"
        :rules="rules"
      >
        <a-form-item field="name" label="角色名称">
          <a-input
            v-model="currentRole.name"
            :disabled="!!currentRole.id"
            placeholder="请输入角色名称"
          />
        </a-form-item>
        <a-alert
          v-if="currentRole.permissions!.includes('All')"
          closable
          type="warning"
          style="margin-bottom: 16px"
        >
          All 权限包含所有权限
        </a-alert>
        <a-form-item field="permissions" label="角色权限">
          <a-checkbox-group
            v-model="currentRole.permissions"
            direction="vertical"
            :options="permissionList"
          />
        </a-form-item>
      </a-form>
      <template #footer>
        <a-button type="outline" @click="handleCancel">取消</a-button
        ><a-button type="primary" @click="handleOk">确认</a-button>
      </template>
    </a-modal>
  </div>
</template>

<script lang="ts" setup>
import { ref, onBeforeMount, computed } from 'vue';

import {
  getRoleList,
  getPermissionList,
  cerateRole,
  updateRole,
  deleteRoleById,
} from '@/api/user';
import useLoading from '@/hooks/loading';
import { useUserStore } from '@/store';
import { RoleType } from '@/store/modules/user/types';

const genBaseRole = (): RoleType => ({
  name: '',
  permissions: [],
});

const formModel = ref({ name: '' });
const formRef = ref();
const roleList = ref<RoleType[]>([]);
const permissionList = ref<string[]>([]);
const { loading, setLoading } = useLoading(true);
const userStore = useUserStore();

const editRoleDialogVisible = ref(false);
const currentRole = ref<RoleType>(genBaseRole());
const rules = {
  name: [
    {
      required: true,
      message: '请输入角色名',
    },
    {
      match: /^[a-z][a-z0-9_]+$/gi,
      message: '格式:字母开头,字母数字下划线组成',
    },
    {
      validator: (value: string, cb: (msg: string) => void) => {
        if (
          !currentRole.value.id &&
          roleList.value.find((item) => item.name === value.trim())
        ) {
          cb('角色名称已存在');
        }
        return true;
      },
    },
  ],
  permissions: [
    {
      required: true,
      validator: (value: string, cb: (msg: string) => void) => {
        if (value.length === 0) {
          cb('必须包含至少一个权限');
        }
        return true;
      },
    },
  ],
};

const computedRoleList = computed(() => {
  return formModel.value.name
    ? roleList.value.filter((item) => item.name.includes(formModel.value.name))
    : roleList.value;
});

const isCurrentRole = (role: RoleType) => {
  return userStore.role!.name === role.name;
};

const fetchData = async () => {
  setLoading(true);
  try {
    const {
      data: { items },
    } = await getRoleList();
    roleList.value = items;
  } catch (err) {
    console.error(err);
  } finally {
    setLoading(false);
  }
};

const fetchPromissionList = async () => {
  try {
    const {
      data: { permissions },
    } = await getPermissionList();
    permissionList.value = permissions;
  } catch (err) {
    console.error(err);
  }
};

const deleteRole = async (role: RoleType) => {
  try {
    await deleteRoleById({ id: role.id! });
    fetchData();
  } catch (err) {
    console.error(err);
  }
};
const editRole = (role: RoleType) => {
  currentRole.value = { ...role };
  editRoleDialogVisible.value = true;
};

const addRole = () => {
  // 清空数据
  currentRole.value = genBaseRole();
  editRoleDialogVisible.value = true;
};

const closeDialog = () => {
  formRef.value.clearValidate();
  editRoleDialogVisible.value = false;
};

const handleCancel = () => {
  closeDialog();
};
const handleOk = async () => {
  const ret = await formRef.value.validate();
  if (ret) return;
  try {
    if (currentRole.value.id) {
      await updateRole(currentRole.value);
    } else {
      await cerateRole(currentRole.value);
    }
    closeDialog();
    // 同步刷新一下全局的列表
    userStore.fetchRoleList();
  } catch (err) {
    console.error(err);
  } finally {
    fetchData();
  }
};

onBeforeMount(() => {
  fetchData();
  fetchPromissionList();
});
</script>

<style lang="less" scoped>
.container {
  height: 100%;
  padding: 0 20px;
  background-color: var(--color-fill-2);
}
</style>
