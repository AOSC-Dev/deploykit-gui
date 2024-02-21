<script setup>
import { invoke } from '@tauri-apps/api';
import DKListSelect from '@/components/DKListSelect.vue';
import DKBottomSteps from '@/components/DKBottomSteps.vue';
import DKSpinner from '@/components/DKSpinner.vue';
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
  created() {
    invoke('get_recipe')
      .then((req) => {
        const { variants } = req;

        variants.forEach((item, index) => {
          const title = item.name;
          variants[index].title = title;
          const body = item.description;
          variants[index].body = body;
        });

        this.options = variants
          .filter((v) => !v.retro && v.name !== 'BuildKit')
          .sort((a, b) => (a.name > b.name ? 1 : -1));
        this.loading = false;
      })
      .catch((e) => {
        this.$router.replace(`/error/${encodeURIComponent(e)}`);
      });
  },
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
        :is_limit_height="true"
        @update:selected="(v) => (selected = v)"
      />
    </section>
  </div>
  <div class="loading" v-else>
    <h1>{{ $t("variant.title") }}</h1>
    <DKSpinner :title="$t('variant.l1')" />
  </div>
  <DKBottomSteps
    :trigger="() => (config.variant = options[selected])"
    :can_proceed="selected != null"
  />
</template>
