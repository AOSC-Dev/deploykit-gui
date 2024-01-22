<script setup>
import DKBottomActions from "@/components/DKBottomActions.vue";
import DKStripButton from "@/components/DKStripButton.vue";
</script>

<template>
  <div>
    <h1>{{ $t("abort.title") }}</h1>
    <i18n-t keypath="abort.p1" tag="p">
      {{ $t("abort.p1-1") }}
    </i18n-t>
    <p>{{ $t("abort.p2") }}</p>
  </div>
  <DKBottomActions>
    <DKStripButton
      omit_bline="1"
      :text="$t('abort.resume')"
      @click="$router.back()"
    >
      <img src="@/assets/resume.svg" height="36" />
    </DKStripButton>
    <DKStripButton omit_bline="1" :text="$t('abort.save')">
      <img src="@/assets/document-save.svg" height="36" />
    </DKStripButton>
    <DKStripButton :text="$t('abort.quit')">
      <img src="@/assets/exit-run.svg" height="36" />
    </DKStripButton>
  </DKBottomActions>
</template>
