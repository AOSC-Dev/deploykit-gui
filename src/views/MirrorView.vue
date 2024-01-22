<script setup>
import DKBottomSteps from "@/components/DKBottomSteps.vue";
</script>

<template>
  <div>
    <h1>{{ $t("mirror.title") }}</h1>
    <section>
      <p>{{ $t("mirror.p1") }}</p>
      <button class="button" @click="$router.push('/mirrors-sel')">
        {{ $t("mirror.b1") }}
      </button>
    </section>
  </div>
  <DKBottomSteps />
</template>
