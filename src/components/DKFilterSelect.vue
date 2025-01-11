<script lang="ts">
import { defineComponent } from "vue";

interface Option {
  text: string;
  data: string;
}

export default defineComponent({
  props: {
    default: Number,
    id: String,
    options: Array,
  },
  data() {
    return {
      opts: this.options as Option[],
      selected: (this.options?.[this.default as number] as Option).text,
    };
  },
  methods: {
    update_select() {
      const index = this.opts.findIndex((v) => v.text == this.selected);
      this.$emit('update:selected', index);
    }
  }
});
</script>

<template>
  <div class="filter-select">
    <el-select
      v-model="selected"
      filterable
      :filter-method="update_select()"
      placeholder="Select"
    >
      <el-option
        v-for="item in opts"
        :key="item.text"
        :label="item.text"
        :value="item.text"
      />
    </el-select>
  </div>
</template>

<style scoped>
</style>
