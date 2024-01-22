<script setup>
import { RouterView } from "vue-router";
import DKLogo from "@/components/DKLogo.vue";
import LangSelect from "@/views/LangSelect.vue";
import DKLayout from "@/components/DKLayout.vue";
</script>

<script>
export default {
  inject: ["switchLocale"],
  data: function () {
    return {
      page_number: 0,
      progress: 0,
      config: {},
      lang_selected: false,
      lightup: 0,
      timer: null,
      progress_detail: {},
      can_quit: true,
    };
  },
  computed: {
    eta_value: function () {
      const details = this.progress_detail;
      if (details.eta_lo > 0) {
        return this.$t("d.eta-0", {
          time_lo: details.eta_lo,
          time_hi: details.eta_hi,
        });
      } else if (details.eta_hi > 5) {
        return this.$tc("d.eta-1", details.eta_hi, { time: details.eta_hi });
      }
      return this.$t("d.eta-2");
    },
    install_info: function () {
      const details = this.progress_detail;
      if (!details.status) return "";
      const status = details.status;
      return this.$t("install.status", {
        curr: status.c,
        total: status.t,
        msg: this.$t(`install.i${status.c}`),
        perc: status.p,
      });
    },
  },
  methods: {
    on_abort: function () {
      this.$router.push("/abort");
    },
    nav_menu_bold: function (step) {
      return this.page_number >= step ? "active" : "";
    },
    lightup_seq: function (step) {
      return this.lightup >= step ? "" : "hidden";
    },
    execute_lightup: function () {
      const timer = setInterval(() => {
        if (++this.lightup >= 4) clearInterval(timer);
      }, 210);
    },
    on_lang_selected: function (id) {
      console.info(`Language: ${id}`);
      if (id == "en") {
        // default locale is always loaded before-hand
        this.lang_selected = true;
        this.execute_lightup();
        return;
      }
      // lazy load the translation strings
      this.switchLocale(id)
        .then(() => {
          this.lang_selected = true;
          this.execute_lightup();
        })
        .catch(() => {
          console.error(`Language ${id} has no translation strings`);
          this.lang_selected = true;
          this.execute_lightup();
        });
    },
    on_progress_update: function (progress) {
      this.progress_detail = progress;
    },
  },
  mounted: function () {
    this.$router.beforeEach((to) => {
      if (to.name == "error" || to.name == "abort") return null;
      this.page_number = to.meta.steps;
      this.progress = this.page_number * 25;
    });
  },
  provide: function () {
    return {
      config: {},
    };
  },
};
</script>

<template>
  <div style="padding: 0 2rem; margin-bottom: 1rem">
    <button
      class="quit-button"
      style="padding-top: 1rem"
      :aria-label="$t('d.sr-close')"
      @click="on_abort"
      @keyup.enter="on_abort"
      :disabled="!can_quit"
      v-show="lang_selected"
    >
      <img
        :alt="$t('d.sr-close-icon')"
        src="@/assets/window-close-symbolic.svg"
        width="30"
        height="30"
      />
    </button>
    <header style="width: 90%" :class="lightup_seq(1)">
      <DKLogo />
    </header>
  </div>
  <!-- language select overlay -->
  <LangSelect v-if="!lang_selected" @update:lang="on_lang_selected" />
  <!-- main content -->
  <DKLayout :main_class="lightup_seq(3)" v-if="lang_selected">
    <RouterView @update:can_quit="(v) => (can_quit = v)" />
    <template #left>
      <div class="wrapper" :class="lightup_seq(2)">
        <nav :class="nav_menu_bold(0)">{{ $t("d.nav-0") }}</nav>
        <nav :class="nav_menu_bold(1)">{{ $t("d.nav-1") }}</nav>
        <nav :class="nav_menu_bold(2)">{{ $t("d.nav-2") }}</nav>
        <nav :class="nav_menu_bold(3)">{{ $t("d.nav-3") }}</nav>
      </div>
    </template>
  </DKLayout>
  <!-- status bar -->
  <div class="status-bar" v-if="lang_selected" :class="lightup_seq(4)">
    <progress
      id="progressbar"
      :aria-label="$t('d.sr-progress')"
      :value="progress"
      max="100"
      class="progress-bar"
    ></progress>
    <span class="info-box" v-if="page_number > 1 && page_number < 4">{{
      install_info
    }}</span>
    <label for="progressbar" class="eta-box">{{ eta_value }}</label>
  </div>
</template>

<style>
main {
  transition: opacity 0.3s;
}
</style>

<style scoped>
.hidden {
  opacity: 0;
}

div,
header {
  transition: opacity 0.3s;
}

.status-bar {
  position: absolute;
  bottom: 2em;
  left: 0;
  width: 100%;
}

.progress-bar {
  appearance: none;
  display: block;
  background: var(--dk-inactive);
  border: 0;
  width: 100%;
  height: 10px;
}

.progress-bar[value]::-webkit-progress-value {
  background: var(--dk-accent);
  transition: width 200ms;
}

.progress-bar::-moz-progress-bar {
  background: var(--dk-accent);
}

.info-box {
  margin-top: 0.5em;
  float: left;
  margin-left: 0.3rem;
}

.eta-box {
  float: right;
  margin-top: 0.5em;
  margin-right: 0.5em;
}

.quit-button {
  float: right;
  cursor: pointer;
  appearance: none;
  background: transparent;
  border: 0;
}

.quit-button[disabled] {
  cursor: not-allowed;
  filter: grayscale(1);
}

header {
  line-height: 1.5;
  width: 30%;
}

nav {
  width: 100%;
  font-size: 12px;
  text-align: center;
  margin-top: 1rem;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: start;
    padding-top: 0.5rem;
    padding-right: calc(var(--section-gap) / 2);
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }

  nav {
    text-align: left;
    margin-left: calc(60px + 0.5em);
    font-size: 1rem;
    color: var(--dk-gray);

    padding: 0.3rem 0;
    margin-top: 1rem;
  }

  nav.active {
    color: #eeeeee;
    font-weight: bold;
  }
}
</style>
