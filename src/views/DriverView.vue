<script setup>
import DKListSelect from "@/components/DKListSelect.vue";
import DKBottomSteps from "@/components/DKBottomSteps.vue";
</script>

<script>
export default {
  inject: ["config"],
  data: function () {
    return {
      options: [
        {
          title: this.$t("rescue.yes"),
          body: this.$t("driver.l1"),
        },
        {
          title: this.$t("rescue.no"),
          body: this.$t("driver.l2"),
        },
      ],
      selected: 1,
    };
  },
  methods: {
    open_eula: function () {
      this.$ipc.notify("open", this.$t("driver.eula_link"));
    },
  },
};
</script>

<template>
  <div>
    <h1>{{ $t("driver.title") }}</h1>
    <i18n-t keypath="driver.p1" tag="p">
      <template>
        <a @click="open_eula" href="javascript:void(0);">
          {{ $t("driver.p1-1") }}
          <img src="@/assets/open-fluent-icon.svg" height="14" />
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
a {
  color: var(--color-text);
}
</style>
