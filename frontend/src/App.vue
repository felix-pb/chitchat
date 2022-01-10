<!--
This component is the root component and encompasses all others.
-->
<template>
  <v-app>
    <v-app-bar app>
      <v-btn color="primary" outlined>ChitChat</v-btn>
      <v-spacer></v-spacer>
      <app-user />
      <v-spacer />
      <app-theme />
      <app-color />
    </v-app-bar>
    <v-main class="ma-4">
      <message-board />
    </v-main>
    <app-snackbar />
  </v-app>
</template>

<script lang="ts">
import Vue from "vue";
import AppColor from "@/components/AppColor.vue";
import AppSnackbar from "@/components/AppSnackbar.vue";
import AppTheme from "@/components/AppTheme.vue";
import AppUser from "@/components/AppUser.vue";
import MessageBoard from "@/components/MessageBoard.vue";

export default Vue.extend({
  name: "App",
  components: {
    AppColor,
    AppSnackbar,
    AppTheme,
    AppUser,
    MessageBoard,
  },
  created() {
    // When the user opens or refreshes the app, we open a websocket connection
    // with the server. If the connection has been closed for any reason, then
    // show an error snackbar to suggest the user to refresh the app.
    const websocket = new WebSocket("ws://localhost:3000/websocket");
    const websocketErrorCallback = () =>
      this.$store.commit(
        "showErrorSnackbar",
        "WebSocket connection closed unexpectedly... Please refresh!"
      );
    websocket.onclose = websocketErrorCallback;
    websocket.onerror = websocketErrorCallback;

    // Every websocket message received represents a chitchat text message,
    // so we insert it in the list of messages in the vuex store.
    websocket.onmessage = (message) =>
      this.$store.commit("insertMessage", JSON.parse(message.data));

    // In addition to opening a websocket connection, we also dispatch two
    // HTTP requests: 1 to create a new user and 1 to read all existing messages.
    this.$store.dispatch("createUser");
    this.$store.dispatch("readMessages");
  },
});
</script>
