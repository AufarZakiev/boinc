<script setup lang="ts">
import { useConnectionStore } from "./stores/connection";

const connection = useConnectionStore();
</script>

<template>
  <div class="app">
    <header v-if="connection.state === 'Connected'" class="app-header">
      <span class="app-title">BOINC Manager</span>
      <span class="status-dot connected"></span>
    </header>
    <router-view />
  </div>
</template>

<style>
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  font-family:
    -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial,
    sans-serif;
  color: #1a202c;
  background: #ffffff;
}

.app {
  min-height: 100vh;
}

.app-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  border-bottom: 1px solid #e2e8f0;
  background: #f7fafc;
}

.app-title {
  font-weight: 600;
  font-size: 15px;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}

.status-dot.connected {
  background: #48bb78;
}
</style>
