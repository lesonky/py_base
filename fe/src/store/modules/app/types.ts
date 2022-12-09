export interface AppState {
  theme: string;
  colorWeak: boolean;
  navbar: boolean;
  menu: boolean;
  hideMenu: boolean;
  menuCollapse: boolean;
  footer: boolean;
  themeColor: string;
  menuWidth: number;
  globalSettings: boolean;
  device: string;
  tabBar: boolean;
  globalSettingsBtn: boolean;
  i18nBtn: boolean;
  themeBtn: boolean;
  messageBtn: boolean;
  fullscerrnBtn: boolean;
  searchBtn: boolean;
  navbarTitle: string;
  loginBanner: boolean;
  canSignup: boolean;
  horizontalMenu: boolean;
  [key: string]: unknown;
}
