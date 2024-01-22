<script>
export default {
  props: {
    title: String,
    explain: String,
  },
};
</script>

<template>
  <button class="button" style="height: 100%">
    <div class="icon-container">
      <slot />
      <div class="text-box">
        <span style="font-size: 1rem">{{ explain }}</span>
        <h2 style="font-size: 1.5rem">{{ title }}</h2>
      </div>
    </div>
  </button>
</template>

<style>
.icon-container img {
  margin: 0 0.5rem 0 0;
  height: 2.25rem;
}
</style>

<style scoped>
.text-box {
  line-height: 0.9;
  margin-top: 0rem;
}

.icon-container {
  display: flex;
  line-height: 1.5;
  place-items: flex-start;
  /* flex-wrap: wrap; */
  text-align: left;
}
</style>
