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
  methods: {
    playUrl(src) {
      return `http://127.0.0.1:23333${src}`;
    },
  },
  mounted() {
    this.$refs.plyr.player.volume = 0.3;
    this.$refs.plyr.player.on('ended', () => {
      this.currentIndex += 1;
      if (this.currentIndex >= this.$props.list.length) {
        this.currentIndex = 0;
      }
      this.$refs.plyr.player.source = {
        type: 'audio',
        sources: [
          {
            src: `http://127.0.0.1:23333${this.$props.list[this.currentIndex].src}`,
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
    <span id="dkaudio-np">Now Playing</span>
    <span id="dkaudio-background">♪</span>
    <span><strong>{{ list[currentIndex].title }}</strong></span>
    <p>{{ list[currentIndex].artist }}</p>
    <vue-plyr ref="plyr" :options="{ controls: ['play', 'mute', 'volume'] }">
      <audio controls crossorigin playsinline autoplay>
        <source :src="playUrl(list[currentIndex].src)" type="audio/mp3" />
      </audio>
    </vue-plyr>
  </div>
</template>

<style scoped>
#dkaudio-np {
  position: absolute;
  top: -25px;
  right: 42px;
  color: #ededed44;
}
#dkaudio-background {
  position: absolute;
  right: -5px;
  font-size: 84px;
  bottom: 20px;
  color: #ededed44;
  font-family: Noto Serif;
}
</style>

<style>
.dkaudio {
  margin-top: 30vh;
  margin-left: 2rem;
}

.dkaudio p {
  margin: unset;
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
