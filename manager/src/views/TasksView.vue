<script setup lang="ts">
import { computed } from "vue";
import { useTasksStore } from "../stores/tasks";
import {
  RESULT_STATE,
  ACTIVE_TASK_STATE,
  SCHEDULER_STATE,
} from "../types/boinc";

const store = useTasksStore();

function formatTime(seconds: number): string {
  if (seconds <= 0) return "---";
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = Math.floor(seconds % 60);
  return `${h.toString().padStart(2, "0")}:${m.toString().padStart(2, "0")}:${s.toString().padStart(2, "0")}`;
}

function formatPercent(fraction: number): string {
  return `${(fraction * 100).toFixed(2)}%`;
}

function taskStatus(task: {
  state: number;
  active_task: boolean;
  active_task_state: number;
  scheduler_state: number;
  suspended_via_gui: boolean;
  ready_to_report: boolean;
}): string {
  if (task.ready_to_report) return "Ready to report";
  if (task.suspended_via_gui) return "Suspended";
  if (task.state === RESULT_STATE.FILES_DOWNLOADING) return "Downloading";
  if (task.state === RESULT_STATE.FILES_UPLOADING) return "Uploading";
  if (task.state === RESULT_STATE.COMPUTE_ERROR) return "Computation error";
  if (task.state === RESULT_STATE.ABORTED) return "Aborted";
  if (task.active_task) {
    if (task.active_task_state === ACTIVE_TASK_STATE.EXECUTING) {
      return task.scheduler_state === SCHEDULER_STATE.SCHEDULED
        ? "Running"
        : "Waiting to run";
    }
    if (task.active_task_state === ACTIVE_TASK_STATE.SUSPENDED)
      return "Suspended";
  }
  return "Waiting";
}

const sortedTasks = computed(() =>
  [...store.tasks].sort((a, b) => {
    // Active running tasks first, then by fraction done descending
    const aRunning = a.active_task ? 1 : 0;
    const bRunning = b.active_task ? 1 : 0;
    if (aRunning !== bRunning) return bRunning - aRunning;
    return b.fraction_done - a.fraction_done;
  }),
);
</script>

<template>
  <div class="tasks-view">
    <h2>Tasks</h2>

    <p v-if="store.error" class="error">{{ store.error }}</p>
    <p v-else-if="store.loading && store.tasks.length === 0" class="loading">
      Loading tasks...
    </p>
    <p v-else-if="store.tasks.length === 0" class="empty">
      No tasks. Attach a project to start computing.
    </p>

    <table v-if="store.tasks.length > 0" class="task-table">
      <thead>
        <tr>
          <th>Task</th>
          <th>Project</th>
          <th>Progress</th>
          <th>Elapsed</th>
          <th>Remaining</th>
          <th>Status</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="task in sortedTasks" :key="task.name">
          <td class="col-name" :title="task.name">{{ task.wu_name }}</td>
          <td class="col-project">{{ task.project_url }}</td>
          <td class="col-progress">
            <div class="progress-bar">
              <div
                class="progress-fill"
                :style="{ width: formatPercent(task.fraction_done) }"
              ></div>
              <span class="progress-text">{{
                formatPercent(task.fraction_done)
              }}</span>
            </div>
          </td>
          <td class="col-time">{{ formatTime(task.elapsed_time) }}</td>
          <td class="col-time">
            {{ formatTime(task.estimated_cpu_time_remaining) }}
          </td>
          <td class="col-status">{{ taskStatus(task) }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
.tasks-view {
  padding: 16px;
}

h2 {
  margin: 0 0 16px 0;
  font-size: 18px;
  font-weight: 600;
}

.error {
  color: #e53e3e;
}
.loading,
.empty {
  color: #718096;
}

.task-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.task-table th {
  text-align: left;
  padding: 8px;
  border-bottom: 2px solid #e2e8f0;
  font-weight: 600;
  color: #4a5568;
  white-space: nowrap;
}

.task-table td {
  padding: 6px 8px;
  border-bottom: 1px solid #edf2f7;
}

.task-table tbody tr:hover {
  background: #f7fafc;
}

.col-name {
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.col-project {
  max-width: 180px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: #718096;
  font-size: 12px;
}

.col-time {
  font-family: monospace;
  white-space: nowrap;
}

.col-status {
  white-space: nowrap;
}

.progress-bar {
  position: relative;
  width: 120px;
  height: 18px;
  background: #edf2f7;
  border-radius: 3px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: #4299e1;
  transition: width 0.3s ease;
}

.progress-text {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  text-align: center;
  font-size: 11px;
  line-height: 18px;
  color: #2d3748;
}
</style>
