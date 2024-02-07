<script setup>
import DKListSelect from "@/components/DKListSelect.vue";
import DKBottomSteps from "@/components/DKBottomSteps.vue";
</script>

<script>
import { invoke } from "@tauri-apps/api";

export default {
  inject: ["config", "humanSize"],
  data: function () {
    return {
      devices: [],
      selected: null,
      loading: true,
      req_size: null,
      err_msg: "",
    }
  },
  methods: {
    validate: function () {
      if (this.req_size > this.devices[this.selected].size) {
        this.err_msg = this.$t("device.e1", { size: Math.ceil(this.req_size / 1024 / 1024 / 1024) });
        return false;
      }

      return true;
    },
    select: function () {
      if (this.req_size > this.devices[this.selected].size) {
        this.err_msg = this.$t("device.e1", { size: Math.ceil(this.req_size / 1024 / 1024 / 1024) });
        return false;
      }

      return true;
    }
  },
  async created() {
    try {
      const devices = await invoke("list_devices");
      this.devices = devices;

      const v = this.config.variant;
      const sqfs_info = await invoke("get_squashfs_info", { v, url: this.config.mirror.url });
      let req_size = sqfs_info.downloadSize + sqfs_info.instSize;

      const is_efi = await invoke("is_efi_api");

      if (is_efi) {
        req_size = req_size + 512 * 1024 * 1024;
      }
    } catch (e) {
      this.$router.replace(`/error/${encodeURIComponent(e)}`);
    }

    this.loading = false;
  }
}
</script>

<template>
  <div v-if="!loading">
    <h1>{{ $t("device.title") }}</h1>
    <p>{{ $t("device.p1") }}</p>
    <section>
      <DKListSelect :no_margin="true" v-model:selected="selected" :is_limit_height="true" :options="devices"
        :click_fn="select">
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
      <DKBottomSteps :trigger="() => (config.device = devices[selected])" :can_proceed="selected != null"
        :guard="validate" />
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
