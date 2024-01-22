<script setup>
import DKLayout from "@/components/DKLayout.vue";
import DKListSelect from "@/components/DKListSelect.vue";
import DKBottomRightButtons from "@/components/DKBottomRightButtons.vue";
import DKBottomActions from "@/components/DKBottomActions.vue";
</script>

<script>
import lang_data from "../lang_select.json";
export default {
  data: function () {
    return {
      lang_data: lang_data,
      display_data: lang_data.map((v) => ({ body: v.lang })),
      selection: 0,
    };
  },
  computed: {
    is_inverted: function () {
      return this.lang_data[this.selection].anastrophe;
    },
  },
  methods: {
    select: function (idx) {
      this.$emit("update:lang", this.lang_data[idx].id);
    },
  },
};
</script>

<template>
  <DKLayout>
    <section style="max-height: 65vh; overflow-y: scroll">
      <DKListSelect
        :options="display_data"
        v-model:selected="selection"
      ></DKListSelect>
    </section>
    <DKBottomActions>
      <DKBottomRightButtons>
        <button class="button" @click="select(selection)">
          {{ lang_data[selection]["next"] }}
        </button>
      </DKBottomRightButtons>
    </DKBottomActions>
    <template #left>
      <div>
        <img />
        <div style="line-height: 1" v-if="!is_inverted">
          <h1 style="font-size: 3rem; text-align: right; margin-bottom: 0">
            {{ lang_data[selection]["aosc"] }}
          </h1>
          <h2 style="font-size: 1.25rem; text-align: right">
            {{ lang_data[selection]["inst"] }}
          </h2>
        </div>
        <div style="line-height: 1" v-else>
          <h2 style="font-size: 1.25rem; text-align: right">
            {{ lang_data[selection]["inst"] }}
          </h2>
          <h1 style="font-size: 3rem; text-align: right">
            {{ lang_data[selection]["aosc"] }}
          </h1>
        </div>
      </div>
    </template>
  </DKLayout>
</template>

<style scoped></style>
