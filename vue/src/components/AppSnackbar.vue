<!--
This component shows a green or red snackbar with a message at the
bottom of the screen whenever a request succeeds or fails.
-->
<template>
  <v-snackbar :color="color" :timeout="8000" v-model="isOn">
    {{ message }}
  </v-snackbar>
</template>

<script lang="ts">
import Vue from "vue";
import { State } from "@/store/state";

export interface Snackbar {
  color: SnackbarColor;
  isOn: boolean;
  message: string;
}

export enum SnackbarColor {
  success = "success",
  error = "error",
}

export default Vue.extend({
  name: "AppSnackbar",
  computed: {
    // Retrieve the snackbar's color from the vuex store.
    color: {
      get(): SnackbarColor {
        return (this.$store.state as State).snackbar.color;
      },
    },
    // Retrieve the snackbar's message from the vuex store.
    message: {
      get(): string {
        return (this.$store.state as State).snackbar.message;
      },
    },
    // Synchronize whether the snackbar is on or off with the vuex store.
    isOn: {
      get(): boolean {
        return (this.$store.state as State).snackbar.isOn;
      },
      set(isOn: boolean): void {
        this.$store.commit("setSnackbar", isOn);
      },
    },
  },
});
</script>
