<script setup>
import DKLayout from '@/components/DKLayout.vue';
import DKListSelect from '@/components/DKListSelect.vue';
import DKBottomRightButtons from '@/components/DKBottomRightButtons.vue';
import DKBottomActions from '@/components/DKBottomActions.vue';
import { invoke } from '@tauri-apps/api';
import DKSpinner from '@/components/DKSpinner.vue';
</script>

<script>
import langData from '../lang_select.json';

export default {
  inject: ['config'],
  data() {
    return {
      langData,
      current_lang: 'zh-cn',
      displayData: langData.map((v) => ({ body: v.lang })),
      selection: 0,
      loading: false,
    };
  },
  computed: {
    is_inverted() {
      return this.langData[this.selection].anastrophe;
    },
  },
  methods: {
    select(idx) {
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
      this.$emit('update:lang', locale.id);
      this.config.locale = locale;
      invoke('set_locale', { locale: this.config.locale.locale });
      this.$router.replace('/');
    }
  },
};
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
        <div style="line-height: 1" v-if="!is_inverted">
          <h1 style="font-size: 3rem; text-align: right; margin-bottom: 0">
            {{ langData[selection]["aosc"] }}
          </h1>
          <h2 style="font-size: 1.25rem; text-align: right">
            {{ langData[selection]["inst"] }}
          </h2>
        </div>
        <div style="line-height: 1" v-else>
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
