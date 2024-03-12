<script setup>
import { invoke } from '@tauri-apps/api';
import DKStripButton from '@/components/DKStripButton.vue';
import DKBottomActions from '@/components/DKBottomActions.vue';
import DKListSelect from '@/components/DKListSelect.vue';
import DKSpinner from '@/components/DKSpinner.vue';
import DKBottomSteps from '@/components/DKBottomSteps.vue';
import DKBody from '../components/DKBody.vue';
</script>

<script>
export default {
  inject: ['config', 'humanSize'],
  data() {
    return {
      selected: null,
      gparted: false,
      partitions: [],
      loading: true,
      error_msg: '',
      sqfs_size: null,
      new_partition_size: null,
      is_efi: false,
      new_disk: !this.partitions || this.partitions.length < 1,
    };
  },
  computed: {
    valid() {
      return !this.gparted && (this.new_disk || this.selected != null);
    },
  },
  watch: {
    loading(newValue) {
      this.$emit('update:can_quit', !newValue && !this.gparted);
    },
    gparted(newValue) {
      this.$emit('update:can_quit', !newValue && !this.loading);
    },
  },
  methods: {
    comment(comment) {
      switch (comment) {
        case 'esp':
          return this.$t('part.k1');
        case 'mbr':
          return this.$t('part.k3');
        case 'winre':
          return this.$t('part.k2');
        default: {
          /* empty */
        }
      }
      if (comment.length > 20) {
        return this.$t('part.k5', { other_os: comment.substring(0, 20) });
      }
      return this.$t('part.k4', { other_os: comment });
    },
    async launch_gparted() {
      invoke('gparted');
      const isDebug = await invoke('is_debug');
      let device;

      if (isDebug) {
        device = {
          path: '/dev/loop30',
          model: 'loop',
          size: 50 * 1024 * 1024 * 1024,
        };
      } else {
        device = this.config.device;
      }

      // 检查 GParted 之后分区表是否合法
      try {
        await invoke('disk_is_right_combo', { disk: device.path });
      } catch (e) {
        const { path } = this.$router.currentRoute.value;

        this.$router.replace({
          path: `/error/${encodeURIComponent(e)}`,
          query: { openGparted: true, currentRoute: path },
        });
      }

      this.gparted = true;
      this.loading = true;

      try {
        const req = await invoke('list_partitions', { dev: device.path });
        const resp = req;
        this.partitions = resp;

        const v = this.config.variant;
        const squashfsInfo = await invoke('get_squashfs_info', {
          v,
          url: this.config.mirror.url,
        });
        this.sqfs_size = squashfsInfo.downloadSize + squashfsInfo.instSize;

        if (this.partitions.length !== 0) {
          this.new_disk = false;
          await invoke('disk_is_right_combo', { disk: device.path });
        } else {
          this.new_disk = true;
          const isEFI = await invoke('is_efi_api');
          this.is_efi = isEFI;

          if (isEFI) {
            this.new_partition_size = Math.round(
              (device.size - 512 * 1024 * 1024) / 1024 / 1024 / 1024,
            );
          } else {
            this.new_partition_size = Math.round(
              device.size / 1024 / 1024 / 1024,
            );
          }
        }
      } catch (e) {
        const { path } = this.$router.currentRoute.value;

        this.$router.replace({
          path: `/error/${encodeURIComponent(e)}`,
          query: { openGparted: true, currentRoute: path },
        });
      }

      this.gparted = false;
      this.loading = false;
    },
    validate() {
      if (this.new_disk) {
        return true;
      }

      if (this.partitions.length === 0) {
        return false;
      }

      const { size } = this.partitions[this.selected];

      if (size < this.sqfs_size) {
        this.error_msg = this.$t('part.e1', {
          size: Math.ceil(this.sqfs_size / 1024 / 1024 / 1024),
        });
        return false;
      }

      if (!['ext4', 'xfs'].includes(this.partitions[this.selected].fs_type)) {
        this.error_msg = this.$t('part.e2');
        return false;
      }

      return true;
    },
    select() {
      const { size } = this.partitions[this.selected];

      if (size < this.sqfs_size) {
        this.error_msg = this.$t('part.e1', {
          size: Math.ceil(this.sqfs_size / 1024 / 1024 / 1024),
        });
        return;
      }

      if (!['ext4', 'xfs'].includes(this.partitions[this.selected].fs_type)) {
        this.error_msg = this.$t('part.e2');
        return;
      }

      this.error_msg = '';
    },
    next() {
      if (!this.new_disk) {
        this.config.partition = this.partitions[this.selected];
      }
    },
  },
  async created() {
    const isDebug = await invoke('is_debug');
    let device;
    if (isDebug) {
      device = {
        path: '/dev/loop30',
        model: 'loop',
        size: 50 * 1024 * 1024 * 1024,
      };
    } else {
      device = this.config.device;
    }

    try {
      const req = await invoke('list_partitions', { dev: device.path });
      const resp = req;
      this.partitions = resp;

      if (this.config.partition) {
        if (this.partitions.length === 0) {
          this.config.partition = null;
        } else {
          this.selected = this.partitions.findIndex(
            (v) => v.path === this.config.partition.path,
          );
        }
      }

      const v = this.config.variant;
      const squashfsInfo = await invoke('get_squashfs_info', {
        v,
        url: this.config.mirror.url,
      });
      this.sqfs_size = squashfsInfo.downloadSize + squashfsInfo.instSize;

      if (this.partitions.length !== 0) {
        this.new_disk = false;
        await invoke('disk_is_right_combo', { disk: device.path });
        const espParts = await invoke('find_all_esp_parts');
        if (espParts.length === 0) {
          const { path } = this.$router.currentRoute.value;

          this.$router.replace({
            path: `/error/${encodeURIComponent('Has no EFI Partition!')}`,
            query: { openGparted: true, currentRoute: path },
          });
        } else if (espParts.length === 1) {
          const selectEFIPart = espParts[0];
          this.config.efi_partition = selectEFIPart;
        } else if (!this.config.efi_partition) {
          this.$router.push(
            `/esp/${encodeURIComponent(JSON.stringify(espParts))}`,
          );
        }
      } else {
        this.new_disk = true;
        const isEFI = await invoke('is_efi_api');
        this.is_efi = isEFI;

        if (isEFI) {
          this.new_partition_size = Math.round(
            (device.size - 512 * 1024 * 1024) / 1024 / 1024 / 1024,
          );
        } else {
          this.new_partition_size = Math.round(
            device.size / 1024 / 1024 / 1024,
          );
        }
      }
    } catch (e) {
      const { path } = this.$router.currentRoute.value;

      this.$router.replace({
        path: `/error/${encodeURIComponent(e)}`,
        query: { openGparted: true, currentRoute: path },
      });
    }

    this.loading = false;
  },
};
</script>

<template>
  <DKBody>
    <div v-if="!loading">
      <h1>{{ $t("part.title") }}</h1>
      <section v-if="!new_disk">
        <p>{{ $t("part.p1") }}</p>
        <section>
          <DKListSelect
            :no_margin="true"
            v-model:selected="selected"
            :options="partitions"
            :is_limit_height="true"
            :click_fn="select"
          >
            <template #item="option">
              <div style="width: 100%">
                <span class="column-80">{{ option.path }}</span>
                <span class="column-20">{{ humanSize(option.size) }}</span>
                <p class="secondary">
                  <span>{{ option.fs_type || $t("part.k0") }}</span>
                </p>
              </div>
            </template>
          </DKListSelect>
        </section>
      </section>

      <section v-if="new_disk">
        <p>{{ $t("part.p2") }}</p>
        <ul>
          <i18n-t v-if="is_efi" keypath="part.l1" tag="li">
            <strong>512MiB</strong>
          </i18n-t>
          <i18n-t keypath="part.l2" tag="li">
            <strong>{{ new_partition_size }}GiB</strong>
          </i18n-t>
        </ul>
        <p>
          {{ $t("part.p3") }}
        </p>
      </section>
    </div>
    <!-- loading screen -->
    <div class="loading" v-else>
      <h1>{{ $t("part.title") }}</h1>
      <DKSpinner :title="$t('part.r1')" />
    </div>
    <div class="error-msg">
      <span>{{ error_msg }}</span>
    </div>
    <DKBottomActions v-if="!gparted && !loading">
      <DKStripButton
        omit_bline="1"
        @click="launch_gparted"
        :text="$t('part.b1')"
      >
        <img src="@/../assets/drive-harddisk-root-symbolic.svg" height="18" />
      </DKStripButton>
      <DKStripButton @click="$router.push('/autopart')" :text="$t('part.b2')">
        <img src="@/../assets/drive-harddisk-root-symbolic.svg" height="18" />
      </DKStripButton>
    </DKBottomActions>
  </DKBody>
  <DKBottomSteps :trigger="next" :guard="validate" :can_proceed="valid" />
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
  height: 1rem;
}

p.secondary {
  margin: 0;
}
</style>
