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
  },
  methods: {
    proceed() {
      if (this.guard) {
        if (!this.guard(this)) return;
      }
      if (this.trigger) this.trigger(this);
      this.$router.push(this.$router.currentRoute.value.meta.next);
    },
    back() {
      if (this.$router.currentRoute.value.meta.prev) {
        this.$router.replace(this.$router.currentRoute.value.meta.prev);
      } else {
        this.$router.back();
      }
    },
  },
};
</script>

<template>
  <DKBottomRightButtons>
    <button class="button" @click="back" :disabled="no_previous">
      {{ $t("previous") }}
    </button>
    <button class="button" @click="proceed" :disabled="!can_proceed">
      {{ $t("next") }}
    </button>
  </DKBottomRightButtons>
</template>
