<script setup lang="ts">
import { ref } from "vue";
import { useConnectionStore } from "../stores/connection";
import { useTasksStore } from "../stores/tasks";
import { useRouter } from "vue-router";

const connection = useConnectionStore();
const tasks = useTasksStore();
const router = useRouter();

const dataDir = ref(defaultDataDir());
const connecting = ref(false);

function defaultDataDir(): string {
  // Common default BOINC data directories per platform
  const platform = navigator.platform.toLowerCase();
  if (platform.includes("win")) return "C:\\ProgramData\\BOINC";
  if (platform.includes("mac")) return "/Library/Application Support/BOINC Data";
  return "/var/lib/boinc-client";
}

async function handleConnect() {
  connecting.value = true;
  await connection.connectToLocal(dataDir.value);
  connecting.value = false;

  if (connection.state === "Connected") {
    tasks.startPolling();
    router.push("/tasks");
  }
}
</script>

<template>
  <div class="connect-view">
    <h2>Connect to BOINC Client</h2>

    <div class="form">
      <label>
        BOINC data directory
        <input v-model="dataDir" type="text" :disabled="connecting" />
      </label>

      <button :disabled="connecting" @click="handleConnect">
        {{ connecting ? "Connecting..." : "Connect" }}
      </button>

      <p v-if="connection.error" class="error">{{ connection.error }}</p>
    </div>
  </div>
</template>

<style scoped>
.connect-view {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 60vh;
  padding: 32px;
}

h2 {
  margin-bottom: 24px;
}

.form {
  display: flex;
  flex-direction: column;
  gap: 16px;
  width: 100%;
  max-width: 480px;
}

label {
  display: flex;
  flex-direction: column;
  gap: 4px;
  font-size: 14px;
  color: #4a5568;
}

input {
  padding: 8px 12px;
  border: 1px solid #cbd5e0;
  border-radius: 4px;
  font-size: 14px;
}

button {
  padding: 10px 20px;
  background: #4299e1;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
}

button:hover:not(:disabled) {
  background: #3182ce;
}

button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.error {
  color: #e53e3e;
  font-size: 13px;
}
</style>
