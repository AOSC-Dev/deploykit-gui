<script setup>
import { invoke } from '@tauri-apps/api';
import DKBottomActions from '@/components/DKBottomActions.vue';
import DKBottomRightButtons from '@/components/DKBottomRightButtons.vue';
import DKSpinner from '@/components/DKSpinner.vue';
import DKBody from '../components/DKBody.vue';
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
        await invoke('reboot');
      } catch (e) {
        const { path } = this.$router.currentRoute.value;

        this.$router.replace({
          path: `/error/${encodeURIComponent(e)}`,
          query: { currentRoute: path },
        });
      }
    },
  },
};
</script>

<template>
  <DKBody>
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
  </DKBody>
  <DKBottomActions>
    <DKBottomRightButtons>
      <button class="button" @click="finish">{{ $t("finish.reboot") }}</button>
    </DKBottomRightButtons>
  </DKBottomActions>
</template>
