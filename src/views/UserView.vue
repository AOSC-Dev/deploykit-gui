<script setup>
import DKBottomSteps from '@/components/DKBottomSteps.vue';
import DKBody from '../components/DKBody.vue';
</script>

<script>
export default {
  inject: ['config'],
  data() {
    return {
      user: this.config.user || '',
      pwd: '',
      pwd2: '',
      error_msg: '',
      name_error: false,
      pwd_error: false,
    };
  },
  computed: {
    name_style() {
      return this.name_error ? 'error-msg' : '';
    },
    pwd_style() {
      return this.pwd_error ? 'error-msg' : '';
    },
  },
  methods: {
    validate_user() {
      const username = this.user.trimEnd();
      if (!username) {
        this.error_msg = this.$t('user.bad1');
        this.name_error = true;
        return false;
      }
      if (!/^[a-z][a-z0-9-]*$/.test(username)) {
        this.error_msg = this.$t('user.bad4');
        this.name_error = true;
        return false;
      }
      this.name_error = false;
      this.error_msg = '';
      return true;
    },
    validateCpassword() {
      if (this.pwd2.length >= this.pwd.length) {
        if (this.pwd !== this.pwd2) {
          this.error_msg = this.$t('user.bad3');
          this.pwd_error = true;
          return false;
        }
        this.error_msg = '';
        this.pwd_error = false;
        return true;
      }

      return true;
    },
    validate() {
      if (!this.validate_user()) return false;

      if (this.pwd.length < 1 || this.pwd2.length < 1) {
        this.error_msg = this.$t('user.bad2');
        this.name_error = true;
        return false;
      }
      this.name_error = false;
      if (this.pwd !== this.pwd2) {
        this.error_msg = this.$t('user.bad3');
        this.pwd_error = true;
        return false;
      }
      this.pwd_error = false;
      this.error_msg = '';
      return true;
    },
    save_config() {
      this.config.user = this.user;
      this.config.pwd = this.pwd;
    },
  },
};
</script>

<template>
  <DKBody>
    <div>
      <h1>{{ $t("user.title") }}</h1>
      <p>{{ $t("user.p1") }}</p>
      <div class="form-layout">
        <label for="username" :class="name_style">{{ $t("user.l1") }}</label>
        <input
          id="username"
          name="username"
          type="text"
          v-model="user"
          :class="name_style"
          @blur="validate_user"
        />
        <label for="pwd" :class="pwd_style">{{ $t("user.l2") }}</label>
        <input
          id="pwd"
          name="pwd"
          type="password"
          v-model="pwd"
          :class="pwd_style"
          @input="pwd2 = ''"
        />
        <label for="pwd2" :class="pwd_style">{{ $t("user.l3") }}</label>
        <input
          id="pwd2"
          name="pwd2"
          type="password"
          v-model="pwd2"
          :class="pwd_style"
          :disabled="!pwd"
          @input="validateCpassword"
        />
      </div>
      <div class="error-msg">
        <span>{{ error_msg }}</span>
      </div>
    </div>
  </DKBody>
  <DKBottomSteps
    :trigger="save_config"
    :guard="validate"
    :can_proceed="pwd2.length && !error_msg"
  />
</template>

<style scoped>
.error-msg {
  color: var(--dk-accent);
  height: 1.3rem;
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
