<template>
  <div ref="editorContainer" class="monaco-editor-container"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import loader from '@monaco-editor/loader';

const props = defineProps<{
  filePath: string;
  content: string;
}>();

const emit = defineEmits<{
  'content-changed': [content: string];
}>();

const editorContainer = ref<HTMLElement | null>(null);
let editor: any = null;
let monaco: any = null;

// Configure Monaco for Typst
const configureMonaco = (monacoInstance: any) => {
  monaco = monacoInstance;

  // Register Typst language
  monaco.languages.register({ id: 'typst' });

  // Set Typst language configuration
  monaco.languages.setLanguageConfiguration('typst', {
    comments: {
      lineComment: '//',
      blockComment: ['/*', '*/'],
    },
    brackets: [
      ['{', '}'],
      ['[', ']'],
      ['(', ')'],
    ],
    autoClosingPairs: [
      { open: '{', close: '}' },
      { open: '[', close: ']' },
      { open: '(', close: ')' },
      { open: '"', close: '"' },
      { open: "'", close: "'" },
    ],
    surroundingPairs: [
      { open: '{', close: '}' },
      { open: '[', close: ']' },
      { open: '(', close: ')' },
      { open: '"', close: '"' },
      { open: "'", close: "'" },
    ],
  });

  // Set Typst tokens (simplified)
  monaco.languages.setMonarchTokensProvider('typst', {
    tokenizer: {
      root: [
        [/\/\/.*$/, 'comment'],
        [/\/\*[\s\S]*?\*\//, 'comment'],
        [/"[^"]*"/, 'string'],
        [/'[^']*'/, 'string'],
        [/\b(set|let|show|if|else|for|while|return|import|include)\b/, 'keyword'],
        [/\b(true|false|none|auto)\b/, 'constant'],
        [/[=+\-*\/<>!]+/, 'operator'],
        [/\d+\.?\d*/, 'number'],
        [/[a-zA-Z_][a-zA-Z0-9_]*/, 'identifier'],
      ],
    },
  });

  // Set Typst theme
  monaco.editor.defineTheme('typst-dark', {
    base: 'vs-dark',
    inherit: true,
    rules: [
      { token: 'comment', foreground: '6A9955' },
      { token: 'string', foreground: 'CE9178' },
      { token: 'keyword', foreground: '569CD6' },
      { token: 'constant', foreground: '4EC9B0' },
      { token: 'operator', foreground: 'D4D4D4' },
      { token: 'number', foreground: 'B5CEA8' },
    ],
    colors: {
      'editor.background': '#1e1e1e',
    },
  });
};

onMounted(async () => {
  if (!editorContainer.value) return;

  try {
    // Load Monaco Editor
    const monacoInstance = await loader.init();
    configureMonaco(monacoInstance);

    // Create editor instance
    editor = monacoInstance.editor.create(editorContainer.value, {
      value: props.content,
      language: 'typst',
      theme: 'typst-dark',
      automaticLayout: true,
      fontSize: 14,
      minimap: { enabled: true },
      scrollBeyondLastLine: false,
      wordWrap: 'on',
      lineNumbers: 'on',
      renderWhitespace: 'selection',
      tabSize: 2,
      insertSpaces: true,
    });

    // Listen for content changes
    editor.onDidChangeModelContent(() => {
      const content = editor.getValue();
      emit('content-changed', content);
    });

    // Note: Diagnostics can be added later if needed
    // For now, we focus on basic editing and compilation
  } catch (error) {
    console.error('Failed to initialize Monaco Editor:', error);
  }
});

// Watch for content changes from parent
watch(
  () => props.content,
  (newContent) => {
    if (editor && editor.getValue() !== newContent) {
      editor.setValue(newContent);
    }
  }
);

// Watch for file path changes
watch(
  () => props.filePath,
  async (newPath) => {
    if (editor && newPath) {
      const content = await invoke<string>('read_file', { path: newPath });
      editor.setValue(content);
    }
  }
);

const getLineNumber = (offset: number): number => {
  if (!editor) return 1;
  const model = editor.getModel();
  const position = model.getPositionAt(offset);
  return position.lineNumber;
};

const getColumnNumber = (offset: number): number => {
  if (!editor) return 1;
  const model = editor.getModel();
  const position = model.getPositionAt(offset);
  return position.column;
};

onUnmounted(() => {
  if (editor) {
    editor.dispose();
  }
});
</script>

<style scoped>
.monaco-editor-container {
  width: 100%;
  height: 100%;
}
</style>
