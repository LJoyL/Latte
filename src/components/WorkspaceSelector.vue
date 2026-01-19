<template>
  <div class="workspace-selector">
    <div class="workspace-header">
      <h2>Select Workspace</h2>
      <button class="close-btn" @click="$emit('close')">×</button>
    </div>

    <div class="workspace-actions">
      <button class="action-btn primary" @click="handleSelectWorkspace">
        <span>📁</span> Select Workspace Folder
      </button>
    </div>

    <div v-if="recentWorkspaces.length > 0" class="recent-workspaces">
      <h3>Recent Workspaces</h3>
      <div class="workspace-list">
        <div
          v-for="workspace in recentWorkspaces"
          :key="workspace.root"
          class="workspace-item"
          @click="selectWorkspace(workspace.root)"
        >
          <div class="workspace-icon">📂</div>
          <div class="workspace-info">
            <div class="workspace-name">{{ workspace.name }}</div>
            <div class="workspace-path">{{ workspace.root }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

const emit = defineEmits<{
  close: [];
  'workspace-selected': [path: string];
}>();

const recentWorkspaces = ref<Array<{ name: string; root: string }>>([]);

onMounted(() => {
  loadRecentWorkspaces();
});

const loadRecentWorkspaces = () => {
  // Load from localStorage
  const stored = localStorage.getItem('recent-workspaces');
  if (stored) {
    try {
      recentWorkspaces.value = JSON.parse(stored);
    } catch (e) {
      console.error('Failed to parse recent workspaces:', e);
    }
  }
};

const saveRecentWorkspace = (name: string, root: string) => {
  const existing = recentWorkspaces.value.findIndex((w) => w.root === root);
  if (existing >= 0) {
    recentWorkspaces.value.splice(existing, 1);
  }
  recentWorkspaces.value.unshift({ name, root });
  recentWorkspaces.value = recentWorkspaces.value.slice(0, 5); // Keep only 5 recent
  localStorage.setItem('recent-workspaces', JSON.stringify(recentWorkspaces.value));
};

const handleSelectWorkspace = async () => {
  try {
    const workspacePath = await open({
      directory: true,
      multiple: false,
      title: 'Select Workspace Folder',
    });

    if (workspacePath && typeof workspacePath === 'string') {
      // Initialize workspace (creates .latte folder if needed)
      const config = await invoke<any>('init_workspace', {
        workspace_path: workspacePath,
      });
      saveRecentWorkspace(config.name, workspacePath);
      emit('workspace-selected', workspacePath);
    }
  } catch (error) {
    console.error('Failed to select workspace:', error);
    alert('Failed to initialize workspace. Please try again.');
  }
};

const selectWorkspace = async (path: string) => {
  try {
    // Open workspace (will initialize if needed)
    const config = await invoke<any>('open_workspace', {
      workspace_path: path,
    });
    saveRecentWorkspace(config.name, path);
    emit('workspace-selected', path);
  } catch (error) {
    console.error('Failed to open workspace:', error);
    alert('Failed to open workspace. Please try again.');
  }
};
</script>

<style scoped>
.workspace-selector {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.8);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.workspace-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  max-width: 600px;
  margin-bottom: 32px;
}

.workspace-header h2 {
  color: #cccccc;
  font-size: 24px;
}

.close-btn {
  background: transparent;
  border: none;
  color: #cccccc;
  font-size: 32px;
  cursor: pointer;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
}

.close-btn:hover {
  background: #3e3e42;
}

.workspace-content {
  background: #252526;
  border-radius: 8px;
  padding: 32px;
  width: 100%;
  max-width: 600px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
}

.workspace-actions {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 32px;
}

.action-btn {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px 24px;
  background: #3e3e42;
  border: none;
  border-radius: 6px;
  color: #cccccc;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s;
  text-align: left;
}

.action-btn:hover {
  background: #505050;
  transform: translateY(-2px);
}

.action-btn.primary {
  background: #007acc;
  color: white;
}

.action-btn.primary:hover {
  background: #0098ff;
}

.recent-workspaces {
  margin-top: 32px;
}

.recent-workspaces h3 {
  color: #cccccc;
  font-size: 16px;
  margin-bottom: 16px;
}

.workspace-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.workspace-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: #2d2d30;
  border: 1px solid #3e3e42;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.workspace-item:hover {
  background: #3e3e42;
  border-color: #007acc;
}

.workspace-icon {
  font-size: 24px;
}

.workspace-info {
  flex: 1;
}

.workspace-name {
  color: #cccccc;
  font-weight: 500;
  margin-bottom: 4px;
}

.workspace-path {
  color: #858585;
  font-size: 12px;
}
</style>
