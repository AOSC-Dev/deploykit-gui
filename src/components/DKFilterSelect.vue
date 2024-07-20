<script lang="ts">
import { defineComponent } from 'vue';

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
      user_input: '',
      show_dropdown: false,
    };
  },
  mounted() {
    if (this.default != null) this.lock_selection(this.default);
  },
  computed: {
    filtered_options() {
      const currentInput = this.user_input.trim().toLowerCase();
      if (
        !currentInput
        || (this.options as Option[]).filter(
          (x) => x.text.toLowerCase() === currentInput,
        ).length !== 0
      ) return this.options;

      return (this.options as Option[]).filter(
        (v) => v.text.toLowerCase().includes(currentInput)
          || v.data.toLowerCase().includes(currentInput),
      );
    },
  },
  methods: {
    lock_selection(index: number | null) {
      let selected: Option | null;
      if (index === null) {
        selected = null;
        this.show_dropdown = true;
        this.user_input = '';
        this.$emit('update:selected', index);
      } else {
        selected = (this.filtered_options as Option[])[index];
        const i = (this.options as Option[]).findIndex((v) => v === selected);
        this.show_dropdown = false;
        this.user_input = selected.text;
        this.$emit('update:selected', i);
      }
    },
    edit_selection() {
      this.$emit('update:selected', null);
      this.show_dropdown = true;
    },
  },
});
</script>

<template>
  <div class="dropdown-content">
    <div class="select">
      <input
        name="search-box"
        type="text"
        :id="id"
        v-model="user_input"
        :style="show_dropdown ? '' : 'text-align: center'"
        :placeholder="show_dropdown ? $t('search') : $t('select')"
        @focus="edit_selection"
      />
    </div>
    <div class="dropdown-gutter" v-if="show_dropdown">
      <a
        v-for="(opt, index) in filtered_options as Option[]"
        v-bind:key="opt.text"
        @click="lock_selection(index)"
        @keyup.enter="lock_selection(index)"
        tabindex="0"
        >{{ opt.text }}</a
      >
      <span class="placeholder" v-if="(filtered_options as Option[]).length < 1">{{
        $t("empty")
      }}</span>
    </div>
  </div>
</template>

<style scoped>
div.select::after {
  margin-top: -1.2em;
}

.placeholder {
  padding: 12px 16px;
  color: #1f1f1f;
  line-height: 1;
  font-style: italic;
}

.dropdown-content a {
  padding: 4px 16px;
  text-decoration: none;
  color: #1f1f1f;
  display: block;
  text-align: left;
  line-height: 1;
}

.dropdown-content a:hover {
  background: #80a9ff;
  cursor: pointer;
}

.dropdown-content input {
  width: 100%;
}

.dropdown-gutter {
  background-color: #f6f6f6;
  overflow-y: scroll;
  position: absolute;
  left: 0;
  width: 100%;
}

.dropdown-content {
  /* background-color: #f6f6f6; */
  width: 100%;
  min-width: 230px;
  /* border: 1px solid #ddd; */
  z-index: 10;
}
</style>
