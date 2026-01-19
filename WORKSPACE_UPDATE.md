# Workspace System Update

## Changes Made

### 1. Workspace Selection via Folder Selection
- **Before**: Created workspace structure with documents/templates/resources folders
- **Now**: Select any folder as workspace, settings stored in `.latte` folder

### 2. Workspace Structure

**New Structure**:
```
your-folder/
├── .latte/                    # Hidden folder for Latte settings
│   ├── workspace.json        # Workspace configuration
│   ├── templates/            # Workspace templates
│   │   └── template-name.typ
│   └── resources/           # Workspace resources
│       └── images/
├── document1.typ              # Your Typst files anywhere
├── document2.typ
└── subfolder/
    └── document3.typ
```

### 3. Key Features

**Folder Selection**
- Click "Workspace" button → "Select Workspace Folder"
- Choose any folder on your system
- `.latte` folder is created automatically if it doesn't exist
- Workspace config stored in `.latte/workspace.json`

**Document Discovery**
- Recursively finds all `.typ` files in workspace
- Skips `.latte` folder (hidden settings)
- Documents can be anywhere in the workspace

**Templates**
- Stored in `.latte/templates/`
- Click template to create new document from it
- New document created in workspace root

**Resources**
- Stored in `.latte/resources/`
- Can include images, fonts, etc.
- Accessible from Typst documents

### 4. Backend Changes

**New Commands**:
- `init_workspace(workspace_path)` - Initialize workspace (creates .latte folder)
- `open_workspace(workspace_path)` - Open workspace (auto-initializes if needed)
- `list_workspace_documents(workspace_path)` - Recursive search for .typ files
- `list_workspace_templates(workspace_path)` - List templates from .latte/templates
- `list_workspace_resources(workspace_path)` - List resources from .latte/resources
- `update_workspace_settings(...)` - Update workspace configuration

**Removed**:
- `create_workspace` - Replaced with `init_workspace`

### 5. Frontend Changes

**Workspace Selector**:
- Simplified to single "Select Workspace Folder" button
- No separate create/open options
- Auto-initializes workspace if `.latte` folder doesn't exist

**Workspace Sidebar**:
- Shows all documents found recursively
- Shows templates from `.latte/templates`
- Shows resources from `.latte/resources`
- Click documents to open
- Click templates to create new document

## Usage

1. **Select Workspace**
   - Click "Workspace" button
   - Choose folder
   - `.latte` folder created automatically

2. **Work with Files**
   - All `.typ` files in workspace appear in sidebar
   - Files can be anywhere in workspace
   - `.latte` folder is hidden from document list

3. **Use Templates**
   - Templates stored in `.latte/templates/`
   - Click template to create new document
   - New document created in workspace root

4. **Add Resources**
   - Place files in `.latte/resources/`
   - Access from Typst documents using relative paths

## Benefits

- ✅ Works with existing folder structures
- ✅ No forced folder organization
- ✅ Settings hidden in `.latte` folder
- ✅ Recursive document discovery
- ✅ Flexible workspace structure
- ✅ Easy to version control (ignore `.latte` folder)
