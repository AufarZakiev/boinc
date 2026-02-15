<script setup lang="ts">
import { ref } from "vue";
import { useTransfersStore } from "../stores/transfers";
import type { FileTransfer } from "../types/boinc";
import ConfirmDialog from "../components/ConfirmDialog.vue";
import PageHeader from "../components/PageHeader.vue";
import DataTable from "../components/DataTable.vue";
import EmptyState from "../components/EmptyState.vue";

const store = useTransfersStore();

const selected = ref<FileTransfer | null>(null);
const confirmAbort = ref(false);

const columns = [
  { key: "file", label: "File" },
  { key: "project", label: "Project" },
  { key: "direction", label: "Direction" },
  { key: "progress", label: "Progress" },
  { key: "size", label: "Size", align: "right" as const },
  { key: "speed", label: "Speed", align: "right" as const },
];

function selectTransfer(transfer: FileTransfer) {
  selected.value =
    selected.value?.name === transfer.name &&
    selected.value?.project_url === transfer.project_url
      ? null
      : transfer;
}

function isSelected(transfer: FileTransfer): boolean {
  return (
    selected.value?.name === transfer.name &&
    selected.value?.project_url === transfer.project_url
  );
}

function formatSize(bytes: number): string {
  if (bytes <= 0) return "0 B";
  const units = ["B", "KB", "MB", "GB"];
  const i = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
  return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${units[i]}`;
}

function formatSpeed(bytesPerSec: number): string {
  if (bytesPerSec <= 0) return "---";
  return `${formatSize(bytesPerSec)}/s`;
}

function transferProgress(t: FileTransfer): string {
  if (t.nbytes <= 0) return "0%";
  return `${((t.bytes_xferred / t.nbytes) * 100).toFixed(1)}%`;
}

function transferDirection(t: FileTransfer): string {
  return t.is_upload ? "Upload" : "Download";
}

async function handleRetry() {
  if (!selected.value) return;
  await store.retryTransfer(selected.value.project_url, selected.value.name);
}

async function doAbort() {
  if (!selected.value) return;
  await store.abortTransfer(selected.value.project_url, selected.value.name);
  selected.value = null;
  confirmAbort.value = false;
}
</script>

<template>
  <div class="transfers-view">
    <PageHeader title="Transfers">
      <template v-if="selected">
        <button class="btn" @click="handleRetry">Retry</button>
        <button class="btn btn-danger" @click="confirmAbort = true">Abort</button>
      </template>
    </PageHeader>

    <p v-if="store.error" class="error">{{ store.error }}</p>

    <EmptyState
      v-else-if="store.loading && store.transfers.length === 0"
      icon="&#8987;"
      message="Loading transfers..."
    />

    <EmptyState
      v-else-if="store.transfers.length === 0"
      icon="&#128259;"
      message="No active file transfers."
    />

    <DataTable v-if="store.transfers.length > 0" :columns="columns">
      <tr
        v-for="transfer in store.transfers"
        :key="`${transfer.project_url}:${transfer.name}`"
        :class="{ 'row-selected': isSelected(transfer) }"
        @click="selectTransfer(transfer)"
      >
        <td class="col-name" :title="transfer.name">{{ transfer.name }}</td>
        <td>{{ transfer.project_name }}</td>
        <td>{{ transferDirection(transfer) }}</td>
        <td class="col-progress">
          <div class="progress-bar">
            <div
              class="progress-fill"
              :style="{ width: transferProgress(transfer) }"
            ></div>
            <span class="progress-text">{{ transferProgress(transfer) }}</span>
          </div>
        </td>
        <td class="col-number">{{ formatSize(transfer.nbytes) }}</td>
        <td class="col-number">{{ formatSpeed(transfer.xfer_speed) }}</td>
      </tr>
    </DataTable>

    <ConfirmDialog
      :open="confirmAbort"
      title="Abort Transfer"
      :message="`Abort transfer of '${selected?.name ?? ''}'?`"
      confirm-label="Abort"
      @confirm="doAbort"
      @cancel="confirmAbort = false"
    />
  </div>
</template>

<style scoped>
.transfers-view {
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

.col-number {
  font-family: monospace;
  text-align: right;
  white-space: nowrap;
}

.col-progress {
  width: 120px;
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
