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
    const req = await invoke("list_devices");
    const resp = JSON.parse(req);
    if (resp.result == "Ok") {
      this.devices = resp.data;
    } else {
      this.$router.push("/abort");
    }
    this.loading = false;
  }
}
</script>

<template>
  <div v-if="!loading">
    <p>{{ $t("part.p1") }}</p>
    <section>
      <DKListSelect :no_margin="true" v-model:selected="selected" :options="devices">
        <template #item="option">
          <div style="width: 100%">
            <span class="column-85">{{ option.path }}</span>
            <span class="column-15">{{  humanSize(option.size) }}</span>
          </div>
        </template>
      </DKListSelect>
      <DKStepButtons :trigger="() => (config.device = devices[selected])" :can_proceed="selected != null" />
    </section>
  </div>
</template>

<style scoped>
.column-85 {
  font-weight: 600;
  width: 85%;
  display: inline-block;
}

.column-15 {
  width: 15%;
  display: inline-block;
}

/* p.secondary span {
  color: var(--dk-gray);
} */

/* p.secondary {
  margin: 0;
} */
</style>
