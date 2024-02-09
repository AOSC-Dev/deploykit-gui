<script setup>
import DKFilterSelect from "@/components/DKFilterSelect.vue";
import DKBottomSteps from "@/components/DKBottomSteps.vue";
</script>

<script>
import lang_data from "../lang_select.json";
import { invoke } from "@tauri-apps/api";
export default {
  props: {},
  inject: ["config"],
  data: function () {
    return {
      loading: true,
      locales: lang_data,
      timezones: [],
      selected_locale: lang_data.findIndex((v) => v.locale === this.config.locale.locale),
      rtc_tz: `${!(this.config.rtc_utc || true) | 0}`,
      timezone: 0,
    };
  },
  methods: {
    save_config: function () {
      this.config.rtc_utc = this.rtc_tz == "0";
      this.config.timezone = this.timezones[this.timezone];
      this.config.locale = lang_data[this.selected_locale];
    },
  },
  async created() {
    try {
      const data = await invoke("list_timezone");
      this.timezones = data;
    } catch (e) {
      this.$router.replace(`/error/${encodeURIComponent(e)}`);
    }

    this.loading = false;
  }
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
          v-if="!loading"
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

select {
  text-align-last: center;
  border-radius: unset;
} 

</style>
