<template>
  <div class="button-group">
    <slot></slot>
  </div>
</template>

<style scoped>
.button-group {
  float: right;
  display: flex;
  align-items: flex-end;
  column-gap: 0.5rem;
  margin-top: 2rem;
}
</style>
