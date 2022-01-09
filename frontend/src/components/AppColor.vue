<!--
This component allows the user to change the primary color of the app.
It stores the preferred color in local storage and retrieves it when mounted.
-->
<template>
  <v-menu offset-y>
    <template v-slot:activator="{ on }">
      <v-btn text v-on="on">
        <v-icon>invert_colors</v-icon>
      </v-btn>
    </template>
    <v-list>
      <v-list-item
        :key="`app-color-${index}`"
        v-for="(color, index) in colors"
        @click="changePreferredColor(color)"
      >
        <v-icon class="pl-1" :color="color">circle</v-icon>
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<script lang="ts">
import Vue from "vue";

export enum Color {
  purple = "#BA68C8",
  fuchsia = "#E91E63",
  red = "#F44336",
  orange = "#FF9800",
  green = "#4CAF50",
  teal = "#009688",
  blue = "#2196F3",
}

export default Vue.extend({
  name: "AppColor",
  data() {
    return {
      colors: Object.values(Color),
    };
  },
  methods: {
    // This method updates the primary color used by vuetify for the entire
    // app and saves it in local storage.
    changePreferredColor(newPreferredColor: string): void {
      this.$vuetify.theme.themes.dark.primary = newPreferredColor;
      this.$vuetify.theme.themes.light.primary = newPreferredColor;
      localStorage.setItem("preferredColor", newPreferredColor);
    },
  },
  mounted() {
    // When this component is mounted, this function attempts to retrieve
    // the preferred color from local storage. If it was indeed saved there,
    // then it updates the primary color used by vuetify for the entire app.
    // Note that this component should be mounted only once when the app is
    // first loaded, and not while the user navigates through the app.
    const savedPreferredColor = localStorage.getItem("preferredColor");
    if (savedPreferredColor) {
      this.$vuetify.theme.themes.dark.primary = savedPreferredColor;
      this.$vuetify.theme.themes.light.primary = savedPreferredColor;
    }
  },
});
</script>
