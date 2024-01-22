<script setup>
import DKBottomActions from "@/components/DKBottomActions.vue";
import DKBottomRightButtons from "@/components/DKBottomRightButtons.vue";
</script>

<script>
export default {
  methods: {
    finish: function () {
      this.$ipc.call("finish").then(window.close);
    },
  },
};
</script>

<template>
  <div>
    <h1>{{ $t("finish.title") }}</h1>
    <p>{{ $t("finish.p1") }}</p>
    <p>{{ $t("finish.p2") }}</p>
    <p>{{ $t("finish.p3") }}</p>
  </div>
  <DKBottomActions>
    <DKBottomRightButtons>
      <button class="button" @click="finish">{{ $t("finish.finish") }}</button>
    </DKBottomRightButtons>
  </DKBottomActions>
</template>
