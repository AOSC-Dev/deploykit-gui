<!-- <script setup lang="ts">
import DKListSelect from '@/components/DKListSelect.vue';
import DKBottomSteps from '@/components/DKBottomSteps.vue';
</script>

<script lang="ts">
export default {
  inject: ['config'],
  methods: {
    load_default() {
      if (this.config.rescue == null) {
        if (this.options[0].disabled) return 1;
        return null;
      }
      return !this.config.rescue || 0;
    },
    rescuekit_options() {
      const noOption = {
        title: this.$t('rescue.no'),
        body: this.$t('rescue.w1'),
        hl: true,
      };
      switch (this.config.rescue_avail || 1) {
        case 0:
          return [
            {
              title: this.$t('rescue.yes'),
              body: this.$t('rescue.l1'),
            },
            noOption,
          ];
        case 1:
          return [
            {
              title: this.$t('rescue.yes'),
              body: this.$t('rescue.bad1'),
              disabled: true,
            },
            noOption,
          ];
        case 2:
          return [
            {
              title: this.$t('rescue.yes'),
              body: this.$t('rescue.bad2'),
              disabled: true,
            },
            noOption,
          ];
        default:
          throw new Error(`Unknown RescueKit status ${this.config.rescue_avail}`);
      }
    },
  },
  data() {
    return {
      options: this.rescuekit_options(),
      selected: null,
    };
  },
  mounted() {
    this.selected = this.load_default();
  },
};
</script>

<template>
  <div>
    <h1>{{ $t("rescue.title") }}</h1>
    <p>{{ $t("rescue.p1") }}</p>
    <section>
      <DKListSelect
        :options="options"
        :selected="selected"
        @update:selected="(v) => (selected = v)"
      />
    </section>
  </div>
  <DKBottomSteps
    :trigger="() => (config.rescue = !selected)"
    :can_proceed="selected != null"
  />
</template>

<style scoped>
.error-msg {
  color: var(--dk-accent);
}
</style> -->
