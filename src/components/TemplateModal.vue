<template>
  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h2>Choose a Template</h2>
        <button class="close-btn" @click="$emit('close')">×</button>
      </div>
      <div class="templates-grid">
        <div
          v-for="template in templates"
          :key="template"
          class="template-card"
          @click="selectTemplate(template)"
        >
          <div class="template-icon">{{ getTemplateIcon(template) }}</div>
          <h3>{{ getTemplateName(template) }}</h3>
          <p>{{ getTemplateDescription(template) }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const emit = defineEmits<{
  close: [];
  'template-selected': [name: string];
}>();

const templates = ref<string[]>([]);

onMounted(async () => {
  try {
    templates.value = await invoke<string[]>('list_templates');
  } catch (error) {
    console.error('Failed to load templates:', error);
  }
});

const selectTemplate = (template: string) => {
  emit('template-selected', template);
};

const getTemplateIcon = (template: string): string => {
  const icons: Record<string, string> = {
    article: '📄',
    letter: '✉️',
    cv: '📋',
    report: '📊',
  };
  return icons[template] || '📝';
};

const getTemplateName = (template: string): string => {
  return template.charAt(0).toUpperCase() + template.slice(1);
};

const getTemplateDescription = (template: string): string => {
  const descriptions: Record<string, string> = {
    article: 'Academic paper or article format',
    letter: 'Formal letter template',
    cv: 'Curriculum vitae / Resume',
    report: 'Professional report template',
  };
  return descriptions[template] || 'Document template';
};
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: #252526;
  border-radius: 8px;
  width: 90%;
  max-width: 800px;
  max-height: 80vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px;
  border-bottom: 1px solid #3e3e42;
}

.modal-header h2 {
  font-size: 20px;
  color: #cccccc;
}

.close-btn {
  background: transparent;
  border: none;
  color: #cccccc;
  font-size: 28px;
  cursor: pointer;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: background 0.2s;
}

.close-btn:hover {
  background: #3e3e42;
}

.templates-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 16px;
  padding: 24px;
  overflow-y: auto;
}

.template-card {
  background: #2d2d30;
  border: 1px solid #3e3e42;
  border-radius: 6px;
  padding: 20px;
  cursor: pointer;
  transition: all 0.2s;
  text-align: center;
}

.template-card:hover {
  background: #3e3e42;
  border-color: #007acc;
  transform: translateY(-2px);
}

.template-icon {
  font-size: 48px;
  margin-bottom: 12px;
}

.template-card h3 {
  font-size: 16px;
  color: #cccccc;
  margin-bottom: 8px;
}

.template-card p {
  font-size: 12px;
  color: #858585;
  line-height: 1.4;
}
</style>
