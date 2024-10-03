<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import DKBottomActions from '@/components/DKBottomActions.vue';
import DKSpinner from '@/components/DKSpinner.vue';
import DKBottomSteps from '@/components/DKBottomSteps.vue';
import { defineComponent, inject } from 'vue';
import { Config } from '../config.ts';
import DKBody from '../components/DKBody.vue';
</script>

<script lang="ts">
export default defineComponent({
  data() {
    const config = inject('config') as Config;

    return {
      loading: false,
      new_partition_size: null as number | null,
      isEFI: false,
      config,
    };
  },
  async created() {
    const { device } = this.config;

    try {
      this.isEFI = await invoke('is_efi_api', {
        dev: (this.config as Config).device.path,
      });
    } catch (e) {
      const { path } = this.$router.currentRoute.value;

      this.$router.replace({
        path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
        query: { currentRoute: path },
      });
    }

    if (this.isEFI) {
      this.new_partition_size = Math.round(
        (device.size - 512 * 1024 * 1024) / 1024 / 1024 / 1024,
      );
    } else {
      this.new_partition_size = Math.round(device.size / 1024 / 1024 / 1024);
    }
  },
});
</script>

<template>
  <DKBody>
    <div v-if="!loading">
      <h1>{{ $t("part.title") }}</h1>
      <p>{{ $t("part.p4") }}</p>
      <ul>
        <i18n-t v-if="isEFI" keypath="part.l1" tag="li">
          <strong>512MiB</strong>
        </i18n-t>
        <i18n-t keypath="part.l2" tag="li">
          <strong>{{ new_partition_size }}GiB</strong>
        </i18n-t>
      </ul>
    </div>
    <!-- loading screen -->
    <div class="loading" v-else>
      <h1>{{ $t("part.title") }}</h1>
      <DKSpinner :title="$t('part.r1')" />
    </div>
  </DKBody>
  <DKBottomActions v-if="!loading">
    <DKBottomSteps :replace="true" :query="{ autoPart: true }"></DKBottomSteps>
  </DKBottomActions>
</template>

<style scoped>
.column-80 {
  font-weight: 600;
  width: 80%;
  display: inline-block;
}

.column-20 {
  width: 20%;
  display: inline-block;
}

/* p.secondary span {
  color: var(--dk-gray);
} */

.error-msg {
  margin: 1rem;
}

p.secondary {
  margin: 0;
}
</style>
