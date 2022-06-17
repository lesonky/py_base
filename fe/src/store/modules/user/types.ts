export type RoleType = { id?: number; name: string; permissions?: string[] };
export interface UserState {
  name?: string;
  nickName?: string;
  avatar?: string;
  organization?: string;
  email?: string;
  introduction?: string;
  phone?: string;
  registrationDate?: string;
  accountId?: string;
  role?: RoleType;
  isActive?: boolean;
  isDeleted?: boolean;
  roleList?: RoleType[];
  password?: string;
}
