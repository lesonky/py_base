import axios from 'axios';
import { UserState, RoleType } from '@/store/modules/user/types';

export interface LoginData {
  name: string;
  password: string;
}

export interface QueryUserData {
  name?: string;
  nickName?: string;
  email?: string;
  phone?: string;
  role?: string;
  pageNum?: number;
  pageSize?: number;
}
export interface LoginRes {
  token: string;
}

// 登录
export function login(data: LoginData) {
  return axios.post<LoginRes>('/api/user/login', data);
}

// 登出
export function logout() {
  return axios.post<LoginRes>('/api/user/logout');
}

// 获取用户信息
export function getUserInfo() {
  return axios.post<UserState>('/api/user/info');
}

// 查询用户列表
export function getUserList(params: QueryUserData) {
  return axios.get<{ total: number; items: UserState[] }>('/api/user/list', {
    params,
  });
}

// 新增/修改用户信息
export function upsertUser(data: Partial<UserState>) {
  return axios.post<UserState>('/api/user/upsert', data);
}

// 获取角色列表
export function getRoleList() {
  return axios.get<{ items: RoleType[] }>('/api/role/list');
}
