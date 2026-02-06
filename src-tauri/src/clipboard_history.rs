use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardEntry {
    pub id: String,
    pub content: String,
    pub content_type: ContentType,
    pub pinned: bool,
    pub created_at: String,
    pub copied_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    Text,
    Code,
    Url,
    Email,
    Password,
}

impl ContentType {
    fn detect(content: &str) -> Self {
        let trimmed = content.trim();

        // URL detection
        if trimmed.starts_with("http://")
            || trimmed.starts_with("https://")
            || trimmed.starts_with("ftp://")
        {
            return ContentType::Url;
        }

        // Email detection
        if trimmed.contains('@')
            && trimmed.contains('.')
            && !trimmed.contains(' ')
            && trimmed.len() < 254
        {
            let at_pos = trimmed.find('@').unwrap();
            let before_at = &trimmed[..at_pos];
            let after_at = &trimmed[at_pos + 1..];
            if !before_at.is_empty() && after_at.contains('.') && !after_at.starts_with('.') {
                return ContentType::Email;
            }
        }

        // Code detection (simple heuristics)
        let code_indicators = [
            "fn ",
            "let ",
            "const ",
            "var ",
            "function ",
            "class ",
            "import ",
            "export ",
            "def ",
            "if ",
            "for ",
            "while ",
            "return ",
            "pub ",
            "async ",
            "await ",
            "=>",
            "->",
            "::",
            "{{",
            "}}",
            "();",
            ");",
            "](",
            "/*",
            "*/",
            "//",
        ];
        let has_code_indicator = code_indicators.iter().any(|&ind| trimmed.contains(ind));
        let has_brackets = trimmed.contains('{') && trimmed.contains('}');
        let has_semicolons = trimmed.matches(';').count() > 1;

        if has_code_indicator || (has_brackets && has_semicolons) {
            return ContentType::Code;
        }

        // Password detection (high entropy, no spaces, mixed chars)
        if !trimmed.contains(' ')
            && trimmed.len() >= 8
            && trimmed.len() <= 128
            && trimmed.chars().any(|c| c.is_ascii_digit())
            && trimmed.chars().any(|c| c.is_ascii_lowercase())
            && (trimmed.chars().any(|c| c.is_ascii_uppercase())
                || trimmed.chars().any(|c| !c.is_alphanumeric()))
        {
            return ContentType::Password;
        }

        ContentType::Text
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardSettings {
    pub max_entries: u32,
    pub exclude_passwords: bool,
    pub exclude_patterns: Vec<String>,
}

impl Default for ClipboardSettings {
    fn default() -> Self {
        Self {
            max_entries: 100,
            exclude_passwords: true,
            exclude_patterns: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClipboardHistoryData {
    pub entries: Vec<ClipboardEntry>,
    pub settings: ClipboardSettings,
}

fn get_data_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data dir: {}", e))?;
    Ok(app_data_dir.join("clipboard_history.json"))
}

pub fn load_clipboard_history(app: &AppHandle) -> Result<ClipboardHistoryData, String> {
    let path = get_data_path(app)?;
    if path.exists() {
        let file_content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read clipboard history file: {}", e))?;
        serde_json::from_str(&file_content)
            .map_err(|e| format!("Failed to parse clipboard history data: {}", e))
    } else {
        Ok(ClipboardHistoryData::default())
    }
}

fn save_data(app: &AppHandle, data: &ClipboardHistoryData) -> Result<(), String> {
    let path = get_data_path(app)?;
    let json =
        serde_json::to_string_pretty(data).map_err(|e| format!("Failed to serialize: {}", e))?;
    fs::write(&path, json).map_err(|e| format!("Failed to write clipboard history file: {}", e))
}

pub fn add_clipboard_entry(app: &AppHandle, content: String) -> Result<ClipboardEntry, String> {
    let mut data = load_clipboard_history(app)?;

    // Detect content type
    let content_type = ContentType::detect(&content);

    // Check if password should be excluded
    if data.settings.exclude_passwords && content_type == ContentType::Password {
        return Err("Password-like content is excluded by settings".to_string());
    }

    // Check exclude patterns
    for pattern in &data.settings.exclude_patterns {
        if content.contains(pattern) {
            return Err(format!("Content matches exclude pattern: {}", pattern));
        }
    }

    // Check if entry already exists (deduplication)
    if let Some(existing) = data.entries.iter_mut().find(|e| e.content == content) {
        existing.copied_count += 1;
        existing.created_at = chrono::Utc::now().to_rfc3339();
        let entry = existing.clone();

        // Move to top (most recent)
        let content_clone = content.clone();
        data.entries.retain(|e| e.content != content_clone);
        data.entries.insert(0, entry.clone());

        save_data(app, &data)?;
        return Ok(entry);
    }

    // Create new entry
    let entry = ClipboardEntry {
        id: uuid::Uuid::new_v4().to_string(),
        content,
        content_type,
        pinned: false,
        created_at: chrono::Utc::now().to_rfc3339(),
        copied_count: 1,
    };

    // Add to beginning of list
    data.entries.insert(0, entry.clone());

    // Trim to max entries (keeping pinned items)
    let max = data.settings.max_entries as usize;
    if data.entries.len() > max {
        // Separate pinned and unpinned
        let (pinned, mut unpinned): (Vec<_>, Vec<_>) =
            data.entries.into_iter().partition(|e| e.pinned);

        // Keep pinned + as many unpinned as possible
        let remaining_slots = max.saturating_sub(pinned.len());
        unpinned.truncate(remaining_slots);

        // Merge back
        data.entries = pinned;
        data.entries.extend(unpinned);

        // Sort by created_at (most recent first)
        data.entries.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    }

    save_data(app, &data)?;
    Ok(entry)
}

pub fn delete_clipboard_entry(
    app: &AppHandle,
    entry_id: String,
) -> Result<ClipboardHistoryData, String> {
    let mut data = load_clipboard_history(app)?;
    data.entries.retain(|e| e.id != entry_id);
    save_data(app, &data)?;
    Ok(data)
}

pub fn clear_clipboard_history(app: &AppHandle) -> Result<ClipboardHistoryData, String> {
    let mut data = load_clipboard_history(app)?;
    // Keep only pinned items
    data.entries.retain(|e| e.pinned);
    save_data(app, &data)?;
    Ok(data)
}

pub fn toggle_pinned(app: &AppHandle, entry_id: String) -> Result<ClipboardEntry, String> {
    let mut data = load_clipboard_history(app)?;
    let entry = data
        .entries
        .iter_mut()
        .find(|e| e.id == entry_id)
        .ok_or_else(|| format!("Entry not found: {}", entry_id))?;

    entry.pinned = !entry.pinned;
    let updated_entry = entry.clone();
    save_data(app, &data)?;
    Ok(updated_entry)
}

pub fn search_clipboard_history(
    app: &AppHandle,
    query: String,
) -> Result<Vec<ClipboardEntry>, String> {
    let data = load_clipboard_history(app)?;
    let query_lower = query.to_lowercase();

    let results: Vec<ClipboardEntry> = data
        .entries
        .into_iter()
        .filter(|e| e.content.to_lowercase().contains(&query_lower))
        .collect();

    Ok(results)
}

pub fn update_clipboard_settings(
    app: &AppHandle,
    settings: ClipboardSettings,
) -> Result<ClipboardHistoryData, String> {
    let mut data = load_clipboard_history(app)?;
    data.settings = settings;
    save_data(app, &data)?;
    Ok(data)
}

pub fn copy_entry_to_clipboard(
    app: &AppHandle,
    entry_id: String,
) -> Result<ClipboardEntry, String> {
    let mut data = load_clipboard_history(app)?;
    let entry = data
        .entries
        .iter_mut()
        .find(|e| e.id == entry_id)
        .ok_or_else(|| format!("Entry not found: {}", entry_id))?;

    entry.copied_count += 1;
    entry.created_at = chrono::Utc::now().to_rfc3339();
    let updated_entry = entry.clone();

    // Move to top
    let entry_id_clone = entry_id.clone();
    let entry_clone = updated_entry.clone();
    data.entries.retain(|e| e.id != entry_id_clone);
    data.entries.insert(0, entry_clone);

    save_data(app, &data)?;
    Ok(updated_entry)
}
