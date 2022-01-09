<template>
  <v-text-field
    :append-icon="message ? 'send' : ''"
    counter="100"
    dense
    label="Message"
    outlined
    v-model="message"
    v-on:keyup.enter="send"
    @click:append="send"
  />
</template>

<script lang="ts">
import Vue from "vue";
import { CreateMessageParams, State, User } from "@/store/state";

export default Vue.extend({
  name: "MessageInput",
  data() {
    return {
      message: "",
    };
  },
  computed: {
    user: {
      get(): User | null {
        return (this.$store.state as State).user;
      },
    },
  },
  methods: {
    send(): void {
      if (this.user && this.message) {
        const params: CreateMessageParams = {
          user: this.user,
          text: this.message,
        };
        this.$store.dispatch("createMessage", params);
        this.message = "";
      }
    },
  },
});
</script>
