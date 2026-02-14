import { defineStore } from "pinia";
import { ref } from "vue";
import type { ConnectionState } from "../types/boinc";
import {
  connectLocal,
  connect,
  disconnect as rpcDisconnect,
} from "../composables/useRpc";

export const useConnectionStore = defineStore("connection", () => {
  const state = ref<ConnectionState>("Disconnected");
  const error = ref<string | null>(null);

  async function connectToLocal(dataDir: string) {
    state.value = "Connecting";
    error.value = null;
    try {
      await connectLocal(dataDir);
      state.value = "Connected";
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      error.value = msg;
      if (msg.includes("Authentication")) {
        state.value = "AuthError";
      } else {
        state.value = { Error: msg };
      }
    }
  }

  async function connectToRemote(
    host: string,
    port: number,
    password: string,
  ) {
    state.value = "Connecting";
    error.value = null;
    try {
      await connect(host, port, password);
      state.value = "Connected";
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      error.value = msg;
      state.value = { Error: msg };
    }
  }

  async function disconnect() {
    await rpcDisconnect();
    state.value = "Disconnected";
    error.value = null;
  }

  return { state, error, connectToLocal, connectToRemote, disconnect };
});
