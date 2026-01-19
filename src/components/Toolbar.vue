<template>
  <div class="toolbar">
    <div class="toolbar-left">
      <button class="toolbar-btn" @click="$emit('new-project')" title="New Project">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path d="M8 2a.5.5 0 0 1 .5.5v5h5a.5.5 0 0 1 0 1h-5v5a.5.5 0 0 1-1 0v-5h-5a.5.5 0 0 1 0-1h5v-5A.5.5 0 0 1 8 2Z" />
        </svg>
        New
      </button>
      <button class="toolbar-btn" @click="$emit('open-file')" title="Open File">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path d="M8.5 4.5a.5.5 0 0 0-1 0v3h-3a.5.5 0 0 0 0 1h3v3a.5.5 0 0 0 1 0v-3h3a.5.5 0 0 0 0-1h-3v-3z" />
          <path d="M.5 1a.5.5 0 0 0-.5.5v13a.5.5 0 0 0 .5.5h15a.5.5 0 0 0 .5-.5v-13a.5.5 0 0 0-.5-.5H.5zm1 1v12h14V2H1.5z" />
        </svg>
        Open
      </button>
      <button
        class="toolbar-btn"
        :disabled="!currentFile"
        @click="$emit('save-file')"
        title="Save File"
      >
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path d="M2 1a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H9.5a1 1 0 0 0-1 1v7a1 1 0 0 1-1 1H9a.5.5 0 0 1 0 1H2.5a.5.5 0 0 1 0-1H3a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H2z" />
          <path d="M9.5 1a.5.5 0 0 0-.5.5v7a.5.5 0 0 0 .5.5h5a.5.5 0 0 0 .5-.5v-7a.5.5 0 0 0-.5-.5h-5zm.5 1h4v6h-4V2z" />
        </svg>
        Save
      </button>
      <div class="divider"></div>
    </div>

    <div class="toolbar-center">
      <span v-if="currentFile" class="file-name">{{ getFileName(currentFile) }}</span>
      <span v-else class="file-name">No file open</span>
    </div>

    <div class="toolbar-right">
      <button
        class="toolbar-btn compile-btn"
        :disabled="isCompiling"
        @click="$emit('compile-pdf')"
        title="Compile to PDF"
      >
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path d="M14 4.5V14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1h5v4.5a.5.5 0 0 0 .5.5H14z" />
          <path d="M9.5 0H3a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V4.5L9.5 0zm0 1v3h3L9.5 1z" />
        </svg>
        {{ isCompiling ? 'Compiling...' : 'Compile PDF' }}
      </button>
      <button
        class="toolbar-btn"
        :disabled="!currentFile || isCompiling"
        @click="$emit('compile-html')"
        title="Compile to HTML"
      >
        HTML
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  currentFile: string | null;
  isCompiling: boolean;
}>();

defineEmits<{
  'new-project': [];
  'open-file': [];
  'save-file': [];
  'compile-pdf': [];
  'compile-html': [];
}>();

const getFileName = (path: string) => {
  return path.split(/[/\\]/).pop() || path;
};
</script>

<style scoped>
.toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 16px;
  background: #2d2d30;
  border-bottom: 1px solid #3e3e42;
  height: 48px;
}

.toolbar-left,
.toolbar-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.toolbar-center {
  flex: 1;
  text-align: center;
}

.file-name {
  color: #cccccc;
  font-size: 13px;
}

.toolbar-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: #3e3e42;
  border: none;
  border-radius: 4px;
  color: #cccccc;
  cursor: pointer;
  font-size: 13px;
  transition: background 0.2s;
}

.toolbar-btn:hover:not(:disabled) {
  background: #505050;
}

.toolbar-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.compile-btn {
  background: #007acc;
  color: white;
}

.compile-btn:hover:not(:disabled) {
  background: #0098ff;
}

.divider {
  width: 1px;
  height: 24px;
  background: #3e3e42;
  margin: 0 8px;
}
</style>
