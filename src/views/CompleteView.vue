<script setup>
import { invoke } from '@tauri-apps/api';
import DKBottomActions from '@/components/DKBottomActions.vue';
import DKBottomRightButtons from '@/components/DKBottomRightButtons.vue';
import DKSpinner from '@/components/DKSpinner.vue';
</script>

<script>
export default {
  data() {
    return {
      exiting: false,
    };
  },
  methods: {
    async finish() {
      this.exiting = true;
      try {
        await invoke('cancel_install_and_exit', { resetConfig: true });
      } catch (e) {
        this.$router.replace(`/error/${encodeURIComponent(e)}`);
      }
    },
  },
};
</script>

<template>
  <div v-if="!exiting">
    <h1>{{ $t("finish.title") }}</h1>
    <p>{{ $t("finish.p1") }}</p>
    <p>{{ $t("finish.p2") }}</p>
    <p>{{ $t("finish.p3") }}</p>
  </div>
  <div class="loading" v-else>
    <h1>{{ $t("exiting.t1") }}</h1>
    <DKSpinner :title="$t('exiting.l1')" />
  </div>
  <DKBottomActions>
    <DKBottomRightButtons>
      <button class="button" @click="finish">{{ $t("finish.finish") }}</button>
    </DKBottomRightButtons>
  </DKBottomActions>
</template>
