<script setup></script>

<template>
  <div class="bottom-bar"><slot></slot></div>
</template>

<style scoped>
.bottom-bar {
  position: absolute;
  bottom: 0;
  margin-bottom: 1rem;
  width: 100%;
}
</style>
