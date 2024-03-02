<script setup>
import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
import DKStripButton from '@/components/DKStripButton.vue';
import DKBottomActions from '@/components/DKBottomActions.vue';
import DKListSelect from '@/components/DKListSelect.vue';
import DKSpinner from '@/components/DKSpinner.vue';
import DKBottomSteps from '@/components/DKBottomSteps.vue';
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
      this.gparted = true;
      this.loading = true;
      try {
        const { device } = this.config;
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
        this.$router.replace({
          path: `/error/${encodeURIComponent(e)}`,
          query: { openGparted: true },
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
    async next() {
      if (!this.new_disk) {
        this.config.partition = this.partitions[this.selected];
      } else {
        try {
          invoke('auto_partition', { dev: this.config.device.path }).catch(
            (e) => {
              this.$router.replace(`/error/${encodeURIComponent(e)}`);
            },
          );

          setTimeout(async () => {
            try {
              await listen('auto_partition_progress', (event) => {
                setTimeout(() => {
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
                  } else {
                    this.loading = true;
                  }
                }, 200);
              });
            } catch (e) {
              this.$router.replace({
                path: `/error/${encodeURIComponent(e)}`,
                query: { openGparted: true },
              });
            }
          }, 200);
        } catch (e) {
          this.$router.replace({
            path: `/error/${encodeURIComponent(e)}`,
            query: { openGparted: true },
          });
        }
      }
    },
  },
  async created() {
    const { device } = this.config;
    // const device = {
    //   path: "/dev/loop30",
    //   model: "loop",
    //   size: 50 * 1024 * 1024 * 1024,
    // };
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
        const espParts = await invoke('find_all_esp_parts');
        if (espParts.length === 0) {
          this.$router.replace({
            path: `/error/${encodeURIComponent('Has no EFI Partition!')}`,
            query: { openGparted: true },
          });
        } else if (espParts.length === 1) {
          const selectEFIPart = espParts[0];
          this.config.efi_partition = selectEFIPart;
        } else if (!this.config.efi_partition) {
          this.$router.replace(
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
      this.$router.replace({
        path: `/error/${encodeURIComponent(e)}`,
        query: { openGparted: true },
      });
    }

    this.loading = false;
  },
};
</script>

<template>
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
    <p>{{ error_msg }}</p>
  </div>
  <DKBottomActions v-if="!gparted && !loading">
    <DKStripButton omit_bline="1" @click="launch_gparted" :text="$t('part.b1')">
      <img src="@/../assets/drive-harddisk-root-symbolic.svg" height="18" />
    </DKStripButton>
    <DKStripButton @click="$router.push('/autopart')" :text="$t('part.b2')">
      <img src="@/../assets/drive-harddisk-root-symbolic.svg" height="18" />
    </DKStripButton>
    <DKBottomSteps :trigger="next" :guard="validate" :can_proceed="valid">
    </DKBottomSteps>
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
