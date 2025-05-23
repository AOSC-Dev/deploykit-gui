<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import DKListSelect from '@/components/DKListSelect.vue';
import DKBottomSteps from '@/components/DKBottomSteps.vue';
import { defineComponent, inject } from 'vue';
import DKBody from '../components/DKBody.vue';
import { Config } from '../config.ts';
</script>

<script lang="ts">
interface Device {
  model: string;
  path: string;
  size: number;
}

interface SquashfsInfo {
  downloadSize: number;
  instSize: number;
}

export default defineComponent({
  inject: ['humanSize'],
  data() {
    return {
      config: inject('config') as Config,
      devices: [] as Device[],
      selected: null as number | null,
      loading: true,
      requireSize: null as number | null,
      err_msg: '',
      humanSize: inject('humanSize') as Function,
    };
  },
  methods: {
    validate() {
      if (this.requireSize === null || this.selected == null) {
        return false;
      }
      if (this.requireSize > (this.devices[this.selected] as Device).size) {
        this.err_msg = this.$t('device.e1', {
          size: Math.ceil(this.requireSize / 1024 / 1024 / 1024),
        });
        return false;
      }

      this.err_msg = '';

      return true;
    },
    select() {
      if (this.requireSize === null || this.selected == null) {
        return false;
      }
      if (this.requireSize > (this.devices[this.selected] as Device).size) {
        this.err_msg = this.$t('device.e1', {
          size: Math.ceil(this.requireSize / 1024 / 1024 / 1024),
        });
        return false;
      }

      this.err_msg = '';

      return true;
    },
  },
  async created() {
    try {
      const isDebug = await invoke('is_debug');

      if (isDebug) {
        this.config.device = {
          model: 'Test Driver',
          path: '/dev/loop30',
          size: 11451400000000,
        };
        this.$router.replace('/partitions');
      } else {
        const devices = (await invoke('list_devices')) as Device[];
        this.devices = devices;
      }

      const v = this.config.variant;

      let requireSize;
      if (!this.config.is_offline_install) {
        const squashfsInfo = (await invoke('get_squashfs_info', {
          v,
          url: this.config.mirror?.url,
        })) as SquashfsInfo;

        requireSize = squashfsInfo.downloadSize + squashfsInfo.instSize;
      } else {
        const info = (await invoke('get_squashfs_info', { v })) as SquashfsInfo;
        requireSize = info.instSize * 1.25;
      }

      const isEFI = await invoke('is_efi_api');

      if (isEFI) {
        requireSize += 512 * 1024 * 1024;
      }

      this.requireSize = requireSize;

      if (this.config.device) {
        this.selected = this.devices.findIndex(
          (value) => value.path === this.config.device.path,
        );
      }
    } catch (e) {
      const { path } = this.$router.currentRoute.value;

      this.$router.replace({
        path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
        query: { currentRoute: path },
      });
    }

    this.loading = false;
  },
});
</script>

<template>
  <DKBody>
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
      </section>
    </div>
    <div class="error-msg" v-if="!loading">
      <span>{{ err_msg }}</span>
    </div>
  </DKBody>
  <DKBottomSteps
    :trigger="
      () => {
        if (selected === null) {
          return;
        }
        config.device = devices[selected];
      }
    "
    :can_proceed="selected != null"
    :guard="validate"
  />
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
