<script>
import DKBottomSteps from "@/components/DKBottomSteps.vue";
export default {
  inject: ["config", "humanSize"],
  computed: {
    max_size: function () {
      return Math.floor(this.rec_size / 536870912);
    },
    rec_size_gb: function () {
      return Math.floor(this.rec_size / 1073741824);
    },
  },
  components: { DKBottomSteps },
  data: function () {
    return {
      type: 0,
      ram_size: 16e9,
      size: 16,
      rec_size: 16e9,
    };
  },
};
</script>

<template>
  <div>
    <h1>{{ $t("swap.title") }}</h1>
    <p>
      {{ $t("swap.p1") }}
    </p>
    <form>
      <section class="form-layout">
        <label for="swap">{{ $t("swap.title") }}</label>
        <p class="select">
          <select name="swap" v-model="type">
            <option :value="0">{{ $t("swap.o1") }}</option>
            <option :value="1">{{ $t("swap.o2") }}</option>
            <option :value="2">{{ $t("swap.o3") }}</option>
          </select>
        </p>
        <div></div>
        <p v-if="type === 0" style="font-size: 0.9rem">
          <i>{{ $t("swap.l1", { size: humanSize(ram_size) }) }}</i>
          <br />
          <i>{{ $t("swap.l2", { size: humanSize(rec_size) }) }}</i>
        </p>
        <p class="error-msg" v-if="type === 2">
          <i>{{ $t("swap.w1") }}</i>
        </p>
      </section>
      <br />
      <div style="display: flex" v-if="type === 1">
        <section style="width: 75%; margin-left: 0.7rem">
          <input
            class="dk-slider"
            type="range"
            :max="max_size"
            min="0"
            step="0.5"
            v-model="size"
          />
          <div class="sliderticks">
            <p>0GiB</p>
            <p>{{ rec_size_gb }}GiB</p>
            <p>{{ max_size }}GiB</p>
          </div>
        </section>
        <span style="float: right; width: 25%; margin-left: 2rem">
          <input
            type="number"
            :max="max_size"
            min="0"
            step="0.5"
            style="width: 67%"
            v-model="size"
            required
          />
          GiB
        </span>
      </div>
      <p v-if="type === 1" style="font-size: 0.9rem; margin-left: 30%">
        <br />
        <i>{{ $t("swap.l1", { size: humanSize(ram_size) }) }}</i>
        <br />
        <i>{{ $t("swap.l3", { size: humanSize(rec_size) }) }}</i>
      </p>
      <p class="error-msg" v-if="type === 1 && size == 0">
        <i>{{ $t("swap.w1") }}</i>
      </p>
    </form>
  </div>
  <DKBottomSteps />
</template>

<style scoped>
.error-msg {
  color: var(--dk-accent);
}

input.dk-slider {
  width: 100%;
  height: 0.25rem;
  background: var(--dk-gray);
  cursor: pointer;
}

input,
select {
  margin-bottom: 0.5em;
  width: 100%;
}

.dk-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 10px;
  height: 20px;
  border-radius: 2px;
  background: var(--vt-c-white-mute);
  cursor: pointer;
}

.dk-slider::-moz-range-thumb {
  width: 10px;
  height: 20px;
  border-radius: 2px;
  background: var(--vt-c-white-mute);
  cursor: pointer;
}

/* https://stackoverflow.com/questions/33699852/show-tick-positions-in-custom-range-input */
.sliderticks {
  display: flex;
  justify-content: space-between;
  padding: 0 5px;
}

.sliderticks p {
  position: relative;
  display: flex;
  justify-content: center;
  text-align: center;
  width: 1px;
  background: #d3d3d3;
  height: 10px;
  line-height: 40px;
}
</style>
