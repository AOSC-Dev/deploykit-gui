<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import DKFilterSelect from "@/components/DKFilterSelect.vue";
import DKBottomSteps from "@/components/DKBottomSteps.vue";
import DKBody from "@/components/DKBody.vue";
import { defineComponent, inject } from "vue";
import { Config } from "../config.ts";
</script>

<script lang="ts">
import langData from "../lang_select.json";

interface Timezone {
  text: string;
  data: string;
}

export default defineComponent({
  props: {},
  data() {
    const config = inject("config") as Config;
    return {
      loading: true,
      locales: langData,
      timezones: [] as Timezone[],
      selectedLocale: langData.findIndex(
        (v) => v.locale === config.locale.locale
      ),
      rtcTimezone: "0",
      selectedTimezone: null as number | null,
      config,
    };
  },
  methods: {
    save_config() {
      this.config.rtc_as_localtime = this.rtcTimezone === "1";
      if (this.selectedTimezone !== null) {
        this.config.timezone = this.timezones[this.selectedTimezone];
      }
      this.config.locale = langData[this.selectedLocale];
    },
    canProced() {
      return this.selectedLocale != null && this.selectedTimezone != null;
    },
  },
  async created() {
    try {
      const data = (await invoke("list_timezone")) as Timezone[];
      this.timezones = data;

      if (this.config.timezone) {
        this.selectedTimezone = this.timezones.findIndex(
          (v) => v.text === this.config.timezone.text
        );
      } else {
        this.selectedTimezone = 0;
      }

      if (this.config.rtc_as_localtime !== null) {
        this.rtcTimezone = this.config.rtc_as_localtime ? "1" : "0";
      }

      if (this.config.locale) {
        this.selectedLocale = langData.findIndex(
          (v) => v.locale === this.config.locale.locale
        );
      }
    } catch (e) {
      const { path } = this.$router.currentRoute.value;

      this.$router.replace({
        path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
        query: { currentRoute: path },
      });
    }

    this.loading = false;
  },
});
</script>

<template>
  <DKBody>
    <div>
      <h1>{{ $t("locale.title") }}</h1>
      <p>{{ $t("locale.p1") }}</p>
      <div class="form-layout">
        <label for="locale" class="label">{{ $t("locale.l1") }}</label>
        <DKFilterSelect
          :default="selectedLocale"
          :options="locales"
          id="locale"
          v-model:selected="selectedLocale"
        />
      </div>
      <br />
      <p>{{ $t("locale.p2") }}</p>
      <p class="error-msg"></p>
      <div class="form-layout">
        <label for="timezone" class="label">{{ $t("locale.l2") }}</label>
        <p>
          <DKFilterSelect
            v-if="!loading"
            :default="selectedTimezone"
            :options="timezones"
            id="timezone"
            v-model:selected="selectedTimezone"
          />
        </p>
        <label for="rtc" class="label">{{ $t("locale.l3") }}</label>
        <p>
          <el-select id="rtc" name="rtc" v-model="rtcTimezone">
            <el-option key="0" :label="$t('locale.o1')" value="0"></el-option>
            <el-option key="1" :label="$t('locale.o2')" value="1"></el-option>
          </el-select>
        </p>
      </div>
    </div>
  </DKBody>
  <DKBottomSteps :trigger="save_config" :can_proceed="canProced()" />
</template>

<style scoped>
.label {
  margin-top: .3em;
}

.form-layout {
  margin-bottom: 1em;
}
</style>
