<script setup>
import DKListSelect from "@/components/DKListSelect.vue";
import DKBottomSteps from "@/components/DKBottomSteps.vue";
import { invoke } from "@tauri-apps/api";
</script>

<script>
export default {
  inject: ["config"],
  data: function () {
    return {
      options: [],
      selected: null,
    };
  },
  created() {
    invoke("get_recipe").then((req) => {
      let variants = req.variants;
      for (let i of variants) {
        i.title = i.name;
        i.body = i.description;
      }

      this.options = variants.filter((v) => !v.retro && v.name !== "BuildKit");
    }).catch((e) => {
      console.error(e);
      this.$router.replace("/error");
    });
  }
}
</script>

<template>
  <div>
    <h1>{{ $t("variant.title") }}</h1>
    <p>{{ $t("variant.p1") }}</p>
    <section>
      <DKListSelect :selected="selected" :options="options" @update:selected="(v) => (selected = v)" />
    </section>
  </div>
  <DKBottomSteps :trigger="() => (config.variant = options[selected])" :can_proceed="selected != null" />
</template>
