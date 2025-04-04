<script setup lang="ts">
import DKBottomSteps from "@/components/DKBottomSteps.vue";
import DKSpinner from "@/components/DKSpinner.vue";
import DKBody from "@/components/DKBody.vue";
</script>

<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { defineComponent, inject } from "vue";
import { Config, SquashfsInfo } from "../config.ts";

function recommendSizeGiB(recommendSize: number) {
  return Math.floor(recommendSize / 1073741824);
}

export default defineComponent({
  computed: {
    max_size() {
      return 32;
    },
    rec_size_gb() {
      return Math.floor(this.recommendSize / 1073741824);
    },
  },
  data() {
    return {
      type: 0,
      ramSize: 0,
      size: 0,
      recommendSize: 0,
      loading: true,
      canRecommend: true,
      config: inject("config") as Config,
      humanSize: inject("humanSize") as Function,
    };
  },
  async created() {
    try {
      const requireRamSize = (await invoke("get_memory")) as number;
      const recommendSwapSize = (await invoke(
        "get_recommend_swap_size"
      )) as number;

      this.ramSize = requireRamSize;
      this.recommendSize =
        recommendSwapSize > 32 * 1024 * 1024 * 1024
          ? 32 * 1024 * 1024 * 1024
          : recommendSwapSize;

      let sqfsSize;
      if (!this.config.is_offline_install) {
        const squashfsInfo = (await invoke("get_squashfs_info", {
          v: this.config.variant,
          url: this.config.mirror?.url,
        })) as SquashfsInfo;
        sqfsSize = squashfsInfo.downloadSize + squashfsInfo.instSize;
      } else {
        const info = (await invoke("get_squashfs_info", {
          v: this.config.variant,
        })) as SquashfsInfo;
        sqfsSize = info.instSize * 1.25;
      }

      if (this.config.partition === undefined) {
        return;
      }

      if (this.recommendSize > this.config.partition.size - sqfsSize) {
        this.canRecommend = false;
        this.type = 1;
        this.recommendSize = this.config.partition.size - sqfsSize;
      }

      if (this.recommendSize < 0) {
        this.recommendSize = 0;
      }

      this.size = recommendSizeGiB(this.recommendSize);

      if (this.config.swapfile) {
        this.type = 1;
        this.size = this.config.swapfile.size;
      }
    } catch (e) {
      const { path } = this.$router.currentRoute.value;

      this.$router.replace({
        path: `/error/${encodeURIComponent(JSON.stringify(e))}`,
        query: { currentRoute: path },
      });
    }

    this.loading = false;
  },
});
</script>

<template>
  <DKBody>
    <div v-if="!loading">
      <h1>{{ $t("swap.title") }}</h1>
      <p>
        {{ $t("swap.p1") }}
      </p>
      <div>
        <section class="form-layout">
          <label for="swap" class="label">{{ $t("swap.title") }}</label>
          <p class="select">
            <el-select id="swap" name="swap" v-model="type">
              <el-option
                key="0"
                v-if="canRecommend"
                :label="$t('swap.o1')"
                :value="0"
              ></el-option>
              <el-option key="1" :label="$t('swap.o2')" :value="1"></el-option>
              <el-option key="2" :label="$t('swap.o3')" :value="2"></el-option>
            </el-select>
          </p>
          <div></div>
          <p v-if="type === 0" style="font-size: 0.9rem">
            <i>{{ $t("swap.l1", { size: humanSize(ramSize) }) }}</i>
            <br />
            <i>{{ $t("swap.l2", { size: humanSize(recommendSize) }) }}</i>
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
              <p>{{ max_size / 2 }}GiB</p>
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
          <i>{{ $t("swap.l1", { size: humanSize(ramSize) }) }}</i>
          <br />
          <i>{{ $t("swap.l3", { size: humanSize(recommendSize) }) }}</i>
        </p>
        <p class="error-msg" v-if="type === 1 && size == 0">
          <i>{{ $t("swap.w1") }}</i>
        </p>
      </div>
    </div>
    <!-- loading screen -->
    <div class="loading" v-else>
      <DKSpinner :title="$t('swap.lo1')" />
    </div>
  </DKBody>
  <DKBottomSteps
    :trigger="() => (config.swapfile = { size: type === 2 ? 0 : Number(size) })"
  />
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

select {
  text-align-last: center;
  border-radius: unset;
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

.label {
  margin-top: .3em;
}
</style>
