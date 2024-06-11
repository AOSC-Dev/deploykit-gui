<script setup>
import DKSpinner from '@/components/DKSpinner.vue';
import { invoke } from '@tauri-apps/api';
</script>

<template>
  <div class="loading" v-if="loading">
    <h1>{{ $t("mirror.b2") }}</h1>
    <DKSpinner :title="$t('mirror.k1')" />
  </div>
</template>

<script>
export default {
  inject: ['config'],
  data() {
    return {
      loading: true,
      selected: null,
    };
  },
  async created() {
    try {
      const recipe = await invoke('get_recipe');
      const { mirrors } = recipe;
      const speedtest = await invoke('mirrors_speedtest', { mirrors });
      const sel = speedtest[0];
      this.config.mirrors = speedtest;
      this.config.mirror = sel;
      this.loading = false;
      this.$router.replace('/mirrors-sel');
    } catch (e) {
      const { path } = this.$router.currentRoute.value;

      this.$router.replace({
        path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
        query: { currentRoute: path },
      });
    }
  },
};
</script>
