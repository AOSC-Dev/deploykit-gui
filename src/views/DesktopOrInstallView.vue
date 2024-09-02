<script setup lang="ts">
import DKLayout from '@/components/DKLayout.vue';
</script>

<script lang="ts">
import { invoke } from '@tauri-apps/api';
import { defineComponent, inject } from 'vue';

interface Config {
  locale: {
    aosc: string;
    inst: string;
  };
}

export default defineComponent({
  data() {
    return {
      config: inject('config') as Config,
    };
  },
  methods: {
    goInstall() {
      this.$emit('update:install');
    },
    async exit() {
      await invoke('cancel_install_and_exit', { resetConfig: false });
    },
    async reboot() {
      await invoke('reboot');
    },
  },
});
</script>

<template>
  <DKLayout>
    <section style="margin-top: 6.5vh">
      <h1>{{ $t("desktop-or-install.title") }}</h1>
      <div class="list-container">
        <button class="button" @click="goInstall">
          <div class="entry-box">
            <img src="@/../assets/install.svg" :height="30" :width="30" />
            <div class="button-box">
              <span
                style="font-size: 1rem; font-weight: 600; margin-bottom: 0.3rem"
              >
                {{ $t("desktop-or-install.install-system-title") }}
              </span>
              <span style="font-size: 0.88rem; line-height: 1.2">
                {{ $t("desktop-or-install.install-system-p1") }}
              </span>
            </div>
          </div>
        </button>
        <button class="button" @click="exit">
          <div class="entry-box">
            <img src="@/../assets/rescue.svg" :height="30" :width="30" />
            <div class="button-box">
              <span
                style="font-size: 1rem; font-weight: 600; margin-bottom: 0.3rem"
              >
                {{ $t("desktop-or-install.rescue-and-recovery") }}
              </span>
              <span style="font-size: 0.88rem; line-height: 1.2">
                {{ $t("desktop-or-install.rescue-and-recovery-p1") }}
              </span>
            </div>
          </div>
        </button>
        <button class="button" @click="reboot">
          <div class="entry-box">
            <img src="@/../assets/reboot.svg" :height="30" :width="30" />
            <div class="button-box">
              <span
                style="font-size: 1rem; font-weight: 600; margin-bottom: 0.3rem"
              >
                {{ $t("desktop-or-install.reboot-title") }}
              </span>
              <span style="font-size: 0.88rem; line-height: 1.2">
                {{ $t("desktop-or-install.reboot-p1") }}
              </span>
            </div>
          </div>
        </button>
      </div>
    </section>
    <template #left>
      <div style="margin-top: 5vh">
        <img />
        <div style="line-height: 1">
          <h1 style="font-size: 3rem; text-align: right; margin-bottom: 0">
            {{ config.locale.aosc }}
          </h1>
          <h2 style="font-size: 1.25rem; text-align: right">
            {{ config.locale.inst }}
          </h2>
        </div>
      </div>
    </template>
  </DKLayout>
</template>

<style scoped>
.list-container {
  overflow-y: auto;
}

.button-box {
  text-align: left;
  width: 100%;
  margin: 1.2rem;
  margin-bottom: 0;
  display: flex;
  flex-flow: column;
}

.list-container button {
  height: unset;
  line-height: 1.2rem;
  background-color: transparent;
  width: 100%;
  align-content: flex-start;
  border: 0;
  color: white;
  display: flex;
  justify-content: flex-start;
}

.list-container button:hover {
  background: #dddddd56;
}

.entry-box {
  display: flex;
  align-content: flex-start;
}

.entry-box img {
  margin: 1rem;
  margin-top: 1.5rem;
  margin-right: 0.3rem;
}
</style>
