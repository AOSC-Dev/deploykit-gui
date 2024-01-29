<script setup>
import DKListSelect from "@/components/DKListSelect.vue";
import DKBottomSteps from "@/components/DKBottomSteps.vue";
import { invoke } from "@tauri-apps/api";
</script>

<script>
export default {
  inject: ["config"],
  loading: true,
  data: function () {
    return {
      options: [],
      selected: null,
    };
  },
  async created() {
    const req = await invoke("get_recipe");
    const resp = JSON.parse(req);
    if (resp.result == "Ok") {
      let variants = resp.data.variants;
      for (let i of variants) {
        i.title = i.name;
        i.body = i.description;
      }
      this.options = variants.filter((v) => !v.retro && v.name !== "BuildKit");
    }

    this.loading = false;
  }
};
</script>

<template>
  <div>
    <h1>{{ $t("variant.title") }}</h1>
    <p>{{ $t("variant.p1") }}</p>
    <section v-if="!loading">
      <DKListSelect :selected="selected" :options="options" @update:selected="(v) => (selected = v)" />
    </section>
  </div>
  <DKBottomSteps :trigger="() => (config.variant = options[selected])" :can_proceed="selected != null" />
</template>
