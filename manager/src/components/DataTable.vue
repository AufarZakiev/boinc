<script setup lang="ts">
defineProps<{
  columns: { key: string; label: string; align?: "left" | "right" | "center" }[];
  emptyMessage?: string;
}>();
</script>

<template>
  <div class="data-table-wrapper">
    <table class="data-table">
      <thead>
        <tr>
          <th
            v-for="col in columns"
            :key="col.key"
            :class="`align-${col.align ?? 'left'}`"
          >
            {{ col.label }}
          </th>
        </tr>
      </thead>
      <tbody>
        <slot />
      </tbody>
    </table>
  </div>
</template>

<style scoped>
.data-table-wrapper {
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  overflow: hidden;
}

.data-table {
  width: 100%;
  border-collapse: collapse;
  font-size: var(--font-size-md);
}

.data-table th {
  text-align: left;
  padding: 10px 12px;
  font-weight: 500;
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.03em;
  background: var(--color-bg-secondary);
  border-bottom: 1px solid var(--color-border);
  white-space: nowrap;
  user-select: none;
}

.data-table :deep(td) {
  padding: 8px 12px;
  border-bottom: 1px solid var(--color-border-light);
}

.data-table :deep(tbody tr) {
  cursor: pointer;
  transition: background var(--transition-fast);
}

.data-table :deep(tbody tr:last-child td) {
  border-bottom: none;
}

.data-table :deep(tbody tr:hover) {
  background: var(--color-bg-secondary);
}

.data-table :deep(tbody tr.row-selected) {
  background: var(--color-accent-light);
}

.align-right {
  text-align: right;
}

.align-center {
  text-align: center;
}
</style>
