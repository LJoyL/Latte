# Basic Features Implementation

## Overview

The application now focuses on the core basics:
1. ✅ Create new Typst files
2. ✅ Edit with raw code editor (Monaco) or rich text editor
3. ✅ Compile to PDF
4. ✅ Live preview with auto-compile

## Features

### 1. File Management

**New File**
- Click "New" button in toolbar
- Choose save location
- Creates a new `.typ` file with default content
- Can also work with untitled documents

**Open File**
- Click "Open" button
- Select existing `.typ` file
- Loads content into editor

**Save File**
- Click "Save" button
- Saves current content to file
- If untitled, prompts for save location

### 2. Editor Modes

**Code Mode** (Default)
- Full Monaco Editor with Typst syntax highlighting
- Professional code editing experience
- Syntax highlighting for Typst
- Auto-completion ready

**Rich Text Mode**
- Simplified textarea editor
- Toolbar with common formatting buttons
- Quick insert buttons for:
  - Headings (`=`)
  - Bold (`*bold*`)
  - Italic (`_italic_`)
  - Lists (`-`)
  - Equations (`$`)

### 3. Compilation & Preview

**Auto-Compile**
- Toggle "Auto-compile" checkbox
- Automatically compiles PDF 1 second after typing stops
- Updates preview panel automatically

**Manual Compile**
- Click "Compile PDF" button
- Compiles immediately
- Shows loading state

**PDF Preview**
- Right panel shows PDF preview
- Updates automatically when compilation succeeds
- Refresh button to manually update
- Loading indicator during compilation

## UI Layout

```
┌─────────────────────────────────────────────────┐
│ Toolbar: New | Open | Save | Code/Rich | Compile│
├──────────────┬──────────────────┬───────────────┤
│              │                  │               │
│   Editor     │   Editor Panel   │   Preview     │
│   (Code or   │   (Monaco or     │   Panel       │
│   Rich)      │   Rich Text)     │   (PDF)       │
│              │                  │               │
└──────────────┴──────────────────┴───────────────┘
```

## Usage Flow

1. **Start Editing**
   - Click "New" to create a file
   - Or click "Open" to open existing file
   - Start typing in the editor

2. **Edit Content**
   - Switch between Code and Rich modes using toolbar buttons
   - In Rich mode, use toolbar buttons to insert formatting
   - In Code mode, type Typst syntax directly

3. **View Preview**
   - Enable "Auto-compile" for live preview
   - Or click "Compile PDF" manually
   - PDF appears in right panel

4. **Save Work**
   - Click "Save" to save changes
   - File is saved before compilation

## Technical Details

### Backend Commands Used
- `write_file` - Save file
- `read_file` - Load file
- `compile_typst_to_pdf` - Compile file to PDF
- `compile_typst_string_to_pdf` - Compile string to PDF

### Frontend Components
- `App.vue` - Main application layout
- `MonacoEditor.vue` - Code editor component
- `RichTextEditor.vue` - Rich text editor component
- `PreviewPanel.vue` - PDF preview component

### Auto-Compile Logic
- Watches `fileContent` for changes
- Debounces compilation by 1 second
- Only compiles if `autoCompile` is enabled
- Saves file before compiling if file exists

## Next Steps

To run the application:
```bash
yarn tauri dev
```

The application is now ready for basic Typst editing with live preview!
