<script setup lang="ts">
import DKSpinner from '@/components/DKSpinner.vue';
import { invoke } from '@tauri-apps/api/core';
import { inject, defineComponent } from 'vue';
import { Config, Mirror } from '../config.ts';
</script>

<template>
  <div class="loading" v-if="loading">
    <h1>{{ $t("mirror.b2") }}</h1>
    <DKSpinner :title="$t('mirror.k1')" />
  </div>
</template>

<script lang="ts">
interface Recipe {
  mirrors: Mirror[];
}

export default defineComponent({
  data() {
    return {
      loading: true,
      selected: null,
      config: inject('config') as Config,
    };
  },
  async created() {
    try {
      const recipe = await invoke('get_recipe');
      const { mirrors } = recipe as Recipe;
      const speedtest = (await invoke('mirrors_speedtest', {
        mirrors,
      })) as Mirror[];
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
});
</script>
