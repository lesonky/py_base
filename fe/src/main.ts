import { createApp } from 'vue';
import ArcoVue from '@arco-design/web-vue';
import ArcoVueIcon from '@arco-design/web-vue/es/icon';
import globalComponents from '@/components';
import { install } from '@icon-park/vue-next/es/all';
import router from './router';
import store from './store';
import i18n from './locale';
import directive from './directive';
// import './mock';
import App from './App.vue';
import '@arco-design/web-vue/dist/arco.css';
import '@icon-park/vue-next/styles/index.css';
import '@/assets/style/global.less';
import '@/api/interceptor';

import 'virtual:windi.css';
import 'virtual:windi-devtools';

const app = createApp(App);

// 安装 icon-park ,设置全局前缀 icon-park
install(app, 'icon-park');

app.use(ArcoVue, {});
app.use(ArcoVueIcon);

app.use(router);
app.use(store);
app.use(i18n);
app.use(globalComponents);
app.use(directive);

app.mount('#app');
