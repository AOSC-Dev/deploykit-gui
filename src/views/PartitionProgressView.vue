<script setup>
import DKSpinner from '@/components/DKSpinner.vue';
import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
</script>

<template>
  <div class="loading" v-if="loading">
    <h1>{{ $t("part.title") }}</h1>
    <DKSpinner :title="$t('loading')" />
  </div>
</template>

<script>
export default {
  inject: ['config'],
  data() {
    return {
      loading: false,
    };
  },
  async created() {
    if (!this.config.partition) {
      this.loading = true;
      try {
        invoke('auto_partition', { dev: this.config.device.path }).catch(
          (e) => {
            this.$router.replace({
              path: `/error/${encodeURIComponent(e)}`,
              query: { openGparted: true, currentRoute: '/partitions' },
            });
          },
        );

        await listen('auto_partition_progress', (event) => {
          if (event.payload.status === 'Finish') {
            const parts = event.payload.res.Ok;

            if (parts.length === 2) {
              const sysPart = parts[1];
              const efiPart = parts[0];
              this.config.partition = sysPart;
              this.config.efi_partition = efiPart;
            } else {
              const sysPart = parts[0];
              this.config.partition = sysPart;
            }
            this.loading = false;
            this.$router.replace('/users');
          } else {
            this.loading = true;
          }
        });
      } catch (e) {
        this.$router.replace({
          path: `/error/${encodeURIComponent(e)}`,
          query: { openGparted: true, currentRoute: '/partitions' },
        });
      }
      this.loading = false;
    } else {
      this.$router.replace('/users');
    }
  },
};
</script>
