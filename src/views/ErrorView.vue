<script lang="ts">
import { invoke } from '@tauri-apps/api/core';
import DKBottomActions from '@/components/DKBottomActions.vue';
import DKBottomRightButtons from '@/components/DKBottomRightButtons.vue';
import { defineComponent } from 'vue';

export default defineComponent({
  props: {
    message: { type: String, required: true },
    openGparted: Number,
    currentRoute: { type: String, required: true },
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
        const { path } = this.$router.currentRoute.value;

        this.$router.replace({
          path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
          query: { currentRoute: path },
        });
      }
    },
    async launchGparted() {
      await invoke('gparted');
    },
    async retry() {
      this.loading = true;
      await invoke('sync_disk');
      this.loading = false;
      this.$router.replace(this.$props.currentRoute);
    },
  },
});
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
        <button class="button" v-if="openGparted === 1" @click="launchGparted">
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
