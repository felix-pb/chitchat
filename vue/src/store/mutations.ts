import { Message, State, User } from "@/store/state";
import { SnackbarColor } from "@/components/AppSnackbar.vue";

export default {
  insertMessage(state: State, message: Message): void {
    state.messages.push(message);
    state.messages.sort((a, b) => b.id - a.id);
  },
  setMessages(state: State, messages: Message[]): void {
    state.messages = messages;
    state.messages.sort((a, b) => b.id - a.id);
  },
  setSnackbar(state: State, isOn: boolean): void {
    state.snackbar.isOn = isOn;
  },
  setUser(state: State, user: User): void {
    state.user = user;
  },
  showErrorSnackbar(state: State, errorMessage: string): void {
    state.snackbar.color = SnackbarColor.error;
    state.snackbar.isOn = true;
    state.snackbar.message = errorMessage;
  },
  showSuccessSnackbar(state: State, successMessage: string): void {
    state.snackbar.color = SnackbarColor.success;
    state.snackbar.isOn = true;
    state.snackbar.message = successMessage;
  },
};
