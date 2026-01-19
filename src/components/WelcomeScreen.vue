<template>
  <div class="welcome-screen">
    <div class="welcome-content">
      <div class="logo">
        <h1>☕ Latte</h1>
        <p class="tagline">Typst Editor</p>
      </div>

      <div class="actions">
        <button class="action-btn primary" @click="$emit('new-project')">
          <svg width="20" height="20" viewBox="0 0 16 16" fill="currentColor">
            <path d="M8 2a.5.5 0 0 1 .5.5v5h5a.5.5 0 0 1 0 1h-5v5a.5.5 0 0 1-1 0v-5h-5a.5.5 0 0 1 0-1h5v-5A.5.5 0 0 1 8 2Z" />
          </svg>
          New Project
        </button>
        <button class="action-btn" @click="$emit('open-file')">
          <svg width="20" height="20" viewBox="0 0 16 16" fill="currentColor">
            <path d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4z" />
          </svg>
          Open File
        </button>
      </div>

      <div class="features">
        <div class="feature">
          <h3>📝 Rich Editor</h3>
          <p>Monaco Editor with Typst syntax highlighting</p>
        </div>
        <div class="feature">
          <h3>⚡ Live Preview</h3>
          <p>Real-time PDF preview as you type</p>
        </div>
        <div class="feature">
          <h3>🔍 Diagnostics</h3>
          <p>Instant error and warning detection</p>
        </div>
        <div class="feature">
          <h3>📚 Templates</h3>
          <p>Start quickly with pre-built templates</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineEmits<{
  'new-project': [];
  'open-file': [];
}>();
</script>

<style scoped>
.welcome-screen {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #1e1e1e;
}

.welcome-content {
  max-width: 600px;
  text-align: center;
  padding: 48px;
}

.logo h1 {
  font-size: 48px;
  margin-bottom: 8px;
  color: #cccccc;
}

.tagline {
  font-size: 18px;
  color: #858585;
  margin-bottom: 48px;
}

.actions {
  display: flex;
  gap: 16px;
  justify-content: center;
  margin-bottom: 64px;
}

.action-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  background: #3e3e42;
  border: none;
  border-radius: 6px;
  color: #cccccc;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s;
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

.features {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 24px;
  text-align: left;
}

.feature h3 {
  font-size: 16px;
  margin-bottom: 8px;
  color: #cccccc;
}

.feature p {
  font-size: 13px;
  color: #858585;
  line-height: 1.5;
}
</style>
