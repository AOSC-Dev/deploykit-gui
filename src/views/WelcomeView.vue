<script setup>
import DKBottomSteps from "@/components/DKBottomSteps.vue";
</script>

<template>
  <div>
    <h1>{{ $t("welcome.title") }}</h1>
    <p>{{ $t("welcome.p1") }}</p>
    <ul>
      <li>{{ $t("welcome.l1") }}</li>
      <li>{{ $t("welcome.l2") }}</li>
      <li>{{ $t("welcome.l3") }}</li>
    </ul>
    <p>{{ $t("welcome.p2") }}</p>
  </div>
  <DKBottomSteps no_previous="1" />
</template>
