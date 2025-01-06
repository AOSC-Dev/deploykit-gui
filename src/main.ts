import { createApp } from 'vue';
import { createI18n } from 'vue-i18n';
import humanFormat from 'human-format';
import VuePlyr from 'vue-plyr';
import ElementPlus from 'element-plus';
import App from './App.vue';
import router from './router/index.ts';

import '@/../assets/vue-plyr.css';

import enMsg from './locales/en.json';
// import 'element-plus/dist/index.css';
import '../assets/element.scss';
import '../assets/main.css';

const app = createApp(App);

const i18n = createI18n({
  legacy: false,
  locale: 'en',
  fallbackLocale: 'en',
});

async function switchLocale(locale: string) {
  const msg = await import(`./locales/${locale}.json`);
  i18n.global.setLocaleMessage(locale, msg.default);
  i18n.global.locale.value = locale;
  document.querySelector('html')?.setAttribute('lang', locale);
}

// provided functions in the global scope
app.provide('switchLocale', switchLocale);
app.provide('humanSize', humanFormat.bytes);

app.use(router);
app.use(i18n);
app.use(ElementPlus);
app.use(VuePlyr);

// load default translations
i18n.global.setLocaleMessage('en', enMsg);
i18n.global.locale.value = 'en';

app.mount('#app');
