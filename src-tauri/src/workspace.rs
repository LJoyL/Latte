use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkspaceConfig {
    pub name: String,
    pub root: String,
    pub templates: Vec<String>,
    pub resources: Vec<String>,
}

/// Initialize workspace - creates .latte folder and config if needed
#[tauri::command]
pub async fn init_workspace(workspace_path: String) -> Result<WorkspaceConfig, String> {
    let workspace_root = PathBuf::from(&workspace_path);
    
    if !workspace_root.exists() {
        return Err("Workspace path does not exist".to_string());
    }

    if !workspace_root.is_dir() {
        return Err("Workspace path is not a directory".to_string());
    }

    // Create .latte folder
    let latte_dir = workspace_root.join(".latte");
    fs::create_dir_all(&latte_dir)
        .map_err(|e| format!("Failed to create .latte directory: {}", e))?;

    // Check if config already exists
    let config_path = latte_dir.join("workspace.json");
    let config = if config_path.exists() {
        // Load existing config
        let config_content = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read workspace config: {}", e))?;
        serde_json::from_str::<WorkspaceConfig>(&config_content)
            .map_err(|e| format!("Failed to parse workspace config: {}", e))?
    } else {
        // Create new config
        let workspace_name = workspace_root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Workspace")
            .to_string();

        let config = WorkspaceConfig {
            name: workspace_name.clone(),
            root: workspace_path.clone(),
            templates: Vec::new(),
            resources: Vec::new(),
        };

        // Save config
        let config_json = serde_json::to_string_pretty(&config)
            .map_err(|e| format!("Failed to serialize workspace config: {}", e))?;
        fs::write(&config_path, config_json)
            .map_err(|e| format!("Failed to write workspace config: {}", e))?;

        config
    };

    Ok(config)
}

/// Open an existing workspace (loads config from .latte folder)
#[tauri::command]
pub async fn open_workspace(workspace_path: String) -> Result<WorkspaceConfig, String> {
    let config_path = PathBuf::from(&workspace_path)
        .join(".latte")
        .join("workspace.json");
    
    if !config_path.exists() {
        // Initialize workspace if config doesn't exist
        return init_workspace(workspace_path).await;
    }

    let config_content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read workspace config: {}", e))?;
    
    let mut config: WorkspaceConfig = serde_json::from_str(&config_content)
        .map_err(|e| format!("Failed to parse workspace config: {}", e))?;

    // Update root path in case it changed
    config.root = workspace_path.clone();

    Ok(config)
}

/// Find all Typst files in workspace (recursive)
#[tauri::command]
pub async fn list_workspace_documents(workspace_path: String) -> Result<Vec<String>, String> {
    let workspace_root = PathBuf::from(&workspace_path);
    let latte_dir = workspace_root.join(".latte");
    
    let mut documents = Vec::new();
    find_typst_files(&workspace_root, &latte_dir, &mut documents)?;

    Ok(documents)
}

fn find_typst_files(
    dir: &Path,
    exclude_dir: &Path,
    results: &mut Vec<String>,
) -> Result<(), String> {
    if dir == exclude_dir {
        return Ok(()); // Skip .latte directory
    }

    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            find_typst_files(&path, exclude_dir, results)?;
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "typ" {
                    if let Some(path_str) = path.to_str() {
                        results.push(path_str.to_string());
                    }
                }
            }
        }
    }

    Ok(())
}

/// List templates in workspace (.latte/templates folder)
#[tauri::command]
pub async fn list_workspace_templates(workspace_path: String) -> Result<Vec<String>, String> {
    let templates_dir = PathBuf::from(&workspace_path)
        .join(".latte")
        .join("templates");
    
    if !templates_dir.exists() {
        return Ok(Vec::new());
    }

    let entries = fs::read_dir(&templates_dir)
        .map_err(|e| format!("Failed to read templates directory: {}", e))?;

    let mut templates = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("typ") {
            if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                templates.push(name.to_string());
            }
        }
    }

    Ok(templates)
}

/// List resources in workspace (.latte/resources folder)
#[tauri::command]
pub async fn list_workspace_resources(workspace_path: String) -> Result<Vec<String>, String> {
    let resources_dir = PathBuf::from(&workspace_path)
        .join(".latte")
        .join("resources");
    
    if !resources_dir.exists() {
        return Ok(Vec::new());
    }

    let entries = fs::read_dir(&resources_dir)
        .map_err(|e| format!("Failed to read resources directory: {}", e))?;

    let mut resources = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        if let Some(path_str) = path.to_str() {
            resources.push(path_str.to_string());
        }
    }

    Ok(resources)
}

/// Get template content from workspace
#[tauri::command]
pub async fn get_workspace_template(
    workspace_path: String,
    template_name: String,
) -> Result<String, String> {
    let template_path = PathBuf::from(&workspace_path)
        .join(".latte")
        .join("templates")
        .join(format!("{}.typ", template_name));

    fs::read_to_string(&template_path)
        .map_err(|e| format!("Failed to read template: {}", e))
}

/// Add template to workspace
#[tauri::command]
pub async fn add_workspace_template(
    workspace_path: String,
    template_name: String,
    template_content: String,
) -> Result<(), String> {
    let templates_dir = PathBuf::from(&workspace_path)
        .join(".latte")
        .join("templates");
    
    fs::create_dir_all(&templates_dir)
        .map_err(|e| format!("Failed to create templates directory: {}", e))?;

    let template_path = templates_dir.join(format!("{}.typ", template_name));
    fs::write(&template_path, template_content)
        .map_err(|e| format!("Failed to write template: {}", e))?;

    // Update workspace config
    let config_path = PathBuf::from(&workspace_path)
        .join(".latte")
        .join("workspace.json");
    
    if config_path.exists() {
        let config_content = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read workspace config: {}", e))?;
        let mut config: WorkspaceConfig = serde_json::from_str(&config_content)
            .map_err(|e| format!("Failed to parse workspace config: {}", e))?;
        
        if !config.templates.contains(&template_name) {
            config.templates.push(template_name);
            let config_json = serde_json::to_string_pretty(&config)
                .map_err(|e| format!("Failed to serialize workspace config: {}", e))?;
            fs::write(&config_path, config_json)
                .map_err(|e| format!("Failed to update workspace config: {}", e))?;
        }
    }

    Ok(())
}

/// Update workspace settings
#[tauri::command]
pub async fn update_workspace_settings(
    workspace_path: String,
    name: Option<String>,
    templates: Option<Vec<String>>,
    resources: Option<Vec<String>>,
) -> Result<(), String> {
    let config_path = PathBuf::from(&workspace_path)
        .join(".latte")
        .join("workspace.json");
    
    if !config_path.exists() {
        return Err("Workspace config not found".to_string());
    }

    let config_content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read workspace config: {}", e))?;
    
    let mut config: WorkspaceConfig = serde_json::from_str(&config_content)
        .map_err(|e| format!("Failed to parse workspace config: {}", e))?;

    if let Some(new_name) = name {
        config.name = new_name;
    }
    if let Some(new_templates) = templates {
        config.templates = new_templates;
    }
    if let Some(new_resources) = resources {
        config.resources = new_resources;
    }

    let config_json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize workspace config: {}", e))?;
    fs::write(&config_path, config_json)
        .map_err(|e| format!("Failed to write workspace config: {}", e))?;

    Ok(())
}
