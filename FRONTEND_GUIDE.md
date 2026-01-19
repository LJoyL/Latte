# Frontend Implementation Guide

## Overview

The Latte frontend is built with Vue 3, TypeScript, and Monaco Editor, providing a full-featured Typst editing environment.

## Architecture

### Component Structure

```
src/
├── App.vue                 # Main application component
├── main.ts                 # Application entry point
├── style.css              # Global styles
└── components/
    ├── Toolbar.vue        # Top toolbar with actions
    ├── FileExplorer.vue   # Sidebar file browser
    ├── MonacoEditor.vue   # Monaco Editor wrapper
    ├── PreviewPanel.vue   # PDF preview viewer
    ├── DiagnosticsPanel.vue # Errors and warnings panel
    ├── WelcomeScreen.vue  # Welcome/empty state
    └── TemplateModal.vue   # Template selection modal
```

## Features

### 1. Monaco Editor Integration
- Typst syntax highlighting
- Real-time diagnostics
- Code completion (future)
- Multi-cursor editing
- Find & replace

### 2. File Management
- Open Typst files
- Save files
- File explorer sidebar
- Delete files

### 3. Compilation
- Compile to PDF
- Compile to HTML
- Real-time preview
- Error highlighting

### 4. Diagnostics
- Live error detection
- Warning display
- Click-to-navigate errors
- Inline markers

### 5. Templates
- Article template
- Letter template
- CV template
- Report template

## Backend Integration

All backend commands are called via Tauri's `invoke` API:

```typescript
import { invoke } from '@tauri-apps/api/core';

// Compile to PDF
const pdfBytes = await invoke<number[]>('compile_typst_to_pdf', {
  entry_path: filePath,
});

// Read file
const content = await invoke<string>('read_file', {
  path: filePath,
});

// Get diagnostics
const diagnostics = await invoke<any[]>('get_typst_diagnostics', {
  entry_path: filePath,
});
```

## Styling

The application uses a dark theme inspired by VS Code:
- Background: `#1e1e1e`
- Sidebar: `#252526`
- Toolbar: `#2d2d30`
- Borders: `#3e3e42`
- Text: `#cccccc`
- Accent: `#007acc`

## Development

1. Install dependencies:
   ```bash
   yarn install
   ```

2. Run development server:
   ```bash
   yarn tauri dev
   ```

3. Build for production:
   ```bash
   yarn tauri build
   ```

## Future Enhancements

- [ ] Auto-save functionality
- [ ] File tabs for multiple files
- [ ] Split editor view
- [ ] Code snippets
- [ ] Theme customization
- [ ] Keyboard shortcuts
- [ ] Export options menu
- [ ] Project settings
- [ ] Git integration
