/**
 * This module is responsible for sending all HTTP requests
 * to the REST API endpoints.
 */
import axios from "axios";
import { ActionContext } from "vuex";

import {
  CreateMessageParams,
  DeleteMessageParams,
  State,
  UpdateMessageParams,
} from "@/store/state";

type Context = ActionContext<State, State>;

export default {
  createUser(context: Context): void {
    axios
      .post("/users")
      .then((response) => context.commit("setUser", response.data))
      .catch((error) => errorHandler(context, error.response.data));
  },
  readMessages(context: Context): void {
    axios
      .get("/messages")
      .then((response) => context.commit("setMessages", response.data))
      .catch((error) => errorHandler(context, error.response.data));
  },
  createMessage(context: Context, params: CreateMessageParams): void {
    axios
      .post("/messages", params)
      .catch((error) => errorHandler(context, error.response.data));
  },
  updateMessage(context: Context, params: UpdateMessageParams): void {
    axios
      .put("/messages", params)
      .catch((error) => errorHandler(context, error.response.data));
  },
  deleteMessage(context: Context, params: DeleteMessageParams): void {
    axios
      .delete("/messages", { data: params })
      .catch((error) => errorHandler(context, error.response.data));
  },
};

function errorHandler(context: Context, errorMessage: string) {
  context.commit("showErrorSnackbar", errorMessage);
}
