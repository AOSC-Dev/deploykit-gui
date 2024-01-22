<script>
import DKBottomSteps from "@/components/DKBottomSteps.vue";
export default {
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
  components: { DKBottomSteps },
};
</script>

<template>
  <div>
    <h1>{{ $t("confirm.title") }}</h1>
    <p>{{ $t("confirm.p1") }}</p>
    <ul>
      <i18n-t keypath="confirm.l1" tag="li">
        <template v-slot:path>
          <span class="emphasis">/dev/nvme0n1p2</span>
        </template>
        <template v-slot:explain>
          <span class="emphasis">{{
            $t("confirm.l1-1", { format: "ext4" })
          }}</span>
        </template>
      </i18n-t>
      <ul v-if="rescue_size > 0">
        <li>
          {{ $t("confirm.l2", { size: `${rescue_size}GiB` }) }}
        </li>
      </ul>
      <i18n-t keypath="confirm.l3" tag="li">
        <template v-slot:variant>
          <span class="emphasis">Desktop</span>
        </template>
        <template v-slot:mirror>
          <span class="emphasis">Fastly CDN</span>
        </template>
      </i18n-t>
      <i18n-t keypath="confirm.l4" tag="li">
        <span class="emphasis">anan</span>
      </i18n-t>
      <i18n-t keypath="confirm.l5" tag="li">
        <span class="emphasis">English (United States)</span>
      </i18n-t>
      <i18n-t keypath="confirm.l6" tag="li">
        <span class="emphasis">Asia/Shanghai</span>
      </i18n-t>
      <ul>
        <i18n-t keypath="confirm.l7" tag="li">
          <span class="emphasis">{{ $t("confirm.l7-1") }}</span>
        </i18n-t>
      </ul>
    </ul>
    <p>{{ $t("confirm.w1") }}</p>
  </div>
  <DKBottomSteps />
</template>

<style scoped>
.emphasis {
  color: var(--dk-accent);
  font-weight: 600 !important;
}
</style>
