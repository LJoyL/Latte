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
      
      <!-- Search View -->
      <div v-show="activeView === 'search'" class="sidebar-container">
        <div class="sidebar-header">
           <h3>Search</h3>
        </div>
        <div class="empty-sidebar">
          <p style="color: #858585; font-size: 13px;">Search functionality coming soon.</p>
        </div>
      </div>

      <!-- Review View -->
      <div v-show="activeView === 'review'" class="sidebar-container">
        <CommentsSidebar />
      </div>

      <!-- Settings View -->
      <div v-show="activeView === 'settings'" class="sidebar-container">
        <div class="sidebar-header">
           <h3>Settings</h3>
        </div>
        <div class="empty-sidebar">
           <div class="setting-item">
             <label class="auto-compile-toggle">
               <input type="checkbox" v-model="autoCompile" />
               <span>Auto-compile on type</span>
             </label>
           </div>
           
           <!-- Collaboration Settings -->
           <div class="sidebar-header" style="margin-top: 20px;">
             <h3>Collaboration</h3>
           </div>
           <div class="collab-settings" style="padding: 16px;">
             <div class="form-group">
               <label>Server URL</label>
               <input v-model="collabServerUrl" placeholder="ws://localhost:1234" class="input-field" />
             </div>
             <div class="form-group">
               <label>Room Name</label>
               <input v-model="collabRoom" placeholder="my-room" class="input-field" />
             </div>
             <div class="form-group">
               <label>Username</label>
               <input v-model="collabUsername" placeholder="Guest" class="input-field" />
             </div>
             <button 
               class="btn full-width" 
               @click="toggleCollaboration"
               :class="{ 'connected': collabStatus === 'connected', 'connecting': collabStatus === 'connecting' }"
             >
               {{ collabBtnText }}
             </button>
             <p class="status-text">Status: {{ collabStatus }}</p>
           </div>
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
            <DiagnosticsPanel 
               :diagnostics="diagnostics" 
               :visible="showDiagnostics"
               @close="showDiagnostics = false"
            />
          </div>
        </div>
      </div>
    </div>

    <!-- Status Bar -->
    <StatusBar
      :file-path="currentFile"
      :status-message="statusMessage"
      @click="toggleDiagnostics"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue';
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
import DiagnosticsPanel from './components/DiagnosticsPanel.vue';
import CommentsSidebar from './components/CommentsSidebar.vue';
import { CollaborationManager } from './utils/collaboration';

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
const diagnostics = ref<any[]>([]);
const showDiagnostics = ref(false);
const statusMessage = ref('Ready');
let compileTimeout: NodeJS.Timeout | null = null;

// Collaboration State
const collabManager = CollaborationManager.getInstance();
const collabStatus = collabManager.status;
const collabServerUrl = ref('wss://demos.yjs.dev');
const collabRoom = ref('latte-demo-room');
const collabUsername = ref(collabManager.currentUser.name);

const collabBtnText = ref('Connect');

watch(collabStatus, (newStatus) => {
  if (newStatus === 'connected') collabBtnText.value = 'Disconnect';
  else if (newStatus === 'connecting') collabBtnText.value = 'Connecting...';
  else collabBtnText.value = 'Connect';
});

const toggleCollaboration = () => {
  if (collabStatus.value === 'connected' || collabStatus.value === 'connecting') {
    collabManager.disconnect();
  } else {
    collabManager.updateUserState(collabUsername.value);
    collabManager.connect(collabRoom.value, collabServerUrl.value);
  }
};

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

  window.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
});

const handleKeydown = (e: KeyboardEvent) => {
  if ((e.ctrlKey || e.metaKey) && e.key === 's') {
    e.preventDefault();
    handleSaveFile();
  }
};

const createUntitledTab = () => {
  const content = '#set page(margin: 2cm)\n\n= New Document\n\n';
  const newTab: OpenFile = {
    path: `Untitled-${Date.now()}`,
    isDirty: false,
    content: content
  };
  openFiles.value.push(newTab);
  activeFileIndex.value = openFiles.value.length - 1;
  currentFile.value = null;
  fileContent.value = content;
};

// Tabs Logic
const handleTabSelected = async (index: number) => {
  if (index === activeFileIndex.value) return;
  
  // Save current content to the previous tab object
  if (activeFileIndex.value >= 0 && activeFileIndex.value < openFiles.value.length) {
    openFiles.value[activeFileIndex.value].content = fileContent.value;
  }
  
  const file = openFiles.value[index];
  currentFile.value = file.path.startsWith('Untitled') ? null : file.path;
  
  // Restore content from tab state
  // For saved files, we might want to check disk, but for now cache is source of truth for UI
  // If content is empty and it's a real file, read from disk (first load)
  if (!file.content && !file.path.startsWith('Untitled')) {
     try {
       file.content = await invoke('read_file', { path: file.path });
     } catch (e) {
       console.error(e);
     }
  }
  
  fileContent.value = file.content;
  activeFileIndex.value = index;
  
  // Trigger compile for the new active tab
  compileToPDF();
};

const handleTabClosed = (index: number) => {
  // If closing active tab, switch to another
  const isActive = index === activeFileIndex.value;
  
  // Warn if dirty? For now just close.
  openFiles.value.splice(index, 1);
  
  if (openFiles.value.length === 0) {
    createUntitledTab();
  } else if (isActive) {
    // If we closed the active tab, pick the previous one or the first one
    const newIndex = Math.max(0, index - 1);
    // We need to set activeFileIndex to -1 temporarily so handleTabSelected treats it as a change
    activeFileIndex.value = -1; 
    handleTabSelected(newIndex);
  } else if (index < activeFileIndex.value) {
    // If we closed a tab before the active one, decrement index
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
      openFiles.value[activeFileIndex.value].content = content;
      openFiles.value[activeFileIndex.value].isDirty = true;
    }
};

// ... copy other existing methods ...
// (handleCompilePDF, compileToPDF, refreshPreview, getFileName, handleWorkspaceSelected, handleFileSelected, handleTemplateSelected, handleSaveAsTemplate)

// Re-implement handleFileSelected to use tabs
const handleFileSelected = async (filePath: string) => {
  // Save current tab content before switching
  if (activeFileIndex.value >= 0 && activeFileIndex.value < openFiles.value.length) {
    openFiles.value[activeFileIndex.value].content = fileContent.value;
  }

  // Check if already open
  const existingIndex = openFiles.value.findIndex(f => f.path === filePath);
  if (existingIndex >= 0) {
    // If already open, just switch to it (which will load content from cache or disk)
    handleTabSelected(existingIndex);
    return;
  }
  
  // Add new tab
  // Read content first
  let content = '';
  try {
    content = await invoke('read_file', { path: filePath });
  } catch (e) {
    console.error(e);
    return;
  }

  openFiles.value.push({ path: filePath, isDirty: false, content: content });
  activeFileIndex.value = openFiles.value.length - 1;
  
  currentFile.value = filePath;
  fileContent.value = content;
  compileToPDF();
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
        path: `Untitled-${templateName}-${Date.now()}`,
        isDirty: true,
        content: content
    };
    
    // Save current tab content before switching
    if (activeFileIndex.value >= 0 && activeFileIndex.value < openFiles.value.length) {
      openFiles.value[activeFileIndex.value].content = fileContent.value;
    }

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
  statusMessage.value = 'Compiling...';
  
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
      
      // Get diagnostics
      diagnostics.value = await invoke('get_typst_diagnostics', {
        entry_path: currentFile.value,
      });
    } else {
      // Compile from string
      // Note: get_typst_diagnostics might not work well with string compilation in current backend
      // So we assume success if no error thrown, or parse error from catch
      pdfBytes = await invoke<number[]>('compile_typst_string_to_pdf', {
        source: fileContent.value,
        root_dir: workspacePath.value, 
      });
      diagnostics.value = [];
    }

    pdfPreviewData.value = new Uint8Array(pdfBytes);
    
    if (diagnostics.value.length > 0) {
      statusMessage.value = `Compiled with ${diagnostics.value.length} problems`;
      showDiagnostics.value = true;
    } else {
      statusMessage.value = 'Ready';
    }
  } catch (error: any) {
    console.error('Compilation failed:', error);
    statusMessage.value = 'Compilation Failed';
    
    // Parse error string into diagnostics if possible, or just show general error
    // The backend returns a string "Compilation errors: ..."
    const errorStr = typeof error === 'string' ? error : JSON.stringify(error);
    diagnostics.value = [{
       severity: 'error',
       message: errorStr
    }];
    showDiagnostics.value = true;
  } finally {
    isCompiling.value = false;
  }
};

const toggleDiagnostics = () => {
  showDiagnostics.value = !showDiagnostics.value;
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

.sidebar-header {
  padding: 12px 16px;
  border-bottom: 1px solid #3e3e42;
}

.sidebar-header h3 {
  font-size: 14px;
  font-weight: 600;
  color: #cccccc;
}

.setting-item {
  display: flex;
  justify-content: center;
  margin-top: 10px;
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

.input-field {
  width: 100%;
  background: #3e3e42;
  border: 1px solid #2d2d30;
  color: #cccccc;
  padding: 6px 8px;
  border-radius: 4px;
  margin-top: 4px;
  font-size: 13px;
}

.form-group {
  margin-bottom: 12px;
}

.form-group label {
  font-size: 11px;
  color: #858585;
  text-transform: uppercase;
  font-weight: 600;
}

.btn.full-width {
  width: 100%;
  justify-content: center;
  margin-top: 8px;
}

.btn.connected {
  background: #2e7d32;
  color: white;
}

.btn.connecting {
  background: #d4a72c;
  color: black;
}

.status-text {
  font-size: 11px;
  color: #858585;
  margin-top: 8px;
  text-align: center;
}
</style>
