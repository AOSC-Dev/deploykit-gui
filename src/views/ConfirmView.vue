<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import DKBottomSteps from '@/components/DKBottomSteps.vue';
import { defineComponent, inject } from 'vue';
import DKBody from '../components/DKBody.vue';
import { Config } from '../config.ts';
</script>

<script lang="ts">
export default defineComponent({
  props: {
    part: String,
    part_fmt: String,
    variant: String,
    cdn_name: String,
    username: String,
    locale: String,
    timezone: String,
    rtc_utc: Boolean,
    rescue_size: Number,
  },
  data() {
    return {
      config: inject('config') as Config,
    };
  },
  methods: {
    async set_config() {
      try {
        await invoke('set_config', { config: JSON.stringify(this.config) });
      } catch (e) {
        const { path } = this.$router.currentRoute.value;

        this.$router.replace({
          path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
          query: { currentRoute: path },
        });
      }
    },
  },
});
</script>

<template>
  <DKBody>
    <div>
      <h1>{{ $t("confirm.title") }}</h1>
      <p>{{ $t("confirm.p1") }}</p>
      <ul>
        <i18n-t keypath="confirm.l1" tag="li">
          <template v-slot:path>
            <span class="emphasis">{{ config.partition?.path }}</span>
          </template>
          <template v-slot:explain>
            <span class="emphasis">{{
              $t("confirm.l1-1", { format: config.partition?.fs_type })
            }}</span>
          </template>
        </i18n-t>
        <i18n-t keypath="confirm.l3" tag="li">
          <template v-slot:variant>
            <span class="emphasis">{{ config.variant.title }}</span>
          </template>
          <template v-slot:mirror>
            <span class="emphasis">{{ config.mirror?.name }}</span>
          </template>
        </i18n-t>
        <i18n-t keypath="confirm.l4" tag="li">
          <span class="emphasis"> {{ config.user }}</span>
        </i18n-t>
        <i18n-t keypath="confirm.l5" tag="li">
          <span class="emphasis">{{ config.locale.text }}</span>
        </i18n-t>
        <i18n-t keypath="confirm.l6" tag="li">
          <span class="emphasis">{{ config.timezone.text }}</span>
        </i18n-t>
        <ul>
          <i18n-t keypath="confirm.l7" tag="li">
            <span v-if="!config.rtc_as_localtime" class="emphasis">{{
              $t("confirm.l7-1")
            }}</span>
            <span v-else class="emphasis">{{ $t("confirm.l7-2") }}</span>
          </i18n-t>
        </ul>
      </ul>
      <p>{{ $t("confirm.w1") }}</p>
    </div>
  </DKBody>
  <DKBottomSteps :trigger="set_config" />
</template>

<style scoped>
.emphasis {
  color: var(--dk-accent);
  font-weight: 600 !important;
}
</style>
