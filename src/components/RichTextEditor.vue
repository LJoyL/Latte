<template>
  <div class="rich-text-editor">
    <div class="editor-toolbar">
      <div class="toolbar-group">
        <button class="toolbar-btn" @click="insertH1" title="Heading 1">H1</button>
        <button class="toolbar-btn" @click="insertH2" title="Heading 2">H2</button>
        <button class="toolbar-btn" @click="insertH3" title="Heading 3">H3</button>
      </div>
      <div class="divider"></div>
      <div class="toolbar-group">
        <button class="toolbar-btn" @click="insertBold" title="Bold"><strong>B</strong></button>
        <button class="toolbar-btn" @click="insertItalic" title="Italic"><em>I</em></button>
        <button class="toolbar-btn" @click="insertQuote" title="Quote">❞</button>
      </div>
      <div class="divider"></div>
      <div class="toolbar-group">
        <button class="toolbar-btn" @click="insertList" title="Bullet List">• List</button>
        <button class="toolbar-btn" @click="insertEnum" title="Numbered List">1. List</button>
      </div>
      <div class="divider"></div>
      <div class="toolbar-group">
        <button class="toolbar-btn" @click="insertLink" title="Link">🔗</button>
        <button class="toolbar-btn" @click="insertImage" title="Image">🖼️</button>
        <button class="toolbar-btn" @click="insertTable" title="Table">📅</button>
        <button class="toolbar-btn" @click="insertEquation" title="Equation">∑</button>
        <button class="toolbar-btn" @click="insertCodeBlock" title="Code Block">Code</button>
      </div>
    </div>
    <textarea
      ref="textareaRef"
      v-model="localContent"
      class="rich-textarea"
      placeholder="Start typing your document..."
      @input="handleInput"
    ></textarea>
    <div class="editor-hint">
      <p>💡 Writing in simplified mode. Use toolbar to insert formatting.</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
  content: string;
}>();

const emit = defineEmits<{
  'content-changed': [content: string];
}>();

const textareaRef = ref<HTMLTextAreaElement | null>(null);
const localContent = ref(props.content);

watch(
  () => props.content,
  (newContent) => {
    if (localContent.value !== newContent) {
      localContent.value = newContent;
    }
  }
);

const handleInput = () => {
  emit('content-changed', localContent.value);
};

const insertAtCursor = (text: string, selectionOffset = 0) => {
  if (!textareaRef.value) return;
  const textarea = textareaRef.value;
  const start = textarea.selectionStart;
  const end = textarea.selectionEnd;
  
  // If there is a selection, wrap it if text contains a placeholder like "text" or "content"
  // For simplicity, we just insert.
  // Advanced: we could wrap selected text.
  
  localContent.value =
    localContent.value.substring(0, start) + text + localContent.value.substring(end);
  
  // Set cursor position
  setTimeout(() => {
    textarea.focus();
    const newPos = start + text.length + selectionOffset;
    textarea.setSelectionRange(newPos, newPos);
  }, 0);
  
  emit('content-changed', localContent.value);
};

const insertH1 = () => insertAtCursor('= ');
const insertH2 = () => insertAtCursor('== ');
const insertH3 = () => insertAtCursor('=== ');

const insertBold = () => insertAtCursor('*bold*', -1);
const insertItalic = () => insertAtCursor('_italic_', -1);
const insertQuote = () => insertAtCursor('> ');

const insertList = () => insertAtCursor('- ');
const insertEnum = () => insertAtCursor('+ ');

const insertLink = () => insertAtCursor('#link("url")[text]', -6);
const insertImage = () => insertAtCursor('#image("path/to/image.png")', -2);

const insertTable = () => {
  insertAtCursor('#table(\n  columns: (1fr, 1fr),\n  [Header 1], [Header 2],\n  [Content 1], [Content 2]\n)');
};

const insertEquation = () => insertAtCursor('$ x = y $', -2);

const insertCodeBlock = () => {
  insertAtCursor('```\n\n```', -4);
};
</script>

<style scoped>
.rich-text-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #1e1e1e;
}

.editor-toolbar {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 8px 12px;
  background: #2d2d30;
  border-bottom: 1px solid #3e3e42;
  flex-wrap: wrap;
}

.toolbar-group {
  display: flex;
  align-items: center;
  gap: 2px;
}

.toolbar-btn {
  padding: 6px 10px;
  background: transparent;
  border: none;
  border-radius: 4px;
  color: #cccccc;
  cursor: pointer;
  font-size: 13px;
  transition: background 0.2s;
  min-width: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.toolbar-btn:hover {
  background: #3e3e42;
}

.divider {
  width: 1px;
  height: 20px;
  background: #3e3e42;
  margin: 0 8px;
}

.rich-textarea {
  flex: 1;
  width: 100%;
  padding: 24px;
  background: #1e1e1e;
  color: #cccccc;
  border: none;
  outline: none;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
  font-size: 15px;
  line-height: 1.6;
  resize: none;
}

.editor-hint {
  padding: 8px 16px;
  background: #252526;
  border-top: 1px solid #3e3e42;
}

.editor-hint p {
  font-size: 11px;
  color: #6a6a6a;
  margin: 0;
}
</style>
