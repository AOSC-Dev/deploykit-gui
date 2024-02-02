<script setup>
import DKBottomActions from "@/components/DKBottomActions.vue";
import DKStripButton from "@/components/DKStripButton.vue";
import DKSpinner from "@/components/DKSpinner.vue";
</script>

<script>
import { invoke } from "@tauri-apps/api";
export default {
  data: function () {
    return {
      exiting: false
    };
  },
  methods: {
    quit: async function (reset_config) {
      try {
        this.exiting = true;
        await invoke("cancel_install_and_exit", { resetConfig: reset_config });
      } catch (e) {
        console.error(e);
      }
    }
  }
}
</script>

<template>
  <div v-if="!exiting">
    <h1>{{ $t("abort.title") }}</h1>
    <i18n-t keypath="abort.p1" tag="p">
      {{ $t("abort.p1-1") }}
    </i18n-t>
    <p>{{ $t("abort.p2") }}</p>
  </div>
  <div class="loading" v-else>
    <h1>{{ $t("exiting.t1") }}</h1>
    <DKSpinner :title="$t('exiting.l1')" />
  </div>
  <DKBottomActions>
    <DKStripButton omit_bline="1" :text="$t('abort.resume')" @click="$router.back()">
      <img src="@/../assets/resume.svg" height="36" />
    </DKStripButton>
    <DKStripButton omit_bline="1" :text="$t('abort.save')" @click="quit(false)">
      <img src="@/../assets/document-save.svg" height="36" />
    </DKStripButton>
    <DKStripButton :text="$t('abort.quit')" @click="quit(true)">
      <img src="@/../assets/exit-run.svg" height="36" />
    </DKStripButton>
  </DKBottomActions>
</template>
