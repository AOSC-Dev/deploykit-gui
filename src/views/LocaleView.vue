<script setup>
import DKFilterSelect from "@/components/DKFilterSelect.vue";
import DKBottomSteps from "@/components/DKBottomSteps.vue";
</script>

<script>
export default {
  // TODO: add locales and timezones here
  props: {},
  inject: ["config"],
  data: function () {
    return {
      locales: [
        {
          text: "English",
          data: "en_US",
        },
        {
          text: "中文",
          data: "zh_CN",
        },
      ],
      timezones: [
        {
          text: "UTC (+0:00)",
          data: "UTC",
        },
        {
          text: "Asia/Shanghai (+8:00)",
          data: "Asia/Shanghai",
        },
      ],
      selected_locale: this.config.locale,
      rtc_tz: `${!(this.config.rtc_utc || true) | 0}`,
      timezone: this.config.timezone,
    };
  },
  methods: {
    save_config: function () {
      this.config.rtc_utc = this.rtc_tz == "0";
      this.config.timezone = this.timezone;
      this.config.locale = this.selected_locale;
    },
  },
};
</script>

<template>
  <div>
    <h1>{{ $t("locale.title") }}</h1>
    <p>{{ $t("locale.p1") }}</p>
    <form class="form-layout">
      <label for="locale">{{ $t("locale.l1") }}</label>
      <DKFilterSelect
        :default="selected_locale"
        :options="locales"
        id="locale"
        v-model:selected="selected_locale"
      />
    </form>
    <br />
    <p>{{ $t("locale.p2") }}</p>
    <p class="error-msg"></p>
    <form class="form-layout">
      <label for="timezone">{{ $t("locale.l2") }}</label>
      <p>
        <DKFilterSelect
          :default="timezone"
          :options="timezones"
          id="timezone"
          v-model:selected="timezone"
        />
      </p>
      <label for="rtc">{{ $t("locale.l3") }}</label>
      <p class="select">
        <select id="rtc" name="rtc" v-model="rtc_tz">
          <option value="0">{{ $t("locale.o1") }}</option>
          <option value="1">{{ $t("locale.o2") }}</option>
        </select>
      </p>
    </form>
  </div>
  <DKBottomSteps
    :trigger="save_config"
    :can_proceed="selected_locale != null && timezone != null"
  />
</template>

<style scoped>
input,
select {
  margin-bottom: 0.5em;
  width: 100%;
}
</style>
