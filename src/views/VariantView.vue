<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import DKListSelect from '@/components/DKListSelect.vue';
import DKBottomSteps from '@/components/DKBottomSteps.vue';
import DKSpinner from '@/components/DKSpinner.vue';
import { defineComponent, inject } from 'vue';
import DKBody from '../components/DKBody.vue';
import { Config, Recipe, Variant } from '../config.ts';
</script>

<script lang="ts">
export default defineComponent({
  data() {
    return {
      loading: true,
      options: [] as Variant[],
      selected: null as null | number,
      config: inject('config') as Config,
    };
  },
  async created() {
    let arch: string;
    try {
      arch = await invoke('get_arch_name');
    } catch (e) {
      const { path } = this.$router.currentRoute.value;
      this.$router.replace({
        path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
        query: { currentRoute: path },
      });
    }

    try {
      const isOfflineInstall = (await invoke('is_offline_install')) as boolean;
      this.config.is_offline_install = isOfflineInstall;

      const recipe = (await invoke('get_recipe')) as Recipe;
      const { variants } = recipe;

      const recipeI18n = (await invoke('i18n_recipe', {
        locale: this.config.locale.id,
      })) as any;

      variants.forEach((item, index) => {
        const title = recipeI18n[item['name-tr']]
          ? recipeI18n[item['name-tr']]
          : (item.name as string);
        variants[index].title = title as string;

        const body = recipeI18n[item['description-tr']]
          ? recipeI18n[item['description-tr']]
          : (item.description as string);
        variants[index].body = body as string;
      });

      this.options = variants
        .filter((v) => !v.retro && v.name !== 'BuildKit')
        .filter((v) => v.squashfs.some((r) => r.arch === arch));

      if (this.config.variant) {
        this.selected = this.options.findIndex(
          (v) => v.name === this.config.variant.name,
        );
      }

      this.loading = false;
    } catch (e) {
      if (this.config.is_offline_install) {
        const { path } = this.$router.currentRoute.value;
        this.$router.replace({
          path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
          query: { currentRoute: path },
        });
      } else {
        this.$router.push('/network');
      }
    }
  },
});
</script>

<template>
  <DKBody>
    <div v-if="!loading">
      <h1>{{ $t("variant.title") }}</h1>
      <p>{{ $t("variant.p1") }}</p>
      <section>
        <DKListSelect
          :selected="selected"
          :options="options"
          :is_limit_height="true"
          @update:selected="(v: number) => (selected = v)"
        />
      </section>
    </div>
    <div class="loading" v-else>
      <h1>{{ $t("variant.title") }}</h1>
      <DKSpinner :title="$t('variant.l1')" />
    </div>
  </DKBody>
  <DKBottomSteps
    :trigger="
      () => {
        if (selected === null) return;
        config.variant = options[selected];
      }
    "
    :can_proceed="selected != null"
  />
</template>
