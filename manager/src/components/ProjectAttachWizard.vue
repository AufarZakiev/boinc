<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import {
  getAllProjectsList,
  lookupAccount,
  lookupAccountPoll,
  projectAttach,
  projectAttachPoll,
} from "../composables/useRpc";
import { useProjectsStore } from "../stores/projects";
import type { ProjectListEntry } from "../types/boinc";

const props = defineProps<{ open: boolean }>();
const emit = defineEmits<{ close: [] }>();

const projects = useProjectsStore();

const step = ref(1);
const projectList = ref<ProjectListEntry[]>([]);
const loading = ref(false);
const error = ref("");
const search = ref("");
const manualUrl = ref("");

// Selected project
const selectedProject = ref<ProjectListEntry | null>(null);

// Credentials
const email = ref("");
const password = ref("");

// Result
const resultMessage = ref("");

const filteredProjects = computed(() => {
  if (!search.value) return projectList.value;
  const q = search.value.toLowerCase();
  return projectList.value.filter(
    (p) =>
      p.name.toLowerCase().includes(q) ||
      p.description.toLowerCase().includes(q) ||
      p.general_area.toLowerCase().includes(q),
  );
});

onMounted(async () => {
  try {
    projectList.value = await getAllProjectsList();
  } catch {
    // Will load when dialog opens
  }
});

async function loadProjects() {
  if (projectList.value.length > 0) return;
  loading.value = true;
  try {
    projectList.value = await getAllProjectsList();
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

function selectProject(project: ProjectListEntry) {
  selectedProject.value = project;
  manualUrl.value = project.url;
  step.value = 2;
}

function goToStep2Manual() {
  if (!manualUrl.value.trim()) return;
  selectedProject.value = null;
  step.value = 2;
}

async function doAttach() {
  if (!email.value || !password.value) {
    error.value = "Please enter email and password";
    return;
  }

  const url = selectedProject.value?.url || manualUrl.value;
  if (!url) return;

  step.value = 3;
  error.value = "";
  loading.value = true;

  try {
    // Start lookup account
    await lookupAccount(url, email.value, password.value);

    // Poll for result
    let attempts = 0;
    let accountResult;
    while (attempts < 30) {
      await new Promise((r) => setTimeout(r, 1000));
      accountResult = await lookupAccountPoll();
      if (accountResult.error_num !== -204) break; // -204 = in progress
      attempts++;
    }

    if (!accountResult || accountResult.error_num !== 0) {
      error.value =
        accountResult?.error_msg || "Account lookup failed";
      step.value = 2;
      loading.value = false;
      return;
    }

    // Start project attach
    const name = selectedProject.value?.name || "";
    await projectAttach(url, accountResult.authenticator, name);

    // Poll for attach result
    attempts = 0;
    let attachResult;
    while (attempts < 30) {
      await new Promise((r) => setTimeout(r, 1000));
      attachResult = await projectAttachPoll();
      if (attachResult.error_num !== -204) break;
      attempts++;
    }

    if (attachResult && attachResult.error_num === 0) {
      resultMessage.value = "Successfully attached to project!";
      step.value = 4;
      projects.fetchProjects();
    } else {
      error.value =
        attachResult?.messages?.join(", ") || "Attach failed";
      step.value = 2;
    }
  } catch (e) {
    error.value = String(e);
    step.value = 2;
  } finally {
    loading.value = false;
  }
}

function reset() {
  step.value = 1;
  error.value = "";
  search.value = "";
  email.value = "";
  password.value = "";
  selectedProject.value = null;
  manualUrl.value = "";
  resultMessage.value = "";
  loadProjects();
}

function close() {
  reset();
  emit("close");
}
</script>

<template>
  <Teleport to="body">
    <div v-if="open" class="dialog-overlay" @click.self="close">
      <div class="wizard">
        <div class="wizard-header">
          <h3>
            {{
              step === 1
                ? "Add Project"
                : step === 2
                  ? "Account"
                  : step === 3
                    ? "Attaching..."
                    : "Done"
            }}
          </h3>
          <button class="close-btn" @click="close">&times;</button>
        </div>

        <!-- Step 1: Choose project -->
        <div v-if="step === 1" class="wizard-body">
          <input
            v-model="search"
            type="text"
            class="search-input"
            placeholder="Search projects..."
            @focus="loadProjects"
          />

          <div v-if="loading" class="wizard-loading">Loading project list...</div>

          <div v-else class="project-list">
            <div
              v-for="p in filteredProjects"
              :key="p.url"
              class="project-item"
              @click="selectProject(p)"
            >
              <div class="project-name">{{ p.name }}</div>
              <div class="project-area">{{ p.general_area }} — {{ p.specific_area }}</div>
              <div class="project-desc">{{ p.description }}</div>
            </div>
            <div v-if="!loading && filteredProjects.length === 0" class="no-results">
              No projects found
            </div>
          </div>

          <div class="manual-url">
            <span class="manual-label">Or enter project URL:</span>
            <div class="manual-row">
              <input
                v-model="manualUrl"
                type="text"
                placeholder="https://..."
              />
              <button class="btn btn-primary" :disabled="!manualUrl.trim()" @click="goToStep2Manual">
                Next
              </button>
            </div>
          </div>
        </div>

        <!-- Step 2: Credentials -->
        <div v-if="step === 2" class="wizard-body">
          <div class="cred-project">
            {{ selectedProject?.name || manualUrl }}
          </div>

          <div v-if="error" class="wizard-error">{{ error }}</div>

          <label class="field">
            <span>Email</span>
            <input v-model="email" type="email" placeholder="you@example.com" />
          </label>
          <label class="field">
            <span>Password</span>
            <input v-model="password" type="password" />
          </label>

          <div class="wizard-actions">
            <button class="btn" @click="step = 1">Back</button>
            <button class="btn btn-primary" @click="doAttach">Attach</button>
          </div>
        </div>

        <!-- Step 3: Progress -->
        <div v-if="step === 3" class="wizard-body wizard-center">
          <div class="spinner"></div>
          <p>Attaching to project...</p>
        </div>

        <!-- Step 4: Success -->
        <div v-if="step === 4" class="wizard-body wizard-center">
          <div class="success-icon">&#10003;</div>
          <p>{{ resultMessage }}</p>
          <button class="btn btn-primary" @click="close">Done</button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(2px);
}

.wizard {
  background: var(--color-bg);
  border-radius: var(--radius-lg);
  width: 520px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: var(--shadow-lg);
}

.wizard-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--color-border);
}

.wizard-header h3 {
  margin: 0;
  font-size: var(--font-size-lg);
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  font-size: 20px;
  color: var(--color-text-tertiary);
  cursor: pointer;
  padding: 4px;
  line-height: 1;
}

.close-btn:hover {
  color: var(--color-text-primary);
}

.wizard-body {
  padding: 16px 20px;
  flex: 1;
  overflow-y: auto;
}

.wizard-center {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 200px;
  gap: var(--space-lg);
}

.wizard-loading {
  padding: var(--space-2xl);
  text-align: center;
  color: var(--color-text-secondary);
}

.wizard-error {
  color: var(--color-danger);
  font-size: var(--font-size-sm);
  margin-bottom: var(--space-md);
  padding: 8px 12px;
  background: var(--color-danger-light);
  border-radius: var(--radius-sm);
}

.search-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  font-size: var(--font-size-base);
  margin-bottom: var(--space-md);
  background: var(--color-bg);
}

.project-list {
  max-height: 280px;
  overflow-y: auto;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  margin-bottom: var(--space-lg);
}

.project-item {
  padding: 10px 12px;
  cursor: pointer;
  border-bottom: 1px solid var(--color-border-light);
  transition: background var(--transition-fast);
}

.project-item:last-child {
  border-bottom: none;
}

.project-item:hover {
  background: var(--color-bg-secondary);
}

.project-name {
  font-weight: 500;
  font-size: var(--font-size-base);
  margin-bottom: 2px;
}

.project-area {
  font-size: var(--font-size-xs);
  color: var(--color-text-tertiary);
  margin-bottom: 4px;
}

.project-desc {
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.no-results {
  padding: var(--space-xl);
  text-align: center;
  color: var(--color-text-tertiary);
}

.manual-url {
  border-top: 1px solid var(--color-border);
  padding-top: var(--space-md);
}

.manual-label {
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  display: block;
  margin-bottom: var(--space-sm);
}

.manual-row {
  display: flex;
  gap: var(--space-sm);
}

.manual-row input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  font-size: var(--font-size-base);
}

.cred-project {
  font-weight: 500;
  padding: 10px 12px;
  background: var(--color-bg-secondary);
  border-radius: var(--radius-sm);
  margin-bottom: var(--space-lg);
  font-size: var(--font-size-md);
}

.field {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin-bottom: var(--space-md);
  font-size: var(--font-size-md);
  color: var(--color-text-secondary);
}

.field input {
  padding: 8px 12px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  font-size: var(--font-size-base);
  color: var(--color-text-primary);
}

.wizard-actions {
  display: flex;
  gap: var(--space-sm);
  justify-content: flex-end;
  margin-top: var(--space-lg);
}

.spinner {
  width: 32px;
  height: 32px;
  border: 3px solid var(--color-border);
  border-top-color: var(--color-accent);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.success-icon {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  background: var(--color-success-light);
  color: var(--color-success);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
  font-weight: 700;
}
</style>
