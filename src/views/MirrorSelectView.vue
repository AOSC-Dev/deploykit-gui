<script setup>
import { invoke } from '@tauri-apps/api';
import DKBottomActions from '@/components/DKBottomActions.vue';
import DKBottomSteps from '@/components/DKBottomSteps.vue';
import DKStripButton from '@/components/DKStripButton.vue';
import DKListSelect from '@/components/DKListSelect.vue';
import DKSpinner from '@/components/DKSpinner.vue';
import DKBody from '../components/DKBody.vue';
</script>

<script>
const covertMirrorsListToUiString = (m, recipeI18n) => {
  const mirrors = m;
  mirrors.forEach((item, index) => {
    mirrors[index].nameTr = recipeI18n[item['name-tr']];
    mirrors[index].locTr = recipeI18n[item['loc-tr']];
  });

  return mirrors;
};

export default {
  inject: ['config'],
  data() {
    return {
      mirrors: [],
      loading: false,
      selected: null,
      recipeI18n: {},
    };
  },
  watch: {
    loading(newValue) {
      this.$emit('update:can_quit', !newValue);
    },
  },
  methods: {
    async run_bench() {
      this.loading = true;
      try {
        const m = await invoke('mirrors_speedtest', {
          mirrors: this.mirrors,
        });
        const mirrors = covertMirrorsListToUiString(m, this.recipeI18n);

        this.mirrors = mirrors;
        this.loading = false;
      } catch (e) {
        const { path } = this.$router.currentRoute.value;

        this.$router.replace({
          path: `/error/${encodeURIComponent(e)}`,
          query: { currentRoute: path },
        });
      }
    },
  },
  async created() {
    try {
      const data = await invoke('get_recipe');
      const recipeI18n = await invoke('i18n_recipe', {
        locale: this.config.locale.id,
      });
      this.recipeI18n = recipeI18n;

      const mirrors = covertMirrorsListToUiString(data.mirrors, recipeI18n);
      this.mirrors = mirrors;

      if (this.config.mirror) {
        this.selected = this.mirrors.findIndex(
          (v) => v.name === this.config.mirror.name,
        );
      }

      this.loading = false;
    } catch (e) {
      const { path } = this.$router.currentRoute.value;

      this.$router.replace({
        path: `/error/${encodeURIComponent(e)}`,
        query: { currentRoute: path },
      });
    }
  },
};
</script>

<template>
  <DKBody>
    <div v-if="!loading">
      <h1>{{ $t("mirror.title") }}</h1>
      <p>{{ $t("mirror.p2") }}</p>
      <section class="mirror-select">
        <DKListSelect
          :no_margin="true"
          :options="mirrors"
          v-model:selected="selected"
          :is_limit_height="true"
        >
          <template #item="option">
            <div>
              <span
                ><b
                  >{{ option.locTr ? option.locTr : option.loc }} -
                  {{ option.nameTr ? option.nameTr : option.name }}</b
                ></span
              >
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
        <img src="@/../assets/histogram-symbolic.svg" height="36" />
      </DKStripButton>
    </DKBottomActions>
  </DKBody>
  <DKBottomSteps
    :trigger="() => (config.mirror = mirrors[selected])"
    :can_proceed="selected != null"
  />
</template>

<style scoped>
span b {
  font-weight: 600;
}
</style>
