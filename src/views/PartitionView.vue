<script setup>
import DKStripButton from "@/components/DKStripButton.vue";
import DKBottomActions from "@/components/DKBottomActions.vue";
import DKStepButtons from "@/components/DKStepButtons.vue";
import DKListSelect from "@/components/DKListSelect.vue";
import DKSpinner from "@/components/DKSpinner.vue";
</script>

<script>
export default {
  inject: ["config", "humanSize"],
  data: function () {
    return {
      selected: null,
      gparted: false,
      partitions: [],
      loading: false,
    };
  },
  computed: {
    new_disk: function () {
      return !this.partitions || this.partitions.length < 1;
    },
    valid: function () {
      return !this.gparted && (this.new_disk || this.selected != null);
    },
  },
  watch: {
    loading(newValue) {
      this.$emit("update:can_quit", !newValue && !this.gparted);
    },
    gparted(newValue) {
      this.$emit("update:can_quit", !newValue && !this.loading);
    },
  },
  methods: {
    comment: function (comment) {
      switch (comment) {
        case "esp":
          return this.$t("part.k1");
        case "mbr":
          return this.$t("part.k3");
        case "winre":
          return this.$t("part.k2");
      }
      if (comment.length > 20) {
        return this.$t("part.k5", { other_os: comment.substring(0, 20) });
      }
      return this.$t("part.k4", { other_os: comment });
    },
    launch_gparted: function () {
      this.gparted = true;
      this.$ipc.call("exec", ["gparted"]).then(this.list_partitions);
    },
    list_partitions: function () {
      this.loading = this.gparted = true;
      this.$ipc.call("list_partitions", []).then((data) => {
        this.partitions = data;
        this.gparted = this.loading = false;
      });
    },
  },
};
</script>

<template>
  <div v-if="!loading">
    <h1>{{ $t("part.title") }}</h1>
    <section v-if="!new_disk">
      <p>{{ $t("part.p1") }}</p>
      <section>
        <DKListSelect
          :no_margin="true"
          v-model:selected="selected"
          :options="partitions"
        >
          <template #item="option">
            <div style="width: 100%">
              <span class="column-85"
                >{{ option.path }}&nbsp;{{ comment(option.comment) }}</span
              >
              <span class="column-15">{{ option.size }}</span>
              <p class="secondary">
                <span>{{ option.fs_type || $t("part.k0") }}</span>
              </p>
            </div>
          </template>
        </DKListSelect>
      </section>
    </section>

    <section v-if="new_disk">
      <p>{{ $t("part.p2") }}</p>
      <ul>
        <i18n-t keypath="part.l1" tag="li">
          <strong>128MiB</strong>
        </i18n-t>
        <i18n-t keypath="part.l2" tag="li">
          <strong>20GiB</strong>
        </i18n-t>
      </ul>
      <p>
        {{ $t("part.p3") }}
      </p>
    </section>
  </div>
  <!-- loading screen -->
  <div class="loading" v-else>
    <h1>{{ $t("part.title") }}</h1>
    <DKSpinner :title="$t('part.r1')" />
  </div>
  <DKBottomActions v-if="!gparted && !loading">
    <DKStripButton @click="launch_gparted" :text="$t('part.b1')">
      <img src="@/assets/drive-harddisk-root-symbolic.svg" height="18" />
    </DKStripButton>
    <DKStepButtons :can_proceed="valid" />
  </DKBottomActions>
</template>

<style scoped>
.column-85 {
  font-weight: 600;
  width: 85%;
  display: inline-block;
}

.column-15 {
  width: 15%;
  display: inline-block;
}

/* p.secondary span {
  color: var(--dk-gray);
} */

p.secondary {
  margin: 0;
}
</style>
