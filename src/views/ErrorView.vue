<script>
import DKBottomActions from "@/components/DKBottomActions.vue";
import DKBottomRightButtons from "@/components/DKBottomRightButtons.vue";
import { invoke } from "@tauri-apps/api";

export default {
  props: {
    message: String,
  },
  components: { DKBottomActions, DKBottomRightButtons },
  methods: {
    proceed: async function () {
      try {
        await invoke("cancel_install_and_exit", { resetConfig: false });
      } catch (e) {
        this.$router.replace(`/error/${encodeURIComponent(e)}`);
      }
    }
  }
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
      <button class="button" @click="$router.replace('/')">
        {{ $t("retry") }}
      </button>
      <button class="button" @click="proceed">{{ $t("exit") }}</button>
    </DKBottomRightButtons>
  </DKBottomActions>
</template>
