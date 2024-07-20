<script setup lang="ts">
import DKBottomActions from '@/components/DKBottomActions.vue';
import { defineComponent } from 'vue';
import DKStepButtons from './DKStepButtons.vue';
</script>

<script lang="ts">
export default defineComponent({
  props: {
    trigger: Function,
    guard: Function,
    no_previous: Boolean,
    can_proceed: { type: Boolean, default: true },
    replace: Boolean,
    query: Object,
  },
});
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
