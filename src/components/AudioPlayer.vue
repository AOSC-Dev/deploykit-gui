<script>
export default {
  props: {
    list: Array,
  },
  data() {
    return {
      currentIndex: 0,
    };
  },
  mounted() {
    this.$refs.plyr.player.on('ended', () => {
      this.currentIndex += 1;
      if (this.currentIndex >= this.$props.list.length) {
        this.currentIndex = 0;
      }
      this.$refs.plyr.player.source = {
        type: 'audio',
        sources: [
          {
            src: this.$props.list[this.currentIndex],
            type: 'audio/mp3',
          },
        ],
      };
      this.$refs.plyr.player.play();
    });
  },
};
</script>

<template>
  <div class="dkaudio">
    <vue-plyr ref="plyr" :options="{ controls: ['play', 'mute', 'volume'] }">
      <audio controls crossorigin playsinline autoplay>
        <source :src="list[currentIndex]" type="audio/mp3" />
      </audio>
    </vue-plyr>
  </div>
</template>

<style>
.dkaudio {
  margin-top: 40vh;
}

.dkaudio {
  --plyr-color-main: #fffff;
  --plyr-audio-controls-background: none;
  --plyr-audio-control-color: #fffff;
}

.dkaudio .plyr__controls .plyr__controls__item:first-child {
  margin-right: unset;
}
</style>
