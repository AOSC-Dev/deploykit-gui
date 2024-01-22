<script>
export default {
  props: {
    text: String,
    omit_bline: Boolean,
    show_arrow: Boolean,
  },
};
</script>

<template>
  <button :class="'button' + (omit_bline ? ' no-bline' : '')">
    <div class="icon-container">
      <slot></slot>
      <div class="text-box">
        <span>{{ text }}</span>
      </div>
    </div>
    <img src="@/assets/open-fluent-icon.svg" height="14" v-if="show_arrow" />
  </button>
</template>

<style>
div.icon-container img {
  margin: 0.2rem 0.5rem 0.2rem 0.2rem;
  height: 1.1rem;
}
</style>

<style scoped>
button.button {
  background-color: transparent;
  border: 1px solid #eee;
  border-left: 0;
  border-right: 0;
  height: 100%;
  width: 100%;
}

button.button.no-bline {
  border-bottom: 0;
}

button.button:hover {
  background: #dddddd56;
}

.icon-container {
  display: flex;
  line-height: 1.5;
  place-items: center;
  width: 100%;
  text-align: left;
}
</style>
