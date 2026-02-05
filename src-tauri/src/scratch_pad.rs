use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScratchPadData {
    pub notes: Vec<Note>,
    pub active_note_id: Option<String>,
}

impl Default for ScratchPadData {
    fn default() -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        let default_note = Note {
            id: uuid::Uuid::new_v4().to_string(),
            content: String::new(),
            created_at: now.clone(),
            updated_at: now,
        };
        Self {
            notes: vec![default_note.clone()],
            active_note_id: Some(default_note.id),
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
    Ok(app_data_dir.join("scratch_pad.json"))
}

pub fn load_scratch_pad(app: &AppHandle) -> Result<ScratchPadData, String> {
    let path = get_data_path(app)?;
    if path.exists() {
        let file_content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read scratch pad file: {}", e))?;
        serde_json::from_str(&file_content)
            .map_err(|e| format!("Failed to parse scratch pad data: {}", e))
    } else {
        Ok(ScratchPadData::default())
    }
}

fn save_data(app: &AppHandle, data: &ScratchPadData) -> Result<(), String> {
    let path = get_data_path(app)?;
    let json =
        serde_json::to_string_pretty(data).map_err(|e| format!("Failed to serialize: {}", e))?;
    fs::write(&path, json).map_err(|e| format!("Failed to write scratch pad file: {}", e))
}

pub fn create_note(app: &AppHandle) -> Result<Note, String> {
    let mut data = load_scratch_pad(app)?;
    let now = chrono::Utc::now().to_rfc3339();
    let note = Note {
        id: uuid::Uuid::new_v4().to_string(),
        content: String::new(),
        created_at: now.clone(),
        updated_at: now,
    };
    data.notes.insert(0, note.clone());
    data.active_note_id = Some(note.id.clone());
    save_data(app, &data)?;
    Ok(note)
}

pub fn update_note(app: &AppHandle, note_id: String, content: String) -> Result<Note, String> {
    let mut data = load_scratch_pad(app)?;
    let note = data
        .notes
        .iter_mut()
        .find(|n| n.id == note_id)
        .ok_or_else(|| format!("Note not found: {}", note_id))?;

    note.content = content;
    note.updated_at = chrono::Utc::now().to_rfc3339();
    let updated_note = note.clone();
    save_data(app, &data)?;
    Ok(updated_note)
}

pub fn delete_note(app: &AppHandle, note_id: String) -> Result<ScratchPadData, String> {
    let mut data = load_scratch_pad(app)?;
    data.notes.retain(|n| n.id != note_id);

    // Update active note
    if data.active_note_id.as_ref() == Some(&note_id) {
        data.active_note_id = data.notes.first().map(|n| n.id.clone());
    }

    // Ensure at least one note exists
    if data.notes.is_empty() {
        let now = chrono::Utc::now().to_rfc3339();
        let default_note = Note {
            id: uuid::Uuid::new_v4().to_string(),
            content: String::new(),
            created_at: now.clone(),
            updated_at: now,
        };
        data.active_note_id = Some(default_note.id.clone());
        data.notes.push(default_note);
    }

    save_data(app, &data)?;
    Ok(data)
}

pub fn set_active_note(app: &AppHandle, note_id: String) -> Result<ScratchPadData, String> {
    let mut data = load_scratch_pad(app)?;
    if !data.notes.iter().any(|n| n.id == note_id) {
        return Err(format!("Note not found: {}", note_id));
    }
    data.active_note_id = Some(note_id);
    save_data(app, &data)?;
    Ok(data)
}

pub fn export_to_file(content: String, path: String) -> Result<(), String> {
    fs::write(&path, content).map_err(|e| format!("Failed to write file: {}", e))
}
