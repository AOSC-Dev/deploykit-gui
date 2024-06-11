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
      nameError: false,
      nameEmpty: false,
    };
  },
  mounted() {
    if (!this.config.hostname) {
      this.generated_name = generateHostname();
      this.name = this.generated_name;
    }
  },
  computed: {
    name_style() {
      return this.nameError || this.nameEmpty ? 'error-msg' : '';
    },
  },
  methods: {
    validate() {
      if (this.name === '') {
        this.err_msg = this.$t('host.empty');
        this.nameEmpty = true;
        return false;
      }
      if (!/^[a-zA-Z0-9-]+$/.test(this.name)) {
        this.err_msg = this.$t('host.bad');
        this.nameError = true;
        return false;
      }
      this.nameError = false;
      this.nameEmpty = false;
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
      <div class="form-layout">
        <label for="hostname">{{ $t("host.title") }}</label>
        <input
          id="hostname"
          type="text"
          name="hostname"
          v-model="name"
          :class="name_style"
          @focus="on_focus"
          @keyup.enter="() => {}"
          @input="validate"
        />
      </div>
      <p class="error-msg">{{ err_msg }}</p>
    </div>
  </DKBody>
  <DKBottomSteps
    :trigger="() => (config.hostname = name)"
    :guard="validate"
    :can_proceed="name != ''"
  />
</template>

<style scoped>
.error-msg {
  color: var(--dk-accent);
  height: 1rem;
}

.form-layout {
  display: grid;
  grid-template-columns: 40% 60%;
}

input {
  margin-bottom: 0.5em;
}

[disabled] {
  cursor: not-allowed;
}
</style>
