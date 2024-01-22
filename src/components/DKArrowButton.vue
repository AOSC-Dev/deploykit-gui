<script>
export default {
  props: {
    left: Boolean,
  },
};
</script>

<template>
  <button class="button" :class="left ? 'button-left' : 'button-right'">
    <slot />
  </button>
</template>

<style scoped>
.button-left::before {
  content: "";

  border-color: transparent var(--dk-button-color) transparent transparent;
  border-style: solid;
  border-width: calc(var(--dk-button-height) / 2)
    calc(var(--dk-button-height) / 2) calc(var(--dk-button-height) / 2) 0rem;

  /* Position */
  left: 0px;
  position: absolute;
  top: 0px;
  transform: translate(-100%, 0px);
}

.button-right::after {
  content: "";

  border-color: transparent transparent transparent var(--dk-button-color);
  border-style: solid;
  border-width: calc(var(--dk-button-height) / 2) 0rem
    calc(var(--dk-button-height) / 2) calc(var(--dk-button-height) / 2);

  /* Position */
  right: 0px;
  position: absolute;
  top: 0px;
  transform: translate(100%, 0px);
}
</style>
