<template>
  <div class="status-bar">
    <div class="left-items">
      <div v-if="filePath" class="status-item" title="Current File">
        {{ getFileName(filePath) }}
      </div>
      <div v-else class="status-item">
        No Open File
      </div>
      <div class="status-item" v-if="statusMessage">
        {{ statusMessage }}
      </div>
    </div>
    <div class="right-items">
      <div class="status-item">
        Typst
      </div>
      <div class="status-item">
        UTF-8
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  filePath: string | null;
  statusMessage?: string;
}>();

const getFileName = (path: string) => {
  return path.split(/[/\\]/).pop() || path;
};
</script>

<style scoped>
.status-bar {
  height: 22px;
  background: #007acc;
  color: #ffffff;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 8px;
  font-size: 12px;
  user-select: none;
}

.left-items,
.right-items {
  display: flex;
  gap: 16px;
}

.status-item {
  display: flex;
  align-items: center;
  gap: 4px;
  cursor: pointer;
}

.status-item:hover {
  background: rgba(255, 255, 255, 0.2);
  border-radius: 2px;
}
</style>
