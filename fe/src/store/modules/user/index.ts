import { defineStore } from 'pinia';
import {
  login as userLogin,
  logout as userLogout,
  getUserInfo,
  getRoleList,
  LoginData,
} from '@/api/user';
import { setToken, clearToken } from '@/utils/auth';
import { removeRouteListener } from '@/utils/route-listener';
import { UserState, RoleType } from './types';

const useUserStore = defineStore('user', {
  state: (): UserState => ({
    name: undefined,
    nickName: undefined,
    avatar: undefined,
    email: undefined,
    introduction: undefined,
    phone: undefined,
    registrationDate: undefined,
    accountId: undefined,
    role: undefined,
    roleList: [],
  }),

  getters: {
    userInfo(
      state: UserState
    ): Omit<UserState, 'role'> & { permissions: string[]; role: string } {
      return {
        ...state,
        permissions: state.role?.permissions || [],
        role: state.role?.name || '',
      };
    },
  },

  actions: {
    switchRoles() {
      return new Promise<RoleType>((resolve) => {
        this.role =
          this.role!.name === 'admin'
            ? { id: 0, name: 'user', permissions: [] }
            : { id: 0, name: 'admin', permissions: ['all'] };
        resolve(this.role);
      });
    },
    // Set user's information
    setInfo(partial: Partial<UserState>) {
      this.$patch(partial);
    },

    // Reset user's information
    resetInfo() {
      this.$reset();
    },

    // Get user's information
    async info() {
      try {
        const { data: userInfo } = await getUserInfo();
        const {
          data: { items: roleList },
        } = await getRoleList();
        this.setInfo({ ...userInfo, roleList });
      } catch (err) {
        console.error(err);
        this.logout();
      }
    },

    // Login
    async login(loginForm: LoginData) {
      try {
        const res = await userLogin(loginForm);
        setToken(res.data.token);
      } catch (err) {
        clearToken();
        throw err;
      }
    },

    // Logout
    async logout() {
      await userLogout();

      this.resetInfo();
      clearToken();
      removeRouteListener();
    },
  },
});

export default useUserStore;
