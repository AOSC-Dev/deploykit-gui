<script setup>
import DKStepButtons from "./DKStepButtons.vue";
import DKBottomActions from "@/components/DKBottomActions.vue";
</script>

<script>
export default {
  props: {
    trigger: Function,
    guard: Function,
    no_previous: Boolean,
    can_proceed: { type: Boolean, default: true },
  },
};
</script>

<template>
  <DKBottomActions>
    <DKStepButtons
      :trigger="trigger"
      :guard="guard"
      :no_previous="no_previous"
      :can_proceed="can_proceed"
    />
  </DKBottomActions>
</template>
