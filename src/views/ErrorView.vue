<script>
import { invoke } from '@tauri-apps/api';
import DKBottomActions from '@/components/DKBottomActions.vue';
import DKBottomRightButtons from '@/components/DKBottomRightButtons.vue';

export default {
  props: {
    message: String,
    openGparted: Boolean,
    isInstalling: Boolean,
  },
  components: { DKBottomActions, DKBottomRightButtons },
  methods: {
    async proceed() {
      try {
        await invoke('cancel_install_and_exit', { resetConfig: false });
      } catch (e) {
        this.$router.replace(`/error/${encodeURIComponent(e)}`);
      }
    },
    async launchGparted() {
      await invoke('gparted');
    },
    async retry() {
      if (this.isInstalling) {
        await invoke('reset_progress_status');
      }

      this.$router.replace('/');
    },
  },
};
</script>

<template>
  <div>
    <h1>{{ $t("error.title") }}</h1>
    <p>{{ $t("error.p1") }}</p>
    <p class="error-msg">{{ decodeURIComponent(message) }}</p>
  </div>
  <DKBottomActions>
    <DKBottomRightButtons>
      <button class="button" v-if="openGparted" @click="launchGparted">
        {{ $t("part.b1") }}
      </button>
      <button class="button" @click="retry">
        {{ $t("retry") }}
      </button>
      <button class="button" @click="proceed">{{ $t("exit") }}</button>
    </DKBottomRightButtons>
  </DKBottomActions>
</template>
