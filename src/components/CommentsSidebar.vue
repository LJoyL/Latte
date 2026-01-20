<template>
  <div class="comments-sidebar">
    <div class="header">
      <h3>Comments</h3>
    </div>
    <div class="comments-list">
      <div v-for="comment in comments" :key="comment.id" class="comment-item">
        <div class="comment-header">
          <span class="author">{{ comment.author }}</span>
          <span class="date">{{ formatDate(comment.date) }}</span>
        </div>
        <div class="comment-text">{{ comment.text }}</div>
      </div>
      <div v-if="comments.length === 0" class="empty-state">
        No comments yet. Select text and click 💬 to add one.
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

interface Comment {
  id: string;
  author: string;
  date: Date;
  text: string;
}

const comments = ref<Comment[]>([
  // Mock data for UI demonstration
  {
    id: '1',
    author: 'Reviewer',
    date: new Date(),
    text: 'This section needs more details about the implementation.'
  }
]);

const formatDate = (date: Date) => {
  return date.toLocaleDateString() + ' ' + date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
};
</script>

<style scoped>
.comments-sidebar {
  width: 250px;
  background: #252526;
  border-left: 1px solid #3e3e42;
  display: flex;
  flex-direction: column;
}

.header {
  padding: 12px 16px;
  border-bottom: 1px solid #3e3e42;
}

.header h3 {
  font-size: 14px;
  font-weight: 600;
  color: #cccccc;
}

.comments-list {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.comment-item {
  background: #2d2d30;
  border: 1px solid #3e3e42;
  border-radius: 6px;
  padding: 12px;
}

.comment-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 6px;
  font-size: 11px;
}

.author {
  font-weight: 600;
  color: #007acc;
}

.date {
  color: #858585;
}

.comment-text {
  font-size: 13px;
  color: #cccccc;
  line-height: 1.4;
}

.empty-state {
  text-align: center;
  color: #858585;
  font-style: italic;
  font-size: 12px;
  margin-top: 20px;
}
</style>
