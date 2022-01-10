<!--
This component allows the user to change the theme of the app (i.e. dark or light).
It stores the preferred theme in local storage and retrieves it when mounted.
-->
<template>
  <v-btn text @click="changePreferredTheme()">
    <v-icon>settings_brightness</v-icon>
  </v-btn>
</template>

<script lang="ts">
import Vue from "vue";

export enum Theme {
  dark = "#121212",
  light = "#FFFFFF",
}

export default Vue.extend({
  name: "AppTheme",
  methods: {
    // This method updates the theme used by vuetify for the entire app and
    // saves it in local storage. It also update the body's background color.
    changePreferredTheme(): void {
      const isDark = !this.$vuetify.theme.dark;
      this.$vuetify.theme.dark = isDark;
      const newPreferredTheme = isDark ? Theme.dark : Theme.light;
      document.body.style.backgroundColor = newPreferredTheme;
      localStorage.setItem("preferredTheme", newPreferredTheme);
    },
  },
  mounted() {
    // When this component is mounted, this function attempts to retrieve
    // the preferred theme from local storage. If it was indeed saved there,
    // then it updates the theme used by vuetify for the entire app.
    // Note that this component should be mounted only once when the app is
    // first loaded, and not while the user navigates through the app.
    const savedPreferredTheme = localStorage.getItem("preferredTheme");
    if (savedPreferredTheme) {
      this.$vuetify.theme.dark = savedPreferredTheme === Theme.dark;
      document.body.style.backgroundColor = savedPreferredTheme;
    } else {
      document.body.style.backgroundColor = Theme.dark;
    }
  },
});
</script>
