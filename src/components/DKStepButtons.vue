<script setup lang="ts">
import { defineComponent } from 'vue';
import DKBottomRightButtons from './DKBottomRightButtons.vue';
</script>

<script lang="ts">
export default defineComponent({
  props: {
    trigger: Function,
    guard: Function,
    can_proceed: { type: Boolean, default: true },
    no_previous: Boolean,
    replace: Boolean,
    query: Object,
  },
  methods: {
    proceed() {
      if (this.guard) {
        if (!this.guard(this)) return;
      }
      if (this.trigger) this.trigger(this);
      if (this.replace) {
        this.$router.replace({
          path: this.$router.currentRoute.value.meta.next as string,
          query: this.query,
        });
      } else {
        this.$router.push({
          path: this.$router.currentRoute.value.meta.next as string,
          query: this.query,
        });
      }
    },
  },
});
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
