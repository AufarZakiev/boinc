<script setup lang="ts">
import { ref } from "vue";
import { useTransfersStore } from "../stores/transfers";
import type { FileTransfer } from "../types/boinc";
import ConfirmDialog from "../components/ConfirmDialog.vue";

const store = useTransfersStore();

const selected = ref<FileTransfer | null>(null);
const confirmAbort = ref(false);

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
    <div class="toolbar">
      <h2>Transfers</h2>
      <div v-if="selected" class="actions">
        <button class="btn" @click="handleRetry">Retry</button>
        <button class="btn btn-danger" @click="confirmAbort = true">Abort</button>
      </div>
    </div>

    <p v-if="store.error" class="error">{{ store.error }}</p>
    <p v-else-if="store.loading && store.transfers.length === 0" class="loading">
      Loading transfers...
    </p>
    <p v-else-if="store.transfers.length === 0" class="empty">
      No active file transfers.
    </p>

    <table v-if="store.transfers.length > 0" class="data-table">
      <thead>
        <tr>
          <th>File</th>
          <th>Project</th>
          <th>Direction</th>
          <th>Progress</th>
          <th>Size</th>
          <th>Speed</th>
        </tr>
      </thead>
      <tbody>
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
      </tbody>
    </table>

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
