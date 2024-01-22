<script>
const CJK_LANG = ["zh-CN", "zh-TW", "ja"];
const INV_LANG = [""];
export default {
  computed: {
    is_inverted: function () {
      return INV_LANG.includes(this.$i18n.locale);
    },
    cjk_quirk: function () {
      return CJK_LANG.includes(this.$i18n.locale) ? "cjk" : "";
    },
  },
};
</script>

<template>
  <div class="logo-container">
    <img
      style="align-self: center"
      alt="AOSC logo"
      class="logo"
      src="@/assets/aosc-logo-core.svg"
      width="32"
      height="32"
    />
    <div style="line-height: 1" v-if="!is_inverted">
      <span style="font-size: 0.82rem">{{ $t("logo-p1") }}</span>
      <h2 :class="cjk_quirk" style="font-size: 1.25rem">
        {{ $t("logo-p2") }}
      </h2>
    </div>
    <div style="line-height: 1" v-else>
      <h2 style="font-size: 1.25rem">
        {{ $t("logo-p2") }}
      </h2>
      <span style="font-size: 0.82rem">{{ $t("logo-p1") }}</span>
    </div>
  </div>
</template>

<style scoped>
.cjk {
  margin-left: -2px;
  padding-top: 1px;
}

.logo {
  margin: 0 0.5rem 0 0;
}

.logo-container {
  display: flex;
  line-height: 1.5;
  margin-left: 1.7rem;
}
</style>
