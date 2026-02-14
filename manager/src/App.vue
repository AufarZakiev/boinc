<script setup lang="ts">
import { useConnectionStore } from "./stores/connection";
import ActivityControls from "./components/ActivityControls.vue";

const connection = useConnectionStore();
</script>

<template>
  <div class="app">
    <header v-if="connection.state === 'Connected'" class="app-header">
      <span class="app-title">BOINC Manager</span>
      <nav class="nav-links">
        <router-link to="/tasks">Tasks</router-link>
        <router-link to="/projects">Projects</router-link>
        <router-link to="/transfers">Transfers</router-link>
      </nav>
      <div class="spacer"></div>
      <ActivityControls />
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

.nav-links {
  display: flex;
  gap: 4px;
  margin-left: 16px;
}

.nav-links a {
  padding: 4px 10px;
  border-radius: 4px;
  font-size: 13px;
  text-decoration: none;
  color: #4a5568;
}

.nav-links a:hover {
  background: #edf2f7;
}

.nav-links a.router-link-active {
  background: #ebf8ff;
  color: #2b6cb0;
  font-weight: 500;
}

.spacer {
  flex: 1;
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
