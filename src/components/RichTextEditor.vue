<template>
  <div class="rich-text-editor">
    <div v-if="editor" class="editor-toolbar">
      <div class="toolbar-group">
        <button
          class="toolbar-btn"
          :class="{ 'is-active': editor.isActive('heading', { level: 1 }) }"
          @click="editor.chain().focus().toggleHeading({ level: 1 }).run()"
        >
          H1
        </button>
        <button
          class="toolbar-btn"
          :class="{ 'is-active': editor.isActive('heading', { level: 2 }) }"
          @click="editor.chain().focus().toggleHeading({ level: 2 }).run()"
        >
          H2
        </button>
        <button
          class="toolbar-btn"
          :class="{ 'is-active': editor.isActive('heading', { level: 3 }) }"
          @click="editor.chain().focus().toggleHeading({ level: 3 }).run()"
        >
          H3
        </button>
      </div>
      <div class="divider"></div>
      <div class="toolbar-group">
        <button
          class="toolbar-btn"
          :class="{ 'is-active': editor.isActive('bold') }"
          @click="editor.chain().focus().toggleBold().run()"
        >
          B
        </button>
        <button
          class="toolbar-btn"
          :class="{ 'is-active': editor.isActive('italic') }"
          @click="editor.chain().focus().toggleItalic().run()"
        >
          I
        </button>
        <button
          class="toolbar-btn"
          :class="{ 'is-active': editor.isActive('blockquote') }"
          @click="editor.chain().focus().toggleBlockquote().run()"
        >
          ❞
        </button>
      </div>
      <div class="divider"></div>
      <div class="toolbar-group">
        <button
          class="toolbar-btn"
          :class="{ 'is-active': editor.isActive('bulletList') }"
          @click="editor.chain().focus().toggleBulletList().run()"
        >
          • List
        </button>
        <button
          class="toolbar-btn"
          :class="{ 'is-active': editor.isActive('orderedList') }"
          @click="editor.chain().focus().toggleOrderedList().run()"
        >
          1. List
        </button>
      </div>
      <div class="divider"></div>
      <div class="toolbar-group">
        <button class="toolbar-btn" @click="addComment" title="Add Comment">
          💬
        </button>
      </div>
    </div>
    
    <editor-content :editor="editor" class="editor-content" />
    
    <div class="editor-hint">
      <p>💡 Rich Text Mode (TipTap). Edits are converted to Typst.</p>
    </div>

    <!-- Floating Comment Menu (Bubble Menu) -->
    <bubble-menu
      v-if="editor"
      :editor="editor"
      :tippy-options="{ duration: 100 }"
      class="bubble-menu"
    >
      <button
        class="bubble-btn"
        @click="editor.chain().focus().toggleBold().run()"
        :class="{ 'is-active': editor.isActive('bold') }"
      >
        B
      </button>
      <button
        class="bubble-btn"
        @click="editor.chain().focus().toggleItalic().run()"
        :class="{ 'is-active': editor.isActive('italic') }"
      >
        I
      </button>
      <button class="bubble-btn" @click="addComment">
        💬
      </button>
    </bubble-menu>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onBeforeUnmount, onMounted } from 'vue';
import { Editor, EditorContent, BubbleMenu } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import Collaboration from '@tiptap/extension-collaboration';
import CollaborationCursor from '@tiptap/extension-collaboration-cursor';
import { CollaborationManager } from '../utils/collaboration';

// ... other imports

onMounted(() => {
  const collabManager = CollaborationManager.getInstance();
  
  editor.value = new Editor({
    extensions: [
      StarterKit.configure({
        // Disable history because Collaboration handles it
        history: false,
      }),
      Collaboration.configure({
        document: collabManager.ydoc,
        field: 'document', // Share RichText content in 'document' field
      }),
      CollaborationCursor.configure({
        provider: collabManager.provider,
        user: collabManager.currentUser,
      }),
      Placeholder.configure({
        placeholder: 'Start writing...',
      }),
      Highlight.configure({ multicolor: true }), 
    ],
    // ... rest of config
    editorProps: {
      attributes: {
        class: 'prose prose-sm sm:prose lg:prose-lg xl:prose-2xl mx-auto focus:outline-none',
      },
    },
    onUpdate: ({ editor }) => {
      isUpdating.value = true;
      const html = editor.getHTML();
      const typst = TypstConverter.fromHtml(html);
      emit('content-changed', typst);
      setTimeout(() => { isUpdating.value = false; }, 0);
    },
// Watch awareness changes to update cursor colors if user changes them elsewhere
  watch(() => collabManager.currentUser, (newUser) => {
    editor.value?.commands.updateUser(newUser);
  }, { deep: true });
});

watch(
  () => props.content,
  (newContent) => {
    if (!editor.value || isUpdating.value) return;
    
    // Check if content is actually different to avoid cursor jumps
    // This is tricky because round-trip conversion isn't perfect
    // For now, we only update if the diff is significant or if completely replaced
    
    // Simple heuristic: if we are not typing (isUpdating is false), assume it's an external change (file load, tab switch)
    // Convert new Typst to HTML
    const newHtml = TypstConverter.toHtml(newContent);
    // Only set if different from current editor HTML
    if (editor.value.getHTML() !== newHtml) {
        editor.value.commands.setContent(newHtml);
    }
  }
);

onBeforeUnmount(() => {
  editor.value?.destroy();
});

const addComment = () => {
  // For MVP, we use Highlight to simulate a comment
  // In a real implementation, we'd add a mark with attributes { id, author, text }
  editor.value?.chain().focus().toggleHighlight().run();
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
  font-weight: 500;
}

.toolbar-btn:hover {
  background: #3e3e42;
}

.toolbar-btn.is-active {
  background: #007acc;
  color: white;
}

.divider {
  width: 1px;
  height: 20px;
  background: #3e3e42;
  margin: 0 8px;
}

.editor-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
  color: #cccccc;
}

/* TipTap Specific Styles */
:deep(.ProseMirror) {
  outline: none;
  min-height: 100px;
}

:deep(.ProseMirror p) {
  margin-bottom: 0.8em;
  line-height: 1.6;
}

:deep(.ProseMirror h1) {
  font-size: 2em;
  font-weight: bold;
  margin-top: 0.5em;
  margin-bottom: 0.5em;
}

:deep(.ProseMirror h2) {
  font-size: 1.5em;
  font-weight: bold;
  margin-top: 0.5em;
  margin-bottom: 0.5em;
}

:deep(.ProseMirror h3) {
  font-size: 1.25em;
  font-weight: bold;
  margin-top: 0.5em;
  margin-bottom: 0.5em;
}

:deep(.ProseMirror ul) {
  list-style-type: disc;
  padding-left: 1.5em;
  margin-bottom: 0.8em;
}

:deep(.ProseMirror ol) {
  list-style-type: decimal;
  padding-left: 1.5em;
  margin-bottom: 0.8em;
}

:deep(.ProseMirror blockquote) {
  border-left: 3px solid #007acc;
  padding-left: 1em;
  margin-left: 0;
  font-style: italic;
  color: #a0a0a0;
}

/* Highlight (Comment) Style */
:deep(mark) {
  background-color: #5d4d00;
  color: inherit;
  border-bottom: 2px solid #cca700;
  cursor: pointer;
}

.bubble-menu {
  display: flex;
  background-color: #252526;
  padding: 4px;
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
  border: 1px solid #3e3e42;
}

.bubble-btn {
  background: transparent;
  border: none;
  color: #cccccc;
  padding: 4px 8px;
  cursor: pointer;
  border-radius: 4px;
}

.bubble-btn:hover {
  background: #3e3e42;
}

.bubble-btn.is-active {
  background: #007acc;
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
