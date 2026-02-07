use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub tool_id: String,
    pub inputs: serde_json::Value,
    pub label: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolHistorySettings {
    pub enabled: bool,
    pub max_entries: usize,
}

impl Default for ToolHistorySettings {
    fn default() -> Self {
        Self {
            enabled: true,
            max_entries: 50,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputHistoryData {
    pub entries: Vec<HistoryEntry>,
    pub settings: HashMap<String, ToolHistorySettings>,
}

impl Default for InputHistoryData {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            settings: HashMap::new(),
        }
    }
}

fn get_data_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data dir: {}", e))?;
    Ok(app_data_dir.join("input_history.json"))
}

fn load_data(app: &AppHandle) -> Result<InputHistoryData, String> {
    let path = get_data_path(app)?;
    if path.exists() {
        let file_content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read input history file: {}", e))?;
        serde_json::from_str(&file_content)
            .map_err(|e| format!("Failed to parse input history data: {}", e))
    } else {
        Ok(InputHistoryData::default())
    }
}

fn save_data(app: &AppHandle, data: &InputHistoryData) -> Result<(), String> {
    let path = get_data_path(app)?;
    let json = serde_json::to_string_pretty(data)
        .map_err(|e| format!("Failed to serialize input history: {}", e))?;
    fs::write(&path, json).map_err(|e| format!("Failed to write input history file: {}", e))
}

pub fn add_history_entry(
    app: &AppHandle,
    tool_id: String,
    inputs: serde_json::Value,
    label: Option<String>,
) -> Result<HistoryEntry, String> {
    let mut data = load_data(app)?;

    let settings = data.settings.get(&tool_id).cloned().unwrap_or_default();

    if !settings.enabled {
        return Err("History is disabled for this tool".to_string());
    }

    let entry = HistoryEntry {
        id: uuid::Uuid::new_v4().to_string(),
        tool_id: tool_id.clone(),
        inputs,
        label,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    data.entries.insert(0, entry.clone());

    // Apply max_entries limit per tool
    let mut tool_count = 0;
    data.entries.retain(|e| {
        if e.tool_id == tool_id {
            tool_count += 1;
            tool_count <= settings.max_entries
        } else {
            true
        }
    });

    save_data(app, &data)?;
    Ok(entry)
}

pub fn get_tool_history(app: &AppHandle, tool_id: String) -> Result<Vec<HistoryEntry>, String> {
    let data = load_data(app)?;
    Ok(data
        .entries
        .into_iter()
        .filter(|e| e.tool_id == tool_id)
        .collect())
}

pub fn search_tool_history(
    app: &AppHandle,
    tool_id: String,
    query: String,
) -> Result<Vec<HistoryEntry>, String> {
    let data = load_data(app)?;
    let query_lower = query.to_lowercase();
    Ok(data
        .entries
        .into_iter()
        .filter(|e| {
            if e.tool_id != tool_id {
                return false;
            }
            let inputs_str = e.inputs.to_string().to_lowercase();
            let label_match = e
                .label
                .as_ref()
                .map(|l| l.to_lowercase().contains(&query_lower))
                .unwrap_or(false);
            inputs_str.contains(&query_lower) || label_match
        })
        .collect())
}

pub fn delete_history_entry(app: &AppHandle, entry_id: String) -> Result<(), String> {
    let mut data = load_data(app)?;
    data.entries.retain(|e| e.id != entry_id);
    save_data(app, &data)
}

pub fn clear_tool_history(app: &AppHandle, tool_id: String) -> Result<(), String> {
    let mut data = load_data(app)?;
    data.entries.retain(|e| e.tool_id != tool_id);
    save_data(app, &data)
}

pub fn update_tool_history_settings(
    app: &AppHandle,
    tool_id: String,
    enabled: bool,
    max_entries: usize,
) -> Result<ToolHistorySettings, String> {
    let mut data = load_data(app)?;
    let settings = ToolHistorySettings {
        enabled,
        max_entries,
    };
    data.settings.insert(tool_id, settings.clone());
    save_data(app, &data)?;
    Ok(settings)
}

pub fn get_tool_history_settings(
    app: &AppHandle,
    tool_id: String,
) -> Result<ToolHistorySettings, String> {
    let data = load_data(app)?;
    Ok(data.settings.get(&tool_id).cloned().unwrap_or_default())
}
