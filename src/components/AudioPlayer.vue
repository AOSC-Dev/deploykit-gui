<script setup>
import stringWidth from 'string-width';
</script>

<script>
export default {
  props: {
    list: Array,
    volume: Number,
    muted: Boolean,
    playing: Boolean,
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
    stop() {
      const fadeInterval = setInterval(() => {
        if (this.$refs.plyr.player.volume <= 0 || this.muted) {
          clearInterval(fadeInterval);
          this.$refs.plyr.player.pause();
        }
        this.$refs.plyr.player.volume -= 0.001;
      }, 100);
    },
  },
  mounted() {
    this.$refs.plyr.player.volume = this.volume;
    this.$refs.plyr.player.muted = this.muted;
    this.$refs.plyr.player.on('ended', () => {
      this.currentIndex += 1;
      if (this.currentIndex >= this.$props.list.length) {
        this.currentIndex = 0;
      }
      this.$refs.plyr.player.source = {
        type: 'audio',
        sources: [
          {
            src: `http://127.0.0.1:23333${
              this.$props.list[this.currentIndex].src
            }`,
            type: 'audio/mp3',
          },
        ],
      };
      this.$refs.plyr.player.play();
    });

    this.$refs.plyr.player.on('volumechange', () => {
      this.$emit('update:volume', this.$refs.plyr.player.volume);
      this.$emit('update:muted', this.$refs.plyr.player.muted);
    });

    this.$refs.plyr.player.on('playing', () => {
      this.$emit('update:playing', true);
    });

    this.$refs.plyr.player.on('pause', () => {
      this.$emit('update:playing', false);
    });
  },
};
</script>

<template>
  <div class="dkaudio">
    <span id="dkaudio-np">Now Playing</span>
    <span id="dkaudio-background">♪</span>
    <marquee
      v-if="stringWidth(list[currentIndex].title) > 16"
      class="music-title"
      scrollamount="3"
      >{{ list[currentIndex].title }}</marquee
    >
    <span v-else
      ><strong>{{ list[currentIndex].title }}</strong></span
    >
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

.music-title {
  display: inline-block;
  font-weight: bold;
}

.music-title-no-loop {
  display: inline-block;
  font-weight: bold;
}

@keyframes wordsLoop {
  0% {
    transform: translateX(200px);
    -webkit-transform: translateX(200px);
  }
  100% {
    transform: translateX(-100%);
    -webkit-transform: translateX(-100%);
  }
}

@-webkit-keyframes wordsLoop {
  0% {
    transform: translateX(200px);
    -webkit-transform: translateX(200px);
  }
  100% {
    transform: translateX(-100%);
    -webkit-transform: translateX(-100%);
  }
}
</style>
