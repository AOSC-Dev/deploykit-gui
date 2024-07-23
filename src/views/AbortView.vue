<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import DKBottomActions from '@/components/DKBottomActions.vue';
import DKStripButton from '@/components/DKStripButton.vue';
import DKSpinner from '@/components/DKSpinner.vue';
</script>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  props: {
    done: Number,
  },
  data() {
    return {
      exiting: false,
    };
  },
  methods: {
    async quit(resetConfig: boolean) {
      try {
        this.exiting = true;
        await invoke('cancel_install_and_exit', { resetConfig });
      } catch (e) {
        const { path } = this.$router.currentRoute.value;
        this.$router.replace({
          path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
          query: { currentRoute: path },
        });
      }
    },
  },
  async mounted() {
    if (this.done === 1) {
      await invoke('cancel_install_and_exit', { resetConfig: true });
    }
  },
});
</script>

<template>
  <div v-if="!exiting && done !== 1">
    <h1>{{ $t("abort.title") }}</h1>
    <i18n-t keypath="abort.p1" tag="p">
      {{ $t("abort.p1-1") }}
    </i18n-t>
    <p>{{ $t("abort.p2") }}</p>
    <DKBottomActions>
      <DKStripButton
        omit_bline="1"
        :text="$t('abort.resume')"
        @click="$router.back()"
      >
        <img src="@/../assets/resume.svg" height="36" />
      </DKStripButton>
      <DKStripButton :text="$t('abort.quit')" @click="quit(true)">
        <img src="@/../assets/exit-run.svg" height="36" />
      </DKStripButton>
    </DKBottomActions>
  </div>
  <div class="loading" v-else>
    <h1>{{ $t("exiting.t1") }}</h1>
    <DKSpinner :title="$t('exiting.l1')" />
  </div>
</template>
