<script setup lang="ts">
import { useClientStore } from "../stores/client";
import { RUN_MODE } from "../types/boinc";

const client = useClientStore();

function modeLabel(mode: number): string {
  switch (mode) {
    case RUN_MODE.ALWAYS:
      return "Always";
    case RUN_MODE.AUTO:
      return "Based on prefs";
    case RUN_MODE.NEVER:
      return "Suspend";
    default:
      return "Unknown";
  }
}

function handleRunMode(event: Event) {
  const value = Number((event.target as HTMLSelectElement).value);
  client.setRunMode(value);
}

function handleGpuMode(event: Event) {
  const value = Number((event.target as HTMLSelectElement).value);
  client.setGpuMode(value);
}

function handleNetworkMode(event: Event) {
  const value = Number((event.target as HTMLSelectElement).value);
  client.setNetworkMode(value);
}
</script>

<template>
  <div class="activity-controls">
    <label class="mode-control">
      <span class="mode-label">CPU</span>
      <select
        :value="client.status.task_mode_perm"
        @change="handleRunMode"
      >
        <option :value="RUN_MODE.ALWAYS">{{ modeLabel(RUN_MODE.ALWAYS) }}</option>
        <option :value="RUN_MODE.AUTO">{{ modeLabel(RUN_MODE.AUTO) }}</option>
        <option :value="RUN_MODE.NEVER">{{ modeLabel(RUN_MODE.NEVER) }}</option>
      </select>
    </label>

    <label class="mode-control">
      <span class="mode-label">GPU</span>
      <select
        :value="client.status.gpu_mode_perm"
        @change="handleGpuMode"
      >
        <option :value="RUN_MODE.ALWAYS">{{ modeLabel(RUN_MODE.ALWAYS) }}</option>
        <option :value="RUN_MODE.AUTO">{{ modeLabel(RUN_MODE.AUTO) }}</option>
        <option :value="RUN_MODE.NEVER">{{ modeLabel(RUN_MODE.NEVER) }}</option>
      </select>
    </label>

    <label class="mode-control">
      <span class="mode-label">Network</span>
      <select
        :value="client.status.network_mode_perm"
        @change="handleNetworkMode"
      >
        <option :value="RUN_MODE.ALWAYS">{{ modeLabel(RUN_MODE.ALWAYS) }}</option>
        <option :value="RUN_MODE.AUTO">{{ modeLabel(RUN_MODE.AUTO) }}</option>
        <option :value="RUN_MODE.NEVER">{{ modeLabel(RUN_MODE.NEVER) }}</option>
      </select>
    </label>
  </div>
</template>

<style scoped>
.activity-controls {
  display: flex;
  gap: 12px;
  align-items: center;
}

.mode-control {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
}

.mode-label {
  color: #718096;
  font-weight: 500;
}

select {
  padding: 2px 6px;
  border: 1px solid #cbd5e0;
  border-radius: 3px;
  font-size: 12px;
  background: #fff;
  cursor: pointer;
}
</style>
