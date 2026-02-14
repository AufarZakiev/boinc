import { defineStore } from "pinia";
import { ref } from "vue";
import type { TaskResult } from "../types/boinc";
import { getResults } from "../composables/useRpc";

export const useTasksStore = defineStore("tasks", () => {
  const tasks = ref<TaskResult[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  let pollTimer: ReturnType<typeof setInterval> | null = null;

  async function fetchTasks() {
    loading.value = true;
    error.value = null;
    try {
      tasks.value = await getResults(false);
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
    } finally {
      loading.value = false;
    }
  }

  function startPolling(intervalMs = 2000) {
    stopPolling();
    fetchTasks();
    pollTimer = setInterval(fetchTasks, intervalMs);
  }

  function stopPolling() {
    if (pollTimer !== null) {
      clearInterval(pollTimer);
      pollTimer = null;
    }
  }

  return { tasks, loading, error, fetchTasks, startPolling, stopPolling };
});
