<script setup>
import DKListSelect from "@/components/DKListSelect.vue";
import DKStepButtons from "@/components/DKStepButtons.vue";
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
    }
  },
  methods: {},
  async created() {
    try {
      const devices = await invoke("list_devices");
      this.devices = devices;
    } catch (e) {
      this.$router.replace("/error");
      console.error(e);
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
      <DKListSelect :no_margin="true" v-model:selected="selected" :is_limit_height="true" :options="devices">
        <template #item="option">
          <div style="width: 100%">
            <span class="column-80">{{ option.model }} ({{ option.path }})</span>
            <span class="column-20">{{  humanSize(option.size) }}</span>
          </div>
        </template>
      </DKListSelect>
      <DKStepButtons :trigger="() => (config.device = devices[selected])" :can_proceed="selected != null" />
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

/* p.secondary span {
  color: var(--dk-gray);
} */

/* p.secondary {
  margin: 0;
} */
</style>
