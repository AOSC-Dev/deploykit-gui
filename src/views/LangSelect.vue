<script setup lang="ts">
import DKLayout from '@/components/DKLayout.vue';
import DKListSelect from '@/components/DKListSelect.vue';
import DKBottomRightButtons from '@/components/DKBottomRightButtons.vue';
import DKBottomActions from '@/components/DKBottomActions.vue';
import { invoke } from '@tauri-apps/api';
import DKSpinner from '@/components/DKSpinner.vue';
</script>

<script lang="ts">
import { defineComponent, inject } from 'vue';
import langData from '../lang_select.json';
import { Config } from '../config.ts';

export default defineComponent({
  data() {
    return {
      config: inject('config') as Config,
      langData,
      current_lang: 'zh-cn',
      displayData: langData.map((v) => ({ body: v.lang })),
      selection: 0,
      loading: false,
    };
  },
  methods: {
    select(idx: number) {
      this.$emit('update:lang', this.langData[idx].id);
      this.config.locale = this.langData[idx];
      invoke('set_locale', { locale: this.config.locale.locale });
    },
  },
  async mounted() {
    const isInstall = await invoke('is_skip');
    if (isInstall) {
      this.loading = true;
      const lang = await invoke('read_locale');
      const locale = this.langData.find((v) => v.locale === lang);
      this.$emit('update:lang', locale?.id);
      if (!locale) return;
      this.config.locale = locale;
      invoke('set_locale', { locale: this.config.locale.locale });
      this.$router.replace('/');
    }
  },
});
</script>

<template>
  <DKLayout
    :class="'lang-' + langData[selection].id.toLowerCase()"
    v-if="!loading"
  >
    <section style="max-height: 65vh; overflow-y: scroll; margin-top: 5vh">
      <DKListSelect
        :options="displayData"
        v-model:selected="selection"
      ></DKListSelect>
    </section>
    <DKBottomActions>
      <DKBottomRightButtons>
        <button class="button" @click="select(selection)">
          {{ langData[selection]["next"] }}
        </button>
      </DKBottomRightButtons>
    </DKBottomActions>
    <template #left>
      <div style="margin-top: 5vh">
        <img />
        <div style="line-height: 1">
          <h2 style="font-size: 1.25rem; text-align: right">
            {{ langData[selection]["inst"] }}
          </h2>
          <h1 style="font-size: 3rem; text-align: right">
            {{ langData[selection]["aosc"] }}
          </h1>
        </div>
      </div>
    </template>
  </DKLayout>
  <div class="loading" v-else>
    <h1>{{ $t("loading") }}</h1>
    <DKSpinner :title="$t('loading')" />
  </div>
</template>

<style scoped></style>
