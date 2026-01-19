use std::fs;
use std::path::PathBuf;

/// Read a file from the filesystem
#[tauri::command]
pub async fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file {}: {}", path, e))
}

/// Write a file to the filesystem
#[tauri::command]
pub async fn write_file(path: String, contents: String) -> Result<(), String> {
    // Create parent directories if they don't exist
    if let Some(parent) = PathBuf::from(&path).parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    fs::write(&path, contents)
        .map_err(|e| format!("Failed to write file {}: {}", path, e))?;
    Ok(())
}

/// Check if a file exists
#[tauri::command]
pub async fn file_exists(path: String) -> Result<bool, String> {
    Ok(PathBuf::from(&path).exists())
}

/// Create a directory
#[tauri::command]
pub async fn create_directory(path: String) -> Result<(), String> {
    fs::create_dir_all(&path)
        .map_err(|e| format!("Failed to create directory {}: {}", path, e))?;
    Ok(())
}

/// List files in a directory
#[tauri::command]
pub async fn list_directory(path: String) -> Result<Vec<String>, String> {
    let entries = fs::read_dir(&path)
        .map_err(|e| format!("Failed to read directory {}: {}", path, e))?;

    let mut files = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        if let Some(name) = entry.path().to_str() {
            files.push(name.to_string());
        }
    }

    Ok(files)
}

/// Delete a file
#[tauri::command]
pub async fn delete_file(path: String) -> Result<(), String> {
    fs::remove_file(&path)
        .map_err(|e| format!("Failed to delete file {}: {}", path, e))?;
    Ok(())
}
