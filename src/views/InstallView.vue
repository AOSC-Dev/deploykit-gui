<script lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { defineComponent } from 'vue';

export default defineComponent({
  data() {
    return {
      current_slide: {
        title: this.$t('loading'),
        paras: [] as string[],
      },
      index: 0,
      timer: undefined as number | undefined,
      slides: [
        { title: this.$t('slides.t1'), body: this.$t('slides.p1') },
        { title: this.$t('slides.t2'), body: this.$t('slides.p2') },
        { title: this.$t('slides.t3'), body: this.$t('slides.p3') },
        { title: this.$t('slides.t4'), body: this.$t('slides.p4') },
        { title: this.$t('slides.t5'), body: this.$t('slides.p5') },
        { title: this.$t('slides.t6'), body: this.$t('slides.p6') },
        { title: this.$t('slides.t7'), body: this.$t('slides.p7') },
      ],
      hide: false,
    };
  },
  methods: {
    next_slide() {
      this.hide = true;
      const newSlide = this.slides[(this.index + 1) % this.slides.length];
      this.index += 1;
      setTimeout(() => {
        this.current_slide = {
          title: newSlide.title,
          paras: newSlide.body.split('\n'),
        };
        this.hide = false;
      }, 200);
    },
    click() {
      this.next_slide();
      clearInterval(this.timer);
      this.timer = window.setInterval(this.next_slide, 15000);
    },
  },
  mounted() {
    this.current_slide = {
      title: this.slides[0].title,
      paras: this.slides[0].body.split('\n'),
    };

    this.timer = window.setInterval(this.next_slide, 15000);

    invoke('start_install');
  },
  beforeUnmount() {
    clearInterval(this.timer);
  },
});
</script>

<template>
  <div :class="'slide-show' + (hide ? ' hidden' : '')" @click="click">
    <h1>{{ current_slide.title }}</h1>
    <article>
      <p v-for="(para, index) in current_slide.paras" v-bind:key="index">
        {{ para }}
      </p>
    </article>
  </div>
</template>

<style>
.hidden {
  opacity: 0;
}

.slide-show {
  transition: all 200ms cubic-bezier(0.075, 0.82, 0.165, 1);
  height: 60vh;
}
</style>
