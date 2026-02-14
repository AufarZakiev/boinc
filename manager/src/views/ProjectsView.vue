<script setup lang="ts">
import { ref } from "vue";
import { useProjectsStore } from "../stores/projects";
import type { Project } from "../types/boinc";
import ConfirmDialog from "../components/ConfirmDialog.vue";

const store = useProjectsStore();

const selected = ref<Project | null>(null);
const confirmAction = ref<{
  title: string;
  message: string;
  action: () => Promise<void>;
} | null>(null);

function selectProject(project: Project) {
  selected.value =
    selected.value?.master_url === project.master_url ? null : project;
}

function isSelected(project: Project): boolean {
  return selected.value?.master_url === project.master_url;
}

function formatCredit(credit: number): string {
  return credit.toLocaleString(undefined, { maximumFractionDigits: 0 });
}

function projectStatus(project: Project): string {
  if (project.suspended_via_gui) return "Suspended";
  if (project.dont_request_more_work) return "No new tasks";
  return "Active";
}

async function handleSuspendResume() {
  if (!selected.value) return;
  if (selected.value.suspended_via_gui) {
    await store.resumeProject(selected.value.master_url);
  } else {
    await store.suspendProject(selected.value.master_url);
  }
}

async function handleNoNewAllowTasks() {
  if (!selected.value) return;
  if (selected.value.dont_request_more_work) {
    await store.allowNewTasks(selected.value.master_url);
  } else {
    await store.noNewTasks(selected.value.master_url);
  }
}

async function handleUpdate() {
  if (!selected.value) return;
  await store.updateProject(selected.value.master_url);
}

function handleReset() {
  if (!selected.value) return;
  const url = selected.value.master_url;
  confirmAction.value = {
    title: "Reset Project",
    message: `Reset "${selected.value.project_name}"? All tasks for this project will be lost.`,
    action: async () => {
      await store.resetProject(url);
      selected.value = null;
    },
  };
}

function handleDetach() {
  if (!selected.value) return;
  const url = selected.value.master_url;
  confirmAction.value = {
    title: "Detach Project",
    message: `Detach from "${selected.value.project_name}"? You will stop contributing to this project.`,
    action: async () => {
      await store.detachProject(url);
      selected.value = null;
    },
  };
}

async function doConfirm() {
  if (confirmAction.value) {
    await confirmAction.value.action();
    confirmAction.value = null;
  }
}
</script>

<template>
  <div class="projects-view">
    <div class="toolbar">
      <h2>Projects</h2>
      <div v-if="selected" class="actions">
        <button class="btn" @click="handleUpdate">Update</button>
        <button class="btn" @click="handleSuspendResume">
          {{ selected.suspended_via_gui ? "Resume" : "Suspend" }}
        </button>
        <button class="btn" @click="handleNoNewAllowTasks">
          {{ selected.dont_request_more_work ? "Allow new tasks" : "No new tasks" }}
        </button>
        <button class="btn btn-danger" @click="handleReset">Reset</button>
        <button class="btn btn-danger" @click="handleDetach">Detach</button>
      </div>
    </div>

    <p v-if="store.error" class="error">{{ store.error }}</p>
    <p v-else-if="store.loading && store.projects.length === 0" class="loading">
      Loading projects...
    </p>
    <p v-else-if="store.projects.length === 0" class="empty">
      No projects attached.
    </p>

    <table v-if="store.projects.length > 0" class="data-table">
      <thead>
        <tr>
          <th>Project</th>
          <th>Account</th>
          <th>Team</th>
          <th>Total Credit</th>
          <th>Avg Credit</th>
          <th>Status</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="project in store.projects"
          :key="project.master_url"
          :class="{ 'row-selected': isSelected(project) }"
          @click="selectProject(project)"
        >
          <td class="col-name">{{ project.project_name }}</td>
          <td>{{ project.user_name }}</td>
          <td>{{ project.team_name || "---" }}</td>
          <td class="col-number">{{ formatCredit(project.user_total_credit) }}</td>
          <td class="col-number">{{ formatCredit(project.user_expavg_credit) }}</td>
          <td class="col-status">{{ projectStatus(project) }}</td>
        </tr>
      </tbody>
    </table>

    <ConfirmDialog
      :open="confirmAction !== null"
      :title="confirmAction?.title ?? ''"
      :message="confirmAction?.message ?? ''"
      @confirm="doConfirm"
      @cancel="confirmAction = null"
    />
  </div>
</template>

<style scoped>
.projects-view {
  padding: 16px;
}

.toolbar {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 16px;
}

h2 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
}

.actions {
  display: flex;
  gap: 6px;
}

.btn {
  padding: 5px 12px;
  border: 1px solid #cbd5e0;
  border-radius: 4px;
  background: #fff;
  font-size: 12px;
  cursor: pointer;
}

.btn:hover {
  background: #f7fafc;
}

.btn-danger {
  color: #e53e3e;
  border-color: #feb2b2;
}

.btn-danger:hover {
  background: #fff5f5;
}

.error {
  color: #e53e3e;
}
.loading,
.empty {
  color: #718096;
}

.data-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.data-table th {
  text-align: left;
  padding: 8px;
  border-bottom: 2px solid #e2e8f0;
  font-weight: 600;
  color: #4a5568;
  white-space: nowrap;
}

.data-table td {
  padding: 6px 8px;
  border-bottom: 1px solid #edf2f7;
}

.data-table tbody tr {
  cursor: pointer;
}

.data-table tbody tr:hover {
  background: #f7fafc;
}

.row-selected {
  background: #ebf8ff !important;
}

.col-name {
  font-weight: 500;
}

.col-number {
  font-family: monospace;
  text-align: right;
}

.col-status {
  white-space: nowrap;
}
</style>
