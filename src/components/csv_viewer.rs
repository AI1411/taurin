use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn open(options: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn save(options: JsValue) -> JsValue;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CsvData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub total_rows: usize,
    pub total_columns: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CsvInfo {
    pub file_name: String,
    pub file_size: u64,
    pub row_count: usize,
    pub column_count: usize,
    pub headers: Vec<String>,
}

#[derive(Serialize)]
struct OpenDialogOptions {
    multiple: bool,
    filters: Vec<FileFilter>,
}

#[derive(Serialize)]
struct SaveDialogOptions {
    filters: Vec<FileFilter>,
    #[serde(rename = "defaultPath")]
    default_path: Option<String>,
}

#[derive(Serialize)]
struct FileFilter {
    name: String,
    extensions: Vec<String>,
}

#[derive(Serialize)]
struct ReadCsvArgs {
    path: String,
}

#[derive(Serialize)]
struct SaveCsvArgs {
    path: String,
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

#[derive(Clone, PartialEq)]
enum SortOrder {
    None,
    Asc,
    Desc,
}

#[function_component(CsvViewer)]
pub fn csv_viewer() -> Html {
    let file_path = use_state(|| String::new());
    let csv_data = use_state(|| Option::<CsvData>::None);
    let csv_info = use_state(|| Option::<CsvInfo>::None);
    let search_query = use_state(|| String::new());
    let sort_column = use_state(|| Option::<usize>::None);
    let sort_order = use_state(|| SortOrder::None);
    let editing_cell = use_state(|| Option::<(usize, usize)>::None);
    let edited_rows = use_state(|| Vec::<Vec<String>>::new());
    let is_modified = use_state(|| false);
    let column_filters = use_state(|| Vec::<String>::new());
    let is_loading = use_state(|| false);

    let on_select_file = {
        let file_path = file_path.clone();
        let csv_data = csv_data.clone();
        let csv_info = csv_info.clone();
        let edited_rows = edited_rows.clone();
        let is_modified = is_modified.clone();
        let column_filters = column_filters.clone();
        let sort_column = sort_column.clone();
        let sort_order = sort_order.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |_| {
            let file_path = file_path.clone();
            let csv_data = csv_data.clone();
            let csv_info = csv_info.clone();
            let edited_rows = edited_rows.clone();
            let is_modified = is_modified.clone();
            let column_filters = column_filters.clone();
            let sort_column = sort_column.clone();
            let sort_order = sort_order.clone();
            let is_loading = is_loading.clone();

            spawn_local(async move {
                let options = OpenDialogOptions {
                    multiple: false,
                    filters: vec![FileFilter {
                        name: "CSV Files".to_string(),
                        extensions: vec!["csv".to_string(), "tsv".to_string(), "txt".to_string()],
                    }],
                };
                let options_js = serde_wasm_bindgen::to_value(&options).unwrap();
                let result = open(options_js).await;

                if let Some(path) = result.as_string() {
                    file_path.set(path.clone());
                    is_loading.set(true);

                    let args = serde_wasm_bindgen::to_value(&ReadCsvArgs { path }).unwrap();
                    let data_result = invoke("read_csv_cmd", args).await;

                    if let Ok(data) = serde_wasm_bindgen::from_value::<CsvData>(data_result) {
                        let filters = vec![String::new(); data.headers.len()];
                        column_filters.set(filters);
                        edited_rows.set(data.rows.clone());
                        csv_info.set(Some(CsvInfo {
                            file_name: file_path.split('/').last().unwrap_or("unknown").to_string(),
                            file_size: 0,
                            row_count: data.total_rows,
                            column_count: data.total_columns,
                            headers: data.headers.clone(),
                        }));
                        csv_data.set(Some(data));
                        is_modified.set(false);
                        sort_column.set(None);
                        sort_order.set(SortOrder::None);
                    }

                    is_loading.set(false);
                }
            });
        })
    };

    let on_search_change = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            search_query.set(input.value());
        })
    };

    let on_sort = {
        let sort_column = sort_column.clone();
        let sort_order = sort_order.clone();
        Callback::from(move |col: usize| {
            let current_col = *sort_column;
            let current_order = (*sort_order).clone();

            if current_col == Some(col) {
                match current_order {
                    SortOrder::None => sort_order.set(SortOrder::Asc),
                    SortOrder::Asc => sort_order.set(SortOrder::Desc),
                    SortOrder::Desc => {
                        sort_order.set(SortOrder::None);
                        sort_column.set(None);
                    }
                }
            } else {
                sort_column.set(Some(col));
                sort_order.set(SortOrder::Asc);
            }
        })
    };

    let on_cell_click = {
        let editing_cell = editing_cell.clone();
        Callback::from(move |(row, col): (usize, usize)| {
            editing_cell.set(Some((row, col)));
        })
    };

    let on_cell_change = {
        let edited_rows = edited_rows.clone();
        let is_modified = is_modified.clone();
        let editing_cell = editing_cell.clone();
        Callback::from(move |(row, col, value): (usize, usize, String)| {
            let mut rows = (*edited_rows).clone();
            if row < rows.len() && col < rows[row].len() {
                rows[row][col] = value;
                edited_rows.set(rows);
                is_modified.set(true);
            }
            editing_cell.set(None);
        })
    };

    let on_cell_blur = {
        let editing_cell = editing_cell.clone();
        Callback::from(move |_| {
            editing_cell.set(None);
        })
    };

    let on_add_row = {
        let edited_rows = edited_rows.clone();
        let csv_data = csv_data.clone();
        let is_modified = is_modified.clone();
        Callback::from(move |_| {
            if let Some(data) = &*csv_data {
                let mut rows = (*edited_rows).clone();
                let new_row = vec![String::new(); data.headers.len()];
                rows.push(new_row);
                edited_rows.set(rows);
                is_modified.set(true);
            }
        })
    };

    let on_delete_row = {
        let edited_rows = edited_rows.clone();
        let is_modified = is_modified.clone();
        Callback::from(move |row_idx: usize| {
            let mut rows = (*edited_rows).clone();
            if row_idx < rows.len() {
                rows.remove(row_idx);
                edited_rows.set(rows);
                is_modified.set(true);
            }
        })
    };

    let on_save = {
        let file_path = file_path.clone();
        let csv_data = csv_data.clone();
        let edited_rows = edited_rows.clone();
        let is_modified = is_modified.clone();
        Callback::from(move |_| {
            let file_path_val = (*file_path).clone();
            let csv_data = csv_data.clone();
            let edited_rows = edited_rows.clone();
            let is_modified = is_modified.clone();

            spawn_local(async move {
                if let Some(data) = &*csv_data {
                    let save_options = SaveDialogOptions {
                        filters: vec![FileFilter {
                            name: "CSV File".to_string(),
                            extensions: vec!["csv".to_string()],
                        }],
                        default_path: Some(
                            file_path_val
                                .split('/')
                                .last()
                                .unwrap_or("export.csv")
                                .to_string(),
                        ),
                    };
                    let save_options_js = serde_wasm_bindgen::to_value(&save_options).unwrap();
                    let save_result = save(save_options_js).await;

                    if let Some(output_path) = save_result.as_string() {
                        let args = SaveCsvArgs {
                            path: output_path,
                            headers: data.headers.clone(),
                            rows: (*edited_rows).clone(),
                        };
                        let args_js = serde_wasm_bindgen::to_value(&args).unwrap();
                        let _ = invoke("save_csv_cmd", args_js).await;
                        is_modified.set(false);
                    }
                }
            });
        })
    };

    let on_column_filter_change = {
        let column_filters = column_filters.clone();
        Callback::from(move |(col, value): (usize, String)| {
            let mut filters = (*column_filters).clone();
            if col < filters.len() {
                filters[col] = value;
                column_filters.set(filters);
            }
        })
    };

    let filtered_and_sorted_rows = {
        let rows = (*edited_rows).clone();
        let query = (*search_query).clone().to_lowercase();
        let filters = (*column_filters).clone();
        let col = *sort_column;
        let order = (*sort_order).clone();

        let mut filtered: Vec<(usize, Vec<String>)> = rows
            .into_iter()
            .enumerate()
            .filter(|(_, row)| {
                let matches_search =
                    query.is_empty() || row.iter().any(|cell| cell.to_lowercase().contains(&query));

                let matches_filters = filters.iter().enumerate().all(|(i, filter)| {
                    filter.is_empty()
                        || row
                            .get(i)
                            .map(|cell| cell.to_lowercase().contains(&filter.to_lowercase()))
                            .unwrap_or(true)
                });

                matches_search && matches_filters
            })
            .collect();

        if let Some(sort_col) = col {
            filtered.sort_by(|(_, a), (_, b)| {
                let a_val = a.get(sort_col).map(|s| s.as_str()).unwrap_or("");
                let b_val = b.get(sort_col).map(|s| s.as_str()).unwrap_or("");

                let cmp = a_val
                    .parse::<f64>()
                    .ok()
                    .and_then(|a_num| {
                        b_val
                            .parse::<f64>()
                            .ok()
                            .map(|b_num| a_num.partial_cmp(&b_num))
                    })
                    .flatten()
                    .unwrap_or_else(|| a_val.cmp(b_val));

                match order {
                    SortOrder::Asc => cmp,
                    SortOrder::Desc => cmp.reverse(),
                    SortOrder::None => std::cmp::Ordering::Equal,
                }
            });
        }

        filtered
    };

    html! {
        <div class="csv-viewer">
            // File Selection
            <div class="section" onclick={on_select_file.clone()}>
                <div class="drop-zone">
                    <div class="drop-zone-icon">{"📊"}</div>
                    <p class="drop-zone-text">{"Click to select a CSV file"}</p>
                    <p class="drop-zone-hint">{"CSV, TSV, TXT"}</p>
                </div>
                {if !file_path.is_empty() {
                    html! { <p class="file-path">{&*file_path}</p> }
                } else {
                    html! {}
                }}
            </div>

            // Loading State
            {if *is_loading {
                html! {
                    <div class="section loading-state">
                        <div class="spinner-large"></div>
                        <p>{"Loading CSV..."}</p>
                    </div>
                }
            } else {
                html! {}
            }}

            // CSV Info
            {if let Some(info) = &*csv_info {
                html! {
                    <div class="section info-box">
                        <h3>{"File Info"}</h3>
                        <div class="info-grid">
                            <div class="info-item">
                                <div class="info-item-label">{"Rows"}</div>
                                <div class="info-item-value">{info.row_count}</div>
                            </div>
                            <div class="info-item">
                                <div class="info-item-label">{"Columns"}</div>
                                <div class="info-item-value">{info.column_count}</div>
                            </div>
                            <div class="info-item">
                                <div class="info-item-label">{"Modified"}</div>
                                <div class="info-item-value">{if *is_modified { "Yes" } else { "No" }}</div>
                            </div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}

            // Search & Actions
            {if csv_data.is_some() {
                html! {
                    <div class="section csv-toolbar">
                        <div class="search-box">
                            <input
                                type="text"
                                placeholder="Search..."
                                value={(*search_query).clone()}
                                oninput={on_search_change}
                                class="search-input"
                            />
                        </div>
                        <div class="toolbar-actions">
                            <button onclick={on_add_row} class="toolbar-btn">
                                {"+ Add Row"}
                            </button>
                            <button
                                onclick={on_save}
                                class={if *is_modified { "toolbar-btn save-btn modified" } else { "toolbar-btn save-btn" }}
                            >
                                {"Save"}
                            </button>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}

            // CSV Table
            {if let Some(data) = &*csv_data {
                html! {
                    <div class="section csv-table-container">
                        <div class="csv-table-wrapper">
                            <table class="csv-table">
                                <thead>
                                    <tr>
                                        <th class="row-number-header">{"#"}</th>
                                        {for data.headers.iter().enumerate().map(|(i, header)| {
                                            let on_sort = on_sort.clone();
                                            let sort_col = *sort_column;
                                            let sort_ord = (*sort_order).clone();
                                            let sort_indicator = if sort_col == Some(i) {
                                                match sort_ord {
                                                    SortOrder::Asc => " ↑",
                                                    SortOrder::Desc => " ↓",
                                                    SortOrder::None => "",
                                                }
                                            } else {
                                                ""
                                            };
                                            html! {
                                                <th onclick={Callback::from(move |_| on_sort.emit(i))} class="sortable-header">
                                                    <span>{header}{sort_indicator}</span>
                                                </th>
                                            }
                                        })}
                                        <th class="actions-header">{"Actions"}</th>
                                    </tr>
                                    // Column Filters
                                    <tr class="filter-row">
                                        <td></td>
                                        {for (0..data.headers.len()).map(|i| {
                                            let on_filter = on_column_filter_change.clone();
                                            let filter_value = column_filters.get(i).cloned().unwrap_or_default();
                                            html! {
                                                <td>
                                                    <input
                                                        type="text"
                                                        placeholder="Filter..."
                                                        value={filter_value}
                                                        oninput={Callback::from(move |e: InputEvent| {
                                                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                            on_filter.emit((i, input.value()));
                                                        })}
                                                        class="filter-input"
                                                    />
                                                </td>
                                            }
                                        })}
                                        <td></td>
                                    </tr>
                                </thead>
                                <tbody>
                                    {for filtered_and_sorted_rows.iter().map(|(original_idx, row)| {
                                        let original_idx = *original_idx;
                                        let on_delete = on_delete_row.clone();
                                        html! {
                                            <tr>
                                                <td class="row-number">{original_idx + 1}</td>
                                                {for row.iter().enumerate().map(|(col_idx, cell)| {
                                                    let is_editing = *editing_cell == Some((original_idx, col_idx));
                                                    let on_click = on_cell_click.clone();
                                                    let on_change_blur = on_cell_change.clone();
                                                    let on_change_key = on_cell_change.clone();
                                                    let on_blur = on_cell_blur.clone();
                                                    let cell_value = cell.clone();

                                                    if is_editing {
                                                        html! {
                                                            <td class="editing-cell">
                                                                <input
                                                                    type="text"
                                                                    value={cell_value.clone()}
                                                                    onblur={Callback::from(move |e: FocusEvent| {
                                                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                                        on_change_blur.emit((original_idx, col_idx, input.value()));
                                                                    })}
                                                                    onkeydown={Callback::from(move |e: KeyboardEvent| {
                                                                        if e.key() == "Enter" {
                                                                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                                            on_change_key.emit((original_idx, col_idx, input.value()));
                                                                        } else if e.key() == "Escape" {
                                                                            on_blur.emit(());
                                                                        }
                                                                    })}
                                                                    class="cell-input"
                                                                    autofocus=true
                                                                />
                                                            </td>
                                                        }
                                                    } else {
                                                        html! {
                                                            <td
                                                                onclick={Callback::from(move |_| on_click.emit((original_idx, col_idx)))}
                                                                class="editable-cell"
                                                            >
                                                                {cell}
                                                            </td>
                                                        }
                                                    }
                                                })}
                                                <td class="actions-cell">
                                                    <button
                                                        onclick={Callback::from(move |_| on_delete.emit(original_idx))}
                                                        class="delete-row-btn"
                                                        title="Delete row"
                                                    >
                                                        {"×"}
                                                    </button>
                                                </td>
                                            </tr>
                                        }
                                    })}
                                </tbody>
                            </table>
                        </div>
                        <div class="table-footer">
                            <span class="row-count">
                                {format!("Showing {} of {} rows", filtered_and_sorted_rows.len(), edited_rows.len())}
                            </span>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
