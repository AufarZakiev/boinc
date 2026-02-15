<script setup lang="ts">
import { ref } from "vue";
import { useProjectsStore } from "../stores/projects";
import type { Project } from "../types/boinc";
import ConfirmDialog from "../components/ConfirmDialog.vue";
import ProjectAttachWizard from "../components/ProjectAttachWizard.vue";
import PageHeader from "../components/PageHeader.vue";
import DataTable from "../components/DataTable.vue";
import EmptyState from "../components/EmptyState.vue";
import StatusBadge from "../components/StatusBadge.vue";

const store = useProjectsStore();

const selected = ref<Project | null>(null);
const showAttachWizard = ref(false);
const confirmAction = ref<{
  title: string;
  message: string;
  action: () => Promise<void>;
} | null>(null);

const columns = [
  { key: "project", label: "Project" },
  { key: "account", label: "Account" },
  { key: "team", label: "Team" },
  { key: "totalCredit", label: "Total Credit", align: "right" as const },
  { key: "avgCredit", label: "Avg Credit", align: "right" as const },
  { key: "status", label: "Status" },
];

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

function statusVariant(status: string): "default" | "success" | "warning" | "danger" | "info" {
  switch (status) {
    case "Active":
      return "success";
    case "Suspended":
      return "warning";
    case "No new tasks":
      return "info";
    default:
      return "default";
  }
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
    <PageHeader title="Projects">
      <button class="btn btn-primary" @click="showAttachWizard = true">Add Project</button>
      <template v-if="selected">
        <button class="btn" @click="handleUpdate">Update</button>
        <button class="btn" @click="handleSuspendResume">
          {{ selected.suspended_via_gui ? "Resume" : "Suspend" }}
        </button>
        <button class="btn" @click="handleNoNewAllowTasks">
          {{ selected.dont_request_more_work ? "Allow new tasks" : "No new tasks" }}
        </button>
        <button class="btn btn-danger" @click="handleReset">Reset</button>
        <button class="btn btn-danger" @click="handleDetach">Detach</button>
      </template>
    </PageHeader>

    <p v-if="store.error" class="error">{{ store.error }}</p>

    <EmptyState
      v-else-if="store.loading && store.projects.length === 0"
      icon="&#8987;"
      message="Loading projects..."
    />

    <EmptyState
      v-else-if="store.projects.length === 0"
      icon="&#128194;"
      message="No projects attached."
    />

    <DataTable v-if="store.projects.length > 0" :columns="columns">
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
        <td>
          <StatusBadge :variant="statusVariant(projectStatus(project))">
            {{ projectStatus(project) }}
          </StatusBadge>
        </td>
      </tr>
    </DataTable>

    <ConfirmDialog
      :open="confirmAction !== null"
      :title="confirmAction?.title ?? ''"
      :message="confirmAction?.message ?? ''"
      @confirm="doConfirm"
      @cancel="confirmAction = null"
    />

    <ProjectAttachWizard
      :open="showAttachWizard"
      @close="showAttachWizard = false"
    />
  </div>
</template>

<style scoped>
.projects-view {
  padding: var(--space-lg);
}

.error {
  color: var(--color-danger);
  font-size: var(--font-size-md);
}

.col-name {
  font-weight: 500;
}

.col-number {
  font-family: monospace;
  text-align: right;
}
</style>
