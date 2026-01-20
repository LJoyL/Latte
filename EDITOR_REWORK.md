# Editor Rework Plan: Robust & Collaborative

## Goals
1.  **Robust Rich Text Editing**: Replace the simple textarea with a structured editor (**TipTap**).
2.  **Advanced Features**: Support **Comments** and **Review** (Track Changes).
3.  **Synchronized Editing**: Ensure seamless switching between Code (Monaco) and Rich Text (TipTap), and prepare for real-time collaboration (Yjs).

## Architecture

### 1. Rich Text Editor (TipTap)
We will use TipTap (Headless wrapper for ProseMirror) because it provides:
-   **Structured Content**: JSON/HTML output, easier to map to Typst AST than raw text.
-   **Extensions**: Built-in support for Comments (Marks), Highlights, and Collaboration (Yjs).
-   **Vue Integration**: First-class Vue 3 support.

### 2. State Management & Synchronization
To support "Synchronized Edition" (Switching views without data loss):
-   **Master State**: The `fileContent` (String) is the source of truth (Typst Code).
-   **Code -> Rich**: When switching to Rich Mode, we parse the Typst Code into HTML/JSON for TipTap.
-   **Rich -> Code**: When switching to Code Mode (or saving), we serialize the TipTap content back to Typst Code.

*Note: For the MVP, we will implement a "Best-Effort" parser/serializer for: Headings, Lists, Bold, Italic, Images, and Links. Unknown syntax will be preserved as raw text or code blocks.*

### 3. Comments & Review
-   **Comments**: Implemented as `Marks` in TipTap. They will store metadata (author, date, content) in the mark attributes.
-   **UI**: A floating bubble menu or a sidebar panel to add/view comments.
-   **Storage**: In Code Mode, comments will be serialized as Typst comments `/* @comment: ... */` to persist them in the file.

## Implementation Steps

### Phase 1: Foundation
- [ ] Install `@tiptap/vue-3`, `@tiptap/starter-kit`, `@tiptap/extension-placeholder`.
- [ ] Create `RichTextEditor.vue` using TipTap.
- [ ] Implement `useTypstConverter`: A composable to handle Typst <-> HTML conversion.

### Phase 2: Comments & Review
- [ ] Install `@tiptap/extension-highlight` (or custom mark).
- [ ] Create `CommentButton` in the bubble menu.
- [ ] Create `CommentsSidebar.vue` to list comments in the current document.
- [ ] Serialize comments to Typst `/* ... */` syntax.

### Phase 3: Synchronization
- [ ] Connect `App.vue` state to trigger conversion on mode switch.
- [ ] Ensure "Dirty" state is tracked correctly across conversions.

## Future (Phase 4): Real-time Collab
-   Integrate `yjs` and `y-websocket`.
-   Bind Monaco to `y-monaco` and TipTap to `y-prosemirror`.
