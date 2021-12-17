<!--
This component allows the user to change their ID and password.
-->
<template>
  <v-menu
    :close-on-content-click="false"
    :nudge-width="300"
    offset-y
    v-if="user"
    v-model="menu"
  >
    <template v-slot:activator="{ on }">
      <v-btn text v-on="on">
        <v-icon>person</v-icon>
        <span>{{ user.id }}</span>
      </v-btn>
    </template>
    <v-card class="px-4 pt-6 pb-2">
      <v-text-field dense label="ID" type="number" v-model.number="user.id" />
      <v-text-field dense label="Password" v-model="user.password" />
      <div class="caption font-italic">
        Copy your ID and password somewhere safe!
      </div>
    </v-card>
  </v-menu>
</template>

<script lang="ts">
import Vue from "vue";
import { State, User } from "@/store/state";

export default Vue.extend({
  name: "AppUser",
  data() {
    return {
      menu: false,
    };
  },
  computed: {
    user: {
      get(): User | null {
        return (this.$store.state as State).user;
      },
    },
  },
});
</script>
