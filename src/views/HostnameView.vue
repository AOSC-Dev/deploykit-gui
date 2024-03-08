<script setup>
import DKBottomSteps from '@/components/DKBottomSteps.vue';
import DKBody from '../components/DKBody.vue';
</script>

<script>
function generateHostname() {
  let output = 'aosc-';
  const random = new Uint8Array(4);
  window.crypto.getRandomValues(random);
  for (let i = 0; i < 4; i += 1) {
    output += random[i].toString(16);
  }
  return output;
}

export default {
  inject: ['config'],
  data() {
    return {
      name: this.config.hostname,
      generated_name: null,
      err_msg: '',
    };
  },
  mounted() {
    if (!this.config.hostname) {
      this.generated_name = generateHostname();
      this.name = this.generated_name;
    }
  },
  methods: {
    validate() {
      if (!/^[a-z0-9-]+$/.test(this.name)) {
        this.err_msg = this.$t('host.bad');
        return false;
      }
      this.err_msg = '';
      return true;
    },
    on_focus() {
      if (this.name === this.generated_name) setTimeout(() => document.getElementById('hostname').select(), 150);
    },
  },
};
</script>

<template>
  <DKBody>
    <div>
      <h1>{{ $t("host.title") }}</h1>
      <p>{{ $t("host.p1") }}</p>
      <form class="form-layout">
        <label for="hostname">{{ $t("host.title") }}</label>
        <input id="hostname" name="hostname" v-model="name" @focus="on_focus" />
      </form>
      <p class="error-msg">{{ err_msg }}</p>
    </div>
  </DKBody>
  <DKBottomSteps
    :trigger="() => (config.hostname = name)"
    :guard="validate"
    :can_proceed="name != ''"
  />
</template>
