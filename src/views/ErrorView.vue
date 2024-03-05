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
  data() {
    return {
      loading: false,
    };
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
        this.loading = true;
        await invoke('sync_disk');
        this.loading = false;
        this.$router.replace('/install');
      } else {
        this.$router.replace('/');
      }
    },
  },
};
</script>

<template>
  <div v-if="!loading">
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
  </div>
  <div v-else>
    <div class="loading">
      <h1>{{ $t("error.title") }}</h1>
      <DKSpinner :title="$t('error.retrying')" />
    </div>
  </div>
</template>
