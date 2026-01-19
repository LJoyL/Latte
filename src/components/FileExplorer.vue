<template>
  <div class="file-explorer">
    <div class="explorer-header">
      <h3>Files</h3>
      <button class="icon-btn" @click="refreshFiles" title="Refresh">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path d="M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2v1z" />
          <path d="M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466z" />
        </svg>
      </button>
    </div>
    <div class="file-list">
      <div
        v-for="file in files"
        :key="file"
        :class="['file-item', { active: file === currentFile }]"
        @click="$emit('file-selected', file)"
        @contextmenu.prevent="showContextMenu($event, file)"
      >
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" class="file-icon">
          <path d="M14 4.5V14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1h5v4.5a.5.5 0 0 0 .5.5H14z" />
        </svg>
        <span class="file-name">{{ getFileName(file) }}</span>
        <button
          class="delete-btn"
          @click.stop="$emit('file-deleted', file)"
          title="Delete file"
        >
          ×
        </button>
      </div>
      <div v-if="files.length === 0" class="empty-state">
        No Typst files found
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';

defineProps<{
  currentFile: string | null;
  files: string[];
}>();

defineEmits<{
  'file-selected': [path: string];
  'file-deleted': [path: string];
}>();

const getFileName = (path: string) => {
  return path.split(/[/\\]/).pop() || path;
};

const refreshFiles = () => {
  // Emit event to parent to refresh
  window.location.reload();
};

const showContextMenu = (event: MouseEvent, file: string) => {
  // Could implement context menu here
  console.log('Context menu for:', file);
};
</script>

<style scoped>
.file-explorer {
  width: 250px;
  background: #252526;
  border-right: 1px solid #3e3e42;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.explorer-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  border-bottom: 1px solid #3e3e42;
}

.explorer-header h3 {
  font-size: 13px;
  font-weight: 600;
  color: #cccccc;
}

.icon-btn {
  background: transparent;
  border: none;
  color: #cccccc;
  cursor: pointer;
  padding: 4px;
  display: flex;
  align-items: center;
  border-radius: 4px;
}

.icon-btn:hover {
  background: #3e3e42;
}

.file-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 8px;
  border-radius: 4px;
  cursor: pointer;
  color: #cccccc;
  font-size: 13px;
  position: relative;
}

.file-item:hover {
  background: #2a2d2e;
}

.file-item:hover .delete-btn {
  opacity: 1;
}

.file-item.active {
  background: #094771;
  color: white;
}

.file-icon {
  flex-shrink: 0;
  color: #4ec9b0;
}

.file-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.delete-btn {
  opacity: 0;
  background: transparent;
  border: none;
  color: #cccccc;
  cursor: pointer;
  font-size: 20px;
  line-height: 1;
  padding: 0;
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: opacity 0.2s;
}

.delete-btn:hover {
  background: #c42b1c;
  color: white;
}

.empty-state {
  padding: 24px;
  text-align: center;
  color: #858585;
  font-size: 13px;
}
</style>
