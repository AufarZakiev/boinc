<script setup lang="ts">
defineProps<{
  open: boolean;
  title: string;
  message: string;
  confirmLabel?: string;
}>();

const emit = defineEmits<{
  confirm: [];
  cancel: [];
}>();
</script>

<template>
  <Teleport to="body">
    <div v-if="open" class="dialog-overlay" @click.self="emit('cancel')">
      <div class="dialog">
        <h3>{{ title }}</h3>
        <p>{{ message }}</p>
        <div class="dialog-actions">
          <button class="btn btn-cancel" @click="emit('cancel')">Cancel</button>
          <button class="btn btn-confirm" @click="emit('confirm')">
            {{ confirmLabel ?? "Confirm" }}
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: #fff;
  border-radius: 8px;
  padding: 24px;
  max-width: 400px;
  width: 90%;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.15);
}

.dialog h3 {
  margin: 0 0 8px 0;
  font-size: 16px;
}

.dialog p {
  margin: 0 0 20px 0;
  color: #4a5568;
  font-size: 14px;
  line-height: 1.5;
}

.dialog-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  font-size: 13px;
  cursor: pointer;
}

.btn-cancel {
  background: #edf2f7;
  color: #4a5568;
}

.btn-cancel:hover {
  background: #e2e8f0;
}

.btn-confirm {
  background: #e53e3e;
  color: white;
}

.btn-confirm:hover {
  background: #c53030;
}
</style>
