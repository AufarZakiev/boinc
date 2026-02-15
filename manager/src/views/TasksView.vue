<script setup lang="ts">
import { computed, ref } from "vue";
import { useTasksStore } from "../stores/tasks";
import type { TaskResult } from "../types/boinc";
import {
  RESULT_STATE,
  ACTIVE_TASK_STATE,
  SCHEDULER_STATE,
} from "../types/boinc";
import ConfirmDialog from "../components/ConfirmDialog.vue";
import PageHeader from "../components/PageHeader.vue";
import DataTable from "../components/DataTable.vue";
import EmptyState from "../components/EmptyState.vue";
import StatusBadge from "../components/StatusBadge.vue";

const store = useTasksStore();

const selectedNames = ref<Set<string>>(new Set());
const lastClickedIndex = ref<number | null>(null);
const confirmAbort = ref(false);

const columns = [
  { key: "task", label: "Task" },
  { key: "project", label: "Project" },
  { key: "progress", label: "Progress" },
  { key: "elapsed", label: "Elapsed" },
  { key: "remaining", label: "Remaining" },
  { key: "status", label: "Status" },
];

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

function statusVariant(status: string): "default" | "success" | "warning" | "danger" | "info" {
  switch (status) {
    case "Running":
      return "success";
    case "Waiting to run":
    case "Waiting":
    case "Downloading":
    case "Uploading":
      return "info";
    case "Suspended":
      return "warning";
    case "Computation error":
    case "Aborted":
      return "danger";
    case "Ready to report":
      return "default";
    default:
      return "default";
  }
}

const sortedTasks = computed(() =>
  [...store.tasks].sort((a, b) => {
    const aRunning = a.active_task ? 1 : 0;
    const bRunning = b.active_task ? 1 : 0;
    if (aRunning !== bRunning) return bRunning - aRunning;
    return b.fraction_done - a.fraction_done;
  }),
);

const selectedTasks = computed(() =>
  store.tasks.filter((t) => selectedNames.value.has(t.name)),
);

const hasSelection = computed(() => selectedNames.value.size > 0);

const allSelectedSuspended = computed(() =>
  selectedTasks.value.length > 0 &&
  selectedTasks.value.every((t) => t.suspended_via_gui),
);

const suspendResumeLabel = computed(() =>
  allSelectedSuspended.value ? "Resume" : "Suspend",
);

function handleRowClick(task: TaskResult, index: number, event: MouseEvent) {
  if (event.ctrlKey || event.metaKey) {
    const next = new Set(selectedNames.value);
    if (next.has(task.name)) {
      next.delete(task.name);
    } else {
      next.add(task.name);
    }
    selectedNames.value = next;
  } else if (event.shiftKey && lastClickedIndex.value !== null) {
    const start = Math.min(lastClickedIndex.value, index);
    const end = Math.max(lastClickedIndex.value, index);
    const next = new Set(selectedNames.value);
    for (let i = start; i <= end; i++) {
      next.add(sortedTasks.value[i].name);
    }
    selectedNames.value = next;
  } else {
    selectedNames.value = new Set([task.name]);
  }
  lastClickedIndex.value = index;
}

function isSelected(task: TaskResult): boolean {
  return selectedNames.value.has(task.name);
}

async function handleSuspendResume() {
  for (const task of selectedTasks.value) {
    if (task.suspended_via_gui) {
      await store.resumeTask(task.project_url, task.name);
    } else {
      await store.suspendTask(task.project_url, task.name);
    }
  }
}

async function doAbort() {
  for (const task of selectedTasks.value) {
    await store.abortTask(task.project_url, task.name);
  }
  selectedNames.value = new Set();
  confirmAbort.value = false;
}
</script>

<template>
  <div class="tasks-view">
    <PageHeader title="Tasks">
      <template v-if="hasSelection">
        <button class="btn" @click="handleSuspendResume">
          {{ suspendResumeLabel }}
        </button>
        <button class="btn btn-danger" @click="confirmAbort = true">
          Abort
        </button>
      </template>
    </PageHeader>

    <p v-if="store.error" class="error">{{ store.error }}</p>

    <EmptyState
      v-else-if="store.loading && store.tasks.length === 0"
      icon="&#8987;"
      message="Loading tasks..."
    />

    <EmptyState
      v-else-if="store.tasks.length === 0"
      icon="&#128203;"
      message="No tasks. Attach a project to start computing."
    />

    <DataTable v-if="store.tasks.length > 0" :columns="columns">
      <tr
        v-for="(task, index) in sortedTasks"
        :key="task.name"
        :class="{ 'row-selected': isSelected(task) }"
        @click="handleRowClick(task, index, $event)"
      >
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
        <td>
          <StatusBadge :variant="statusVariant(taskStatus(task))">
            {{ taskStatus(task) }}
          </StatusBadge>
        </td>
      </tr>
    </DataTable>

    <ConfirmDialog
      :open="confirmAbort"
      title="Abort Tasks"
      :message="`Abort ${selectedNames.size} selected task(s)? This cannot be undone.`"
      confirm-label="Abort"
      @confirm="doAbort"
      @cancel="confirmAbort = false"
    />
  </div>
</template>

<style scoped>
.tasks-view {
  padding: var(--space-lg);
}

.error {
  color: var(--color-danger);
  font-size: var(--font-size-md);
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
  color: var(--color-text-secondary);
  font-size: var(--font-size-sm);
}

.col-time {
  font-family: monospace;
  white-space: nowrap;
}

.progress-bar {
  position: relative;
  width: 120px;
  height: 18px;
  background: var(--color-bg-tertiary);
  border-radius: var(--radius-sm);
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--color-accent);
  transition: width var(--transition-normal);
}

.progress-text {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  text-align: center;
  font-size: var(--font-size-xs);
  line-height: 18px;
  color: var(--color-text-primary);
}
</style>
