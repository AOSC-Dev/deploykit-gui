<script setup>
import DKBottomActions from '@/components/DKBottomActions.vue';
import DKStepButtons from './DKStepButtons.vue';
</script>

<script>
export default {
  props: {
    trigger: Function,
    guard: Function,
    no_previous: Boolean,
    can_proceed: { type: Boolean, default: true },
    replace: Boolean,
    query: Object,
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
      :replace="replace"
      :query="query"
    />
  </DKBottomActions>
</template>
