<script setup lang="ts">
import DKBottomSteps from '@/components/DKBottomSteps.vue';
import DKBody from '../components/DKBody.vue';
</script>

<template>
  <DKBody>
    <h1>{{ $t("welcome.title") }}</h1>
    <p>{{ $t("welcome.p1") }}</p>
    <ul>
      <li>{{ $t("welcome.l1") }}</li>
      <li>{{ $t("welcome.l2") }}</li>
      <li>{{ $t("welcome.l3") }}</li>
    </ul>
    <p>{{ $t("welcome.p2") }}</p>
  </DKBody>
  <DKBottomSteps :no_previous="true" />
</template>
