import localeMessageBox from '@/components/message-box/locale/zh-CN/zh-CN';
import localeLogin from '@/views/login/locale/zh-CN/zh-CN';

import localeWorkplace from '@/views/dashboard/workplace/locale/zh-CN/zh-CN';

import localeSettings from './zh-CN/settings';
import localeMenu from './zh-CN/menu';
import localeNavbar from './zh-CN/navbar';

export default {
  ...localeNavbar,
  ...localeMenu,
  ...localeSettings,
  ...localeMessageBox,
  ...localeLogin,
  ...localeWorkplace,
};
