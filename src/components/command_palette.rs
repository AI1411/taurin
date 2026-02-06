use i18nrs::yew::use_translation;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ToolItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub icon: String,
    pub keywords: Vec<String>,
}

#[derive(Properties, PartialEq)]
pub struct CommandPaletteProps {
    pub visible: bool,
    pub on_close: Callback<()>,
    pub on_select: Callback<String>,
    pub tools: Vec<ToolItem>,
}

fn fuzzy_match(query: &str, target: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let query_lower = query.to_lowercase();
    let target_lower = target.to_lowercase();

    // Substring match
    if target_lower.contains(&query_lower) {
        return true;
    }

    // Fuzzy character-by-character match
    let mut query_chars = query_lower.chars().peekable();
    for ch in target_lower.chars() {
        if let Some(&q) = query_chars.peek() {
            if ch == q {
                query_chars.next();
            }
        }
    }
    query_chars.peek().is_none()
}

fn score_match(query: &str, tool: &ToolItem) -> i32 {
    if query.is_empty() {
        return 0;
    }
    let query_lower = query.to_lowercase();

    // Exact name match
    if tool.name.to_lowercase() == query_lower {
        return 100;
    }

    // Name starts with query
    if tool.name.to_lowercase().starts_with(&query_lower) {
        return 90;
    }

    // Name contains query
    if tool.name.to_lowercase().contains(&query_lower) {
        return 80;
    }

    // ID contains query
    if tool.id.to_lowercase().contains(&query_lower) {
        return 70;
    }

    // Keyword match
    for keyword in &tool.keywords {
        if keyword.to_lowercase().contains(&query_lower) {
            return 60;
        }
    }

    // Description match
    if tool.description.to_lowercase().contains(&query_lower) {
        return 50;
    }

    // Category match
    if tool.category.to_lowercase().contains(&query_lower) {
        return 40;
    }

    // Fuzzy match on name
    if fuzzy_match(&query_lower, &tool.name.to_lowercase()) {
        return 30;
    }

    // Fuzzy match on keywords
    for keyword in &tool.keywords {
        if fuzzy_match(&query_lower, &keyword.to_lowercase()) {
            return 20;
        }
    }

    -1
}

#[function_component(CommandPalette)]
pub fn command_palette(props: &CommandPaletteProps) -> Html {
    let (i18n, _) = use_translation();
    let query = use_state(String::new);
    let selected_index = use_state(|| 0usize);
    let input_ref = use_node_ref();

    // Filter and sort tools
    let filtered_tools = {
        let query_str = (*query).clone();
        let mut tools: Vec<(i32, ToolItem)> = props
            .tools
            .iter()
            .filter_map(|tool| {
                let s = score_match(&query_str, tool);
                if s >= 0 {
                    Some((s, tool.clone()))
                } else {
                    None
                }
            })
            .collect();
        tools.sort_by(|a, b| b.0.cmp(&a.0));
        tools.into_iter().map(|(_, tool)| tool).collect::<Vec<_>>()
    };

    // Focus input when palette becomes visible
    {
        let input_ref = input_ref.clone();
        let visible = props.visible;
        let query = query.clone();
        let selected_index = selected_index.clone();
        use_effect_with(visible, move |visible| {
            if *visible {
                query.set(String::new());
                selected_index.set(0);
                let input_ref = input_ref.clone();
                gloo_timers::callback::Timeout::new(50, move || {
                    if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                        let _ = input.focus();
                    }
                })
                .forget();
            }
            || {}
        });
    }

    let on_input = {
        let query = query.clone();
        let selected_index = selected_index.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            query.set(input.value());
            selected_index.set(0);
        })
    };

    let on_keydown = {
        let selected_index = selected_index.clone();
        let filtered_len = filtered_tools.len();
        let on_close = props.on_close.clone();
        let on_select = props.on_select.clone();
        let filtered_tools = filtered_tools.clone();
        Callback::from(move |e: KeyboardEvent| {
            match e.key().as_str() {
                "ArrowDown" => {
                    e.prevent_default();
                    if filtered_len > 0 {
                        selected_index.set((*selected_index + 1) % filtered_len);
                    }
                }
                "ArrowUp" => {
                    e.prevent_default();
                    if filtered_len > 0 {
                        selected_index.set(if *selected_index == 0 {
                            filtered_len - 1
                        } else {
                            *selected_index - 1
                        });
                    }
                }
                "Enter" => {
                    e.prevent_default();
                    if let Some(tool) = filtered_tools.get(*selected_index) {
                        on_select.emit(tool.id.clone());
                    }
                }
                "Escape" => {
                    e.prevent_default();
                    on_close.emit(());
                }
                _ => {}
            }
        })
    };

    let on_overlay_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_: MouseEvent| {
            on_close.emit(());
        })
    };

    let on_content_click = Callback::from(move |e: MouseEvent| {
        e.stop_propagation();
    });

    if !props.visible {
        return html! {};
    }

    let search_placeholder = i18n.t("command_palette.search_placeholder");
    let no_results = i18n.t("command_palette.no_results");
    let shortcut_hint = i18n.t("command_palette.shortcut_hint");

    html! {
        <div class="command-palette-overlay" onclick={on_overlay_click}>
            <div class="command-palette" onclick={on_content_click}>
                <div class="command-palette-input-wrapper">
                    <svg class="command-palette-search-icon" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <circle cx="11" cy="11" r="8"/>
                        <line x1="21" y1="21" x2="16.65" y2="16.65"/>
                    </svg>
                    <input
                        ref={input_ref}
                        class="command-palette-input"
                        type="text"
                        placeholder={search_placeholder}
                        value={(*query).clone()}
                        oninput={on_input}
                        onkeydown={on_keydown}
                    />
                    <kbd class="command-palette-kbd">{"esc"}</kbd>
                </div>
                <div class="command-palette-results">
                    if filtered_tools.is_empty() {
                        <div class="command-palette-empty">
                            <p>{no_results}</p>
                        </div>
                    } else {
                        { for filtered_tools.iter().enumerate().map(|(index, tool)| {
                            let is_selected = index == *selected_index;
                            let on_select = props.on_select.clone();
                            let tool_id = tool.id.clone();
                            let selected_index = selected_index.clone();
                            let on_click = Callback::from(move |_: MouseEvent| {
                                on_select.emit(tool_id.clone());
                            });
                            let on_mouse_enter = {
                                let selected_index = selected_index.clone();
                                Callback::from(move |_: MouseEvent| {
                                    selected_index.set(index);
                                })
                            };
                            html! {
                                <button
                                    class={classes!("command-palette-item", is_selected.then_some("selected"))}
                                    onclick={on_click}
                                    onmouseenter={on_mouse_enter}
                                >
                                    <span class="command-palette-item-icon">
                                        {render_palette_icon(&tool.icon)}
                                    </span>
                                    <div class="command-palette-item-info">
                                        <span class="command-palette-item-name">{&tool.name}</span>
                                        <span class="command-palette-item-description">{&tool.description}</span>
                                    </div>
                                    <span class="command-palette-item-category">{&tool.category}</span>
                                </button>
                            }
                        })}
                    }
                </div>
                <div class="command-palette-footer">
                    <span class="command-palette-hint">
                        <kbd>{"↑↓"}</kbd>{" "}
                        {i18n.t("command_palette.navigate")}
                        <kbd>{"↵"}</kbd>{" "}
                        {i18n.t("command_palette.select")}
                        <kbd>{"esc"}</kbd>{" "}
                        {i18n.t("command_palette.close")}
                    </span>
                    <span class="command-palette-shortcut">{shortcut_hint}</span>
                </div>
            </div>
        </div>
    }
}

fn render_palette_icon(name: &str) -> Html {
    match name {
        "photo.stack" => html! {
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <rect x="3" y="3" width="18" height="18" rx="2"/>
                <circle cx="8.5" cy="8.5" r="1.5"/>
                <path d="M21 15l-5-5L5 21"/>
            </svg>
        },
        "paintbrush" => html! {
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M18.37 2.63L14 7l-1.59-1.59a2 2 0 00-2.82 0L8 7l9 9 1.59-1.59a2 2 0 000-2.82L17 10l4.37-4.37a2.12 2.12 0 10-3-3z"/>
                <path d="M9 8c-2 3-4 3.5-7 4l8 10c2-1 6-5 6-7"/>
            </svg>
        },
        "tablecells" => html! {
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <rect x="3" y="3" width="18" height="18" rx="2"/>
                <line x1="3" y1="9" x2="21" y2="9"/>
                <line x1="3" y1="15" x2="21" y2="15"/>
                <line x1="9" y1="3" x2="9" y2="21"/>
                <line x1="15" y1="3" x2="15" y2="21"/>
            </svg>
        },
        "doc.fill" => html! {
            <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
                <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z"/>
                <path d="M14 2v6h6" fill="none" stroke="currentColor" stroke-width="1.5"/>
            </svg>
        },
        "doc.text" => html! {
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z"/>
                <path d="M14 2v6h6"/>
                <line x1="8" y1="13" x2="16" y2="13"/>
                <line x1="8" y1="17" x2="16" y2="17"/>
            </svg>
        },
        "rectangle.3.group" => html! {
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <rect x="3" y="3" width="5" height="18" rx="1"/>
                <rect x="10" y="3" width="5" height="18" rx="1"/>
                <rect x="17" y="3" width="5" height="18" rx="1"/>
            </svg>
        },
        "key.fill" => html! {
            <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
                <path d="M21 2l-2 2m-7.61 7.61a5.5 5.5 0 11-7.778 7.778 5.5 5.5 0 017.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4"/>
            </svg>
        },
        "lock.fill" => html! {
            <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
                <rect x="3" y="11" width="18" height="11" rx="2"/>
                <path d="M7 11V7a5 5 0 0110 0v4" fill="none" stroke="currentColor" stroke-width="1.5"/>
            </svg>
        },
        "arrow.left.arrow.right" => html! {
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <line x1="5" y1="12" x2="19" y2="12"/>
                <polyline points="12 5 19 12 12 19"/>
                <line x1="19" y1="12" x2="5" y2="12"/>
                <polyline points="12 19 5 12 12 5"/>
            </svg>
        },
        "clock" => html! {
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <circle cx="12" cy="12" r="10"/>
                <polyline points="12 6 12 12 16 14"/>
            </svg>
        },
        "arrow.triangle.branch" => html! {
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <line x1="6" y1="3" x2="6" y2="15"/>
                <circle cx="18" cy="6" r="3"/>
                <circle cx="6" cy="18" r="3"/>
                <path d="M18 9a9 9 0 01-9 9"/>
            </svg>
        },
        "note.text" => html! {
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z"/>
                <path d="M14 2v6h6"/>
                <line x1="8" y1="13" x2="16" y2="13"/>
                <line x1="8" y1="17" x2="13" y2="17"/>
            </svg>
        },
        "asterisk.circle" => html! {
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <circle cx="12" cy="12" r="10"/>
                <line x1="12" y1="6" x2="12" y2="18"/>
                <line x1="6.5" y1="9" x2="17.5" y2="15"/>
                <line x1="6.5" y1="15" x2="17.5" y2="9"/>
            </svg>
        },
        "curlybraces" => html! {
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M8 3H6a2 2 0 00-2 2v4a2 2 0 01-2 2 2 2 0 012 2v4a2 2 0 002 2h2"/>
                <path d="M16 3h2a2 2 0 012 2v4a2 2 0 002 2 2 2 0 00-2 2v4a2 2 0 01-2 2h-2"/>
            </svg>
        },
        "doc.badge.gearshape" => html! {
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z"/>
                <path d="M14 2v6h6"/>
                <text x="8" y="17" font-size="8" font-weight="bold" fill="currentColor">{"64"}</text>
            </svg>
        },
        _ => html! {
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <circle cx="12" cy="12" r="10"/>
            </svg>
        },
    }
}
