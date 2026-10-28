<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import DKBottomSteps from '@/components/DKBottomSteps.vue';
import { defineComponent, inject } from 'vue';
import DKBody from '../components/DKBody.vue';
import { Config, KeyboardLayout } from '../config.ts';
</script>

<script lang="ts">
export default defineComponent({
  data() {
    const config = inject('config') as Config;

    return {
      config,
      layouts: [] as KeyboardLayout[],
      selectedLayout: config.keyboard?.layout ?? 'us',
      selectedVariant: config.keyboard?.variant ?? '',
      testText: '',
      loading: true,
      err_msg: '',
    };
  },
  computed: {
    currentLayout(): KeyboardLayout | undefined {
      return this.layouts.find((x) => x.name === this.selectedLayout);
    },
    currentVariants() {
      return this.currentLayout?.variants ?? [];
    },
    currentVariant() {
      return this.currentVariants.find((x) => x.name === this.selectedVariant);
    },
    layout_description() {
      return this.currentLayout?.description ?? this.selectedLayout;
    },
    keyboard_summary() {
      if (!this.currentVariant) return this.layout_description;
      return `${this.layout_description} (${this.currentVariant.description})`;
    },
  },
  methods: {
    async apply_layout() {
      this.err_msg = '';
      try {
        await invoke('set_xkeyboard_config', {
          layout: this.selectedLayout,
          variant: this.selectedVariant || null,
        });
      } catch (e) {
        this.err_msg = this.$t('keyboard.error-apply', { err: e });
      }
    },
    on_layout_change() {
      // The previous variant may not exist in the newly selected layout. Reset
      // it and let the variant watcher apply the new layout instead.
      if (
        this.selectedVariant
        && !this.currentVariants.some((x) => x.name === this.selectedVariant)
      ) {
        this.selectedVariant = '';
        return;
      }
      this.apply_layout();
    },
    on_variant_change() {
      this.apply_layout();
    },
    save_config() {
      this.config.keyboard = {
        layout: this.selectedLayout,
        variant: this.selectedVariant || undefined,
        layout_description: this.layout_description,
        variant_description: this.currentVariant?.description,
      };
    },
  },
  watch: {
    selectedLayout() {
      this.on_layout_change();
    },
    selectedVariant() {
      this.on_variant_change();
    },
  },
  async created() {
    try {
      const layouts = (await invoke('list_xkeyboard_config')) as KeyboardLayout[];
      this.layouts = layouts;

      if (!this.layouts.some((x) => x.name === this.selectedLayout)) {
        this.selectedLayout = this.layouts[0]?.name ?? 'us';
      }
      if (
        this.selectedVariant
        && !this.currentVariants.some((x) => x.name === this.selectedVariant)
      ) {
        this.selectedVariant = '';
      }

      // Re-apply the layout when returning to this page (or when resuming).
      if (this.config.keyboard) {
        await this.apply_layout();
      }
    } catch (e) {
      this.err_msg = this.$t('keyboard.error-list', { err: e });
    }

    this.loading = false;
  },
});
</script>

<template>
  <DKBody>
    <div>
      <h1>{{ $t("keyboard.title") }}</h1>
      <p>{{ $t("keyboard.p1") }}</p>
      <div class="form-layout">
        <label for="keyboard-layout" class="label">{{
          $t("keyboard.l1")
        }}</label>
        <el-select
          id="keyboard-layout"
          v-model="selectedLayout"
          filterable
          :disabled="loading"
        >
          <el-option
            v-for="item in layouts"
            :key="item.name"
            :label="`${item.description} (${item.name})`"
            :value="item.name"
          />
        </el-select>

        <label for="keyboard-variant" class="label">{{
          $t("keyboard.l2")
        }}</label>
        <el-select
          id="keyboard-variant"
          v-model="selectedVariant"
          :disabled="loading || currentVariants.length === 0"
        >
          <el-option
            :label="$t('keyboard.default-variant')"
            value=""
          />
          <el-option
            v-for="item in currentVariants"
            :key="item.name"
            :label="`${item.description} (${item.name})`"
            :value="item.name"
          />
        </el-select>
      </div>
      <p class="keyboard-summary">{{ keyboard_summary }}</p>
      <p>{{ $t("keyboard.p2") }}</p>
      <div class="form-layout">
        <label for="keyboard-test" class="label">{{
          $t("keyboard.l3")
        }}</label>
        <input
          id="keyboard-test"
          type="text"
          name="keyboard-test"
          v-model="testText"
          autocomplete="off"
          autocapitalize="off"
          spellcheck="false"
          :placeholder="$t('keyboard.placeholder')"
          :disabled="loading"
        />
      </div>
      <p class="error-msg">{{ err_msg }}</p>
    </div>
  </DKBody>
  <DKBottomSteps :trigger="save_config" :can_proceed="!loading" />
</template>

<style scoped>
.error-msg {
  color: var(--dk-accent);
  height: 1.3rem;
}

.form-layout {
  display: grid;
  grid-template-columns: 40% 60%;
}

.keyboard-summary {
  color: var(--dk-gray);
  margin-top: 0;
}

input {
  margin-bottom: 0.5em;
}

[disabled] {
  cursor: not-allowed;
}
</style>
