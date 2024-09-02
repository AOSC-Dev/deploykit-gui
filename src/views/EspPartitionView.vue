<script setup lang="ts">
import DKListSelect from '@/components/DKListSelect.vue';
import DKBottomSteps from '@/components/DKBottomSteps.vue';
import { defineComponent, inject } from 'vue';
import DKBody from '../components/DKBody.vue';
import { Config } from '../config.ts';
</script>

<script lang="ts">
export default defineComponent({
  props: {
    esps: String,
  },
  data() {
    return {
      esp_parts: [],
      selected: null as number | null,
      loading: true,
      req_size: null,
      err_msg: '',
      config: inject('config') as Config,
      humanSize: inject('humanSize') as Function,
    };
  },
  async created() {
    if (this.$props.esps) {
      const esps = JSON.parse(decodeURIComponent(this.$props.esps));
      this.esp_parts = esps;
    } else {
      this.err_msg = this.$t('esp.error_msg');
    }
  },
});
</script>

<template>
  <DKBody>
    <h1>{{ $t("esp.title") }}</h1>
    <p>{{ $t("esp.p1") }}</p>
    <p>{{ $t("esp.p2") }}</p>
    <section>
      <DKListSelect
        :no_margin="true"
        v-model:selected="selected"
        :is_limit_height="true"
        :options="esp_parts"
      >
        <template #item="option">
          <div style="width: 100%">
            <span class="column-80">{{ option.path }}</span>
            <span class="column-20">{{ humanSize(option.size) }}</span>
          </div>
        </template>
      </DKListSelect>
    </section>
  </DKBody>
  <DKBottomSteps
    :trigger="
      () => {
        if (selected === null) {
          return;
        }
        config.efi_partition = esp_parts[selected];
      }
    "
    :can_proceed="selected != null"
  />
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
