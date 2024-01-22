<script>
export default {
  props: {
    options: Array,
    selected: Number,
    no_margin: Boolean,
  },
  methods: {
    select: function (index) {
      this.$emit("update:selected", index);
    },
  },
};
</script>

<template>
  <div class="list-container">
    <button
      v-for="(option, index) in options"
      v-bind:key="option.title"
      :class="(index === selected ? 'selected ' : ' ') + 'button'"
      :disabled="option.disabled"
      @click="select(index)"
      @keyup.enter="select(index)"
      @keyup.space="select(index)"
    >
      <div :class="'button-box' + (no_margin ? '' : ' use-margin')">
        <slot name="item" v-bind="option">
          <h2 style="font-size: 1rem; font-weight: 600; margin-bottom: 0.3rem">
            {{ option.title }}
          </h2>
          <p
            style="font-size: 0.88rem; line-height: 1.2"
            :class="option.hl ? 'hl-msg' : ''"
          >
            {{ option.body }}
          </p>
        </slot>
      </div>
    </button>
  </div>
</template>

<style scoped>
button.button.selected {
  background-color: var(--dk-button-color);
}

button.button.selected .hl-msg {
  color: var(--dk-accent);
}

.button:hover {
  background: #dddddd56;
}

button[disabled].button:hover {
  background: transparent;
}

.list-container button {
  height: unset;
  line-height: 1.2rem;
  background-color: transparent;
  width: 100%;
  align-content: flex-start;
}

.button-box.use-margin {
  margin: 0.5rem 0.5rem 0 0.5rem;
}

.button-box {
  text-align: left;
  width: 100%;
}

.error-msg {
  color: var(--dk-accent);
}
</style>
