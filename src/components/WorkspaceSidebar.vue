<template>
  <div class="workspace-sidebar">
    <div class="sidebar-header">
      <h3>{{ workspaceName }}</h3>
    </div>

    <div class="sidebar-section">
      <div class="section-header">
        <span>📄 Documents</span>
        <button class="icon-btn-small" @click="refreshDocuments" title="Refresh">↻</button>
      </div>
      <div class="file-list">
        <div
          v-for="doc in documents"
          :key="doc"
          :class="['file-item', { active: doc === currentFile }]"
          @click="$emit('file-selected', doc)"
        >
          <span class="file-icon">📄</span>
          <span class="file-name">{{ getFileName(doc) }}</span>
        </div>
        <div v-if="documents.length === 0" class="empty-state">No documents</div>
      </div>
    </div>

    <div class="sidebar-section">
      <div class="section-header">
        <span>📋 Templates</span>
      </div>
      <div class="file-list">
        <div
          v-for="template in templates"
          :key="template"
          class="file-item template-item"
          @click="$emit('template-selected', template)"
        >
          <span class="file-icon">📋</span>
          <span class="file-name">{{ template }}</span>
        </div>
        <div v-if="templates.length === 0" class="empty-state">No templates</div>
      </div>
    </div>

    <div class="sidebar-section">
      <div class="section-header">
        <span>📦 Resources</span>
      </div>
      <div class="file-list">
        <div
          v-for="resource in resources"
          :key="resource"
          class="file-item resource-item"
        >
          <span class="file-icon">📦</span>
          <span class="file-name">{{ getFileName(resource) }}</span>
        </div>
        <div v-if="resources.length === 0" class="empty-state">No resources</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps<{
  workspacePath: string | null;
  workspaceName: string;
  currentFile: string | null;
}>();

defineEmits<{
  'file-selected': [path: string];
  'template-selected': [name: string];
}>();

const documents = ref<string[]>([]);
const templates = ref<string[]>([]);
const resources = ref<string[]>([]);

const getFileName = (path: string) => {
  return path.split(/[/\\]/).pop() || path;
};

const loadWorkspaceData = async () => {
  if (!props.workspacePath) return;

  try {
    const [docs, tmpls, res] = await Promise.all([
      invoke<string[]>('list_workspace_documents', {
        workspace_path: props.workspacePath,
      }),
      invoke<string[]>('list_workspace_templates', {
        workspace_path: props.workspacePath,
      }),
      invoke<string[]>('list_workspace_resources', {
        workspace_path: props.workspacePath,
      }),
    ]);

    documents.value = docs;
    templates.value = tmpls;
    resources.value = res;
  } catch (error) {
    console.error('Failed to load workspace data:', error);
  }
};

const refreshDocuments = async () => {
  await loadWorkspaceData();
};

watch(() => props.workspacePath, loadWorkspaceData, { immediate: true });
onMounted(() => {
  loadWorkspaceData();
});
</script>

<style scoped>
.workspace-sidebar {
  width: 250px;
  background: #252526;
  border-right: 1px solid #3e3e42;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.sidebar-header {
  padding: 12px 16px;
  border-bottom: 1px solid #3e3e42;
}

.sidebar-header h3 {
  font-size: 14px;
  font-weight: 600;
  color: #cccccc;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.sidebar-section {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 4px;
  font-size: 12px;
  font-weight: 600;
  color: #858585;
  text-transform: uppercase;
}

.icon-btn-small {
  background: transparent;
  border: none;
  color: #858585;
  cursor: pointer;
  padding: 2px 4px;
  font-size: 12px;
}

.icon-btn-small:hover {
  color: #cccccc;
}

.file-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
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
}

.file-item:hover {
  background: #2a2d2e;
}

.file-item.active {
  background: #094771;
  color: white;
}

.file-icon {
  flex-shrink: 0;
}

.file-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.template-item:hover {
  background: #2a3a2e;
}

.resource-item:hover {
  background: #3a2a2e;
}

.empty-state {
  padding: 12px;
  text-align: center;
  color: #6a6a6a;
  font-size: 12px;
}
</style>
