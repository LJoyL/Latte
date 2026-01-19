# Development Plan: VS Code Style Interface

## Goals
1.  **VS Code-like Layout**: Implement the standard 3-panel layout + activity bar + status bar.
    *   Activity Bar (Leftmost)
    *   Sidebar (Explorer, etc.)
    *   Editor Area (with Tabs)
    *   Status Bar (Bottom)
2.  **Tabbed Editing**: Allow multiple files to be open simultaneously.
3.  **Workspace Settings**: UI to manage `.latte` configuration.

## Implementation Steps

### Phase 1: Core Layout Components
- [ ] **ActivityBar.vue**: Vertical strip with icons for "Explorer", "Search", "Settings". Toggles the Sidebar.
- [ ] **StatusBar.vue**: Bottom bar showing current file info (Line/Col, Encoding, Language) and global notifications/status.
- [ ] **EditorTabs.vue**: Bar above the editor showing open files. Handles switching and closing.

### Phase 2: App Refactoring
- [ ] **State Management**: Move from `currentFile` to `openFiles` (array) and `activeFileId`.
- [ ] **Layout**: Update `App.vue` CSS Grid/Flexbox to match VS Code structure:
  ```
  [Activity Bar] [Sidebar] [       Editor Area       ] [Preview]
                           [ Tabs                    ]
                           [ Content                 ]
  [                      Status Bar                            ]
  ```

### Phase 3: Workspace Polish
- [ ] **Folder Open**: Ensure "Open Folder" behavior is native-like.
- [ ] **Settings UI**: Click "Settings" in Activity Bar to open Workspace Settings (modifying `.latte/workspace.json`).

## Technical Details
- **Tabs**: Will store path, dirty state (unsaved changes), and potentially scroll position.
- **Icons**: Use SVG icons for the Activity Bar.
