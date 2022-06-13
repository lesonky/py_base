import localeMessageBox from '@/components/message-box/locale/en-US/en-US';
import localeLogin from '@/views/login/locale/en-US/en-US';

import localeWorkplace from '@/views/dashboard/workplace/locale/en-US/en-US';

import localeSettings from './en-US/settings';
import localeMenu from './en-US/menu';
import localeNavbar from './en-US/navbar';

export default {
  ...localeNavbar,
  ...localeMenu,
  ...localeSettings,
  ...localeMessageBox,
  ...localeLogin,
  ...localeWorkplace,
};
