<script setup>
import DKBottomActions from "@/components/DKBottomActions.vue";
import DKStepButtons from "@/components/DKStepButtons.vue";
import DKStripButton from "@/components/DKStripButton.vue";
import DKListSelect from "@/components/DKListSelect.vue";
import DKSpinner from "@/components/DKSpinner.vue";
</script>

<script>
export default {
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
        this.loading = false;
        this.selected = 0;
      });
    },
  },
};
</script>

<template>
  <div v-if="!loading">
    <h1>{{ $t("mirror.title") }}</h1>
    <section>
      <p>{{ $t("mirror.p2") }}</p>
      <DKListSelect
        :no_margin="true"
        :options="mirrors"
        v-model:selected="selected"
      >
        <template #item="option">
          <div>
            <span
              ><b>{{ option.name }}</b></span
            >
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
    <DKStepButtons />
  </DKBottomActions>
</template>

<style scoped>
span b {
  font-weight: 600;
}
</style>
