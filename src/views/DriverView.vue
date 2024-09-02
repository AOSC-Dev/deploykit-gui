<script setup lang="ts">
import DKListSelect from '@/components/DKListSelect.vue';
import DKBottomSteps from '@/components/DKBottomSteps.vue';
</script>

<script lang="ts">
import { defineComponent, inject } from 'vue';

interface Config {
  rescue: boolean;
}

export default defineComponent({
  data() {
    return {
      config: inject('config') as Config,
      options: [
        {
          title: this.$t('rescue.yes'),
          body: this.$t('driver.l1'),
        },
        {
          title: this.$t('rescue.no'),
          body: this.$t('driver.l2'),
        },
      ],
      selected: 1,
    };
  },
  methods: {
    open_eula() {},
  },
});
</script>

<template>
  <div>
    <h1>{{ $t("driver.title") }}</h1>
    <i18n-t keypath="driver.p1" tag="p">
      <template>
        <a @click="open_eula" href="javascript:void(0);">
          {{ $t("driver.p1-1") }}
          <img src="@/../assets/open-fluent-icon.svg" height="14" />
        </a>
      </template>
    </i18n-t>
    <p>
      {{ $t("driver.p2") }}
    </p>
    <section>
      <DKListSelect
        :options="options"
        :selected="selected"
        @update:selected="(v: number) => (selected = v)"
      />
    </section>
  </div>
  <DKBottomSteps
    :trigger="() => (config.rescue = !selected)"
    :can_proceed="selected != null"
  />
</template>

<style scoped>
a {
  color: var(--color-text);
}
</style>
