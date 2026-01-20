# Development Plan: Professional & Simple Template-Oriented Editor

## Status: Phase 1 & 2 Completed

## Achievements
- **Unified Template System**: Seamlessly access both standard (global) and custom (workspace) templates via the new `TemplateModal`.
- **Enhanced Simple Editor**: `RichTextEditor` now features a comprehensive toolbar with Headings, Lists, Images, Links, Tables, and Code blocks.
- **Streamlined UX**: "New" button now opens the Template selector. Added "Save as Template" button to easily create custom templates.
- **Workspace Integration**: Sidebar automatically refreshes when new templates are added.

## Future Roadmap

### Phase 3: Advanced Visual Editing (Next)
- [ ] **Live Preview Sync**: sync scroll position between editor and preview.
- [ ] **Form-Based Editing**: For templates with variables (e.g. `title`, `author`), provide a form interface to fill them out without touching code.
- [ ] **Drag & Drop**: Allow dragging images into the editor to insert them.

### Phase 4: Polish & Distribution
- [ ] **Settings Menu**: Configuration for editor font, auto-save interval, etc.
- [ ] **Themes**: Light/Dark mode toggle (currently Dark only).
- [ ] **Cross-Platform Testing**: Ensure Windows/macOS/Linux compatibility.

## Technical Notes
- The `TemplateModal` uses `list_templates` (global) and `list_workspace_templates` (local).
- `App.vue` manages the coordination between modal selection and file creation.
- `RichTextEditor` uses raw string manipulation; consider migrating to a structured editor model (e.g. Prosemirror) if complexity grows.
