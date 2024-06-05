<script setup>
import { invoke } from '@tauri-apps/api';
import DKListSelect from '@/components/DKListSelect.vue';
import DKBottomSteps from '@/components/DKBottomSteps.vue';
import DKSpinner from '@/components/DKSpinner.vue';
import DKBody from '../components/DKBody.vue';
</script>

<script>
export default {
  inject: ['config'],
  data() {
    return {
      loading: true,
      options: [],
      selected: null,
    };
  },
  async created() {
    try {
      const recipe = await invoke('get_recipe');
      const { variants } = recipe;

      const recipeI18n = await invoke('i18n_recipe', {
        locale: this.config.locale.id,
      });

      variants.forEach((item, index) => {
        const title = recipeI18n[item['name-tr']]
          ? recipeI18n[item['name-tr']]
          : item.name;
        variants[index].title = title;

        const body = recipeI18n[item['description-tr']]
          ? recipeI18n[item['description-tr']]
          : item.description;
        variants[index].body = body;
      });

      this.options = variants.filter((v) => !v.retro && v.name !== 'BuildKit');

      if (this.config.variant) {
        this.selected = this.options.findIndex(
          (v) => v.name === this.config.variant.name,
        );
      }

      this.loading = false;
    } catch (e) {
      this.$router.push('/network');
    }
  },
};
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
          @update:selected="(v) => (selected = v)"
        />
      </section>
    </div>
    <div class="loading" v-else>
      <h1>{{ $t("variant.title") }}</h1>
      <DKSpinner :title="$t('variant.l1')" />
    </div>
  </DKBody>
  <DKBottomSteps
    :trigger="() => (config.variant = options[selected])"
    :can_proceed="selected != null"
  />
</template>
