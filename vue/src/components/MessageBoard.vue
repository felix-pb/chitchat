<template>
  <v-container>
    <message-input />
    <v-list dense>
      <v-list-item :key="message.id" v-for="message in messages">
        <v-list-item-icon>
          <v-icon :color="isAuthor(message) ? 'primary' : ''">person</v-icon>
          <span :class="isAuthor(message) ? 'primary--text' : ''">
            {{ message.author }}
          </span>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>
            {{ message.text }}
          </v-list-item-title>
          <v-list-item-subtitle>
            <div class="caption font-italic">
              {{ new Date(message.created * 1000).toLocaleString() }}
            </div>
          </v-list-item-subtitle>
        </v-list-item-content>
      </v-list-item>
    </v-list>
  </v-container>
</template>

<script lang="ts">
import Vue from "vue";
import MessageInput from "@/components/MessageInput.vue";
import { Message, State, User } from "@/store/state";

export default Vue.extend({
  name: "MessageBoard",
  components: {
    MessageInput,
  },
  computed: {
    messages: {
      get(): Message[] {
        return (this.$store.state as State).messages;
      },
    },
    user: {
      get(): User | null {
        return (this.$store.state as State).user;
      },
    },
  },
  methods: {
    isAuthor(message: Message): boolean {
      return this.user && this.user.id === message.author ? true : false;
    },
  },
});
</script>
