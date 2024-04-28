<script setup>
import { RouterView } from 'vue-router';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api';
import DKLogo from '@/components/DKLogo.vue';
import LangSelect from '@/views/LangSelect.vue';
import DKLayout from '@/components/DKLayout.vue';
import DesktopOrInstall from '@/views/DesktopOrInstallView.vue';
import AudioPlayer from '@/components/AudioPlayer.vue';
</script>

<script>
export default {
  inject: ['switchLocale'],
  data() {
    return {
      page_number: 0,
      progress: 0,
      config: {},
      langSelected: false,
      lightup: 0,
      timer: null,
      progressDetail: {},
      can_quit: true,
      isInstall: false,
      playList: [],
    };
  },
  computed: {
    eta_value() {
      const details = this.progressDetail;
      if (details.eta_lo > 0) {
        return this.$t('d.eta-0', {
          time_lo: details.eta_lo,
          time_hi: details.eta_hi,
        });
      }

      if (details.eta_hi > 0) {
        return this.$t('d.eta-1', { time: details.eta_hi });
      }

      return '';
    },
    install_info() {
      const details = this.progressDetail;
      if (
        Object.keys(details).length === 0
        || !Object.keys(details).includes('status')
      ) return '';
      if (
        details.status
        && (details.status === 'Pending'
          || details.status === 'Error'
          || details.status === 'Finish')
      ) return '';
      const { status } = details;
      return this.$t('install.status', {
        curr: status.c,
        total: status.t,
        msg: this.$t(`install.i${status.c}`),
        perc: status.p,
      });
    },
  },
  methods: {
    on_abort() {
      const { path } = this.$router.currentRoute.value;
      if (path === '/finish') {
        this.$router.push({
          path: '/abort',
          query: { done: true },
        });
      } else {
        this.$router.push('/abort');
      }
    },
    nav_menu_bold(step) {
      return this.page_number >= step ? 'active' : '';
    },
    lightup_seq(step) {
      return this.lightup >= step ? '' : 'hidden';
    },
    execute_lightup() {
      const timer = setInterval(() => {
        if (this.lightup + 1 >= 4) clearInterval(timer);
        this.lightup += 1;
      }, 210);
    },

    // Something to do before playing
    // handleBeforePlay(next) {
    //   // There are a few things you can do here...
    //   this.currentAudioName = this.audioList[this.$refs.audioPlayer.currentPlayIndex].name;

    //   next(); // Start playing
    // },
    on_lang_selected(id) {
      if (id === 'en') {
        // default locale is always loaded before-hand
        this.langSelected = true;
        this.execute_lightup();
        return;
      }
      // lazy load the translation strings
      this.switchLocale(id)
        .then(() => {
          this.langSelected = true;
          this.execute_lightup();
        })
        .catch(() => {
          this.langSelected = true;
          this.execute_lightup();
        });
    },
    onInstallDk() {
      this.isInstall = true;
    },
    on_progress_update(progress) {
      this.progressDetail = progress;
    },
  },
  mounted() {
    window.addEventListener('contextmenu', (event) => {
      event.preventDefault();
    });

    this.$router.beforeEach((to) => {
      if (to.name === 'error' || to.name === 'abort') return;
      this.page_number = to.meta.steps;
      this.progress = this.page_number * 25;
    });
  },
  provide() {
    return {
      config: {},
    };
  },
  async created() {
    let isStop = false;
    listen('progress', (event) => {
      this.progressDetail = event.payload;
      const details = this.progressDetail;
      const { status } = details;
      const { path } = this.$router.currentRoute.value;

      if (status === 'Finish') {
        this.$router.replace('/finish');
        if (!isStop) {
          this.$refs.player.stop();
          isStop = true;
        }
      } else if (status === 'Error' && path !== '/error' && path !== '/abort') {
        this.$router.replace({
          path: `/error/${encodeURIComponent(JSON.stringify(event.payload))}`,
          query: { currentRoute: path },
        });
      } else if (
        status !== 'Pending'
        && path !== '/install'
        && path !== '/abort'
      ) {
        this.$router.replace('/install');
      }
    });

    const isInstall = await invoke('is_skip');
    this.isInstall = isInstall;

    try {
      const playlist = await invoke('get_bgm_list');
      this.playList = playlist;
    } catch (e) {
      const { path } = this.$router.currentRoute.value;
      this.$router.replace({
        path: `/error/${encodeURIComponent(e)}`,
        query: { currentRoute: path },
      });
    }
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
      v-show="langSelected && isInstall"
    >
      <img
        :alt="$t('d.sr-close-icon')"
        src="@/../assets/window-close-symbolic.svg"
        width="30"
        height="30"
      />
    </button>
    <header style="width: 90%" v-if="langSelected && isInstall">
      <DKLogo />
    </header>
  </div>
  <!-- language select overlay -->
  <LangSelect v-if="!langSelected" @update:lang="on_lang_selected" />
  <DesktopOrInstall
    v-if="langSelected && !isInstall"
    @update:install="onInstallDk"
  />
  <!-- main content -->
  <DKLayout :main_class="lightup_seq(1)" v-if="langSelected && isInstall">
    <RouterView @update:can_quit="(v) => (can_quit = v)" />
    <template #left>
      <div class="wrapper" :class="lightup_seq(1)">
        <nav :class="nav_menu_bold(0)">{{ $t("d.nav-0") }}</nav>
        <nav :class="nav_menu_bold(1)">{{ $t("d.nav-1") }}</nav>
        <nav :class="nav_menu_bold(2)">{{ $t("d.nav-2") }}</nav>
        <nav :class="nav_menu_bold(3)">{{ $t("d.nav-3") }}</nav>
      </div>
      <div v-if="page_number >= 2">
        <AudioPlayer ref="player" :list="playList"></AudioPlayer>
      </div>
    </template>
  </DKLayout>
  <!-- status bar -->
  <div
    class="status-bar"
    v-if="langSelected && isInstall"
    :class="lightup_seq(4)"
  >
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
