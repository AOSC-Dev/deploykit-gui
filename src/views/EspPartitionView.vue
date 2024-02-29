<script setup>
import DKListSelect from '@/components/DKListSelect.vue';
import DKBottomSteps from '@/components/DKBottomSteps.vue';
</script>

<script>
export default {
  props: {
    esps: String,
  },
  inject: ['config', 'humanSize'],
  data() {
    return {
      esp_parts: [],
      selected: null,
      loading: true,
      req_size: null,
      err_msg: '',
    };
  },
  async created() {
    const esps = JSON.parse(decodeURIComponent(this.$props.esps));
    this.esp_parts = esps;
  },
};
</script>

<template>
  <h1>{{ $t("esp.title") }}</h1>
  <p>{{ $t("esp.p1") }}</p>
  <p>{{ $t("esp.p2") }}</p>
  <section>
    <DKListSelect
      :no_margin="true"
      v-model:selected="selected"
      :is_limit_height="true"
      :options="esp_parts"
      :small_vh="true"
    >
      <template #item="option">
        <div style="width: 100%">
          <span class="column-80">{{ option.path }}</span>
          <span class="column-20">{{ humanSize(option.size) }}</span>
        </div>
      </template>
    </DKListSelect>
    <DKBottomSteps
      :trigger="
        () => {
          config.efi_partition = esp_parts[selected];
        }
      "
      :can_proceed="selected != null"
    />
  </section>
</template>

<style scoped>
.column-80 {
  font-weight: 600;
  width: 80%;
  display: inline-block;
}

.column-20 {
  width: 20%;
  display: inline-block;
}

p.secondary span {
  color: var(--dk-gray);
}

p.secondary {
  margin: 0;
}
</style>
