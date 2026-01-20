# Fix Report: Frontend-Backend Linkage

## Issues Identified & Fixed

### 1. Missing Dialog Permissions
**Symptoms:** File dialogs ("New", "Open", "Save") would fail silently or throw errors in the console because the application lacked permission to spawn system dialogs.
**Fix:** Added `"dialog:default"` to `src-tauri/capabilities/default.json`. This enables the `open` and `save` functions from `@tauri-apps/plugin-dialog`.

### 2. Broken In-Memory Compilation
**Symptoms:** "Untitled" documents (not yet saved to disk) would fail to compile to PDF.
**Cause:** The `compile_typst_string_to_pdf` function in the backend created a temporary file but deleted it (`drop(temp_file)`) *before* the compilation process (`compile_typst_to_pdf`) could read it.
**Fix:** Updated `src-tauri/src/typst.rs` to ensure the temporary file is kept alive until the compilation process completes.

## Verification
- **File Operations:** The `App.vue` uses `invoke` to call custom Rust commands (`read_file`, `write_file`) for file I/O, which were already correctly implemented and did not require additional permissions.
- **Command Signatures:** Verified that all `invoke` calls in `App.vue` and `TemplateModal.vue` match the argument names and types defined in `lib.rs` and its modules.

The application should now correctly handle file dialogs and compile unsaved documents.
