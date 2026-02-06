use crate::components::base64_encoder::Base64Encoder;
use crate::components::char_counter::CharCounter;
use crate::components::command_palette::{CommandPalette, ToolItem};
use crate::components::csv_viewer::CsvViewer;
use crate::components::image_compressor::ImageCompressor;
use crate::components::image_editor::ImageEditor;
use crate::components::json_formatter::JsonFormatter;
use crate::components::kanban_board::KanbanBoardComponent;
use crate::components::language_switcher::LanguageSwitcher;
use crate::components::markdown_to_pdf::MarkdownToPdf;
use crate::components::password_generator::PasswordGenerator;
use crate::components::pdf_tools::PdfTools;
use crate::components::regex_tester::RegexTester;
use crate::components::scratch_pad::ScratchPad;
use crate::components::shortcut_dictionary::ShortcutDictionary;
use crate::components::text_diff::TextDiffComponent;
use crate::components::unit_converter::UnitConverter;
use crate::components::unix_time_converter::UnixTimeConverter;
use crate::components::uuid_generator::UuidGenerator;
use crate::i18n::{EN_TRANSLATIONS, JA_TRANSLATIONS};
use i18nrs::yew::{use_translation, I18nProvider, I18nProviderConfig};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"], js_name = listen)]
    async fn tauri_listen(event: &str, handler: &Closure<dyn Fn(JsValue)>) -> JsValue;
}

#[derive(Clone, PartialEq, Copy)]
enum Tab {
    ImageCompressor,
    ImageEditor,
    CsvViewer,
    PdfTools,
    MarkdownToPdf,
    KanbanBoard,
    ScratchPad,
    UuidGenerator,
    PasswordGenerator,
    UnitConverter,
    UnixTimeConverter,
    TextDiff,
    RegexTester,
    JsonFormatter,
    Base64Encoder,
    ShortcutDictionary,
    CharCounter,
}

impl Tab {
    fn translation_key(&self) -> &'static str {
        match self {
            Tab::ImageCompressor => "app.tabs.compress",
            Tab::ImageEditor => "app.tabs.edit",
            Tab::CsvViewer => "app.tabs.csv",
            Tab::PdfTools => "app.tabs.pdf",
            Tab::MarkdownToPdf => "app.tabs.markdown",
            Tab::KanbanBoard => "app.tabs.kanban",
            Tab::ScratchPad => "app.tabs.notes",
            Tab::UuidGenerator => "app.tabs.uuid",
            Tab::PasswordGenerator => "app.tabs.password",
            Tab::UnitConverter => "app.tabs.unit",
            Tab::UnixTimeConverter => "app.tabs.unix_time",
            Tab::TextDiff => "app.tabs.diff",
            Tab::RegexTester => "app.tabs.regex",
            Tab::JsonFormatter => "app.tabs.json",
            Tab::Base64Encoder => "app.tabs.base64",
            Tab::ShortcutDictionary => "app.tabs.shortcut_dictionary",
            Tab::CharCounter => "app.tabs.char_counter",
        }
    }

    fn id(&self) -> &'static str {
        match self {
            Tab::ImageCompressor => "image_compressor",
            Tab::ImageEditor => "image_editor",
            Tab::CsvViewer => "csv_viewer",
            Tab::PdfTools => "pdf_tools",
            Tab::MarkdownToPdf => "markdown_to_pdf",
            Tab::KanbanBoard => "kanban_board",
            Tab::ScratchPad => "scratch_pad",
            Tab::UuidGenerator => "uuid_generator",
            Tab::PasswordGenerator => "password_generator",
            Tab::UnitConverter => "unit_converter",
            Tab::UnixTimeConverter => "unix_time_converter",
            Tab::TextDiff => "text_diff",
            Tab::RegexTester => "regex_tester",
            Tab::JsonFormatter => "json_formatter",
            Tab::Base64Encoder => "base64_encoder",
            Tab::ShortcutDictionary => "shortcut_dictionary",
            Tab::CharCounter => "char_counter",
        }
    }

    fn from_id(id: &str) -> Option<Tab> {
        match id {
            "image_compressor" => Some(Tab::ImageCompressor),
            "image_editor" => Some(Tab::ImageEditor),
            "csv_viewer" => Some(Tab::CsvViewer),
            "pdf_tools" => Some(Tab::PdfTools),
            "markdown_to_pdf" => Some(Tab::MarkdownToPdf),
            "kanban_board" => Some(Tab::KanbanBoard),
            "scratch_pad" => Some(Tab::ScratchPad),
            "uuid_generator" => Some(Tab::UuidGenerator),
            "password_generator" => Some(Tab::PasswordGenerator),
            "unit_converter" => Some(Tab::UnitConverter),
            "unix_time_converter" => Some(Tab::UnixTimeConverter),
            "text_diff" => Some(Tab::TextDiff),
            "regex_tester" => Some(Tab::RegexTester),
            "json_formatter" => Some(Tab::JsonFormatter),
            "base64_encoder" => Some(Tab::Base64Encoder),
            "shortcut_dictionary" => Some(Tab::ShortcutDictionary),
            "char_counter" => Some(Tab::CharCounter),
            _ => None,
        }
    }

    fn description_key(&self) -> &'static str {
        match self {
            Tab::ImageCompressor => "command_palette.desc.compress",
            Tab::ImageEditor => "command_palette.desc.edit",
            Tab::CsvViewer => "command_palette.desc.csv",
            Tab::PdfTools => "command_palette.desc.pdf",
            Tab::MarkdownToPdf => "command_palette.desc.markdown",
            Tab::KanbanBoard => "command_palette.desc.kanban",
            Tab::ScratchPad => "command_palette.desc.notes",
            Tab::UuidGenerator => "command_palette.desc.uuid",
            Tab::PasswordGenerator => "command_palette.desc.password",
            Tab::UnitConverter => "command_palette.desc.unit",
            Tab::UnixTimeConverter => "command_palette.desc.unix_time",
            Tab::TextDiff => "command_palette.desc.diff",
            Tab::RegexTester => "command_palette.desc.regex",
            Tab::JsonFormatter => "command_palette.desc.json",
            Tab::Base64Encoder => "command_palette.desc.base64",
            Tab::ShortcutDictionary => "command_palette.desc.shortcut_dictionary",
            Tab::CharCounter => "command_palette.desc.char_counter",
        }
    }

    fn keywords(&self) -> Vec<String> {
        match self {
            Tab::ImageCompressor => vec![
                "image".into(),
                "compress".into(),
                "png".into(),
                "jpeg".into(),
                "webp".into(),
                "avif".into(),
                "画像".into(),
                "圧縮".into(),
            ],
            Tab::ImageEditor => vec![
                "image".into(),
                "edit".into(),
                "resize".into(),
                "crop".into(),
                "rotate".into(),
                "filter".into(),
                "画像".into(),
                "編集".into(),
                "リサイズ".into(),
            ],
            Tab::CsvViewer => vec![
                "csv".into(),
                "tsv".into(),
                "table".into(),
                "spreadsheet".into(),
                "テーブル".into(),
            ],
            Tab::PdfTools => vec![
                "pdf".into(),
                "split".into(),
                "merge".into(),
                "分割".into(),
                "結合".into(),
            ],
            Tab::MarkdownToPdf => vec![
                "markdown".into(),
                "md".into(),
                "pdf".into(),
                "convert".into(),
                "変換".into(),
            ],
            Tab::KanbanBoard => vec![
                "kanban".into(),
                "task".into(),
                "board".into(),
                "タスク".into(),
                "ボード".into(),
            ],
            Tab::ScratchPad => vec![
                "note".into(),
                "memo".into(),
                "scratch".into(),
                "メモ".into(),
                "ノート".into(),
            ],
            Tab::UuidGenerator => vec![
                "uuid".into(),
                "guid".into(),
                "generate".into(),
                "v4".into(),
                "v7".into(),
                "生成".into(),
            ],
            Tab::PasswordGenerator => vec![
                "password".into(),
                "passphrase".into(),
                "generate".into(),
                "security".into(),
                "パスワード".into(),
                "生成".into(),
            ],
            Tab::UnitConverter => vec![
                "unit".into(),
                "convert".into(),
                "length".into(),
                "weight".into(),
                "temperature".into(),
                "単位".into(),
                "変換".into(),
            ],
            Tab::UnixTimeConverter => vec![
                "unix".into(),
                "time".into(),
                "timestamp".into(),
                "epoch".into(),
                "datetime".into(),
                "時間".into(),
            ],
            Tab::TextDiff => vec![
                "diff".into(),
                "compare".into(),
                "text".into(),
                "差分".into(),
                "比較".into(),
            ],
            Tab::RegexTester => vec![
                "regex".into(),
                "regular".into(),
                "expression".into(),
                "pattern".into(),
                "test".into(),
                "正規表現".into(),
            ],
            Tab::JsonFormatter => vec![
                "json".into(),
                "format".into(),
                "validate".into(),
                "tree".into(),
                "整形".into(),
                "フォーマット".into(),
            ],
            Tab::Base64Encoder => vec![
                "base64".into(),
                "encode".into(),
                "decode".into(),
                "エンコード".into(),
                "デコード".into(),
            ],
            Tab::ShortcutDictionary => vec![
                "shortcut".into(),
                "keybinding".into(),
                "keyboard".into(),
                "hotkey".into(),
                "vscode".into(),
                "intellij".into(),
                "vim".into(),
                "terminal".into(),
                "ショートカット".into(),
                "キーバインド".into(),
            ],
            Tab::CharCounter => vec![
                "char".into(),
                "character".into(),
                "count".into(),
                "counter".into(),
                "byte".into(),
                "word".into(),
                "line".into(),
                "文字数".into(),
                "バイト数".into(),
                "カウント".into(),
                "カウンター".into(),
            ],
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Tab::ImageCompressor => "photo.stack",
            Tab::ImageEditor => "paintbrush",
            Tab::CsvViewer => "tablecells",
            Tab::PdfTools => "doc.fill",
            Tab::MarkdownToPdf => "doc.text",
            Tab::KanbanBoard => "rectangle.3.group",
            Tab::ScratchPad => "note.text",
            Tab::UuidGenerator => "key.fill",
            Tab::PasswordGenerator => "lock.fill",
            Tab::UnitConverter => "arrow.left.arrow.right",
            Tab::UnixTimeConverter => "clock",
            Tab::TextDiff => "arrow.triangle.branch",
            Tab::RegexTester => "asterisk.circle",
            Tab::JsonFormatter => "curlybraces",
            Tab::Base64Encoder => "doc.badge.gearshape",
            Tab::ShortcutDictionary => "keyboard",
            Tab::CharCounter => "textformat.abc",
        }
    }
}

#[derive(Clone, PartialEq)]
enum Category {
    Media,
    Documents,
    Generators,
    Productivity,
}

impl Category {
    fn translation_key(&self) -> &'static str {
        match self {
            Category::Media => "app.categories.media",
            Category::Documents => "app.categories.documents",
            Category::Generators => "app.categories.generators",
            Category::Productivity => "app.categories.productivity",
        }
    }

    fn tabs(&self) -> Vec<Tab> {
        match self {
            Category::Media => vec![Tab::ImageCompressor, Tab::ImageEditor],
            Category::Documents => vec![
                Tab::CsvViewer,
                Tab::PdfTools,
                Tab::MarkdownToPdf,
                Tab::TextDiff,
                Tab::JsonFormatter,
                Tab::CharCounter,
            ],
            Category::Generators => vec![
                Tab::UuidGenerator,
                Tab::PasswordGenerator,
                Tab::UnitConverter,
                Tab::UnixTimeConverter,
                Tab::RegexTester,
                Tab::Base64Encoder,
            ],
            Category::Productivity => {
                vec![Tab::KanbanBoard, Tab::ScratchPad, Tab::ShortcutDictionary]
            }
        }
    }
}

fn get_file_extension(path: &str) -> Option<String> {
    path.rsplit('.').next().map(|s| s.to_lowercase())
}

fn is_image_file(path: &str) -> bool {
    matches!(
        get_file_extension(path).as_deref(),
        Some("png")
            | Some("jpg")
            | Some("jpeg")
            | Some("webp")
            | Some("avif")
            | Some("gif")
            | Some("bmp")
    )
}

fn is_csv_file(path: &str) -> bool {
    matches!(
        get_file_extension(path).as_deref(),
        Some("csv") | Some("tsv")
    )
}

fn is_pdf_file(path: &str) -> bool {
    matches!(get_file_extension(path).as_deref(), Some("pdf"))
}

fn is_markdown_file(path: &str) -> bool {
    matches!(
        get_file_extension(path).as_deref(),
        Some("md") | Some("markdown")
    )
}

fn is_json_file(path: &str) -> bool {
    matches!(get_file_extension(path).as_deref(), Some("json"))
}

fn is_text_file(path: &str) -> bool {
    matches!(
        get_file_extension(path).as_deref(),
        Some("txt")
            | Some("text")
            | Some("log")
            | Some("xml")
            | Some("yaml")
            | Some("yml")
            | Some("toml")
            | Some("ini")
            | Some("cfg")
            | Some("conf")
            | Some("rs")
            | Some("js")
            | Some("ts")
            | Some("py")
            | Some("go")
            | Some("java")
            | Some("c")
            | Some("cpp")
            | Some("h")
            | Some("hpp")
            | Some("html")
            | Some("css")
            | Some("scss")
            | Some("sass")
            | Some("less")
            | Some("sh")
            | Some("bash")
            | Some("zsh")
            | Some("sql")
            | Some("rb")
            | Some("php")
            | Some("swift")
            | Some("kt")
            | Some("scala")
            | Some("ex")
            | Some("exs")
            | Some("erl")
            | Some("hs")
            | Some("ml")
            | Some("clj")
            | Some("lisp")
            | Some("el")
            | Some("vim")
            | Some("lua")
            | Some("r")
            | Some("m")
            | Some("mm")
            | Some("pl")
            | Some("pm")
    )
}

#[function_component(App)]
pub fn app() -> Html {
    let translations = HashMap::from([("en", EN_TRANSLATIONS), ("ja", JA_TRANSLATIONS)]);

    let config = I18nProviderConfig {
        translations,
        default_language: "ja".to_string(),
        ..Default::default()
    };

    html! {
        <I18nProvider ..config>
            <AppInner />
        </I18nProvider>
    }
}

#[function_component(AppInner)]
fn app_inner() -> Html {
    let (i18n, _set_language) = use_translation();
    let active_tab = use_state(|| Tab::ImageCompressor);
    let sidebar_collapsed = use_state(|| false);
    let command_palette_visible = use_state(|| false);
    let dropped_image_path = use_state(|| Option::<String>::None);
    let dropped_editor_path = use_state(|| Option::<String>::None);
    let dropped_csv_path = use_state(|| Option::<String>::None);
    let dropped_pdf_path = use_state(|| Option::<String>::None);
    let dropped_markdown_path = use_state(|| Option::<String>::None);
    let dropped_text_path = use_state(|| Option::<String>::None);
    let dropped_json_path = use_state(|| Option::<String>::None);
    let dropped_base64_image_path = use_state(|| Option::<String>::None);

    // Set up drag-drop event listeners (only once on mount)
    {
        let active_tab = active_tab.clone();
        let dropped_image_path = dropped_image_path.clone();
        let dropped_editor_path = dropped_editor_path.clone();
        let dropped_csv_path = dropped_csv_path.clone();
        let dropped_pdf_path = dropped_pdf_path.clone();
        let dropped_markdown_path = dropped_markdown_path.clone();
        let dropped_text_path = dropped_text_path.clone();
        let dropped_json_path = dropped_json_path.clone();
        let dropped_base64_image_path = dropped_base64_image_path.clone();

        use_effect_with((), move |_| {
            let active_tab = active_tab.clone();
            let dropped_image_path = dropped_image_path.clone();
            let dropped_editor_path = dropped_editor_path.clone();
            let dropped_csv_path = dropped_csv_path.clone();
            let dropped_pdf_path = dropped_pdf_path.clone();
            let dropped_markdown_path = dropped_markdown_path.clone();
            let dropped_text_path = dropped_text_path.clone();
            let dropped_json_path = dropped_json_path.clone();
            let dropped_base64_image_path = dropped_base64_image_path.clone();

            spawn_local(async move {
                let drop_handler = {
                    let active_tab = active_tab.clone();
                    let dropped_image_path = dropped_image_path.clone();
                    let dropped_editor_path = dropped_editor_path.clone();
                    let dropped_csv_path = dropped_csv_path.clone();
                    let dropped_pdf_path = dropped_pdf_path.clone();
                    let dropped_markdown_path = dropped_markdown_path.clone();
                    let dropped_text_path = dropped_text_path.clone();
                    let dropped_json_path = dropped_json_path.clone();
                    let dropped_base64_image_path = dropped_base64_image_path.clone();
                    Closure::new(move |event: JsValue| {
                        if let Ok(paths) = serde_wasm_bindgen::from_value::<DropEvent>(event) {
                            if let Some(first_path) = paths.payload.first() {
                                if is_image_file(first_path) {
                                    if *active_tab == Tab::ImageEditor {
                                        dropped_editor_path.set(Some(first_path.clone()));
                                    } else if *active_tab == Tab::Base64Encoder {
                                        dropped_base64_image_path.set(Some(first_path.clone()));
                                    } else {
                                        dropped_image_path.set(Some(first_path.clone()));
                                        active_tab.set(Tab::ImageCompressor);
                                    }
                                } else if is_csv_file(first_path) {
                                    dropped_csv_path.set(Some(first_path.clone()));
                                    active_tab.set(Tab::CsvViewer);
                                } else if is_pdf_file(first_path) {
                                    dropped_pdf_path.set(Some(first_path.clone()));
                                    active_tab.set(Tab::PdfTools);
                                } else if is_markdown_file(first_path) {
                                    dropped_markdown_path.set(Some(first_path.clone()));
                                    active_tab.set(Tab::MarkdownToPdf);
                                } else if is_json_file(first_path) {
                                    dropped_json_path.set(Some(first_path.clone()));
                                    active_tab.set(Tab::JsonFormatter);
                                } else if is_text_file(first_path) || *active_tab == Tab::TextDiff {
                                    dropped_text_path.set(Some(first_path.clone()));
                                    active_tab.set(Tab::TextDiff);
                                }
                            }
                        }
                    })
                };
                let _ = tauri_listen("file-drop", &drop_handler).await;
                drop_handler.forget();
            });

            || {}
        });
    }

    // Set up Cmd+K / Ctrl+K keyboard shortcut for command palette
    {
        let command_palette_visible = command_palette_visible.clone();
        use_effect_with((), move |_| {
            let command_palette_visible = command_palette_visible.clone();
            let closure =
                Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
                    if (e.meta_key() || e.ctrl_key()) && e.key() == "k" {
                        e.prevent_default();
                        command_palette_visible.set(!*command_palette_visible);
                    }
                });
            let window = web_sys::window().unwrap();
            let _ = window
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
            closure.forget();
            || {}
        });
    }

    let on_tab_click = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: Tab| {
            active_tab.set(tab);
        })
    };

    let on_toggle_sidebar = {
        let sidebar_collapsed = sidebar_collapsed.clone();
        Callback::from(move |_| {
            sidebar_collapsed.set(!*sidebar_collapsed);
        })
    };

    let on_image_file_processed = {
        let dropped_image_path = dropped_image_path.clone();
        Callback::from(move |_| {
            dropped_image_path.set(None);
        })
    };

    let on_csv_file_processed = {
        let dropped_csv_path = dropped_csv_path.clone();
        Callback::from(move |_| {
            dropped_csv_path.set(None);
        })
    };

    let on_pdf_file_processed = {
        let dropped_pdf_path = dropped_pdf_path.clone();
        Callback::from(move |_| {
            dropped_pdf_path.set(None);
        })
    };

    let on_editor_file_processed = {
        let dropped_editor_path = dropped_editor_path.clone();
        Callback::from(move |_| {
            dropped_editor_path.set(None);
        })
    };

    let on_markdown_file_processed = {
        let dropped_markdown_path = dropped_markdown_path.clone();
        Callback::from(move |_| {
            dropped_markdown_path.set(None);
        })
    };

    let on_text_file_processed = {
        let dropped_text_path = dropped_text_path.clone();
        Callback::from(move |_| {
            dropped_text_path.set(None);
        })
    };

    let on_json_file_processed = {
        let dropped_json_path = dropped_json_path.clone();
        Callback::from(move |_| {
            dropped_json_path.set(None);
        })
    };

    let on_base64_image_file_processed = {
        let dropped_base64_image_path = dropped_base64_image_path.clone();
        Callback::from(move |_| {
            dropped_base64_image_path.set(None);
        })
    };

    let categories = vec![
        Category::Media,
        Category::Documents,
        Category::Generators,
        Category::Productivity,
    ];

    let on_palette_close = {
        let command_palette_visible = command_palette_visible.clone();
        Callback::from(move |_| {
            command_palette_visible.set(false);
        })
    };

    let on_palette_select = {
        let active_tab = active_tab.clone();
        let command_palette_visible = command_palette_visible.clone();
        Callback::from(move |id: String| {
            if let Some(tab) = Tab::from_id(&id) {
                active_tab.set(tab);
            }
            command_palette_visible.set(false);
        })
    };

    let tool_items: Vec<ToolItem> = {
        let all_tabs = vec![
            Tab::ImageCompressor,
            Tab::ImageEditor,
            Tab::CsvViewer,
            Tab::PdfTools,
            Tab::MarkdownToPdf,
            Tab::KanbanBoard,
            Tab::ScratchPad,
            Tab::UuidGenerator,
            Tab::PasswordGenerator,
            Tab::UnitConverter,
            Tab::UnixTimeConverter,
            Tab::TextDiff,
            Tab::RegexTester,
            Tab::JsonFormatter,
            Tab::Base64Encoder,
            Tab::ShortcutDictionary,
            Tab::CharCounter,
        ];
        all_tabs
            .iter()
            .map(|tab| {
                let category_name = match tab {
                    Tab::ImageCompressor | Tab::ImageEditor => i18n.t("app.categories.media"),
                    Tab::CsvViewer
                    | Tab::PdfTools
                    | Tab::MarkdownToPdf
                    | Tab::TextDiff
                    | Tab::JsonFormatter
                    | Tab::CharCounter => i18n.t("app.categories.documents"),
                    Tab::UuidGenerator
                    | Tab::PasswordGenerator
                    | Tab::UnitConverter
                    | Tab::UnixTimeConverter
                    | Tab::RegexTester
                    | Tab::Base64Encoder => i18n.t("app.categories.generators"),
                    Tab::KanbanBoard | Tab::ScratchPad | Tab::ShortcutDictionary => {
                        i18n.t("app.categories.productivity")
                    }
                };
                ToolItem {
                    id: tab.id().to_string(),
                    name: i18n.t(tab.translation_key()).to_string(),
                    description: i18n.t(tab.description_key()).to_string(),
                    category: category_name.to_string(),
                    icon: tab.icon().to_string(),
                    keywords: tab.keywords(),
                }
            })
            .collect()
    };

    let sidebar_class = if *sidebar_collapsed {
        "sidebar collapsed"
    } else {
        "sidebar"
    };

    html! {
        <div class="app-layout">
            <CommandPalette
                visible={*command_palette_visible}
                on_close={on_palette_close}
                on_select={on_palette_select}
                tools={tool_items}
            />
            <aside class={sidebar_class}>
                <div class="sidebar-header">
                    <h1 class="sidebar-title">
                        if !*sidebar_collapsed {
                            {"Taurin"}
                        }
                    </h1>
                    <button class="sidebar-toggle" onclick={on_toggle_sidebar}>
                        if *sidebar_collapsed {
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M9 18l6-6-6-6"/>
                            </svg>
                        } else {
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M15 18l-6-6 6-6"/>
                            </svg>
                        }
                    </button>
                </div>
                <nav class="sidebar-nav">
                    { for categories.iter().map(|category| {
                        let tabs = category.tabs();
                        let category_label = i18n.t(category.translation_key());
                        html! {
                            <div class="nav-group">
                                if !*sidebar_collapsed {
                                    <div class="nav-group-label">{category_label}</div>
                                }
                                <div class="nav-items">
                                    { for tabs.iter().map(|tab| {
                                        let is_active = *active_tab == *tab;
                                        let on_click = on_tab_click.clone();
                                        let t = *tab;
                                        let tab_label = i18n.t(tab.translation_key());
                                        html! {
                                            <button
                                                class={classes!("nav-item", is_active.then_some("active"))}
                                                onclick={Callback::from(move |_| on_click.emit(t))}
                                                title={tab_label.clone()}
                                            >
                                                <span class="nav-icon">{render_icon(tab.icon())}</span>
                                                if !*sidebar_collapsed {
                                                    <span class="nav-label">{tab_label}</span>
                                                }
                                            </button>
                                        }
                                    })}
                                </div>
                            </div>
                        }
                    })}
                </nav>
                <div class="sidebar-footer">
                    <LanguageSwitcher />
                </div>
            </aside>
            <main class="main-content">
                <div class={if *active_tab == Tab::ImageCompressor { "content-panel active" } else { "content-panel" }}>
                    <ImageCompressor
                        dropped_file={(*dropped_image_path).clone()}
                        on_file_processed={on_image_file_processed}
                    />
                </div>
                <div class={if *active_tab == Tab::ImageEditor { "content-panel active" } else { "content-panel" }}>
                    <ImageEditor
                        dropped_file={(*dropped_editor_path).clone()}
                        on_file_processed={on_editor_file_processed}
                    />
                </div>
                <div class={if *active_tab == Tab::CsvViewer { "content-panel active" } else { "content-panel" }}>
                    <CsvViewer
                        dropped_file={(*dropped_csv_path).clone()}
                        on_file_processed={on_csv_file_processed}
                    />
                </div>
                <div class={if *active_tab == Tab::PdfTools { "content-panel active" } else { "content-panel" }}>
                    <PdfTools
                        dropped_file={(*dropped_pdf_path).clone()}
                        on_file_processed={on_pdf_file_processed}
                    />
                </div>
                <div class={if *active_tab == Tab::MarkdownToPdf { "content-panel active" } else { "content-panel" }}>
                    <MarkdownToPdf
                        dropped_file={(*dropped_markdown_path).clone()}
                        on_file_processed={on_markdown_file_processed}
                    />
                </div>
                <div class={if *active_tab == Tab::KanbanBoard { "content-panel active" } else { "content-panel" }}>
                    <KanbanBoardComponent />
                </div>
                <div class={if *active_tab == Tab::ScratchPad { "content-panel active" } else { "content-panel" }}>
                    <ScratchPad />
                </div>
                <div class={if *active_tab == Tab::UuidGenerator { "content-panel active" } else { "content-panel" }}>
                    <UuidGenerator />
                </div>
                <div class={if *active_tab == Tab::PasswordGenerator { "content-panel active" } else { "content-panel" }}>
                    <PasswordGenerator />
                </div>
                <div class={if *active_tab == Tab::UnitConverter { "content-panel active" } else { "content-panel" }}>
                    <UnitConverter />
                </div>
                <div class={if *active_tab == Tab::UnixTimeConverter { "content-panel active" } else { "content-panel" }}>
                    <UnixTimeConverter />
                </div>
                <div class={if *active_tab == Tab::TextDiff { "content-panel active" } else { "content-panel" }}>
                    <TextDiffComponent
                        dropped_file={(*dropped_text_path).clone()}
                        on_file_processed={on_text_file_processed}
                    />
                </div>
                <div class={if *active_tab == Tab::RegexTester { "content-panel active" } else { "content-panel" }}>
                    <RegexTester />
                </div>
                <div class={if *active_tab == Tab::JsonFormatter { "content-panel active" } else { "content-panel" }}>
                    <JsonFormatter
                        dropped_file={(*dropped_json_path).clone()}
                        on_file_processed={on_json_file_processed}
                    />
                </div>
                <div class={if *active_tab == Tab::Base64Encoder { "content-panel active" } else { "content-panel" }}>
                    <Base64Encoder
                        dropped_file={(*dropped_base64_image_path).clone()}
                        on_file_processed={on_base64_image_file_processed}
                    />
                </div>
                <div class={if *active_tab == Tab::ShortcutDictionary { "content-panel active" } else { "content-panel" }}>
                    <ShortcutDictionary />
                </div>
                <div class={if *active_tab == Tab::CharCounter { "content-panel active" } else { "content-panel" }}>
                    <CharCounter />
                </div>
            </main>
        </div>
    }
}

fn render_icon(name: &str) -> Html {
    match name {
        "photo.stack" => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <rect x="3" y="3" width="18" height="18" rx="2"/>
                <circle cx="8.5" cy="8.5" r="1.5"/>
                <path d="M21 15l-5-5L5 21"/>
            </svg>
        },
        "paintbrush" => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M18.37 2.63L14 7l-1.59-1.59a2 2 0 00-2.82 0L8 7l9 9 1.59-1.59a2 2 0 000-2.82L17 10l4.37-4.37a2.12 2.12 0 10-3-3z"/>
                <path d="M9 8c-2 3-4 3.5-7 4l8 10c2-1 6-5 6-7"/>
            </svg>
        },
        "tablecells" => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <rect x="3" y="3" width="18" height="18" rx="2"/>
                <line x1="3" y1="9" x2="21" y2="9"/>
                <line x1="3" y1="15" x2="21" y2="15"/>
                <line x1="9" y1="3" x2="9" y2="21"/>
                <line x1="15" y1="3" x2="15" y2="21"/>
            </svg>
        },
        "doc.fill" => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z"/>
                <path d="M14 2v6h6" fill="none" stroke="currentColor" stroke-width="1.5"/>
            </svg>
        },
        "doc.text" => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z"/>
                <path d="M14 2v6h6"/>
                <line x1="8" y1="13" x2="16" y2="13"/>
                <line x1="8" y1="17" x2="16" y2="17"/>
            </svg>
        },
        "rectangle.3.group" => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <rect x="3" y="3" width="5" height="18" rx="1"/>
                <rect x="10" y="3" width="5" height="18" rx="1"/>
                <rect x="17" y="3" width="5" height="18" rx="1"/>
            </svg>
        },
        "key.fill" => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                <path d="M21 2l-2 2m-7.61 7.61a5.5 5.5 0 11-7.778 7.778 5.5 5.5 0 017.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4"/>
            </svg>
        },
        "lock.fill" => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                <rect x="3" y="11" width="18" height="11" rx="2"/>
                <path d="M7 11V7a5 5 0 0110 0v4" fill="none" stroke="currentColor" stroke-width="1.5"/>
            </svg>
        },
        "arrow.left.arrow.right" => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <line x1="5" y1="12" x2="19" y2="12"/>
                <polyline points="12 5 19 12 12 19"/>
                <line x1="19" y1="12" x2="5" y2="12"/>
                <polyline points="12 19 5 12 12 5"/>
            </svg>
        },
        "clock" => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <circle cx="12" cy="12" r="10"/>
                <polyline points="12 6 12 12 16 14"/>
            </svg>
        },
        "arrow.triangle.branch" => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <line x1="6" y1="3" x2="6" y2="15"/>
                <circle cx="18" cy="6" r="3"/>
                <circle cx="6" cy="18" r="3"/>
                <path d="M18 9a9 9 0 01-9 9"/>
            </svg>
        },
        "note.text" => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z"/>
                <path d="M14 2v6h6"/>
                <line x1="8" y1="13" x2="16" y2="13"/>
                <line x1="8" y1="17" x2="13" y2="17"/>
            </svg>
        },
        "clipboard" => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M16 4h2a2 2 0 012 2v14a2 2 0 01-2 2H6a2 2 0 01-2-2V6a2 2 0 012-2h2"/>
                <rect x="8" y="2" width="8" height="4" rx="1"/>
            </svg>
        },
        "asterisk.circle" => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <circle cx="12" cy="12" r="10"/>
                <line x1="12" y1="6" x2="12" y2="18"/>
                <line x1="6.5" y1="9" x2="17.5" y2="15"/>
                <line x1="6.5" y1="15" x2="17.5" y2="9"/>
            </svg>
        },
        "curlybraces" => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M8 3H6a2 2 0 00-2 2v4a2 2 0 01-2 2 2 2 0 012 2v4a2 2 0 002 2h2"/>
                <path d="M16 3h2a2 2 0 012 2v4a2 2 0 002 2 2 2 0 00-2 2v4a2 2 0 01-2 2h-2"/>
            </svg>
        },
        "keyboard" => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <rect x="2" y="4" width="20" height="16" rx="2"/>
                <line x1="6" y1="8" x2="6" y2="8"/>
                <line x1="10" y1="8" x2="10" y2="8"/>
                <line x1="14" y1="8" x2="14" y2="8"/>
                <line x1="18" y1="8" x2="18" y2="8"/>
                <line x1="6" y1="12" x2="6" y2="12"/>
                <line x1="10" y1="12" x2="10" y2="12"/>
                <line x1="14" y1="12" x2="14" y2="12"/>
                <line x1="18" y1="12" x2="18" y2="12"/>
                <line x1="7" y1="16" x2="17" y2="16"/>
            </svg>
        },
        "doc.badge.gearshape" => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z"/>
                <path d="M14 2v6h6"/>
                <text x="8" y="17" font-size="8" font-weight="bold" fill="currentColor">{"64"}</text>
            </svg>
        },
        "textformat.abc" => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M3 7V5a2 2 0 012-2h14a2 2 0 012 2v2"/>
                <path d="M12 3v18"/>
                <path d="M8 21h8"/>
                <text x="2" y="15" font-size="7" font-weight="bold" fill="currentColor" stroke="none">{"#"}</text>
            </svg>
        },
        _ => html! {
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <circle cx="12" cy="12" r="10"/>
            </svg>
        },
    }
}

#[derive(serde::Deserialize)]
struct DropEvent {
    payload: Vec<String>,
}
