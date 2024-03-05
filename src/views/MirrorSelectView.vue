<script setup>
import { invoke } from '@tauri-apps/api';
import DKBottomActions from '@/components/DKBottomActions.vue';
import DKStepButtons from '@/components/DKStepButtons.vue';
import DKStripButton from '@/components/DKStripButton.vue';
import DKListSelect from '@/components/DKListSelect.vue';
import DKSpinner from '@/components/DKSpinner.vue';
</script>

<script>
export default {
  inject: ['config'],
  data() {
    return {
      mirrors: [],
      loading: false,
      selected: null,
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
        const mirrors = await invoke('mirrors_speedtest', {
          mirrors: this.mirrors,
        });
        this.mirrors = mirrors;
        this.loading = false;
      } catch (e) {
        this.$router.replace(`/error/${encodeURIComponent(e)}`);
      }
    },
  },
  async created() {
    try {
      const data = await invoke('get_recipe');
      const recipeI18n = await invoke('i18n_recipe', {
        locale: this.config.locale.id,
      });

      this.mirrors = data.mirrors;
      data.mirrors.forEach((item, index) => {
        data.mirrors[index].nameTr = recipeI18n[item['name-tr']];
        data.mirrors[index].locTr = recipeI18n[item['loc-tr']];
      });

      this.loading = false;
    } catch (e) {
      this.$router.replace(`/error/${encodeURIComponent(e)}`);
    }
  },
};
</script>

<template>
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
    <DKStepButtons
      :trigger="() => (config.mirror = mirrors[selected])"
      :can_proceed="selected != null"
    />
  </DKBottomActions>
</template>

<style scoped>
span b {
  font-weight: 600;
}
</style>
