<script>
export default {
  props: {
    main_class: String,
  },
};
</script>

<template>
  <div class="main-container">
    <div style="width: 12rem">
      <slot name="left"></slot>
    </div>
    <main :class="main_class" style="min-width: 34em">
      <div style="height: 100%; overflow-y: auto; margin-right: 3rem">
        <slot></slot>
      </div>
    </main>
  </div>
</template>

<style scoped>
.main-container {
  display: grid;
  grid-template-columns: 1fr 3fr;
  padding: 0 2rem;
  height: 75vh;
}

.main-container main {
  padding-top: 1rem;
  margin-left: 4rem;
  max-width: 35vw;
  height: 100%;
}
</style>
