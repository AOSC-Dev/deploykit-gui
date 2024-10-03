<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import DKBottomActions from '@/components/DKBottomActions.vue';
import DKBottomRightButtons from '@/components/DKBottomRightButtons.vue';
import DKSpinner from '@/components/DKSpinner.vue';
import { defineComponent } from 'vue';
import DKBody from '../components/DKBody.vue';
</script>

<script lang="ts">
export default defineComponent({
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
          path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
          query: { currentRoute: path },
        });
      }
    },
  },
});
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
