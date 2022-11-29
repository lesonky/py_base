<template>
  <div class="navbar">
    <div class="left-side">
      <a-space>
        <img
          alt="logo"
          src="//p3-armor.byteimg.com/tos-cn-i-49unhts6dw/dfdba5317c0c20ce20e64fac803d52bc.svg~tplv-49unhts6dw-image.image"
          @click="goBackToRoot"
        />
        <a-typography-title
          :style="{ margin: 0, fontSize: '18px' }"
          :heading="5"
        >
          {{ appStore.navbarTitle }}
        </a-typography-title>
        <icon-menu-fold
          v-if="appStore.device === 'mobile'"
          style="font-size: 22px; cursor: pointer"
          @click="toggleDrawerMenu"
        />
      </a-space>
    </div>
    <ul class="right-side">
      <li v-if="appStore.searchBtn">
        <a-tooltip :content="$t('settings.search')">
          <a-button class="nav-btn" type="outline" :shape="'circle'">
            <template #icon>
              <icon-search />
            </template>
          </a-button>
        </a-tooltip>
      </li>
      <li v-if="appStore.i18nBtn">
        <a-tooltip :content="$t('settings.language')">
          <a-button
            class="nav-btn"
            type="outline"
            :shape="'circle'"
            @click="setDropDownVisible"
          >
            <template #icon>
              <icon-language />
            </template>
          </a-button>
        </a-tooltip>
        <a-dropdown trigger="click" @select="changeLocale as any">
          <div ref="triggerBtn" class="trigger-btn"></div>
          <template #content>
            <a-doption
              v-for="item in locales"
              :key="item.value"
              :value="item.value"
            >
              {{ item.label }}
            </a-doption>
          </template>
        </a-dropdown>
      </li>
      <li v-if="appStore.themeBtn">
        <a-tooltip
          :content="
            theme === 'light'
              ? $t('settings.navbar.theme.toDark')
              : $t('settings.navbar.theme.toLight')
          "
        >
          <a-button
            class="nav-btn"
            type="outline"
            :shape="'circle'"
            @click="handleToggleTheme"
          >
            <template #icon>
              <icon-moon-fill v-if="theme === 'dark'" />
              <icon-sun-fill v-else />
            </template>
          </a-button>
        </a-tooltip>
      </li>
      <li v-if="appStore.messageBtn">
        <a-tooltip :content="$t('settings.navbar.alerts')">
          <div class="message-box-trigger">
            <a-badge :count="9" dot>
              <a-button
                class="nav-btn"
                type="outline"
                :shape="'circle'"
                @click="setPopoverVisible"
              >
                <icon-notification />
              </a-button>
            </a-badge>
          </div>
        </a-tooltip>
        <a-popover
          trigger="click"
          :arrow-style="{ display: 'none' }"
          :content-style="{ padding: 0, minWidth: '400px' }"
          content-class="message-popover"
        >
          <div ref="refBtn" class="ref-btn"></div>
          <template #content>
            <message-box />
          </template>
        </a-popover>
      </li>
      <li v-if="appStore.fullscerrnBtn">
        <a-tooltip
          :content="
            isFullscreen
              ? $t('settings.navbar.screen.toExit')
              : $t('settings.navbar.screen.toFull')
          "
        >
          <a-button
            class="nav-btn"
            type="outline"
            :shape="'circle'"
            @click="toggleFullScreen"
          >
            <template #icon>
              <icon-fullscreen-exit v-if="isFullscreen" />
              <icon-fullscreen v-else />
            </template>
          </a-button>
        </a-tooltip>
      </li>
      <li v-if="appStore.globalSettingsBtn">
        <a-tooltip :content="$t('settings.title')">
          <a-button
            class="nav-btn"
            type="outline"
            :shape="'circle'"
            @click="setVisible"
          >
            <template #icon>
              <icon-settings />
            </template>
          </a-button>
        </a-tooltip>
      </li>
      <li>
        <a-dropdown trigger="click">
          <div style="color: rgb(var(--gray-8)); cursor: pointer">
            <a-avatar
              :size="32"
              :style="{
                marginRight: '8px',
                cursor: 'pointer',
                backgroundColor: '#00d0b6',
              }"
            >
              <img v-if="!!avatar" alt="avatar" :src="avatar" />
              <template v-else>
                {{ displayName }}
              </template>
            </a-avatar>
            <span>
              {{ displayName }}
            </span>
          </div>
          <template #content>
            <a-doption>
              <a-space v-if="debug" @click="switchRoles">
                <icon-tag />
                <span>
                  {{ $t('messageBox.switchRoles') }}
                </span>
              </a-space>
            </a-doption>
            <!-- <a-doption>
              <a-space @click="$router.push({ name: 'Info' })">
                <icon-user />
                <span>
                  {{ $t('messageBox.userCenter') }}
                </span>
              </a-space>
            </a-doption> -->
            <a-doption>
              <a-space @click="editUserState">
                <icon-settings />
                <span>
                  {{ $t('messageBox.userSettings') }}
                </span>
              </a-space>
            </a-doption>
            <a-doption>
              <a-space @click="handleLogout">
                <icon-export />
                <span>
                  {{ $t('messageBox.logout') }}
                </span>
              </a-space>
            </a-doption>
          </template>
        </a-dropdown>
      </li>
    </ul>
    <UserDialog
      v-model:visible="userDialogVisible"
      :user-state="userState"
      @ok="updateUserState"
    ></UserDialog>
  </div>
</template>

<script lang="ts" setup>
import { computed, ref, inject, onBeforeMount } from 'vue';
import { Message } from '@arco-design/web-vue';
import { useDark, useToggle, useFullscreen } from '@vueuse/core';
import { useAppStore, useUserStore } from '@/store';
import { useRouter } from 'vue-router';
import { omit } from 'lodash-es';
import { LOCALE_OPTIONS } from '@/locale';
import useLocale from '@/hooks/locale';
import useUser from '@/hooks/user';
import debug from '@/utils/env';
import UserDialog from '@/views/userManagement/components/UserDialog.vue';
import { UserState } from '@/store/modules/user/types';
import { upsertUser } from '@/api/user';
import defaultSettings from '@/config/settings.json';
import MessageBox from '../message-box/index.vue';

const appStore = useAppStore();
const userStore = useUserStore();
const router = useRouter();
const { logout } = useUser();
const { changeLocale } = useLocale();
const { isFullscreen, toggle: toggleFullScreen } = useFullscreen();
const locales = [...LOCALE_OPTIONS];

const userDialogVisible = ref(false);
const getUserStateForEdit = () => ({
  name: userStore.name,
  nickName: userStore.nickName,
  avatar: userStore.avatar,
  email: userStore.email,
  introduction: userStore.introduction,
  phone: userStore.phone,
  accountId: userStore.accountId,
  role: userStore.role,
  isActive: true,
  isDeleted: false,
});
const userState = ref<UserState>({});

const avatar = computed(() => {
  return userStore.avatar;
});

const displayName = computed(() => {
  return userStore.nickName || userStore.name;
});

const theme = computed(() => {
  return appStore.theme;
});
const isDark = useDark({
  selector: 'body',
  attribute: 'arco-theme',
  valueDark: 'dark',
  valueLight: 'light',
  storageKey: 'arco-theme',
  onChanged(dark: boolean) {
    // overridden default behavior
    appStore.toggleTheme(dark);
  },
});
const toggleTheme = useToggle(isDark);
const handleToggleTheme = () => {
  toggleTheme();
};
const setVisible = () => {
  appStore.updateSettings({ globalSettings: true });
};
const refBtn = ref();
const triggerBtn = ref();
const setPopoverVisible = () => {
  const event = new MouseEvent('click', {
    view: window,
    bubbles: true,
    cancelable: true,
  });
  refBtn.value.dispatchEvent(event);
};
const handleLogout = () => {
  logout();
};
const setDropDownVisible = () => {
  const event = new MouseEvent('click', {
    view: window,
    bubbles: true,
    cancelable: true,
  });
  triggerBtn.value.dispatchEvent(event);
};
const switchRoles = async () => {
  const res = await userStore.switchRoles();
  Message.success(res.name as string);
};

const goBackToRoot = () => {
  router.push({
    path: '/login',
  });
};

const toggleDrawerMenu = inject('toggleDrawerMenu');

const editUserState = () => {
  userDialogVisible.value = true;
  userState.value = getUserStateForEdit();
};

const updateUserState = async (user: UserState & { password?: string }) => {
  try {
    if (!user.password) {
      user = omit(user, 'password');
    }
    const { data } = await upsertUser({ ...user });
    if (user.password) {
      // 修改密码后需要重新登录
      logout();
    } else {
      userStore.setInfo(data);
    }
  } catch (err) {
    console.error(err);
  }
};
// 不展示主题按钮时，默认为setting中设置的主题
onBeforeMount(() => {
  if (appStore.themeBtn === false) {
    toggleTheme(defaultSettings.theme === 'dark');
  }
});
</script>

<style scoped lang="less">
.navbar {
  display: flex;
  justify-content: space-between;
  height: 100%;
  background-color: var(--color-bg-2);
  border-bottom: 1px solid var(--color-border);
}

.left-side {
  display: flex;
  align-items: center;
  padding-left: 20px;
}

.right-side {
  display: flex;
  padding-right: 20px;
  list-style: none;

  :deep(.locale-select) {
    border-radius: 20px;
  }

  li {
    display: flex;
    align-items: center;
    padding: 0 10px;
  }

  a {
    color: var(--color-text-1);
    text-decoration: none;
  }

  .nav-btn {
    color: rgb(var(--gray-8));
    font-size: 16px;
    border-color: rgb(var(--gray-2));
  }

  .trigger-btn,
  .ref-btn {
    position: absolute;
    bottom: 14px;
  }

  .trigger-btn {
    margin-left: 14px;
  }
}
</style>

<style lang="less">
.message-popover {
  .arco-popover-content {
    margin-top: 0;
  }
}
</style>
