<template>
  <div class="editor-tabs">
    <div
      v-for="(file, index) in openFiles"
      :key="file.path"
      class="tab"
      :class="{ active: index === activeIndex }"
      @click="$emit('tab-selected', index)"
    >
      <div class="tab-content">
        <span class="file-icon">📄</span>
        <span class="file-name">{{ getFileName(file.path) }}</span>
        <span v-if="file.isDirty" class="dirty-indicator">●</span>
      </div>
      <button class="close-btn" @click.stop="$emit('tab-closed', index)">×</button>
    </div>
  </div>
</template>

<script setup lang="ts">
export interface OpenFile {
  path: string;
  isDirty: boolean;
  content: string;
}

defineProps<{
  openFiles: OpenFile[];
  activeIndex: number;
}>();

defineEmits<{
  'tab-selected': [index: number];
  'tab-closed': [index: number];
}>();

const getFileName = (path: string) => {
  return path.split(/[/\\]/).pop() || path;
};
</script>

<style scoped>
.editor-tabs {
  display: flex;
  background: #252526;
  height: 35px;
  overflow-x: auto;
  border-bottom: 1px solid #252526;
}

.editor-tabs::-webkit-scrollbar {
  height: 3px;
}

.editor-tabs::-webkit-scrollbar-thumb {
  background: #424242;
}

.tab {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 10px;
  min-width: 120px;
  max-width: 200px;
  background: #2d2d2d;
  color: #969696;
  border-right: 1px solid #252526;
  cursor: pointer;
  user-select: none;
  font-size: 13px;
}

.tab:hover {
  background: #383838;
}

.tab.active {
  background: #1e1e1e;
  color: #ffffff;
  border-top: 1px solid #007acc; /* Optional accent */
}

.tab-content {
  display: flex;
  align-items: center;
  gap: 6px;
  overflow: hidden;
  white-space: nowrap;
  flex: 1;
}

.file-name {
  overflow: hidden;
  text-overflow: ellipsis;
}

.dirty-indicator {
  font-size: 10px;
  margin-left: 4px;
  color: #cccccc;
}

.close-btn {
  background: transparent;
  border: none;
  color: #cccccc;
  font-size: 16px;
  margin-left: 6px;
  padding: 0 4px;
  border-radius: 4px;
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.2s, background 0.2s;
}

.tab:hover .close-btn {
  opacity: 1;
}

.close-btn:hover {
  background: #4e4e4e;
}
</style>
