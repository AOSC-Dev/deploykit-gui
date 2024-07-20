<script setup lang="ts">
import DKBody from '@/components/DKBody.vue';
import { invoke } from '@tauri-apps/api';
import DKStripButton from '@/components/DKStripButton.vue';
import DKBottomActions from '@/components/DKBottomActions.vue';
import DKSpinner from '@/components/DKSpinner.vue';
import { inject, defineComponent } from 'vue';
import { Config } from '../config.ts';
</script>

<template>
  <DKBody>
    <!-- loading screen -->
    <div class="loading" v-if="loading && !running">
      <h1>{{ $t("network.title") }}</h1>
      <DKSpinner :title="$t('loading')" />
    </div>
    <div class="loading" v-else-if="loading && running">
      <h1>{{ $t("network.title") }}</h1>
      <DKSpinner :title="$t('loading')" />
    </div>
    <div v-else>
      <div>
        <h1>{{ $t("network.title") }}</h1>
        <p>{{ $t("network.p1") }}</p>
        <p>{{ $t("network.p2") }}</p>
      </div>
      <DKBottomActions>
        <DKStripButton :text="$t('network.b1')" @click="runNmtui">
          <img src="@/../assets/network.svg" height="18" />
        </DKStripButton>
      </DKBottomActions>
    </div>
  </DKBody>
</template>

<script lang="ts">
export default defineComponent({
  data() {
    return {
      config: inject('config') as Config,
      loading: false,
      running: false,
    };
  },
  methods: {
    async runNmtui() {
      try {
        this.loading = true;
        this.running = true;
        await invoke('run_nmtui', { lang: this.config.locale.locale });
        this.loading = false;
        this.running = false;
        this.$router.back();
      } catch (e) {
        const { path } = this.$router.currentRoute.value;

        this.$router.replace({
          path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
          query: { currentRoute: path },
        });
      }
    },
  },
});
</script>
