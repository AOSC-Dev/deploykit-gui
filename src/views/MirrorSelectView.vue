<script setup>
import DKBottomActions from "@/components/DKBottomActions.vue";
import DKStepButtons from "@/components/DKStepButtons.vue";
import DKStripButton from "@/components/DKStripButton.vue";
import DKListSelect from "@/components/DKListSelect.vue";
import DKSpinner from "@/components/DKSpinner.vue";
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
  data: function () {
    return {
      mirrors: [],
      loading: false,
      selected: null,
    };
  },
  watch: {
    loading(newValue) {
      this.$emit("update:can_quit", !newValue);
    },
  },
  methods: {
    run_bench: function () {
      this.loading = true;
      this.$ipc.call("bench", []).then((data) => {
        this.mirrors = data;
        this.loading = true;
        this.selected = 0;
      });
    },
  },
  async created () {
    const recipe = await getRecipe();
    this.mirrors = recipe.data.mirrors;
    this.loading = false;
  }
};
</script>

<template>
  <div v-if="!loading">
    <h1>{{ $t("mirror.title") }}</h1>
      <p>{{ $t("mirror.p2") }}</p>
      <section style="max-height: 50vh; overflow-y: scroll">
      <DKListSelect :no_margin="true" :options="mirrors" v-model:selected="selected">
        <template #item="option">
          <div>
            <span><b>{{ option.name }}</b></span>
            &nbsp;
            <span>({{ option.region }})</span>
          </div>
        </template>
      </DKListSelect>
    </section>
  </div>
  <!-- loading screen -->
  <div class="loading" v-else>
    <h1>{{ $t("mirror.title") }}</h1>
    <DKSpinner :title="$t('mirror.k1')" />
  </div>
  <DKBottomActions v-if="!loading">
    <DKStripButton :text="$t('mirror.b2')" @click="run_bench">
      <img src="@/assets/histogram-symbolic.svg" height="36" />
    </DKStripButton>
    <DKStepButtons
      :trigger="() => (config.mirror = mirrors[selected])"
      :can_proceed="selected != null"/>
  </DKBottomActions>
</template>

<style scoped>
span b {
  font-weight: 600;
}
</style>
