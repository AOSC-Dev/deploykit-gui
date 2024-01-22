<script setup>
import DKBottomSteps from "@/components/DKBottomSteps.vue";
</script>

<script>
export default {
  inject: ["config"],
  data: function () {
    return {
      user: this.config.user || "",
      pwd: "",
      pwd2: "",
      error_msg: "",
      name_error: false,
      pwd_error: false,
    };
  },
  computed: {
    name_style: function () {
      return this.name_error ? "error-msg" : "";
    },
    pwd_style: function () {
      return this.pwd_error ? "error-msg" : "";
    },
  },
  methods: {
    validate_user: function () {
      const username = this.user.trimEnd();
      if (!username) {
        this.error_msg = this.$t("user.bad1");
        this.name_error = true;
        return false;
      }
      if (!/^[a-z][a-z0-9-]*$/.test(username)) {
        this.error_msg = this.$t("user.bad4");
        this.name_error = true;
        return false;
      }
      this.name_error = false;
      this.error_msg = "";
      return true;
    },
    validate_cpassword: function () {
      if (this.pwd2.length >= this.pwd.length) {
        if (this.pwd !== this.pwd2) {
          this.error_msg = this.$t("user.bad3");
          this.pwd_error = true;
          return false;
        }
        this.error_msg = "";
        this.pwd_error = false;
        return true;
      }
    },
    validate: function () {
      if (!this.validate_user()) return false;

      if (this.pwd.length < 1 || this.pwd2.length < 1) {
        this.error_msg = this.$t("user.bad2");
        this.name_error = true;
        return false;
      }
      this.name_error = false;
      if (this.pwd !== this.pwd2) {
        this.error_msg = this.$t("user.bad3");
        this.pwd_error = true;
        return false;
      }
      this.pwd_error = false;
      this.error_msg = "";
      return true;
    },
    save_config: function () {
      this.config.user = this.user;
      this.config.pwd = this.pwd;
    },
  },
};
</script>

<template>
  <div>
    <h1>{{ $t("user.title") }}</h1>
    <p>{{ $t("user.p1") }}</p>
    <form class="form-layout">
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
        @input="validate_cpassword"
      />
    </form>
    <p class="error-msg">{{ error_msg }}</p>
  </div>
  <DKBottomSteps
    :trigger="save_config"
    :guard="validate"
    :can_proceed="pwd2.length && !error_msg"
  />
</template>

<style scoped>
.error-msg {
  color: var(--dk-accent);
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
