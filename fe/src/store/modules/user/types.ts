export type RoleType = '' | '*' | 'admin' | 'user';
export interface UserState {
  name?: string;
  avatar?: string;
  organization?: string;
  email?: string;
  introduction?: string;
  phone?: string;
  registrationDate?: string;
  accountId?: string;
  role: RoleType;
}
