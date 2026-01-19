use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use typst::diag::{FileError, SourceDiagnostic};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::World;
use typst_utils::LazyHash;
use typst_pdf::{pdf, PdfOptions};
use typst_library::layout::PagedDocument;
use typst::LibraryExt;

/// A filesystem-backed Typst world implementation
pub struct FileWorld {
    root: PathBuf,
    main: FileId,
    id_to_path: HashMap<FileId, PathBuf>,
    path_to_id: HashMap<PathBuf, FileId>,
    sources: HashMap<FileId, Source>,
    next_id: u64,
    library: LazyHash<typst_library::Library>,
    book: LazyHash<FontBook>,
}

impl FileWorld {
    /// Create a new FileWorld with the given root directory and main file
    pub fn new(root: PathBuf, entry: PathBuf) -> Result<Self, String> {
        let entry_abs = if entry.is_absolute() {
            entry.clone()
        } else {
            root.join(&entry)
        };

        let entry_abs = entry_abs
            .canonicalize()
            .map_err(|e| format!("failed to canonicalize {}: {}", entry_abs.display(), e))?;

        let root = entry_abs
            .parent()
            .ok_or_else(|| format!("entry file has no parent: {}", entry_abs.display()))?
            .to_path_buf();

        // Create virtual path for the main file
        let main_path = VirtualPath::new(entry_abs.strip_prefix(&root).unwrap_or(&entry_abs));
        let main = FileId::new(None, main_path.clone());

        let mut id_to_path = HashMap::new();
        let mut path_to_id = HashMap::new();
        let mut sources = HashMap::new();

        let text = std::fs::read_to_string(&entry_abs)
            .map_err(|e| format!("failed to read {}: {}", entry_abs.display(), e))?;
        sources.insert(main, Source::new(main, text));
        id_to_path.insert(main, entry_abs.clone());
        path_to_id.insert(entry_abs, main);

        // Build font book from typst-assets
        let mut fonts = Vec::new();
        for data in typst_assets::fonts() {
            if let Some(font) = Font::new(Bytes::new(data), 0) {
                fonts.push(font);
            }
        }
        let book = FontBook::from_fonts(&fonts);

        // Get library - use the one from typst-library
        let library = typst_library::Library::default();

        Ok(Self {
            root,
            main,
            id_to_path,
            path_to_id,
            sources,
            next_id: 1,
            library: LazyHash::new(library),
            book: LazyHash::new(book),
        })
    }

    /// Resolve a virtual path to a file ID, loading it if necessary
    fn resolve_file(&mut self, path: &VirtualPath) -> Result<FileId, FileError> {
        // Try to resolve relative to root
        let resolved = self.root.join(path.as_rooted_path());
        let canonical = resolved
            .canonicalize()
            .map_err(|_| FileError::NotFound(path.as_rooted_path().to_path_buf()))?;

        // Check if we already have this file
        if let Some(&id) = self.path_to_id.get(&canonical) {
            return Ok(id);
        }

        // Load the file
        let text = std::fs::read_to_string(&canonical)
            .map_err(|e| FileError::from_io(e, &canonical))?;

        let virtual_path = VirtualPath::new(
            canonical
                .strip_prefix(&self.root)
                .unwrap_or(&canonical)
                .to_path_buf(),
        );
        let id = FileId::new(None, virtual_path);

        self.sources.insert(id, Source::new(id, text));
        self.id_to_path.insert(id, canonical.clone());
        self.path_to_id.insert(canonical, id);

        Ok(id)
    }

    fn try_path_of(&self, id: FileId) -> Option<&Path> {
        self.id_to_path.get(&id).map(|p| p.as_path())
    }
}

impl World for FileWorld {
    fn library(&self) -> &LazyHash<typst_library::Library> {
        &self.library
    }

    fn main(&self) -> FileId {
        self.main
    }

    fn source(&self, id: FileId) -> Result<Source, FileError> {
        self.sources
            .get(&id)
            .cloned()
            .ok_or_else(|| FileError::NotFound(PathBuf::new()))
    }

    fn file(&self, id: FileId) -> Result<Bytes, FileError> {
        let path = self
            .try_path_of(id)
            .ok_or_else(|| FileError::NotFound(PathBuf::new()))?;
        let bytes = std::fs::read(path).map_err(|e| FileError::from_io(e, path))?;
        Ok(Bytes::new(bytes))
    }

    fn font(&self, index: usize) -> Option<Font> {
        // Get font from typst-assets
        let fonts: Vec<_> = typst_assets::fonts().collect();
        if index < fonts.len() {
            Font::new(Bytes::new(fonts[index]), 0)
        } else {
            None
        }
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        None
    }
}

/// Helper struct that wraps FileWorld and handles dynamic file resolution
struct ResolvingWorld {
    world: RwLock<FileWorld>,
    library: Arc<LazyHash<typst_library::Library>>,
    book: Arc<LazyHash<FontBook>>,
}

impl ResolvingWorld {
    fn new(root: PathBuf, entry: PathBuf) -> Result<Self, String> {
        let file_world = FileWorld::new(root, entry)?;
        let library = Arc::new(file_world.library.clone());
        let book = Arc::new(file_world.book.clone());
        
        Ok(Self {
            world: RwLock::new(file_world),
            library,
            book,
        })
    }
}

impl World for ResolvingWorld {
    fn library(&self) -> &LazyHash<typst_library::Library> {
        &self.library
    }

    fn main(&self) -> FileId {
        self.world.read().unwrap().main()
    }

    fn source(&self, id: FileId) -> Result<Source, FileError> {
        // Try to get source from existing sources
        {
            let world = self.world.read().unwrap();
            if let Ok(source) = world.source(id) {
                return Ok(source);
            }
        }

        // If not found, try to resolve and load the file
        let path = {
            let world = self.world.read().unwrap();
            world
                .try_path_of(id)
                .and_then(|p| p.strip_prefix(&world.root).ok())
                .map(|p| VirtualPath::new(p.to_path_buf()))
        };

        if let Some(virtual_path) = path {
            let mut world = self.world.write().unwrap();
            if let Ok(new_id) = world.resolve_file(&virtual_path) {
                return world.source(new_id);
            }
        }

        Err(FileError::NotFound(PathBuf::new()))
    }

    fn file(&self, id: FileId) -> Result<Bytes, FileError> {
        self.world.read().unwrap().file(id)
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.world.read().unwrap().font(index)
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        self.world.read().unwrap().today(offset)
    }
}

/// Compile Typst source to PDF
#[tauri::command]
pub async fn compile_typst_to_pdf(entry_path: String) -> Result<Vec<u8>, String> {
    let entry = PathBuf::from(&entry_path);
    let root = entry
        .parent()
        .ok_or_else(|| format!("entry file has no parent: {}", entry_path))?
        .to_path_buf();

    let world = ResolvingWorld::new(root, entry)?;

    // Compile to PagedDocument
    let result = typst::compile::<PagedDocument>(&world);

    match result.output {
        Ok(document) => {
            // Generate PDF
            let pdf_bytes = pdf(&document, &PdfOptions::default())
                .map_err(|e| format!("PDF generation error: {:?}", e))?;
            Ok(pdf_bytes)
        }
        Err(errors) => {
            let error_msg = format!(
                "Compilation errors: {}",
                errors
                    .iter()
                    .map(|e| e.message.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            Err(error_msg)
        }
    }
}

/// Compile Typst source to HTML
#[tauri::command]
pub async fn compile_typst_to_html(entry_path: String) -> Result<String, String> {
    let entry = PathBuf::from(&entry_path);
    let root = entry
        .parent()
        .ok_or_else(|| format!("entry file has no parent: {}", entry_path))?
        .to_path_buf();

    let world = ResolvingWorld::new(root, entry)?;

    // Compile to HtmlDocument
    let result = typst::compile::<typst_html::HtmlDocument>(&world);

    match result.output {
        Ok(_document) => {
            // Generate HTML - HtmlDocument structure in typst-html 0.14.0
            // The HtmlDocument contains a DOM structure that needs to be serialized
            // For a complete implementation, we would traverse the DOM tree
            // For now, return a basic HTML structure indicating successful compilation
            let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Typst Document</title>
    <style>
        body {
            font-family: 'Linux Libertine', serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 2em;
            line-height: 1.6;
        }
        .typst-document {
            background: white;
        }
    </style>
</head>
<body>
    <div class="typst-document">
        <p>Document compiled successfully. Full HTML export coming soon.</p>
        <p>For now, please use PDF export for complete document rendering.</p>
    </div>
</body>
</html>"#;
            Ok(html.to_string())
        }
        Err(errors) => {
            let error_msg = format!(
                "Compilation errors: {}",
                errors
                    .iter()
                    .map(|e| e.message.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            Err(error_msg)
        }
    }
}

/// Get compilation diagnostics (errors and warnings)
#[tauri::command]
pub async fn get_typst_diagnostics(
    entry_path: String,
) -> Result<Vec<DiagnosticInfo>, String> {
    let entry = PathBuf::from(&entry_path);
    let root = entry
        .parent()
        .ok_or_else(|| format!("entry file has no parent: {}", entry_path))?
        .to_path_buf();

    let world = ResolvingWorld::new(root, entry)?;

    // Compile with diagnostics
    let result = typst::compile::<PagedDocument>(&world);

    let mut diagnostics: Vec<DiagnosticInfo> = result
        .warnings
        .iter()
        .map(|w| DiagnosticInfo::from_diagnostic(w, &world))
        .collect();

    // If there are errors, add them too
    if let Err(errors) = &result.output {
        let error_diagnostics: Vec<DiagnosticInfo> = errors
            .iter()
            .map(|e| DiagnosticInfo::from_diagnostic(e, &world))
            .collect();
        diagnostics.extend(error_diagnostics);
    }

    Ok(diagnostics)
}

/// Compile Typst from source string (in-memory compilation)
#[tauri::command]
pub async fn compile_typst_string_to_pdf(
    source: String,
    root_dir: Option<String>,
) -> Result<Vec<u8>, String> {
    use std::io::Write;
    use tempfile::NamedTempFile;

    // Create a temporary file for the source
    let root = root_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    let mut temp_file = NamedTempFile::new_in(&root)
        .map_err(|e| format!("failed to create temp file: {}", e))?;
    temp_file
        .write_all(source.as_bytes())
        .map_err(|e| format!("failed to write temp file: {}", e))?;

    let entry_path = temp_file.path().to_path_buf();
    drop(temp_file); // Close the file

    compile_typst_to_pdf(entry_path.to_string_lossy().to_string()).await
}

/// Diagnostic information for frontend display
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct DiagnosticInfo {
    pub severity: String,
    pub message: String,
    pub span: Option<SpanInfo>,
    pub hints: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SpanInfo {
    pub start: usize,
    pub end: usize,
    pub file_id: u64,
}

impl DiagnosticInfo {
    fn from_diagnostic(diag: &SourceDiagnostic, _world: &ResolvingWorld) -> Self {
        let severity = match diag.severity {
            typst::diag::Severity::Error => "error",
            typst::diag::Severity::Warning => "warning",
        }
        .to_string();

        let message = diag.message.to_string();
        let hints = diag.hints.iter().map(|h| h.to_string()).collect();

        // Span handling - Span is opaque, we'll extract what we can
        // For now, we'll skip detailed span info and just indicate if there's a span
        let span = None; // TODO: Implement proper span resolution when needed

        Self {
            severity,
            message,
            span,
            hints,
        }
    }
}
