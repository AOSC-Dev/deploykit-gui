<script>
export default {
  props: {
    title: String,
    can_cancel: Boolean,
  },
  methods: {
    on_cancel: function () {
      this.$emit("cancel");
    },
  },
};
</script>

<template>
  <div class="rcenter">
    <div class="sp">
      <div style="display: block">
        <img class="c1" height="70" src="@/assets/cog.svg" />
        <img class="c2" height="35" src="@/assets/cog.svg" />
      </div>
      <h2 style="text-align: center">{{ title }}</h2>
      <button
        style="margin-left: auto; margin-right: auto"
        class="button"
        v-if="can_cancel"
        @click="on_cancel"
      >
        {{ $t("cancel") }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.sp {
  cursor: wait;
  text-align: center;
}

.rcenter {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 50vh;
}

.c1 {
  animation: 2s linear 0s infinite normal rotate;
}

.c2 {
  animation: 1s linear 0s infinite reverse rotate;
}

@keyframes rotate {
  0% {
    transform: rotate(0);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>
