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
async function handleEFI(obj) {
  const o = obj;
  const espParts = await invoke('find_all_esp_parts');
  if (espParts.length === 1 && !o.config.efi_partition) {
    const selectEFIPart = espParts[0];
    if (selectEFIPart.parent_path !== o.config.device.path) {
      o.$router.replace(`/esp/${encodeURIComponent(JSON.stringify(espParts))}`);
    } else {
      o.config.efi_partition = selectEFIPart;
      o.$router.replace('/users');
    }
  } else if (espParts.length > 1 && !o.config.efi_partition) {
    o.$router.replace(`/esp/${encodeURIComponent(JSON.stringify(espParts))}`);
  } else if (!o.config.efi_partition) {
    const selectEFIPart = espParts[0];
    o.config.efi_partition = selectEFIPart;
    o.$router.replace('/users');
  }
}

export default {
  inject: ['config'],
  props: {
    autoPart: Boolean,
  },
  data() {
    return {
      loading: false,
    };
  },
  async created() {
    if (!this.autoPart) {
      this.loading = true;
      try {
        invoke('auto_partition', { dev: this.config.device.path }).catch(
          (e) => {
            this.$router.replace({
              path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
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
          path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
          query: { openGparted: true, currentRoute: '/partitions' },
        });
      }
      this.loading = false;
    } else {
      this.loading = true;
      if (this.config.is_efi && !this.config.efi_partition) {
        await handleEFI(this);
      } else {
        this.loading = false;
      }
    }
  },
};
</script>
