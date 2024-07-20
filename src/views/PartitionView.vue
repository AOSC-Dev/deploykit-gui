<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import DKStripButton from '@/components/DKStripButton.vue';
import DKBottomActions from '@/components/DKBottomActions.vue';
import DKListSelect from '@/components/DKListSelect.vue';
import DKSpinner from '@/components/DKSpinner.vue';
import DKBottomSteps from '@/components/DKBottomSteps.vue';
import { defineComponent, inject } from 'vue';
import DKBody from '../components/DKBody.vue';
import {
  Config,
  Device,
  IsLvmDeviceResp,
  Partition,
  SquashfsInfo,
} from '../config.ts';
</script>

<script lang="ts">
export default defineComponent({
  data() {
    return {
      selected: null as null | number,
      gparted: false,
      partitions: [] as Partition[],
      loading: true,
      error_msg: '',
      sqfs_size: null as null | number,
      new_partition_size: null as null | number,
      is_efi: false,
      new_disk: !this.partitions || this.partitions.length < 1,
      rightCombine: false,
      efiError: false,
      unsupportedTable: false,
      lvmError: false,
      otherError: false,
      config: inject('config') as Config,
      humanSize: inject('humanSize') as Function,
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
    async checkDisk(device: Device) {
      try {
        const req = (await invoke('list_partitions', {
          dev: device.path,
        })) as Partition[];
        const resp = req;
        this.partitions = resp;

        if (this.config.partition) {
          if (this.partitions.length === 0) {
            this.config.partition = null;
          } else {
            this.selected = this.partitions.findIndex(
              (v) => v.path === this.config.partition?.path,
            );
          }
        }

        const v = this.config.variant;

        if (!this.config.is_offline_install) {
          const squashfsInfo = (await invoke('get_squashfs_info', {
            v,
            url: this.config.mirror?.url,
          })) as SquashfsInfo;
          this.sqfs_size = squashfsInfo.downloadSize + squashfsInfo.instSize;
        } else {
          const info = (await invoke('get_squashfs_info', {
            v,
          })) as SquashfsInfo;
          this.sqfs_size = info.instSize * 1.25;
        }

        if (this.partitions.length !== 0) {
          this.new_disk = false;

          if (this.is_efi) {
            try {
              const espParts = (await invoke(
                'find_all_esp_parts',
              )) as Partition[];
              if (espParts.length === 0) {
                this.error_msg = this.$t('part.e4');
                this.efiError = true;
                return;
              }
              this.error_msg = '';
              this.efiError = false;
            } catch (e) {
              const { path } = this.$router.currentRoute.value;

              this.$router.replace({
                path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
                query: { openGparted: 1, currentRoute: path },
              });
            }
          }

          try {
            await invoke('disk_is_right_combo', { disk: device.path });
            this.rightCombine = true;
          } catch (e: any) {
            switch (e.data.t) {
              case 'WrongCombine': {
                const { bootmode, table } = e.data.data;
                this.error_msg = this.$t('part.e3', {
                  bootmode,
                  table,
                });
                this.rightCombine = false;
                break;
              }
              case 'UnsupportedTable': {
                const { table } = e.data.data;
                this.error_msg = this.$t('part.e5', { table });
                this.unsupportedTable = true;
                break;
              }
              case 'PartitionType': {
                this.error_msg = e.message;
                this.otherError = true;
                break;
              }
              default: {
                this.rightCombine = true;
                this.unsupportedTable = false;
                this.otherError = false;
                this.error_msg = '';
              }
            }
          }

          try {
            const isLvmDevice = (
              (await invoke('is_lvm_device', {
                dev: device.path,
              })) as IsLvmDeviceResp
            ).data;

            if (isLvmDevice) {
              this.error_msg = this.$t('part.e6');
              this.lvmError = true;
              return;
            }
          } catch (err) {
            const { path } = this.$router.currentRoute.value;

            this.$router.replace({
              path: `/error/${encodeURIComponent(JSON.stringify(err))}`,
              query: { openGparted: 1, currentRoute: path },
            });
          }
        } else {
          this.new_disk = true;
          this.rightCombine = true;
          this.unsupportedTable = false;
          this.otherError = false;
          this.error_msg = '';

          if (this.is_efi) {
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
          path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
          query: { openGparted: 1, currentRoute: path },
        });
      }

      this.loading = false;
    },
    comment(comment: string) {
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
    launch_gparted() {
      this.loading = true;
      this.gparted = true;
      invoke('gparted', { lang: this.config.locale.locale }).then(() => {
        invoke('is_debug').then((res) => {
          let device;

          if (res) {
            device = {
              path: '/dev/loop30',
              model: 'loop',
              size: 50 * 1024 * 1024 * 1024,
            };
          } else {
            device = this.config.device;
          }

          // 检查 GParted 之后分区表是否合法
          this.checkDisk(device).then(() => {
            this.gparted = false;
            this.loading = false;
          });
        });
      });
    },
    validate() {
      if (this.new_disk) {
        return true;
      }

      if (
        !this.rightCombine
        || this.efiError
        || this.unsupportedTable
        || this.lvmError
        || this.otherError
      ) {
        return false;
      }

      if (this.partitions.length === 0) {
        return false;
      }

      if (this.selected === null || this.sqfs_size === null) {
        return false;
      }

      const { size } = this.partitions[this.selected];

      if (size < this.sqfs_size) {
        this.error_msg = this.$t('part.e1', {
          size: Math.ceil(this.sqfs_size / 1024 / 1024 / 1024),
        });
        return false;
      }

      const fsType = this.partitions[this.selected].fs_type;
      if (fsType !== null && !['ext4', 'xfs'].includes(fsType)) {
        this.error_msg = this.$t('part.e2');
        return false;
      }

      return true;
    },
    select() {
      if (this.selected === null || this.sqfs_size === null) {
        return;
      }

      const { size } = this.partitions[this.selected];

      if (size < this.sqfs_size) {
        if (
          this.efiError
          || this.otherError
          || this.unsupportedTable
          || !this.rightCombine
          || this.lvmError
        ) {
          return;
        }
        this.error_msg = this.$t('part.e1', {
          size: Math.ceil(this.sqfs_size / 1024 / 1024 / 1024),
        });
        return;
      }

      const fsType = this.partitions[this.selected].fs_type;

      if (fsType !== null && !['ext4', 'xfs'].includes(fsType)) {
        if (
          this.efiError
          || this.otherError
          || this.unsupportedTable
          || !this.rightCombine
          || this.lvmError
        ) {
          return;
        }
        this.error_msg = this.$t('part.e2');
        return;
      }

      if (
        this.rightCombine
        && !this.efiError
        && !this.unsupportedTable
        && !this.lvmError
        && !this.otherError
        && size >= this.sqfs_size
      ) {
        this.error_msg = '';
      }
    },
    next() {
      if (!this.new_disk) {
        if (this.selected === null || this.sqfs_size === null) {
          return;
        }

        this.config.partition = this.partitions[this.selected];
        if (!this.config.partition.fs_type) {
          this.config.partition.fs_type = 'ext4';
        }
      }

      this.config.is_efi = this.is_efi;
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

    const isEFI = (await invoke('is_efi_api')) as boolean;
    this.is_efi = isEFI;

    await this.checkDisk(device);

    this.loading = false;
  },
});
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
    <div class="loading" v-else-if="loading && !gparted">
      <h1>{{ $t("part.title") }}</h1>
      <DKSpinner :title="$t('part.r1')" />
    </div>
    <div class="loading" v-else-if="loading && gparted">
      <h1>{{ $t("part.title") }}</h1>
      <DKSpinner :title="$t('part.r2')" />
    </div>
    <div class="error-msg" v-if="!loading">
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
  <DKBottomSteps
    :trigger="next"
    :guard="validate"
    :can_proceed="valid"
    :query="new_disk ? { autoPart: true } : {}"
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

/* p.secondary span {
  color: var(--dk-gray);
} */

.error-msg {
  height: 1.2rem;
}

p.secondary {
  margin: 0;
}
</style>
