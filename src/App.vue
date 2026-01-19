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

    <!-- Simple Toolbar -->
    <div class="toolbar">
      <div class="toolbar-left">
        <button class="btn" @click="showWorkspaceSelector = true" title="Workspace">
          <span>📁</span> Workspace
        </button>
        <div class="divider"></div>
        <button class="btn" @click="handleNewFile" title="New File">
          <span>+</span> New
        </button>
        <button class="btn" @click="handleOpenFile" title="Open File">Open</button>
        <button class="btn" @click="handleSaveFile" :disabled="!currentFile" title="Save">
          Save
        </button>
        <button class="btn" @click="handleSaveAsTemplate" :disabled="!currentFile" title="Save as Template">
          Save Tmpl
        </button>
        <div class="divider"></div>
        <button
          class="btn mode-btn"
          :class="{ active: editorMode === 'code' }"
          @click="editorMode = 'code'"
        >
          Code
        </button>
        <button
          class="btn mode-btn"
          :class="{ active: editorMode === 'rich' }"
          @click="editorMode = 'rich'"
        >
          Rich
        </button>
      </div>
      <div class="toolbar-center">
        <span v-if="workspaceName" class="workspace-name">{{ workspaceName }}</span>
        <span v-if="currentFile" class="file-name">{{ getFileName(currentFile) }}</span>
        <span v-else class="file-name">Untitled</span>
      </div>
      <div class="toolbar-right">
        <label class="auto-compile-toggle">
          <input type="checkbox" v-model="autoCompile" />
          <span>Auto-compile</span>
        </label>
        <button
          class="btn compile-btn"
          :disabled="isCompiling || !fileContent"
          @click="handleCompilePDF"
        >
          {{ isCompiling ? 'Compiling...' : 'Compile PDF' }}
        </button>
      </div>
    </div>

    <!-- Main Layout -->
    <div class="main-layout">
      <!-- Workspace Sidebar -->
      <WorkspaceSidebar
        v-if="workspacePath"
        :key="sidebarKey"
        :workspace-path="workspacePath"
        :workspace-name="workspaceName"
        :current-file="currentFile"
        @file-selected="handleFileSelected"
        @template-selected="(name) => handleTemplateSelected(name, true)"
      />

      <!-- Editor Panel -->
      <div class="editor-panel">
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

const currentFile = ref<string | null>(null);
const fileContent = ref<string>('#set page(margin: 2cm)\n\n= Hello, Typst!\n\nThis is a simple Typst document.\n\n');
const editorMode = ref<'code' | 'rich'>('code');
const autoCompile = ref(true);
const pdfPreviewData = ref<Uint8Array | null>(null);
const isCompiling = ref(false);
const showWorkspaceSelector = ref(true); // Show on startup
const showTemplateModal = ref(false);
const workspacePath = ref<string | null>(null);
const workspaceName = ref<string>('');
const sidebarKey = ref(0);
let compileTimeout: NodeJS.Timeout | null = null;

// Auto-compile on content change
watch(
  () => fileContent.value,
  () => {
    if (autoCompile.value && fileContent.value) {
      // Debounce compilation
      if (compileTimeout) {
        clearTimeout(compileTimeout);
      }
      compileTimeout = setTimeout(() => {
        compileToPDF();
      }, 1000); // Wait 1 second after typing stops
    }
  }
);

const handleContentChanged = (content: string) => {
  fileContent.value = content;
};

const handleNewFile = () => {
  showTemplateModal.value = true;
};

const handleOpenFile = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: 'Typst',
          extensions: ['typ'],
        },
      ],
    });
    if (selected && typeof selected === 'string') {
      await handleFileSelected(selected);
    }
  } catch (error) {
    console.error('Failed to open file:', error);
  }
};

const handleSaveFile = async () => {
  if (!currentFile.value) {
    // Save as new file
    await handleNewFile();
    return;
  }

  try {
    await invoke('write_file', {
      path: currentFile.value,
      contents: fileContent.value,
    });
  } catch (error) {
    console.error('Failed to save file:', error);
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
    sidebarKey.value++; // Refresh sidebar
    // Trigger refresh of sidebar if possible, or just wait for next load
    // Ideally we would emit an event or update a shared store, but for now this is fine.
    // To update sidebar we would need to trigger it. 
    // We can re-fetch workspace data in App.vue if we extracted that logic.
    // But Sidebar has its own fetch.
  } catch (error) {
    console.error('Failed to save template:', error);
    alert('Failed to save template');
  }
};

const handleCompilePDF = async () => {
  await compileToPDF();
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
      // Compile from string (untitled document)
      pdfBytes = await invoke<number[]>('compile_typst_string_to_pdf', {
        source: fileContent.value,
        root_dir: null,
      });
    }

    pdfPreviewData.value = new Uint8Array(pdfBytes);
  } catch (error) {
    console.error('Compilation failed:', error);
    // Show error in preview or handle it
  } finally {
    isCompiling.value = false;
  }
};

const refreshPreview = () => {
  compileToPDF();
};

const getFileName = (path: string) => {
  return path.split(/[/\\]/).pop() || path;
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

    // Load workspace documents (recursive search)
    const documents = await invoke<string[]>('list_workspace_documents', {
      workspace_path: path,
    });

    // Open main.typ if it exists, otherwise first document
    const mainFile = documents.find((d) => d.endsWith('main.typ')) || documents[0];
    if (mainFile) {
      await handleFileSelected(mainFile);
    }
  } catch (error) {
    console.error('Failed to load workspace:', error);
  }
};

const handleFileSelected = async (filePath: string) => {
  try {
    currentFile.value = filePath;
    fileContent.value = await invoke('read_file', { path: filePath });
    // Auto-compile if enabled
    if (autoCompile.value) {
      await compileToPDF();
    }
  } catch (error) {
    console.error('Failed to open file:', error);
  }
};

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
    
    // Ask user where to save
    const filePath = await save({
      filters: [{ name: 'Typst', extensions: ['typ'] }],
      defaultPath: `${templateName}.typ`
    });
    
    if (filePath && typeof filePath === 'string') {
        // Write file
        await invoke('write_file', { path: filePath, contents: content });
        // Open it
        await handleFileSelected(filePath);
    }
  } catch (error) {
    console.error('Failed to create file from template:', error);
  }
};

onMounted(() => {
  // Check if there's a saved workspace
  const savedWorkspace = localStorage.getItem('current-workspace');
  if (savedWorkspace) {
    handleWorkspaceSelected(savedWorkspace);
  }
  // Initial compile if there's content and workspace is loaded
  if (!showWorkspaceSelector.value && autoCompile.value && fileContent.value) {
    setTimeout(() => compileToPDF(), 500);
  }
});
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell,
    sans-serif;
  overflow: hidden;
}

.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: #1e1e1e;
  color: #cccccc;
}

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

.btn {
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

.btn:hover:not(:disabled) {
  background: #505050;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.mode-btn.active {
  background: #007acc;
  color: white;
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

.auto-compile-toggle {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  cursor: pointer;
}

.auto-compile-toggle input {
  cursor: pointer;
}

.main-layout {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.editor-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border-right: 1px solid #3e3e42;
}

.preview-panel {
  width: 50%;
  display: flex;
  flex-direction: column;
  background: #252526;
}

.preview-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 16px;
  background: #2d2d30;
  border-bottom: 1px solid #3e3e42;
  font-size: 13px;
  color: #cccccc;
}

.icon-btn {
  background: transparent;
  border: none;
  color: #cccccc;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 16px;
}

.icon-btn:hover {
  background: #3e3e42;
}
</style>
