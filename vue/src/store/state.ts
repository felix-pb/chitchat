import { Snackbar, SnackbarColor } from "@/components/AppSnackbar.vue";

/**
 * This interface is the type definition of our global state.
 */
export interface State {
  messages: Message[];
  snackbar: Snackbar;
  user: User | null;
}

/**
 * This interface is the type definition of
 * the `Message` struct in the backend.
 */
export interface Message {
  id: number;
  author: number;
  text: string;
  created: number;
  modified: number | null;
}

/**
 * This interface is the type definition of
 * the `User` struct in the backend.
 */
export interface User {
  id: number;
  password: string;
}

/**
 * This interface is the type definition of
 * the `CreateMessageParams` struct in the backend.
 */
export interface CreateMessageParams {
  user: User;
  text: string;
}

/**
 * This interface is the type definition of
 * the `UpdateMessageParams` struct in the backend.
 */
export interface UpdateMessageParams {
  message: number;
  user: User;
  text: string;
}

/**
 * This interface is the type definition of
 * the `DeleteMessageParams` struct in the backend.
 */
export interface DeleteMessageParams {
  message: number;
  user: User;
}

/**
 * This object is our global state.
 */
const state: State = {
  messages: [],
  snackbar: {
    color: SnackbarColor.success,
    isOn: false,
    message: "",
  },
  user: null,
};

export default state;
