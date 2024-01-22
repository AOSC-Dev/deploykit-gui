<script setup>
import DKListSelect from "@/components/DKListSelect.vue";
import DKBottomSteps from "@/components/DKBottomSteps.vue";

const options = [
  {
    title: "Desktop",
    body: "Powered by the KDE Plasma Desktop, comes complete with an elegant and powerful set of applications.",
  },
  {
    title: "Server",
    body: "Comes with tools for hosting remote files and contents, network routing, and other functions that enriches network resource access client devices.",
  },
  {
    title: "Base",
    body: "Minimal system release with a basic set of tools.",
  },
];
</script>

<script>
export default {
  inject: ["config"],
  data: function () {
    return {
      selected: this.config.variant,
    };
  },
};
</script>

<template>
  <div>
    <h1>{{ $t("variant.title") }}</h1>
    <p>{{ $t("variant.p1") }}</p>
    <section>
      <DKListSelect
        :selected="selected"
        :options="options"
        @update:selected="(v) => (selected = v)"
      />
    </section>
  </div>
  <DKBottomSteps
    :trigger="() => (config.variant = selected)"
    :can_proceed="selected != null"
  />
</template>
