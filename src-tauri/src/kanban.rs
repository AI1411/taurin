use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskColumn {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub column: TaskColumn,
    pub priority: TaskPriority,
    pub assignee: Option<String>,
    pub due_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanBoard {
    pub tasks: Vec<Task>,
    pub columns: Vec<ColumnConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnConfig {
    pub id: TaskColumn,
    pub name: String,
    pub color: String,
}

impl Default for KanbanBoard {
    fn default() -> Self {
        Self {
            tasks: Vec::new(),
            columns: vec![
                ColumnConfig {
                    id: TaskColumn::Todo,
                    name: "To Do".to_string(),
                    color: "#007aff".to_string(),
                },
                ColumnConfig {
                    id: TaskColumn::InProgress,
                    name: "In Progress".to_string(),
                    color: "#ff9500".to_string(),
                },
                ColumnConfig {
                    id: TaskColumn::Done,
                    name: "Done".to_string(),
                    color: "#34c759".to_string(),
                },
            ],
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
    Ok(app_data_dir.join("kanban.json"))
}

pub fn load_board(app: &AppHandle) -> Result<KanbanBoard, String> {
    let path = get_data_path(app)?;
    if path.exists() {
        let content =
            fs::read_to_string(&path).map_err(|e| format!("Failed to read kanban file: {}", e))?;
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse kanban data: {}", e))
    } else {
        Ok(KanbanBoard::default())
    }
}

pub fn save_board(app: &AppHandle, board: &KanbanBoard) -> Result<(), String> {
    let path = get_data_path(app)?;
    let content =
        serde_json::to_string_pretty(board).map_err(|e| format!("Failed to serialize: {}", e))?;
    fs::write(&path, content).map_err(|e| format!("Failed to write kanban file: {}", e))
}

pub fn create_task(
    app: &AppHandle,
    title: String,
    description: Option<String>,
    priority: TaskPriority,
    assignee: Option<String>,
    due_date: Option<String>,
) -> Result<Task, String> {
    let mut board = load_board(app)?;

    let now = chrono::Utc::now().to_rfc3339();
    let task = Task {
        id: uuid::Uuid::new_v4().to_string(),
        title,
        description,
        column: TaskColumn::Todo,
        priority,
        assignee,
        due_date,
        created_at: now.clone(),
        updated_at: now,
    };

    board.tasks.push(task.clone());
    save_board(app, &board)?;

    Ok(task)
}

pub fn update_task(
    app: &AppHandle,
    task_id: String,
    title: Option<String>,
    description: Option<String>,
    column: Option<TaskColumn>,
    priority: Option<TaskPriority>,
    assignee: Option<String>,
    due_date: Option<String>,
) -> Result<Task, String> {
    let mut board = load_board(app)?;

    let task = board
        .tasks
        .iter_mut()
        .find(|t| t.id == task_id)
        .ok_or_else(|| format!("Task not found: {}", task_id))?;

    if let Some(t) = title {
        task.title = t;
    }
    if let Some(d) = description {
        task.description = Some(d);
    }
    if let Some(c) = column {
        task.column = c;
    }
    if let Some(p) = priority {
        task.priority = p;
    }
    if assignee.is_some() {
        task.assignee = assignee;
    }
    if due_date.is_some() {
        task.due_date = due_date;
    }
    task.updated_at = chrono::Utc::now().to_rfc3339();

    let updated_task = task.clone();
    save_board(app, &board)?;

    Ok(updated_task)
}

pub fn delete_task(app: &AppHandle, task_id: String) -> Result<(), String> {
    let mut board = load_board(app)?;
    let initial_len = board.tasks.len();
    board.tasks.retain(|t| t.id != task_id);

    if board.tasks.len() == initial_len {
        return Err(format!("Task not found: {}", task_id));
    }

    save_board(app, &board)?;
    Ok(())
}

pub fn move_task(app: &AppHandle, task_id: String, column: TaskColumn) -> Result<Task, String> {
    update_task(app, task_id, None, None, Some(column), None, None, None)
}
