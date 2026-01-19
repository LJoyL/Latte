<template>
  <div class="rich-text-editor">
    <div class="editor-toolbar">
      <button class="toolbar-btn" @click="insertHeading" title="Heading">H</button>
      <button class="toolbar-btn" @click="insertBold" title="Bold">B</button>
      <button class="toolbar-btn" @click="insertItalic" title="Italic">I</button>
      <div class="divider"></div>
      <button class="toolbar-btn" @click="insertList" title="List">• List</button>
      <button class="toolbar-btn" @click="insertEquation" title="Equation">∑</button>
    </div>
    <textarea
      ref="textareaRef"
      v-model="localContent"
      class="rich-textarea"
      placeholder="Start typing your Typst document..."
      @input="handleInput"
    ></textarea>
    <div class="editor-hint">
      <p>💡 Rich text editor is a simplified view. Switch to Code mode for full Typst features.</p>
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

const insertAtCursor = (text: string) => {
  if (!textareaRef.value) return;
  const textarea = textareaRef.value;
  const start = textarea.selectionStart;
  const end = textarea.selectionEnd;
  const selectedText = localContent.value.substring(start, end);
  
  localContent.value =
    localContent.value.substring(0, start) + text + selectedText + localContent.value.substring(end);
  
  // Set cursor position
  setTimeout(() => {
    textarea.focus();
    textarea.setSelectionRange(start + text.length, start + text.length);
  }, 0);
  
  emit('content-changed', localContent.value);
};

const insertHeading = () => {
  insertAtCursor('= ');
};

const insertBold = () => {
  insertAtCursor('*bold text*');
};

const insertItalic = () => {
  insertAtCursor('_italic text_');
};

const insertList = () => {
  insertAtCursor('- List item\n');
};

const insertEquation = () => {
  insertAtCursor('$ x = y + z $');
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
}

.toolbar-btn {
  padding: 6px 12px;
  background: #3e3e42;
  border: none;
  border-radius: 4px;
  color: #cccccc;
  cursor: pointer;
  font-size: 13px;
  transition: background 0.2s;
}

.toolbar-btn:hover {
  background: #505050;
}

.divider {
  width: 1px;
  height: 20px;
  background: #3e3e42;
  margin: 0 4px;
}

.rich-textarea {
  flex: 1;
  width: 100%;
  padding: 16px;
  background: #1e1e1e;
  color: #cccccc;
  border: none;
  outline: none;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.6;
  resize: none;
}

.editor-hint {
  padding: 12px 16px;
  background: #2d2d30;
  border-top: 1px solid #3e3e42;
}

.editor-hint p {
  font-size: 12px;
  color: #858585;
  margin: 0;
}
</style>
