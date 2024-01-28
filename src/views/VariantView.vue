<script setup>
import DKListSelect from "@/components/DKListSelect.vue";
import DKBottomSteps from "@/components/DKBottomSteps.vue";
</script>

<script>
import { getClient } from '@tauri-apps/api/http';

async function getRecipe() {
  const client = await getClient();
  const response = await client.get("https://releases.aosc.io/manifest/recipe.json");
  const data = response.data;
  
  return data;
}

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
    const recipe = await getRecipe();
    let variants = recipe.variants;
    for (let i of variants) {
      i.title = i.name;
      i.body = i.description;
    }
    this.options = variants.filter((v) => !v.retro);

    this.loading = false;
  }
};
</script>

<template>
  <div v-if="!loading">
    <h1>{{ $t("variant.title") }}</h1>
    <p>{{ $t("variant.p1") }}</p>
    <section>
      <DKListSelect
        :selected="selected"
        :options="options"
        @update:selected="(v) => (selected = v)"
      />
    </section>
  </div>
  <DKBottomSteps
    :trigger="() => (config.variant = options[selected])"
    :can_proceed="selected != null"
  />
</template>
