<script>
import DKBottomActions from "@/components/DKBottomActions.vue";
import DKBottomRightButtons from "@/components/DKBottomRightButtons.vue";

export default {
  props: {
    message: String,
  },
  components: { DKBottomActions, DKBottomRightButtons },
};
</script>

<template>
  <div>
    <h1>{{ $t("error.title") }}</h1>
    <p>{{ $t("error.p1") }}</p>
    <p class="error-msg">{{ message }}</p>
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
