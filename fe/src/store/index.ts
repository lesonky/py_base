import { createPinia } from 'pinia';
import useAppStore from './modules/app';
import useUserStore from './modules/user';
import useTabBarStore from './modules/tab-bar';
import useDictionaryStore from './modules/dictionary';

const pinia = createPinia();

export { useAppStore, useUserStore, useTabBarStore, useDictionaryStore };
export default pinia;
