<script>
export default {
  props: {
    options: Array,
    selected: Number,
    no_margin: Boolean,
    is_limit_height: Boolean,
    click_fn: Function,
  },
  methods: {
    select(index) {
      this.$emit('update:selected', index);
      if (this.click_fn) {
        this.click_fn(this);
      }
    },
  },
};
</script>

<template>
  <div
    class="list-container"
    :class="[
      is_limit_height ? 'limit-height' : 'no-limit-height',
    ]"
  >
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

.list-container {
  overflow-y: auto;
}

.limit-height {
  height: 37vh;
}

.no-limit-height {
  height: 100%;
}

::-webkit-scrollbar {
  display: flex;
}

/* ref: https://www.jianshu.com/p/c2addb233acd*/
::-webkit-scrollbar {
  /*滚动条整体样式*/
  width: 10px;
  /*高宽分别对应横竖滚动条的尺寸*/
  height: 1px;
}

::-webkit-scrollbar-thumb {
  /*滚动条里面小方块*/
  border-radius: 10px;
  background-color: skyblue;
  background-image: -webkit-linear-gradient(
    45deg,
    rgba(255, 255, 255, 0.2) 25%,
    transparent 25%,
    transparent 50%,
    rgba(255, 255, 255, 0.2) 50%,
    rgba(255, 255, 255, 0.2) 75%,
    transparent 75%,
    transparent
  );
}

::-webkit-scrollbar-track {
  /*滚动条里面轨道*/
  box-shadow: inset 0 0 5px rgba(0, 0, 0, 0.2);
  background: #ededed;
  border-radius: 10px;
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
