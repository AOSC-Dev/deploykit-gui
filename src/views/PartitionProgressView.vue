<script setup lang="ts">
import DKSpinner from '@/components/DKSpinner.vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { defineComponent, inject } from 'vue';
import { Config, Partition } from '../config.ts';
</script>

<template>
  <div class="loading" v-if="loading">
    <h1>{{ $t("part.title") }}</h1>
    <DKSpinner :title="$t('loading')" />
  </div>
</template>

<script lang="ts">
export default defineComponent({
  inject: ['config'],
  props: {
    autoPart: Boolean,
  },
  data() {
    return {
      loading: false,
      config: inject('config') as Config,
    };
  },
  methods: {
    async handleEFI() {
      try {
        const espParts = (await invoke('find_all_esp_parts')) as Partition[];

        if (espParts.length === 1 && !this.config.efi_partition) {
          const selectEFIPart = espParts[0];
          if (selectEFIPart.parent_path !== this.config.device.path) {
            this.$router.replace(
              `/esp/${encodeURIComponent(JSON.stringify(espParts))}`,
            );
          } else {
            this.config.efi_partition = selectEFIPart;
            this.$router.replace('/users');
          }
        } else if (espParts.length > 1 && !this.config.efi_partition) {
          this.$router.replace(
            `/esp/${encodeURIComponent(JSON.stringify(espParts))}`,
          );
        } else if (!this.config.efi_partition) {
          const selectEFIPart = espParts[0];
          this.config.efi_partition = selectEFIPart;
          this.$router.replace('/users');
        }
      } catch (e) {
        const { path } = this.$router.currentRoute.value;

        this.$router.replace({
          path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
          query: { openGparted: 1, currentRoute: path },
        });
      }
    },
  },
  async created() {
    if (this.autoPart) {
      this.loading = true;
      try {
        invoke('auto_partition', { dev: this.config.device.path }).catch(
          (e) => {
            this.$router.replace({
              path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
              query: { openGparted: 1, currentRoute: '/partitions' },
            });
          },
        );

        // TODO: 完善类型
        await listen('auto_partition_progress', (event: any) => {
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
          path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
          query: { openGparted: 1, currentRoute: '/partitions' },
        });
      }
      this.loading = false;
    } else {
      this.loading = true;
      if (this.config.is_efi && !this.config.efi_partition) {
        await this.handleEFI();
      } else {
        this.loading = false;
        this.$router.replace('/users');
      }
    }
  },
});
</script>
