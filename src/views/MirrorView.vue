<script setup>
import DKBottomSteps from '@/components/DKBottomSteps.vue';
import DKBody from '@/components/DKBody.vue';
</script>

<template>
  <DKBody>
    <div>
      <h1>{{ $t("mirror.title") }}</h1>
      <section>
        <p>{{ $t("mirror.p1") }}</p>
        <button class="button" @click="$router.push('/mirrors-sel')">
          {{ $t("mirror.b1") }}
        </button>
      </section>
    </div>
  </DKBody>
  <DKBottomSteps />
</template>

<script>
export default {
  inject: ['config'],
  mounted() {
    if (this.config.is_offline_install) {
      this.$router.replace('/device');
    }
  },
};
</script>
