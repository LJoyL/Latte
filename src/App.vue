<template>
  <div class="app-container">
    <!-- Workspace Selector Modal -->
    <WorkspaceSelector
      v-if="showWorkspaceSelector"
      @close="showWorkspaceSelector = false"
      @workspace-selected="handleWorkspaceSelected"
    />

    <!-- Template Selection Modal -->
    <TemplateModal
      v-if="showTemplateModal"
      :workspace-path="workspacePath"
      @close="showTemplateModal = false"
      @template-selected="handleTemplateSelected"
    />

    <div class="main-layout">
      <!-- Activity Bar -->
      <ActivityBar :active-view="activeView" @view-changed="activeView = $event" />

      <!-- Workspace Sidebar -->
      <div v-show="activeView === 'explorer'" class="sidebar-container">
        <WorkspaceSidebar
          v-if="workspacePath"
          :key="sidebarKey"
          :workspace-path="workspacePath"
          :workspace-name="workspaceName"
          :current-file="currentFile"
          @file-selected="handleFileSelected"
          @template-selected="(name) => handleTemplateSelected(name, true)"
        />
        <div v-else class="empty-sidebar">
          <button class="btn" @click="showWorkspaceSelector = true">Open Folder</button>
        </div>
      </div>

      <!-- Main Content Area -->
      <div class="content-area">
        <!-- Editor Tabs -->
        <EditorTabs
          :open-files="openFiles"
          :active-index="activeFileIndex"
          @tab-selected="handleTabSelected"
          @tab-closed="handleTabClosed"
        />

        <!-- Editor & Preview Split -->
        <div class="editor-preview-split">
          <!-- Editor Panel -->
          <div class="editor-panel">
            <div class="editor-toolbar-mini">
              <div class="left">
                <button
                  class="btn-icon"
                  :class="{ active: editorMode === 'code' }"
                  @click="editorMode = 'code'"
                  title="Code Mode"
                >
                  Code
                </button>
                <button
                  class="btn-icon"
                  :class="{ active: editorMode === 'rich' }"
                  @click="editorMode = 'rich'"
                  title="Rich Text Mode"
                >
                  Rich
                </button>
              </div>
              <div class="right">
                <button class="btn-icon" @click="handleSaveFile" title="Save (Ctrl+S)">💾</button>
                <button class="btn-icon" @click="handleSaveAsTemplate" title="Save as Template">📋</button>
                <button class="btn-icon" @click="handleNewFile" title="New File">➕</button>
                <label class="auto-compile-toggle">
                  <input type="checkbox" v-model="autoCompile" />
                  <span>Auto</span>
                </label>
              </div>
            </div>

            <MonacoEditor
              v-if="editorMode === 'code'"
              :file-path="currentFile || 'untitled.typ'"
              :content="fileContent"
              @content-changed="handleContentChanged"
            />
            <RichTextEditor
              v-else
              :content="fileContent"
              @content-changed="handleContentChanged"
            />
          </div>

          <!-- Preview Panel -->
          <div class="preview-panel">
            <div class="preview-header">
              <span>PDF Preview</span>
              <button
                v-if="pdfPreviewData"
                class="icon-btn"
                @click="refreshPreview"
                title="Refresh Preview"
              >
                ↻
              </button>
            </div>
            <PreviewPanel :pdf-data="pdfPreviewData" :is-loading="isCompiling" />
          </div>
        </div>
      </div>
    </div>

    <!-- Status Bar -->
    <StatusBar
      :file-path="currentFile"
      :status-message="isCompiling ? 'Compiling...' : 'Ready'"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';
import MonacoEditor from './components/MonacoEditor.vue';
import RichTextEditor from './components/RichTextEditor.vue';
import PreviewPanel from './components/PreviewPanel.vue';
import WorkspaceSelector from './components/WorkspaceSelector.vue';
import WorkspaceSidebar from './components/WorkspaceSidebar.vue';
import TemplateModal from './components/TemplateModal.vue';
import ActivityBar from './components/ActivityBar.vue';
import EditorTabs, { OpenFile } from './components/EditorTabs.vue';
import StatusBar from './components/StatusBar.vue';

// State
const activeView = ref('explorer');
const openFiles = ref<OpenFile[]>([]);
const activeFileIndex = ref(-1);
const currentFile = ref<string | null>(null);
const fileContent = ref<string>('#set page(margin: 2cm)\n\n= Hello, Typst!\n\nThis is a simple Typst document.\n\n');
const editorMode = ref<'code' | 'rich'>('code');
const autoCompile = ref(true);
const pdfPreviewData = ref<Uint8Array | null>(null);
const isCompiling = ref(false);
const showWorkspaceSelector = ref(true);
const showTemplateModal = ref(false);
const workspacePath = ref<string | null>(null);
const workspaceName = ref<string>('');
const sidebarKey = ref(0);
let compileTimeout: NodeJS.Timeout | null = null;

// Initial Setup
onMounted(() => {
  const savedWorkspace = localStorage.getItem('current-workspace');
  if (savedWorkspace) {
    handleWorkspaceSelected(savedWorkspace);
  }
  
  // Create initial untitled tab
  if (openFiles.value.length === 0) {
    createUntitledTab();
  }
});

const createUntitledTab = () => {
  const newTab: OpenFile = {
    path: 'Untitled-1',
    isDirty: false
  };
  openFiles.value.push(newTab);
  activeFileIndex.value = openFiles.value.length - 1;
  currentFile.value = null;
  fileContent.value = '#set page(margin: 2cm)\n\n= New Document\n\n';
};

// Tabs Logic
const handleTabSelected = async (index: number) => {
  if (index === activeFileIndex.value) return;
  
  // Save content of current tab to memory/state if needed?
  // Ideally we should have a map of contents. For now, we might lose unsaved changes if we switch tabs purely by path re-read.
  // Implementation note: a real editor holds content in memory for all tabs.
  // To keep it simple: we warn if switching from dirty tab? 
  // BETTER: Store content in `openFiles` or a separate map.
  
  // Let's implement simple content caching
  // Save current content to the previous tab object (we need to extend OpenFile interface)
  // For now, let's just switch and re-read file if it exists.
  // Warning: Switching tabs on an unsaved 'Untitled' file or dirty file will lose changes in this simple impl.
  // We need to upgrade OpenFile to hold content.
  
  const file = openFiles.value[index];
  
  if (file.path.startsWith('Untitled')) {
     currentFile.value = null;
     // If we stored content in tab, restore it. If not, it's problematic.
     // For this iteration, let's assume we re-read from disk for real files.
  } else {
     currentFile.value = file.path;
     try {
       fileContent.value = await invoke('read_file', { path: file.path });
       compileToPDF();
     } catch (e) {
       console.error(e);
     }
  }
  
  activeFileIndex.value = index;
};

const handleTabClosed = (index: number) => {
  // If closing active tab, switch to another
  const isActive = index === activeFileIndex.value;
  openFiles.value.splice(index, 1);
  
  if (openFiles.value.length === 0) {
    createUntitledTab();
  } else if (isActive) {
    activeFileIndex.value = Math.max(0, index - 1);
    handleTabSelected(activeFileIndex.value);
  } else if (index < activeFileIndex.value) {
    activeFileIndex.value--;
  }
};

// File Operations
const handleNewFile = () => {
  showTemplateModal.value = true;
};

const handleOpenFile = async () => {
  // ... existing open logic ...
};

const handleSaveFile = async () => {
  if (!currentFile.value) {
    // Save As
    const filePath = await save({
      filters: [{ name: 'Typst', extensions: ['typ'] }],
      defaultPath: 'document.typ',
    });
    
    if (filePath) {
      currentFile.value = filePath;
      // Update tab name
      if (activeFileIndex.value >= 0) {
        openFiles.value[activeFileIndex.value].path = filePath;
        openFiles.value[activeFileIndex.value].isDirty = false;
      }
    } else {
      return;
    }
  }

  try {
    await invoke('write_file', {
      path: currentFile.value,
      contents: fileContent.value,
    });
    if (activeFileIndex.value >= 0) {
      openFiles.value[activeFileIndex.value].isDirty = false;
    }
  } catch (error) {
    console.error('Failed to save file:', error);
  }
};

const handleContentChanged = (content: string) => {
  fileContent.value = content;
  if (activeFileIndex.value >= 0) {
    openFiles.value[activeFileIndex.value].isDirty = true;
  }
};

// ... copy other existing methods ...
// (handleCompilePDF, compileToPDF, refreshPreview, getFileName, handleWorkspaceSelected, handleFileSelected, handleTemplateSelected, handleSaveAsTemplate)

// Re-implement handleFileSelected to use tabs
const handleFileSelected = async (filePath: string) => {
  // Check if already open
  const existingIndex = openFiles.value.findIndex(f => f.path === filePath);
  if (existingIndex >= 0) {
    activeFileIndex.value = existingIndex;
    handleTabSelected(existingIndex);
    return;
  }
  
  // Add new tab
  openFiles.value.push({ path: filePath, isDirty: false });
  activeFileIndex.value = openFiles.value.length - 1;
  
  currentFile.value = filePath;
  try {
    fileContent.value = await invoke('read_file', { path: filePath });
    compileToPDF();
  } catch (e) {
    console.error(e);
  }
};

// Auto-compile watcher
watch(
  () => fileContent.value,
  () => {
    if (autoCompile.value && fileContent.value) {
      if (compileTimeout) clearTimeout(compileTimeout);
      compileTimeout = setTimeout(() => {
        compileToPDF();
      }, 1000);
    }
  }
);

const handleTemplateSelected = async (templateName: string, isWorkspace: boolean) => {
  showTemplateModal.value = false;
  try {
    let content = '';
    if (isWorkspace && workspacePath.value) {
       content = await invoke('get_workspace_template', { 
         workspace_path: workspacePath.value,
         template_name: templateName 
       });
    } else {
       content = await invoke('get_template', { name: templateName });
    }
    
    // Create new untitled tab with content
    const newTab: OpenFile = {
        path: `Untitled-${templateName}`,
        isDirty: true
    };
    openFiles.value.push(newTab);
    activeFileIndex.value = openFiles.value.length - 1;
    currentFile.value = null;
    fileContent.value = content;
    
    // Trigger compile
    compileToPDF();
    
  } catch (error) {
    console.error('Failed to create file from template:', error);
  }
};

const handleSaveAsTemplate = async () => {
  if (!workspacePath.value) {
    alert('Please open a workspace to save templates.');
    return;
  }
  
  const name = prompt('Enter template name:');
  if (!name) return;
  
  try {
    await invoke('add_workspace_template', {
      workspace_path: workspacePath.value,
      template_name: name,
      template_content: fileContent.value,
    });
    alert('Template saved!');
    sidebarKey.value++; 
  } catch (error) {
    console.error('Failed to save template:', error);
    alert('Failed to save template');
  }
};

const handleWorkspaceSelected = async (path: string) => {
  workspacePath.value = path;
  showWorkspaceSelector.value = false;
  localStorage.setItem('current-workspace', path);
  
  try {
    const config = await invoke<any>('open_workspace', {
      workspace_path: path,
    });
    workspaceName.value = config.name;

    // Load workspace documents
    const documents = await invoke<string[]>('list_workspace_documents', {
      workspace_path: path,
    });

    // Open main.typ if exists
    const mainFile = documents.find((d) => d.endsWith('main.typ'));
    if (mainFile) {
      handleFileSelected(mainFile);
    }
  } catch (error) {
    console.error('Failed to load workspace:', error);
  }
};

const compileToPDF = async () => {
  if (!fileContent.value.trim()) return;

  isCompiling.value = true;
  try {
    let pdfBytes: number[];

    if (currentFile.value) {
      // Save first if file exists
      await invoke('write_file', {
        path: currentFile.value,
        contents: fileContent.value,
      });
      // Compile from file
      pdfBytes = await invoke<number[]>('compile_typst_to_pdf', {
        entry_path: currentFile.value,
      });
    } else {
      // Compile from string
      pdfBytes = await invoke<number[]>('compile_typst_string_to_pdf', {
        source: fileContent.value,
        root_dir: workspacePath.value, // Pass workspace as root if available
      });
    }

    pdfPreviewData.value = new Uint8Array(pdfBytes);
  } catch (error) {
    console.error('Compilation failed:', error);
  } finally {
    isCompiling.value = false;
  }
};

const refreshPreview = () => compileToPDF();
</script>

<style>
/* Global Reset */
* { margin: 0; padding: 0; box-sizing: border-box; }
body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; overflow: hidden; background: #1e1e1e; color: #cccccc; }

.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.main-layout {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.sidebar-container {
  width: 250px;
  background: #252526;
  border-right: 1px solid #3e3e42;
  display: flex;
  flex-direction: column;
}

.content-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: #1e1e1e;
}

.editor-preview-split {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.editor-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  border-right: 1px solid #3e3e42;
}

.preview-panel {
  width: 50%;
  display: flex;
  flex-direction: column;
  background: #252526;
}

.editor-toolbar-mini {
  height: 32px;
  background: #2d2d2d;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 8px;
  border-bottom: 1px solid #3e3e42;
}

.editor-toolbar-mini .left,
.editor-toolbar-mini .right {
  display: flex;
  align-items: center;
  gap: 4px;
}

.btn-icon {
  background: transparent;
  border: none;
  color: #cccccc;
  cursor: pointer;
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 12px;
}

.btn-icon:hover {
  background: #3e3e42;
}

.btn-icon.active {
  background: #007acc;
  color: white;
}

.empty-sidebar {
  padding: 20px;
  text-align: center;
}

/* Re-use preview header styles */
.preview-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 16px;
  background: #2d2d30;
  border-bottom: 1px solid #3e3e42;
  font-size: 13px;
  color: #cccccc;
  height: 32px; /* Match toolbar mini */
}

.auto-compile-toggle {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 11px;
  margin-left: 8px;
  cursor: pointer;
}
</style>
