<script setup>
import DKBottomRightButtons from './DKBottomRightButtons.vue';
</script>

<script>
export default {
  props: {
    trigger: Function,
    guard: Function,
    can_proceed: { type: Boolean, default: true },
    no_previous: Boolean,
    replace: Boolean,
  },
  methods: {
    proceed() {
      if (this.guard) {
        if (!this.guard(this)) return;
      }
      if (this.trigger) this.trigger(this);
      if (this.replace) {
        this.$router.replace(this.$router.currentRoute.value.meta.next);
      } else {
        this.$router.push(this.$router.currentRoute.value.meta.next);
      }
    },
  },
};
</script>

<template>
  <DKBottomRightButtons>
    <button class="button" @click="$router.back()" :disabled="no_previous">
      {{ $t("previous") }}
    </button>
    <button class="button" @click="proceed" :disabled="!can_proceed">
      {{ $t("next") }}
    </button>
  </DKBottomRightButtons>
</template>
