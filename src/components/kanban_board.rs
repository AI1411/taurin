use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}

impl TaskPriority {
    fn label(&self) -> &'static str {
        match self {
            TaskPriority::Low => "低",
            TaskPriority::Medium => "中",
            TaskPriority::High => "高",
            TaskPriority::Urgent => "緊急",
        }
    }

    fn class(&self) -> &'static str {
        match self {
            TaskPriority::Low => "priority-low",
            TaskPriority::Medium => "priority-medium",
            TaskPriority::High => "priority-high",
            TaskPriority::Urgent => "priority-urgent",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskColumn {
    Todo,
    InProgress,
    Done,
}

impl TaskColumn {
    fn label(&self) -> &'static str {
        match self {
            TaskColumn::Todo => "To Do",
            TaskColumn::InProgress => "In Progress",
            TaskColumn::Done => "Done",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            TaskColumn::Todo => "📋",
            TaskColumn::InProgress => "🔄",
            TaskColumn::Done => "✅",
        }
    }
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
pub struct ColumnConfig {
    pub id: TaskColumn,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanBoard {
    pub tasks: Vec<Task>,
    pub columns: Vec<ColumnConfig>,
}

#[derive(Serialize)]
struct EmptyArgs {}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateTaskArgs {
    title: String,
    description: Option<String>,
    priority: TaskPriority,
    assignee: Option<String>,
    due_date: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteTaskArgs {
    task_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MoveTaskArgs {
    task_id: String,
    column: TaskColumn,
}

#[derive(Properties, PartialEq)]
pub struct KanbanBoardProps {}

#[function_component(KanbanBoardComponent)]
pub fn kanban_board(_props: &KanbanBoardProps) -> Html {
    let board = use_state(|| Option::<KanbanBoard>::None);
    let is_loading = use_state(|| true);
    let show_create_modal = use_state(|| false);
    let editing_task = use_state(|| Option::<Task>::None);
    let search_query = use_state(String::new);
    let dragging_task_id = use_state(|| Option::<String>::None);
    let hover_column = use_state(|| Option::<TaskColumn>::None);
    let drag_pos = use_state(|| (0i32, 0i32));
    let drag_offset = use_state(|| (0i32, 0i32));

    // Form states
    let new_title = use_state(String::new);
    let new_description = use_state(String::new);
    let new_priority = use_state(|| TaskPriority::Medium);
    let new_due_date = use_state(String::new);

    // Load board on mount
    {
        let board = board.clone();
        let is_loading = is_loading.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&EmptyArgs {}).unwrap();
                let result = invoke("load_kanban_board_cmd", args).await;
                if let Ok(data) = serde_wasm_bindgen::from_value::<KanbanBoard>(result) {
                    board.set(Some(data));
                }
                is_loading.set(false);
            });
            || {}
        });
    }

    // Global mouse event listeners for drag and drop
    {
        let drag_pos = drag_pos.clone();
        let dragging_task_id = dragging_task_id.clone();

        use_effect_with((*dragging_task_id).clone(), move |dragging| {
            let closures: Rc<RefCell<Option<(Closure<dyn Fn(web_sys::MouseEvent)>, Closure<dyn Fn(web_sys::MouseEvent)>)>>> =
                Rc::new(RefCell::new(None));

            if dragging.is_some() {
                let document = web_sys::window().unwrap().document().unwrap();

                let drag_pos_clone = drag_pos.clone();
                let mousemove_closure = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
                    drag_pos_clone.set((e.client_x(), e.client_y()));
                });

                let dragging_task_id_clone = dragging_task_id.clone();
                let mouseup_closure = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
                    dragging_task_id_clone.set(None);
                });

                document
                    .add_event_listener_with_callback("mousemove", mousemove_closure.as_ref().unchecked_ref())
                    .unwrap();
                document
                    .add_event_listener_with_callback("mouseup", mouseup_closure.as_ref().unchecked_ref())
                    .unwrap();

                *closures.borrow_mut() = Some((mousemove_closure, mouseup_closure));
            }

            let closures_clone = closures.clone();
            move || {
                if let Some((mousemove, mouseup)) = closures_clone.borrow_mut().take() {
                    let document = web_sys::window().unwrap().document().unwrap();
                    let _ = document.remove_event_listener_with_callback(
                        "mousemove",
                        mousemove.as_ref().unchecked_ref(),
                    );
                    let _ = document.remove_event_listener_with_callback(
                        "mouseup",
                        mouseup.as_ref().unchecked_ref(),
                    );
                }
            }
        });
    }

    let on_search_change = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            search_query.set(input.value());
        })
    };

    let on_open_create_modal = {
        let show_create_modal = show_create_modal.clone();
        let new_title = new_title.clone();
        let new_description = new_description.clone();
        let new_priority = new_priority.clone();
        let new_due_date = new_due_date.clone();
        Callback::from(move |_| {
            new_title.set(String::new());
            new_description.set(String::new());
            new_priority.set(TaskPriority::Medium);
            new_due_date.set(String::new());
            show_create_modal.set(true);
        })
    };

    let on_close_modal = {
        let show_create_modal = show_create_modal.clone();
        let editing_task = editing_task.clone();
        Callback::from(move |_| {
            show_create_modal.set(false);
            editing_task.set(None);
        })
    };

    let on_create_task = {
        let board = board.clone();
        let show_create_modal = show_create_modal.clone();
        let new_title = new_title.clone();
        let new_description = new_description.clone();
        let new_priority = new_priority.clone();
        let new_due_date = new_due_date.clone();
        Callback::from(move |_| {
            let board = board.clone();
            let show_create_modal = show_create_modal.clone();
            let title = (*new_title).clone();
            let description = if new_description.is_empty() {
                None
            } else {
                Some((*new_description).clone())
            };
            let priority = (*new_priority).clone();
            let due_date = if new_due_date.is_empty() {
                None
            } else {
                Some((*new_due_date).clone())
            };

            if title.is_empty() {
                return;
            }

            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&CreateTaskArgs {
                    title,
                    description,
                    priority,
                    assignee: None,
                    due_date,
                })
                .unwrap();
                let result = invoke("create_task_cmd", args).await;
                if let Ok(task) = serde_wasm_bindgen::from_value::<Task>(result) {
                    if let Some(mut b) = (*board).clone() {
                        b.tasks.push(task);
                        board.set(Some(b));
                    }
                }
                show_create_modal.set(false);
            });
        })
    };

    let on_delete_task = {
        let board = board.clone();
        Callback::from(move |task_id: String| {
            let board = board.clone();
            let task_id_clone = task_id.clone();
            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&DeleteTaskArgs {
                    task_id: task_id_clone.clone(),
                })
                .unwrap();
                let result = invoke("delete_task_cmd", args).await;
                if serde_wasm_bindgen::from_value::<()>(result).is_ok() {
                    if let Some(mut b) = (*board).clone() {
                        b.tasks.retain(|t| t.id != task_id_clone);
                        board.set(Some(b));
                    }
                }
            });
        })
    };

    let on_move_task = {
        let board = board.clone();
        Callback::from(move |(task_id, column): (String, TaskColumn)| {
            let board = board.clone();
            let task_id_clone = task_id.clone();
            let column_clone = column.clone();
            web_sys::console::log_1(&format!("Moving task {} to {:?}", task_id, column).into());
            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&MoveTaskArgs {
                    task_id: task_id_clone.clone(),
                    column: column_clone,
                })
                .unwrap();
                let result = invoke("move_task_cmd", args).await;
                web_sys::console::log_1(&format!("move_task_cmd result: {:?}", result).into());
                match serde_wasm_bindgen::from_value::<Task>(result.clone()) {
                    Ok(updated_task) => {
                        web_sys::console::log_1(&"Task updated successfully".into());
                        if let Some(mut b) = (*board).clone() {
                            if let Some(task) = b.tasks.iter_mut().find(|t| t.id == task_id_clone) {
                                *task = updated_task;
                            }
                            board.set(Some(b));
                        }
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to parse result: {:?}", e).into());
                    }
                }
            });
        })
    };

    // Filter tasks based on search query
    let filtered_tasks: Vec<Task> = if let Some(b) = (*board).clone() {
        if search_query.is_empty() {
            b.tasks
        } else {
            let query = (*search_query).to_lowercase();
            b.tasks
                .into_iter()
                .filter(|t| {
                    t.title.to_lowercase().contains(&query)
                        || t.description
                            .as_ref()
                            .map(|d| d.to_lowercase().contains(&query))
                            .unwrap_or(false)
                        || t.assignee
                            .as_ref()
                            .map(|a| a.to_lowercase().contains(&query))
                            .unwrap_or(false)
                })
                .collect()
        }
    } else {
        Vec::new()
    };

    let columns = [TaskColumn::Todo, TaskColumn::InProgress, TaskColumn::Done];

    // Get dragging task for ghost rendering
    let dragging_task: Option<Task> = if let Some(ref task_id) = *dragging_task_id {
        filtered_tasks.iter().find(|t| &t.id == task_id).map(|t| (*t).clone())
    } else {
        None
    };

    let ghost_style = if dragging_task.is_some() {
        let (x, y) = *drag_pos;
        let (ox, oy) = *drag_offset;
        format!(
            "position: fixed; left: {}px; top: {}px; pointer-events: none; z-index: 10000; width: 250px; opacity: 0.9; transform: rotate(3deg);",
            x - ox,
            y - oy
        )
    } else {
        String::new()
    };

    html! {
        <div class="kanban-board">
            <h2>{"📋 Kanban Board"}</h2>

            // Toolbar
            <div class="kanban-toolbar section">
                <div class="search-box">
                    <input
                        type="text"
                        class="search-input"
                        placeholder="タスクを検索..."
                        value={(*search_query).clone()}
                        oninput={on_search_change}
                    />
                </div>
                <button class="primary-btn" onclick={on_open_create_modal}>
                    {"＋ タスク追加"}
                </button>
            </div>

            if *is_loading {
                <div class="loading-state">
                    <div class="spinner-large"></div>
                    <p>{"読み込み中..."}</p>
                </div>
            } else {
                // Columns
                <div class="kanban-columns">
                    { for columns.iter().map(|col| {
                        let col_tasks: Vec<&Task> = filtered_tasks.iter()
                            .filter(|t| t.column == *col)
                            .collect();

                        let on_delete = on_delete_task.clone();
                        let on_move = on_move_task.clone();
                        let col_clone = col.clone();
                        let dragging = (*dragging_task_id).clone();
                        let current_hover = (*hover_column).clone();

                        // Mouse-based drag and drop
                        let onmouseenter = {
                            let hover_column = hover_column.clone();
                            let col = col.clone();
                            Callback::from(move |_: MouseEvent| {
                                hover_column.set(Some(col.clone()));
                            })
                        };

                        let onmouseleave = {
                            let hover_column = hover_column.clone();
                            Callback::from(move |_: MouseEvent| {
                                hover_column.set(None);
                            })
                        };

                        let onmouseup_column = {
                            let dragging_task_id = dragging_task_id.clone();
                            let on_move = on_move.clone();
                            let col = col.clone();
                            Callback::from(move |_: MouseEvent| {
                                if let Some(task_id) = (*dragging_task_id).clone() {
                                    web_sys::console::log_1(&format!("Mouse drop on column: {:?}", col).into());
                                    on_move.emit((task_id, col.clone()));
                                    dragging_task_id.set(None);
                                }
                            })
                        };

                        let column_data = match col {
                            TaskColumn::Todo => "Todo",
                            TaskColumn::InProgress => "InProgress",
                            TaskColumn::Done => "Done",
                        };

                        let is_hover_target = current_hover.as_ref() == Some(col) && dragging.is_some();

                        html! {
                            <div
                                class={classes!(
                                    "kanban-column",
                                    dragging.is_some().then_some("drag-active"),
                                    is_hover_target.then_some("drag-over")
                                )}
                                data-column={column_data}
                                onmouseenter={onmouseenter}
                                onmouseleave={onmouseleave}
                                onmouseup={onmouseup_column}
                            >
                                <div class="kanban-column-header">
                                    <span class="column-icon">{col.icon()}</span>
                                    <span class="column-title">{col.label()}</span>
                                    <span class="column-count">{col_tasks.len()}</span>
                                </div>
                                <div class="kanban-column-body">
                                    { for col_tasks.iter().map(|task| {
                                        let task_id = task.id.clone();
                                        let task_id_delete = task.id.clone();
                                        let on_delete = on_delete.clone();
                                        let is_dragging = dragging.as_ref() == Some(&task.id);

                                        let onmousedown_card = {
                                            let dragging_task_id = dragging_task_id.clone();
                                            let drag_pos = drag_pos.clone();
                                            let drag_offset = drag_offset.clone();
                                            let task_id = task_id.clone();
                                            Callback::from(move |e: MouseEvent| {
                                                // Don't start drag if clicking on a button
                                                let target = e.target().unwrap();
                                                let element: web_sys::Element = target.dyn_into().unwrap();
                                                if element.closest("button").ok().flatten().is_some() {
                                                    return;
                                                }
                                                // Get card element and calculate offset
                                                if let Some(card) = element.closest(".kanban-card").ok().flatten() {
                                                    let rect = card.get_bounding_client_rect();
                                                    let offset_x = e.client_x() - rect.left() as i32;
                                                    let offset_y = e.client_y() - rect.top() as i32;
                                                    drag_offset.set((offset_x, offset_y));
                                                }
                                                drag_pos.set((e.client_x(), e.client_y()));
                                                dragging_task_id.set(Some(task_id.clone()));
                                            })
                                        };

                                        let onmouseup_card = {
                                            Callback::from(move |e: MouseEvent| {
                                                // Stop propagation so column doesn't also handle it
                                                e.stop_propagation();
                                            })
                                        };

                                        html! {
                                            <div
                                                class={classes!("kanban-card", is_dragging.then_some("dragging"))}
                                                data-task-id={task.id.clone()}
                                                onmousedown={onmousedown_card}
                                                onmouseup={onmouseup_card}
                                            >
                                                <div class="card-header">
                                                    <span class={classes!("priority-badge", task.priority.class())}>
                                                        {task.priority.label()}
                                                    </span>
                                                    <button
                                                        class="card-delete-btn"
                                                        onclick={Callback::from(move |_| on_delete.emit(task_id_delete.clone()))}
                                                    >
                                                        {"×"}
                                                    </button>
                                                </div>
                                                <div class="card-title">{&task.title}</div>
                                                if let Some(desc) = &task.description {
                                                    <div class="card-description">{desc}</div>
                                                }
                                                <div class="card-footer">
                                                    if let Some(assignee) = &task.assignee {
                                                        <span class="card-assignee">{"👤 "}{assignee}</span>
                                                    }
                                                    if let Some(due) = &task.due_date {
                                                        <span class="card-due-date">{"📅 "}{due}</span>
                                                    }
                                                </div>
                                                <div class="card-actions">
                                                    { if col_clone != TaskColumn::Todo {
                                                        let on_move = on_move.clone();
                                                        let task_id = task_id.clone();
                                                        let prev_col = match col_clone {
                                                            TaskColumn::InProgress => TaskColumn::Todo,
                                                            TaskColumn::Done => TaskColumn::InProgress,
                                                            _ => TaskColumn::Todo,
                                                        };
                                                        html! {
                                                            <button
                                                                class="move-btn"
                                                                draggable="false"
                                                                onclick={{
                                                                    let on_move = on_move.clone();
                                                                    let task_id = task_id.clone();
                                                                    let prev_col = prev_col.clone();
                                                                    Callback::from(move |e: MouseEvent| {
                                                                        e.stop_propagation();
                                                                        on_move.emit((task_id.clone(), prev_col.clone()));
                                                                    })
                                                                }}
                                                            >
                                                                {"← "}
                                                            </button>
                                                        }
                                                    } else {
                                                        html! {}
                                                    }}
                                                    { if col_clone != TaskColumn::Done {
                                                        let on_move = on_move.clone();
                                                        let task_id = task_id.clone();
                                                        let next_col = match col_clone {
                                                            TaskColumn::Todo => TaskColumn::InProgress,
                                                            TaskColumn::InProgress => TaskColumn::Done,
                                                            _ => TaskColumn::Done,
                                                        };
                                                        html! {
                                                            <button
                                                                class="move-btn"
                                                                draggable="false"
                                                                onclick={{
                                                                    let on_move = on_move.clone();
                                                                    let task_id = task_id.clone();
                                                                    let next_col = next_col.clone();
                                                                    Callback::from(move |e: MouseEvent| {
                                                                        e.stop_propagation();
                                                                        on_move.emit((task_id.clone(), next_col.clone()));
                                                                    })
                                                                }}
                                                            >
                                                                {" →"}
                                                            </button>
                                                        }
                                                    } else {
                                                        html! {}
                                                    }}
                                                </div>
                                            </div>
                                        }
                                    })}
                                </div>
                            </div>
                        }
                    })}
                </div>
            }

            // Create Task Modal
            if *show_create_modal {
                <div class="modal-overlay" onclick={on_close_modal.clone()}>
                    <div class="modal-content" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                        <div class="modal-header">
                            <h3>{"新しいタスク"}</h3>
                            <button class="modal-close-btn" onclick={on_close_modal.clone()}>{"×"}</button>
                        </div>
                        <div class="modal-body">
                            <div class="form-group">
                                <label>{"タイトル *"}</label>
                                <input
                                    type="text"
                                    class="form-input"
                                    placeholder="タスクのタイトル"
                                    value={(*new_title).clone()}
                                    oninput={{
                                        let new_title = new_title.clone();
                                        Callback::from(move |e: InputEvent| {
                                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                            new_title.set(input.value());
                                        })
                                    }}
                                />
                            </div>
                            <div class="form-group">
                                <label>{"説明"}</label>
                                <textarea
                                    class="form-textarea"
                                    placeholder="タスクの詳細説明"
                                    value={(*new_description).clone()}
                                    oninput={{
                                        let new_description = new_description.clone();
                                        Callback::from(move |e: InputEvent| {
                                            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                            new_description.set(input.value());
                                        })
                                    }}
                                />
                            </div>
                            <div class="form-group">
                                <label>{"優先度"}</label>
                                <select
                                    class="form-select"
                                    onchange={{
                                        let new_priority = new_priority.clone();
                                        Callback::from(move |e: Event| {
                                            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                            let priority = match select.value().as_str() {
                                                "Low" => TaskPriority::Low,
                                                "Medium" => TaskPriority::Medium,
                                                "High" => TaskPriority::High,
                                                "Urgent" => TaskPriority::Urgent,
                                                _ => TaskPriority::Medium,
                                            };
                                            new_priority.set(priority);
                                        })
                                    }}
                                >
                                    <option value="Low" selected={*new_priority == TaskPriority::Low}>{"低"}</option>
                                    <option value="Medium" selected={*new_priority == TaskPriority::Medium}>{"中"}</option>
                                    <option value="High" selected={*new_priority == TaskPriority::High}>{"高"}</option>
                                    <option value="Urgent" selected={*new_priority == TaskPriority::Urgent}>{"緊急"}</option>
                                </select>
                            </div>
                            <div class="form-group">
                                <label>{"期限"}</label>
                                <input
                                    type="date"
                                    class="form-input"
                                    value={(*new_due_date).clone()}
                                    oninput={{
                                        let new_due_date = new_due_date.clone();
                                        Callback::from(move |e: InputEvent| {
                                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                            new_due_date.set(input.value());
                                        })
                                    }}
                                />
                            </div>
                        </div>
                        <div class="modal-footer">
                            <button class="secondary-btn" onclick={on_close_modal}>{"キャンセル"}</button>
                            <button class="primary-btn" onclick={on_create_task}>{"作成"}</button>
                        </div>
                    </div>
                </div>
            }

            // Drag ghost
            if let Some(task) = dragging_task {
                <div class="kanban-card drag-ghost" style={ghost_style}>
                    <div class="card-header">
                        <span class={classes!("priority-badge", task.priority.class())}>
                            {task.priority.label()}
                        </span>
                    </div>
                    <div class="card-title">{&task.title}</div>
                </div>
            }
        </div>
    }
}
