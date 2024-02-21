<script setup>
import { invoke } from '@tauri-apps/api';
import DKListSelect from '@/components/DKListSelect.vue';
import DKBottomSteps from '@/components/DKBottomSteps.vue';
</script>

<script>
export default {
  inject: ['config', 'humanSize'],
  data() {
    return {
      devices: [],
      selected: null,
      loading: true,
      requireSize: null,
      err_msg: '',
    };
  },
  methods: {
    validate() {
      if (this.requireSize > this.devices[this.selected].size) {
        this.err_msg = this.$t('device.e1', {
          size: Math.ceil(this.requireSize / 1024 / 1024 / 1024),
        });
        return false;
      }

      return true;
    },
    select() {
      if (this.requireSize > this.devices[this.selected].size) {
        this.err_msg = this.$t('device.e1', {
          size: Math.ceil(this.requireSize / 1024 / 1024 / 1024),
        });
        return false;
      }

      return true;
    },
  },
  async created() {
    try {
      const devices = await invoke('list_devices');
      this.devices = devices;

      const v = this.config.variant;
      const squashfsInfo = await invoke('get_squashfs_info', {
        v,
        url: this.config.mirror.url,
      });

      let requireSize = squashfsInfo.downloadSize + squashfsInfo.instSize;

      const isEFI = await invoke('is_efi_api');

      if (isEFI) {
        requireSize += 512 * 1024 * 1024;
      }

      this.requireSize = requireSize;
    } catch (e) {
      this.$router.replace(`/error/${encodeURIComponent(e)}`);
    }

    this.loading = false;
  },
};
</script>

<template>
  <div v-if="!loading">
    <h1>{{ $t("device.title") }}</h1>
    <p>{{ $t("device.p1") }}</p>
    <section>
      <DKListSelect
        :no_margin="true"
        v-model:selected="selected"
        :is_limit_height="true"
        :options="devices"
        :click_fn="select"
      >
        <template #item="option">
          <div style="width: 100%">
            <span class="column-80">{{ option.model }}</span>
            <span class="column-20">{{ humanSize(option.size) }}</span>
            <p class="secondary">
              <span>{{ option.path }}</span>
            </p>
          </div>
        </template>
      </DKListSelect>
      <DKBottomSteps
        :trigger="() => (config.device = devices[selected])"
        :can_proceed="selected != null"
        :guard="validate"
      />
    </section>
  </div>
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

p.secondary span {
  color: var(--dk-gray);
}

p.secondary {
  margin: 0;
}
</style>
