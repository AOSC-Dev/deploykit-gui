import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { createI18n } from "vue-i18n";
import ipc from "./ipc";
import humanFormat from "human-format";

import messages from "./locales/en.json";

import "./assets/main.css";

const app = createApp(App);

const i18n = createI18n({
  legacy: false,
  locale: "en",
  fallbackLocale: "en",
});

async function switchLocale(locale) {
  const messages = await import(`./locales/${locale}.json`);
  i18n.global.setLocaleMessage(locale, messages.default);
  i18n.global.locale.value = locale;
  document.querySelector("html").setAttribute("lang", locale);
}

// provided functions in the global scope
app.provide("switchLocale", switchLocale);
app.provide("humanSize", humanFormat.bytes);

app.use(router);
app.use(i18n);
app.use(ipc);

// load default translations
i18n.global.setLocaleMessage("en", messages);
i18n.global.locale.value = "en";

app.mount("#app");
