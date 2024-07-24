<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import DKBottomActions from '@/components/DKBottomActions.vue';
import DKBottomSteps from '@/components/DKBottomSteps.vue';
import DKStripButton from '@/components/DKStripButton.vue';
import DKListSelect from '@/components/DKListSelect.vue';
import DKSpinner from '@/components/DKSpinner.vue';
import { inject, defineComponent } from 'vue';
import DKBody from '../components/DKBody.vue';
import { Config, Mirror, Recipe } from '../config.ts';
</script>

<script lang="ts">
type RecipeI18n = Record<string, string>;

const covertMirrorsListToUiString = (
  m: Mirror[],
  recipeI18n: RecipeI18n,
) => {
  const mirrors = m;
  mirrors.forEach((item, index) => {
    mirrors[index].nameTr = recipeI18n[item['name-tr']];
    mirrors[index].locTr = recipeI18n[item['loc-tr']];
    mirrors[index].score = item.score;
  });

  return mirrors;
};

export default defineComponent({
  data() {
    const config = inject('config') as Config;
    return {
      mirrors: config.mirrors ? config.mirrors : [],
      loading: false,
      selected: null as number | null,
      recipeI18n: null as RecipeI18n | null,
      config,
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
        const m = (await invoke('mirrors_speedtest', {
          mirrors: this.mirrors,
        })) as Mirror[];

        if (this.recipeI18n === null) {
          return;
        }

        const mirrors = covertMirrorsListToUiString(m, this.recipeI18n);

        this.mirrors = mirrors;
        this.loading = false;
      } catch (e) {
        const { path } = this.$router.currentRoute.value;

        this.$router.replace({
          path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
          query: { currentRoute: path },
        });
      }
    },
    calc(time: number | undefined, fileSize: number) {
      if (typeof time !== 'number') {
        return '';
      }

      const speed = fileSize / 1024 / 1024 / time;

      return `${speed.toFixed(2)}MiB/s`;
    },
  },
  async created() {
    try {
      let mirrors;
      if (!this.config.mirrors) {
        const data = (await invoke('get_recipe')) as Recipe;
        mirrors = data.mirrors;
      } else {
        mirrors = this.config.mirrors;
      }

      const recipeI18n = (await invoke('i18n_recipe', {
        locale: this.config.locale.id,
      })) as RecipeI18n;
      this.recipeI18n = recipeI18n;

      mirrors = covertMirrorsListToUiString(mirrors as Mirror[], recipeI18n);
      this.mirrors = mirrors;

      if (this.config.mirror) {
        this.selected = this.mirrors.findIndex(
          (v) => v.name === this.config.mirror?.name,
        );
      }

      this.loading = false;
    } catch (e) {
      const { path } = this.$router.currentRoute.value;

      this.$router.replace({
        path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
        query: { currentRoute: path },
      });
    }
  },
});
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
            <div class="mirror">
              <span
                ><b
                  >{{ option.locTr ? option.locTr : option.loc }} -
                  {{ option.nameTr ? option.nameTr : option.name }}</b
                ></span
              >
              <span>{{ calc(option.score, option.downloaded_size) }}</span>
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
    :trigger="() => {
      if (selected === null) {
        return;
      }
      config.mirror = mirrors[selected];
    }"
    :can_proceed="selected != null"
  />
</template>

<style scoped>
span b {
  font-weight: 600;
}

.mirror {
  display: flex;
  flex-flow: row;
  justify-content: space-between;
}
</style>
