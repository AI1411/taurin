use i18nrs::yew::use_translation;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum AppType {
    VSCode,
    IntelliJ,
    Vim,
    Terminal,
}

impl AppType {
    fn all() -> Vec<AppType> {
        vec![
            AppType::VSCode,
            AppType::IntelliJ,
            AppType::Vim,
            AppType::Terminal,
        ]
    }

    fn translation_key(&self) -> &'static str {
        match self {
            AppType::VSCode => "shortcut_dictionary.app_vscode",
            AppType::IntelliJ => "shortcut_dictionary.app_intellij",
            AppType::Vim => "shortcut_dictionary.app_vim",
            AppType::Terminal => "shortcut_dictionary.app_terminal",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            AppType::VSCode => "VS",
            AppType::IntelliJ => "IJ",
            AppType::Vim => "Vi",
            AppType::Terminal => ">_",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum OsType {
    Mac,
    Windows,
    Linux,
}

impl OsType {
    fn all() -> Vec<OsType> {
        vec![OsType::Mac, OsType::Windows, OsType::Linux]
    }

    fn translation_key(&self) -> &'static str {
        match self {
            OsType::Mac => "shortcut_dictionary.os_mac",
            OsType::Windows => "shortcut_dictionary.os_windows",
            OsType::Linux => "shortcut_dictionary.os_linux",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ShortcutCategory {
    All,
    Editing,
    Navigation,
    Search,
    Debug,
    Terminal,
    View,
    File,
}

impl ShortcutCategory {
    fn all() -> Vec<ShortcutCategory> {
        vec![
            ShortcutCategory::All,
            ShortcutCategory::Editing,
            ShortcutCategory::Navigation,
            ShortcutCategory::Search,
            ShortcutCategory::Debug,
            ShortcutCategory::Terminal,
            ShortcutCategory::View,
            ShortcutCategory::File,
        ]
    }

    fn translation_key(&self) -> &'static str {
        match self {
            ShortcutCategory::All => "shortcut_dictionary.category_all",
            ShortcutCategory::Editing => "shortcut_dictionary.category_editing",
            ShortcutCategory::Navigation => "shortcut_dictionary.category_navigation",
            ShortcutCategory::Search => "shortcut_dictionary.category_search",
            ShortcutCategory::Debug => "shortcut_dictionary.category_debug",
            ShortcutCategory::Terminal => "shortcut_dictionary.category_terminal",
            ShortcutCategory::View => "shortcut_dictionary.category_view",
            ShortcutCategory::File => "shortcut_dictionary.category_file",
        }
    }
}

#[derive(Clone)]
struct ShortcutEntry {
    action_en: &'static str,
    action_ja: &'static str,
    key_mac: &'static str,
    key_win: &'static str,
    key_linux: &'static str,
    category: ShortcutCategory,
}

fn get_vscode_shortcuts() -> Vec<ShortcutEntry> {
    vec![
        // Editing
        ShortcutEntry {
            action_en: "Cut line",
            action_ja: "行の切り取り",
            key_mac: "Cmd+X",
            key_win: "Ctrl+X",
            key_linux: "Ctrl+X",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Copy line",
            action_ja: "行のコピー",
            key_mac: "Cmd+C",
            key_win: "Ctrl+C",
            key_linux: "Ctrl+C",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Move line up",
            action_ja: "行を上に移動",
            key_mac: "Option+\u{2191}",
            key_win: "Alt+\u{2191}",
            key_linux: "Alt+\u{2191}",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Move line down",
            action_ja: "行を下に移動",
            key_mac: "Option+\u{2193}",
            key_win: "Alt+\u{2193}",
            key_linux: "Alt+\u{2193}",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Copy line up",
            action_ja: "行を上にコピー",
            key_mac: "Shift+Option+\u{2191}",
            key_win: "Shift+Alt+\u{2191}",
            key_linux: "Shift+Alt+\u{2191}",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Copy line down",
            action_ja: "行を下にコピー",
            key_mac: "Shift+Option+\u{2193}",
            key_win: "Shift+Alt+\u{2193}",
            key_linux: "Shift+Alt+\u{2193}",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Delete line",
            action_ja: "行の削除",
            key_mac: "Cmd+Shift+K",
            key_win: "Ctrl+Shift+K",
            key_linux: "Ctrl+Shift+K",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Insert line below",
            action_ja: "下に空行を挿入",
            key_mac: "Cmd+Enter",
            key_win: "Ctrl+Enter",
            key_linux: "Ctrl+Enter",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Insert line above",
            action_ja: "上に空行を挿入",
            key_mac: "Cmd+Shift+Enter",
            key_win: "Ctrl+Shift+Enter",
            key_linux: "Ctrl+Shift+Enter",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Indent line",
            action_ja: "行のインデント",
            key_mac: "Cmd+]",
            key_win: "Ctrl+]",
            key_linux: "Ctrl+]",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Outdent line",
            action_ja: "行のアウトデント",
            key_mac: "Cmd+[",
            key_win: "Ctrl+[",
            key_linux: "Ctrl+[",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Toggle comment",
            action_ja: "コメントの切り替え",
            key_mac: "Cmd+/",
            key_win: "Ctrl+/",
            key_linux: "Ctrl+/",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Block comment",
            action_ja: "ブロックコメント",
            key_mac: "Shift+Option+A",
            key_win: "Shift+Alt+A",
            key_linux: "Shift+Alt+A",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Undo",
            action_ja: "元に戻す",
            key_mac: "Cmd+Z",
            key_win: "Ctrl+Z",
            key_linux: "Ctrl+Z",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Redo",
            action_ja: "やり直し",
            key_mac: "Cmd+Shift+Z",
            key_win: "Ctrl+Y",
            key_linux: "Ctrl+Shift+Z",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Format document",
            action_ja: "ドキュメントの整形",
            key_mac: "Shift+Option+F",
            key_win: "Shift+Alt+F",
            key_linux: "Shift+Alt+F",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Multi-cursor",
            action_ja: "マルチカーソル",
            key_mac: "Option+Click",
            key_win: "Alt+Click",
            key_linux: "Alt+Click",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Select all occurrences",
            action_ja: "同じ単語をすべて選択",
            key_mac: "Cmd+Shift+L",
            key_win: "Ctrl+Shift+L",
            key_linux: "Ctrl+Shift+L",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Add cursor to next match",
            action_ja: "次の一致にカーソル追加",
            key_mac: "Cmd+D",
            key_win: "Ctrl+D",
            key_linux: "Ctrl+D",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Rename symbol",
            action_ja: "シンボルの名前変更",
            key_mac: "F2",
            key_win: "F2",
            key_linux: "F2",
            category: ShortcutCategory::Editing,
        },
        // Navigation
        ShortcutEntry {
            action_en: "Go to file",
            action_ja: "ファイルに移動",
            key_mac: "Cmd+P",
            key_win: "Ctrl+P",
            key_linux: "Ctrl+P",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Go to line",
            action_ja: "行に移動",
            key_mac: "Ctrl+G",
            key_win: "Ctrl+G",
            key_linux: "Ctrl+G",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Go to symbol",
            action_ja: "シンボルに移動",
            key_mac: "Cmd+Shift+O",
            key_win: "Ctrl+Shift+O",
            key_linux: "Ctrl+Shift+O",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Go to definition",
            action_ja: "定義に移動",
            key_mac: "F12",
            key_win: "F12",
            key_linux: "F12",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Peek definition",
            action_ja: "定義をプレビュー",
            key_mac: "Option+F12",
            key_win: "Alt+F12",
            key_linux: "Alt+F12",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Go back",
            action_ja: "前の位置に戻る",
            key_mac: "Ctrl+-",
            key_win: "Alt+\u{2190}",
            key_linux: "Alt+\u{2190}",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Go forward",
            action_ja: "次の位置に進む",
            key_mac: "Ctrl+Shift+-",
            key_win: "Alt+\u{2192}",
            key_linux: "Alt+\u{2192}",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Go to bracket",
            action_ja: "対応する括弧に移動",
            key_mac: "Cmd+Shift+\\",
            key_win: "Ctrl+Shift+\\",
            key_linux: "Ctrl+Shift+\\",
            category: ShortcutCategory::Navigation,
        },
        // Search
        ShortcutEntry {
            action_en: "Find",
            action_ja: "検索",
            key_mac: "Cmd+F",
            key_win: "Ctrl+F",
            key_linux: "Ctrl+F",
            category: ShortcutCategory::Search,
        },
        ShortcutEntry {
            action_en: "Replace",
            action_ja: "置換",
            key_mac: "Cmd+Option+F",
            key_win: "Ctrl+H",
            key_linux: "Ctrl+H",
            category: ShortcutCategory::Search,
        },
        ShortcutEntry {
            action_en: "Find in files",
            action_ja: "ファイル全体を検索",
            key_mac: "Cmd+Shift+F",
            key_win: "Ctrl+Shift+F",
            key_linux: "Ctrl+Shift+F",
            category: ShortcutCategory::Search,
        },
        ShortcutEntry {
            action_en: "Replace in files",
            action_ja: "ファイル全体を置換",
            key_mac: "Cmd+Shift+H",
            key_win: "Ctrl+Shift+H",
            key_linux: "Ctrl+Shift+H",
            category: ShortcutCategory::Search,
        },
        ShortcutEntry {
            action_en: "Command palette",
            action_ja: "コマンドパレット",
            key_mac: "Cmd+Shift+P",
            key_win: "Ctrl+Shift+P",
            key_linux: "Ctrl+Shift+P",
            category: ShortcutCategory::Search,
        },
        // Debug
        ShortcutEntry {
            action_en: "Start debugging",
            action_ja: "デバッグ開始",
            key_mac: "F5",
            key_win: "F5",
            key_linux: "F5",
            category: ShortcutCategory::Debug,
        },
        ShortcutEntry {
            action_en: "Toggle breakpoint",
            action_ja: "ブレークポイント切り替え",
            key_mac: "F9",
            key_win: "F9",
            key_linux: "F9",
            category: ShortcutCategory::Debug,
        },
        ShortcutEntry {
            action_en: "Step over",
            action_ja: "ステップオーバー",
            key_mac: "F10",
            key_win: "F10",
            key_linux: "F10",
            category: ShortcutCategory::Debug,
        },
        ShortcutEntry {
            action_en: "Step into",
            action_ja: "ステップイン",
            key_mac: "F11",
            key_win: "F11",
            key_linux: "F11",
            category: ShortcutCategory::Debug,
        },
        ShortcutEntry {
            action_en: "Step out",
            action_ja: "ステップアウト",
            key_mac: "Shift+F11",
            key_win: "Shift+F11",
            key_linux: "Shift+F11",
            category: ShortcutCategory::Debug,
        },
        ShortcutEntry {
            action_en: "Stop debugging",
            action_ja: "デバッグ停止",
            key_mac: "Shift+F5",
            key_win: "Shift+F5",
            key_linux: "Shift+F5",
            category: ShortcutCategory::Debug,
        },
        // Terminal
        ShortcutEntry {
            action_en: "Toggle terminal",
            action_ja: "ターミナル切り替え",
            key_mac: "Ctrl+`",
            key_win: "Ctrl+`",
            key_linux: "Ctrl+`",
            category: ShortcutCategory::Terminal,
        },
        ShortcutEntry {
            action_en: "New terminal",
            action_ja: "新しいターミナル",
            key_mac: "Ctrl+Shift+`",
            key_win: "Ctrl+Shift+`",
            key_linux: "Ctrl+Shift+`",
            category: ShortcutCategory::Terminal,
        },
        // View
        ShortcutEntry {
            action_en: "Toggle sidebar",
            action_ja: "サイドバー切り替え",
            key_mac: "Cmd+B",
            key_win: "Ctrl+B",
            key_linux: "Ctrl+B",
            category: ShortcutCategory::View,
        },
        ShortcutEntry {
            action_en: "Explorer",
            action_ja: "エクスプローラー",
            key_mac: "Cmd+Shift+E",
            key_win: "Ctrl+Shift+E",
            key_linux: "Ctrl+Shift+E",
            category: ShortcutCategory::View,
        },
        ShortcutEntry {
            action_en: "Source control",
            action_ja: "ソース管理",
            key_mac: "Ctrl+Shift+G",
            key_win: "Ctrl+Shift+G",
            key_linux: "Ctrl+Shift+G",
            category: ShortcutCategory::View,
        },
        ShortcutEntry {
            action_en: "Extensions",
            action_ja: "拡張機能",
            key_mac: "Cmd+Shift+X",
            key_win: "Ctrl+Shift+X",
            key_linux: "Ctrl+Shift+X",
            category: ShortcutCategory::View,
        },
        ShortcutEntry {
            action_en: "Zoom in",
            action_ja: "拡大",
            key_mac: "Cmd+=",
            key_win: "Ctrl+=",
            key_linux: "Ctrl+=",
            category: ShortcutCategory::View,
        },
        ShortcutEntry {
            action_en: "Zoom out",
            action_ja: "縮小",
            key_mac: "Cmd+-",
            key_win: "Ctrl+-",
            key_linux: "Ctrl+-",
            category: ShortcutCategory::View,
        },
        ShortcutEntry {
            action_en: "Split editor",
            action_ja: "エディター分割",
            key_mac: "Cmd+\\",
            key_win: "Ctrl+\\",
            key_linux: "Ctrl+\\",
            category: ShortcutCategory::View,
        },
        // File
        ShortcutEntry {
            action_en: "New file",
            action_ja: "新規ファイル",
            key_mac: "Cmd+N",
            key_win: "Ctrl+N",
            key_linux: "Ctrl+N",
            category: ShortcutCategory::File,
        },
        ShortcutEntry {
            action_en: "Save",
            action_ja: "保存",
            key_mac: "Cmd+S",
            key_win: "Ctrl+S",
            key_linux: "Ctrl+S",
            category: ShortcutCategory::File,
        },
        ShortcutEntry {
            action_en: "Save as",
            action_ja: "名前を付けて保存",
            key_mac: "Cmd+Shift+S",
            key_win: "Ctrl+Shift+S",
            key_linux: "Ctrl+Shift+S",
            category: ShortcutCategory::File,
        },
        ShortcutEntry {
            action_en: "Close tab",
            action_ja: "タブを閉じる",
            key_mac: "Cmd+W",
            key_win: "Ctrl+W",
            key_linux: "Ctrl+W",
            category: ShortcutCategory::File,
        },
        ShortcutEntry {
            action_en: "Reopen closed tab",
            action_ja: "閉じたタブを再度開く",
            key_mac: "Cmd+Shift+T",
            key_win: "Ctrl+Shift+T",
            key_linux: "Ctrl+Shift+T",
            category: ShortcutCategory::File,
        },
        ShortcutEntry {
            action_en: "Open settings",
            action_ja: "設定を開く",
            key_mac: "Cmd+,",
            key_win: "Ctrl+,",
            key_linux: "Ctrl+,",
            category: ShortcutCategory::File,
        },
    ]
}

fn get_intellij_shortcuts() -> Vec<ShortcutEntry> {
    vec![
        // Editing
        ShortcutEntry {
            action_en: "Duplicate line",
            action_ja: "行の複製",
            key_mac: "Cmd+D",
            key_win: "Ctrl+D",
            key_linux: "Ctrl+D",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Delete line",
            action_ja: "行の削除",
            key_mac: "Cmd+Backspace",
            key_win: "Ctrl+Y",
            key_linux: "Ctrl+Y",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Move line up",
            action_ja: "行を上に移動",
            key_mac: "Shift+Option+\u{2191}",
            key_win: "Shift+Alt+\u{2191}",
            key_linux: "Shift+Alt+\u{2191}",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Move line down",
            action_ja: "行を下に移動",
            key_mac: "Shift+Option+\u{2193}",
            key_win: "Shift+Alt+\u{2193}",
            key_linux: "Shift+Alt+\u{2193}",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Comment line",
            action_ja: "行コメント",
            key_mac: "Cmd+/",
            key_win: "Ctrl+/",
            key_linux: "Ctrl+/",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Block comment",
            action_ja: "ブロックコメント",
            key_mac: "Cmd+Shift+/",
            key_win: "Ctrl+Shift+/",
            key_linux: "Ctrl+Shift+/",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Reformat code",
            action_ja: "コード整形",
            key_mac: "Cmd+Option+L",
            key_win: "Ctrl+Alt+L",
            key_linux: "Ctrl+Alt+L",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Optimize imports",
            action_ja: "インポート最適化",
            key_mac: "Ctrl+Option+O",
            key_win: "Ctrl+Alt+O",
            key_linux: "Ctrl+Alt+O",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Complete code",
            action_ja: "コード補完",
            key_mac: "Ctrl+Space",
            key_win: "Ctrl+Space",
            key_linux: "Ctrl+Space",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Smart completion",
            action_ja: "スマート補完",
            key_mac: "Ctrl+Shift+Space",
            key_win: "Ctrl+Shift+Space",
            key_linux: "Ctrl+Shift+Space",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Generate code",
            action_ja: "コード生成",
            key_mac: "Cmd+N",
            key_win: "Alt+Insert",
            key_linux: "Alt+Insert",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Surround with",
            action_ja: "囲む",
            key_mac: "Cmd+Option+T",
            key_win: "Ctrl+Alt+T",
            key_linux: "Ctrl+Alt+T",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Rename",
            action_ja: "名前変更",
            key_mac: "Shift+F6",
            key_win: "Shift+F6",
            key_linux: "Shift+F6",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Extract variable",
            action_ja: "変数の抽出",
            key_mac: "Cmd+Option+V",
            key_win: "Ctrl+Alt+V",
            key_linux: "Ctrl+Alt+V",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Extract method",
            action_ja: "メソッドの抽出",
            key_mac: "Cmd+Option+M",
            key_win: "Ctrl+Alt+M",
            key_linux: "Ctrl+Alt+M",
            category: ShortcutCategory::Editing,
        },
        // Navigation
        ShortcutEntry {
            action_en: "Go to class",
            action_ja: "クラスに移動",
            key_mac: "Cmd+O",
            key_win: "Ctrl+N",
            key_linux: "Ctrl+N",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Go to file",
            action_ja: "ファイルに移動",
            key_mac: "Cmd+Shift+O",
            key_win: "Ctrl+Shift+N",
            key_linux: "Ctrl+Shift+N",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Go to symbol",
            action_ja: "シンボルに移動",
            key_mac: "Cmd+Option+O",
            key_win: "Ctrl+Alt+Shift+N",
            key_linux: "Ctrl+Alt+Shift+N",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Go to declaration",
            action_ja: "宣言に移動",
            key_mac: "Cmd+B",
            key_win: "Ctrl+B",
            key_linux: "Ctrl+B",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Go to implementation",
            action_ja: "実装に移動",
            key_mac: "Cmd+Option+B",
            key_win: "Ctrl+Alt+B",
            key_linux: "Ctrl+Alt+B",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Go to line",
            action_ja: "行に移動",
            key_mac: "Cmd+L",
            key_win: "Ctrl+G",
            key_linux: "Ctrl+G",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Recent files",
            action_ja: "最近のファイル",
            key_mac: "Cmd+E",
            key_win: "Ctrl+E",
            key_linux: "Ctrl+E",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Navigate back",
            action_ja: "前の位置に戻る",
            key_mac: "Cmd+[",
            key_win: "Ctrl+Alt+\u{2190}",
            key_linux: "Ctrl+Alt+\u{2190}",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Navigate forward",
            action_ja: "次の位置に進む",
            key_mac: "Cmd+]",
            key_win: "Ctrl+Alt+\u{2192}",
            key_linux: "Ctrl+Alt+\u{2192}",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Find usages",
            action_ja: "使用箇所を検索",
            key_mac: "Option+F7",
            key_win: "Alt+F7",
            key_linux: "Alt+F7",
            category: ShortcutCategory::Navigation,
        },
        // Search
        ShortcutEntry {
            action_en: "Search everywhere",
            action_ja: "どこでも検索",
            key_mac: "Shift+Shift",
            key_win: "Shift+Shift",
            key_linux: "Shift+Shift",
            category: ShortcutCategory::Search,
        },
        ShortcutEntry {
            action_en: "Find",
            action_ja: "検索",
            key_mac: "Cmd+F",
            key_win: "Ctrl+F",
            key_linux: "Ctrl+F",
            category: ShortcutCategory::Search,
        },
        ShortcutEntry {
            action_en: "Replace",
            action_ja: "置換",
            key_mac: "Cmd+R",
            key_win: "Ctrl+R",
            key_linux: "Ctrl+R",
            category: ShortcutCategory::Search,
        },
        ShortcutEntry {
            action_en: "Find in path",
            action_ja: "パス内を検索",
            key_mac: "Cmd+Shift+F",
            key_win: "Ctrl+Shift+F",
            key_linux: "Ctrl+Shift+F",
            category: ShortcutCategory::Search,
        },
        ShortcutEntry {
            action_en: "Replace in path",
            action_ja: "パス内を置換",
            key_mac: "Cmd+Shift+R",
            key_win: "Ctrl+Shift+R",
            key_linux: "Ctrl+Shift+R",
            category: ShortcutCategory::Search,
        },
        ShortcutEntry {
            action_en: "Find action",
            action_ja: "アクション検索",
            key_mac: "Cmd+Shift+A",
            key_win: "Ctrl+Shift+A",
            key_linux: "Ctrl+Shift+A",
            category: ShortcutCategory::Search,
        },
        // Debug
        ShortcutEntry {
            action_en: "Debug",
            action_ja: "デバッグ実行",
            key_mac: "Ctrl+D",
            key_win: "Shift+F9",
            key_linux: "Shift+F9",
            category: ShortcutCategory::Debug,
        },
        ShortcutEntry {
            action_en: "Run",
            action_ja: "実行",
            key_mac: "Ctrl+R",
            key_win: "Shift+F10",
            key_linux: "Shift+F10",
            category: ShortcutCategory::Debug,
        },
        ShortcutEntry {
            action_en: "Toggle breakpoint",
            action_ja: "ブレークポイント切り替え",
            key_mac: "Cmd+F8",
            key_win: "Ctrl+F8",
            key_linux: "Ctrl+F8",
            category: ShortcutCategory::Debug,
        },
        ShortcutEntry {
            action_en: "Step over",
            action_ja: "ステップオーバー",
            key_mac: "F8",
            key_win: "F8",
            key_linux: "F8",
            category: ShortcutCategory::Debug,
        },
        ShortcutEntry {
            action_en: "Step into",
            action_ja: "ステップイン",
            key_mac: "F7",
            key_win: "F7",
            key_linux: "F7",
            category: ShortcutCategory::Debug,
        },
        ShortcutEntry {
            action_en: "Step out",
            action_ja: "ステップアウト",
            key_mac: "Shift+F8",
            key_win: "Shift+F8",
            key_linux: "Shift+F8",
            category: ShortcutCategory::Debug,
        },
        ShortcutEntry {
            action_en: "Resume program",
            action_ja: "プログラム再開",
            key_mac: "Cmd+Option+R",
            key_win: "F9",
            key_linux: "F9",
            category: ShortcutCategory::Debug,
        },
        // View
        ShortcutEntry {
            action_en: "Project view",
            action_ja: "プロジェクトビュー",
            key_mac: "Cmd+1",
            key_win: "Alt+1",
            key_linux: "Alt+1",
            category: ShortcutCategory::View,
        },
        ShortcutEntry {
            action_en: "Structure view",
            action_ja: "構造ビュー",
            key_mac: "Cmd+7",
            key_win: "Alt+7",
            key_linux: "Alt+7",
            category: ShortcutCategory::View,
        },
        ShortcutEntry {
            action_en: "Terminal",
            action_ja: "ターミナル",
            key_mac: "Option+F12",
            key_win: "Alt+F12",
            key_linux: "Alt+F12",
            category: ShortcutCategory::Terminal,
        },
        ShortcutEntry {
            action_en: "Version control",
            action_ja: "バージョン管理",
            key_mac: "Cmd+9",
            key_win: "Alt+9",
            key_linux: "Alt+9",
            category: ShortcutCategory::View,
        },
        // File
        ShortcutEntry {
            action_en: "Save all",
            action_ja: "すべて保存",
            key_mac: "Cmd+S",
            key_win: "Ctrl+S",
            key_linux: "Ctrl+S",
            category: ShortcutCategory::File,
        },
        ShortcutEntry {
            action_en: "Close tab",
            action_ja: "タブを閉じる",
            key_mac: "Cmd+W",
            key_win: "Ctrl+F4",
            key_linux: "Ctrl+F4",
            category: ShortcutCategory::File,
        },
        ShortcutEntry {
            action_en: "Open settings",
            action_ja: "設定を開く",
            key_mac: "Cmd+,",
            key_win: "Ctrl+Alt+S",
            key_linux: "Ctrl+Alt+S",
            category: ShortcutCategory::File,
        },
    ]
}

fn get_vim_shortcuts() -> Vec<ShortcutEntry> {
    vec![
        // Editing
        ShortcutEntry {
            action_en: "Insert mode",
            action_ja: "挿入モード",
            key_mac: "i",
            key_win: "i",
            key_linux: "i",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Append after cursor",
            action_ja: "カーソルの後に追加",
            key_mac: "a",
            key_win: "a",
            key_linux: "a",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Open line below",
            action_ja: "下に行を追加",
            key_mac: "o",
            key_win: "o",
            key_linux: "o",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Open line above",
            action_ja: "上に行を追加",
            key_mac: "O",
            key_win: "O",
            key_linux: "O",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Delete character",
            action_ja: "文字削除",
            key_mac: "x",
            key_win: "x",
            key_linux: "x",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Delete word",
            action_ja: "単語削除",
            key_mac: "dw",
            key_win: "dw",
            key_linux: "dw",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Delete line",
            action_ja: "行削除",
            key_mac: "dd",
            key_win: "dd",
            key_linux: "dd",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Yank (copy) line",
            action_ja: "行をコピー",
            key_mac: "yy",
            key_win: "yy",
            key_linux: "yy",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Paste after",
            action_ja: "後に貼り付け",
            key_mac: "p",
            key_win: "p",
            key_linux: "p",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Paste before",
            action_ja: "前に貼り付け",
            key_mac: "P",
            key_win: "P",
            key_linux: "P",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Undo",
            action_ja: "元に戻す",
            key_mac: "u",
            key_win: "u",
            key_linux: "u",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Redo",
            action_ja: "やり直し",
            key_mac: "Ctrl+R",
            key_win: "Ctrl+R",
            key_linux: "Ctrl+R",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Replace character",
            action_ja: "文字置換",
            key_mac: "r",
            key_win: "r",
            key_linux: "r",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Change word",
            action_ja: "単語を変更",
            key_mac: "cw",
            key_win: "cw",
            key_linux: "cw",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Change line",
            action_ja: "行を変更",
            key_mac: "cc",
            key_win: "cc",
            key_linux: "cc",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Visual mode",
            action_ja: "ビジュアルモード",
            key_mac: "v",
            key_win: "v",
            key_linux: "v",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Visual line mode",
            action_ja: "ビジュアル行モード",
            key_mac: "V",
            key_win: "V",
            key_linux: "V",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Visual block mode",
            action_ja: "ビジュアルブロックモード",
            key_mac: "Ctrl+V",
            key_win: "Ctrl+V",
            key_linux: "Ctrl+V",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Indent",
            action_ja: "インデント",
            key_mac: ">>",
            key_win: ">>",
            key_linux: ">>",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Outdent",
            action_ja: "アウトデント",
            key_mac: "<<",
            key_win: "<<",
            key_linux: "<<",
            category: ShortcutCategory::Editing,
        },
        // Navigation
        ShortcutEntry {
            action_en: "Move to line start",
            action_ja: "行頭に移動",
            key_mac: "0",
            key_win: "0",
            key_linux: "0",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Move to first non-blank",
            action_ja: "最初の非空白文字",
            key_mac: "^",
            key_win: "^",
            key_linux: "^",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Move to line end",
            action_ja: "行末に移動",
            key_mac: "$",
            key_win: "$",
            key_linux: "$",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Next word",
            action_ja: "次の単語",
            key_mac: "w",
            key_win: "w",
            key_linux: "w",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Previous word",
            action_ja: "前の単語",
            key_mac: "b",
            key_win: "b",
            key_linux: "b",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Go to file top",
            action_ja: "ファイル先頭に移動",
            key_mac: "gg",
            key_win: "gg",
            key_linux: "gg",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Go to file bottom",
            action_ja: "ファイル末尾に移動",
            key_mac: "G",
            key_win: "G",
            key_linux: "G",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Go to line N",
            action_ja: "N行目に移動",
            key_mac: ":N",
            key_win: ":N",
            key_linux: ":N",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Match bracket",
            action_ja: "対応する括弧に移動",
            key_mac: "%",
            key_win: "%",
            key_linux: "%",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Page down",
            action_ja: "ページダウン",
            key_mac: "Ctrl+F",
            key_win: "Ctrl+F",
            key_linux: "Ctrl+F",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Page up",
            action_ja: "ページアップ",
            key_mac: "Ctrl+B",
            key_win: "Ctrl+B",
            key_linux: "Ctrl+B",
            category: ShortcutCategory::Navigation,
        },
        // Search
        ShortcutEntry {
            action_en: "Search forward",
            action_ja: "前方検索",
            key_mac: "/pattern",
            key_win: "/pattern",
            key_linux: "/pattern",
            category: ShortcutCategory::Search,
        },
        ShortcutEntry {
            action_en: "Search backward",
            action_ja: "後方検索",
            key_mac: "?pattern",
            key_win: "?pattern",
            key_linux: "?pattern",
            category: ShortcutCategory::Search,
        },
        ShortcutEntry {
            action_en: "Next match",
            action_ja: "次の一致",
            key_mac: "n",
            key_win: "n",
            key_linux: "n",
            category: ShortcutCategory::Search,
        },
        ShortcutEntry {
            action_en: "Previous match",
            action_ja: "前の一致",
            key_mac: "N",
            key_win: "N",
            key_linux: "N",
            category: ShortcutCategory::Search,
        },
        ShortcutEntry {
            action_en: "Search & replace",
            action_ja: "検索と置換",
            key_mac: ":%s/old/new/g",
            key_win: ":%s/old/new/g",
            key_linux: ":%s/old/new/g",
            category: ShortcutCategory::Search,
        },
        ShortcutEntry {
            action_en: "Search word under cursor",
            action_ja: "カーソル下の単語を検索",
            key_mac: "*",
            key_win: "*",
            key_linux: "*",
            category: ShortcutCategory::Search,
        },
        // File
        ShortcutEntry {
            action_en: "Save",
            action_ja: "保存",
            key_mac: ":w",
            key_win: ":w",
            key_linux: ":w",
            category: ShortcutCategory::File,
        },
        ShortcutEntry {
            action_en: "Quit",
            action_ja: "終了",
            key_mac: ":q",
            key_win: ":q",
            key_linux: ":q",
            category: ShortcutCategory::File,
        },
        ShortcutEntry {
            action_en: "Save & quit",
            action_ja: "保存して終了",
            key_mac: ":wq",
            key_win: ":wq",
            key_linux: ":wq",
            category: ShortcutCategory::File,
        },
        ShortcutEntry {
            action_en: "Force quit",
            action_ja: "強制終了",
            key_mac: ":q!",
            key_win: ":q!",
            key_linux: ":q!",
            category: ShortcutCategory::File,
        },
        ShortcutEntry {
            action_en: "Open file",
            action_ja: "ファイルを開く",
            key_mac: ":e filename",
            key_win: ":e filename",
            key_linux: ":e filename",
            category: ShortcutCategory::File,
        },
        ShortcutEntry {
            action_en: "Split horizontal",
            action_ja: "水平分割",
            key_mac: ":sp",
            key_win: ":sp",
            key_linux: ":sp",
            category: ShortcutCategory::View,
        },
        ShortcutEntry {
            action_en: "Split vertical",
            action_ja: "垂直分割",
            key_mac: ":vsp",
            key_win: ":vsp",
            key_linux: ":vsp",
            category: ShortcutCategory::View,
        },
        ShortcutEntry {
            action_en: "Switch window",
            action_ja: "ウィンドウ切り替え",
            key_mac: "Ctrl+W, W",
            key_win: "Ctrl+W, W",
            key_linux: "Ctrl+W, W",
            category: ShortcutCategory::View,
        },
    ]
}

fn get_terminal_shortcuts() -> Vec<ShortcutEntry> {
    vec![
        // Navigation
        ShortcutEntry {
            action_en: "Move to line start",
            action_ja: "行頭に移動",
            key_mac: "Ctrl+A",
            key_win: "Home",
            key_linux: "Ctrl+A",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Move to line end",
            action_ja: "行末に移動",
            key_mac: "Ctrl+E",
            key_win: "End",
            key_linux: "Ctrl+E",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Move forward word",
            action_ja: "次の単語に移動",
            key_mac: "Option+F",
            key_win: "Ctrl+\u{2192}",
            key_linux: "Alt+F",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Move backward word",
            action_ja: "前の単語に移動",
            key_mac: "Option+B",
            key_win: "Ctrl+\u{2190}",
            key_linux: "Alt+B",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Previous command",
            action_ja: "前のコマンド",
            key_mac: "Ctrl+P / \u{2191}",
            key_win: "\u{2191}",
            key_linux: "Ctrl+P / \u{2191}",
            category: ShortcutCategory::Navigation,
        },
        ShortcutEntry {
            action_en: "Next command",
            action_ja: "次のコマンド",
            key_mac: "Ctrl+N / \u{2193}",
            key_win: "\u{2193}",
            key_linux: "Ctrl+N / \u{2193}",
            category: ShortcutCategory::Navigation,
        },
        // Editing
        ShortcutEntry {
            action_en: "Delete char before cursor",
            action_ja: "カーソル前の文字を削除",
            key_mac: "Ctrl+H",
            key_win: "Backspace",
            key_linux: "Ctrl+H",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Delete char after cursor",
            action_ja: "カーソル後の文字を削除",
            key_mac: "Ctrl+D",
            key_win: "Delete",
            key_linux: "Ctrl+D",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Delete word before cursor",
            action_ja: "カーソル前の単語を削除",
            key_mac: "Ctrl+W",
            key_win: "Ctrl+Backspace",
            key_linux: "Ctrl+W",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Delete to line start",
            action_ja: "行頭まで削除",
            key_mac: "Ctrl+U",
            key_win: "Ctrl+U",
            key_linux: "Ctrl+U",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Delete to line end",
            action_ja: "行末まで削除",
            key_mac: "Ctrl+K",
            key_win: "Ctrl+K",
            key_linux: "Ctrl+K",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Paste (yank)",
            action_ja: "貼り付け（ヤンク）",
            key_mac: "Ctrl+Y",
            key_win: "Ctrl+Y",
            key_linux: "Ctrl+Y",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Swap characters",
            action_ja: "文字の入れ替え",
            key_mac: "Ctrl+T",
            key_win: "Ctrl+T",
            key_linux: "Ctrl+T",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Uppercase word",
            action_ja: "単語を大文字に",
            key_mac: "Option+U",
            key_win: "Alt+U",
            key_linux: "Alt+U",
            category: ShortcutCategory::Editing,
        },
        ShortcutEntry {
            action_en: "Lowercase word",
            action_ja: "単語を小文字に",
            key_mac: "Option+L",
            key_win: "Alt+L",
            key_linux: "Alt+L",
            category: ShortcutCategory::Editing,
        },
        // Search
        ShortcutEntry {
            action_en: "Reverse search history",
            action_ja: "履歴を逆方向検索",
            key_mac: "Ctrl+R",
            key_win: "Ctrl+R",
            key_linux: "Ctrl+R",
            category: ShortcutCategory::Search,
        },
        ShortcutEntry {
            action_en: "Forward search history",
            action_ja: "履歴を順方向検索",
            key_mac: "Ctrl+S",
            key_win: "Ctrl+S",
            key_linux: "Ctrl+S",
            category: ShortcutCategory::Search,
        },
        // Terminal
        ShortcutEntry {
            action_en: "Clear screen",
            action_ja: "画面クリア",
            key_mac: "Ctrl+L",
            key_win: "cls",
            key_linux: "Ctrl+L",
            category: ShortcutCategory::Terminal,
        },
        ShortcutEntry {
            action_en: "Cancel command",
            action_ja: "コマンド中止",
            key_mac: "Ctrl+C",
            key_win: "Ctrl+C",
            key_linux: "Ctrl+C",
            category: ShortcutCategory::Terminal,
        },
        ShortcutEntry {
            action_en: "EOF / Exit",
            action_ja: "EOF / 終了",
            key_mac: "Ctrl+D",
            key_win: "Ctrl+D",
            key_linux: "Ctrl+D",
            category: ShortcutCategory::Terminal,
        },
        ShortcutEntry {
            action_en: "Suspend process",
            action_ja: "プロセスの一時停止",
            key_mac: "Ctrl+Z",
            key_win: "Ctrl+Z",
            key_linux: "Ctrl+Z",
            category: ShortcutCategory::Terminal,
        },
        ShortcutEntry {
            action_en: "New tab",
            action_ja: "新しいタブ",
            key_mac: "Cmd+T",
            key_win: "Ctrl+Shift+T",
            key_linux: "Ctrl+Shift+T",
            category: ShortcutCategory::Terminal,
        },
        ShortcutEntry {
            action_en: "Close tab",
            action_ja: "タブを閉じる",
            key_mac: "Cmd+W",
            key_win: "Ctrl+Shift+W",
            key_linux: "Ctrl+Shift+W",
            category: ShortcutCategory::Terminal,
        },
        ShortcutEntry {
            action_en: "Copy",
            action_ja: "コピー",
            key_mac: "Cmd+C",
            key_win: "Ctrl+Shift+C",
            key_linux: "Ctrl+Shift+C",
            category: ShortcutCategory::Terminal,
        },
        ShortcutEntry {
            action_en: "Paste",
            action_ja: "貼り付け",
            key_mac: "Cmd+V",
            key_win: "Ctrl+Shift+V",
            key_linux: "Ctrl+Shift+V",
            category: ShortcutCategory::Terminal,
        },
    ]
}

fn get_shortcuts(app: &AppType) -> Vec<ShortcutEntry> {
    match app {
        AppType::VSCode => get_vscode_shortcuts(),
        AppType::IntelliJ => get_intellij_shortcuts(),
        AppType::Vim => get_vim_shortcuts(),
        AppType::Terminal => get_terminal_shortcuts(),
    }
}

fn get_key_for_os(entry: &ShortcutEntry, os: &OsType) -> &'static str {
    match os {
        OsType::Mac => entry.key_mac,
        OsType::Windows => entry.key_win,
        OsType::Linux => entry.key_linux,
    }
}

fn get_action_for_lang(entry: &ShortcutEntry, lang: &str) -> String {
    if lang == "ja" {
        entry.action_ja.to_string()
    } else {
        entry.action_en.to_string()
    }
}

#[function_component(ShortcutDictionary)]
pub fn shortcut_dictionary() -> Html {
    let (i18n, _) = use_translation();
    let selected_app = use_state(|| AppType::VSCode);
    let selected_os = use_state(|| OsType::Mac);
    let selected_category = use_state(|| ShortcutCategory::All);
    let search_query = use_state(String::new);
    let copied_index = use_state(|| Option::<usize>::None);

    let current_lang = if i18n.t("common.copy") == "コピー" {
        "ja"
    } else {
        "en"
    };

    let on_app_change = {
        let selected_app = selected_app.clone();
        Callback::from(move |app: AppType| {
            selected_app.set(app);
        })
    };

    let on_os_change = {
        let selected_os = selected_os.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let os = match select.value().as_str() {
                "windows" => OsType::Windows,
                "linux" => OsType::Linux,
                _ => OsType::Mac,
            };
            selected_os.set(os);
        })
    };

    let on_category_change = {
        let selected_category = selected_category.clone();
        Callback::from(move |cat: ShortcutCategory| {
            selected_category.set(cat);
        })
    };

    let on_search_change = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            search_query.set(input.value());
        })
    };

    let on_clear_search = {
        let search_query = search_query.clone();
        Callback::from(move |_| {
            search_query.set(String::new());
        })
    };

    let shortcuts = get_shortcuts(&selected_app);
    let filtered: Vec<(usize, &ShortcutEntry)> = shortcuts
        .iter()
        .enumerate()
        .filter(|(_, entry)| {
            if *selected_category != ShortcutCategory::All && entry.category != *selected_category {
                return false;
            }
            if !search_query.is_empty() {
                let query = search_query.to_lowercase();
                let action = get_action_for_lang(entry, current_lang).to_lowercase();
                let action_en = entry.action_en.to_lowercase();
                let key = get_key_for_os(entry, &selected_os).to_lowercase();
                return action.contains(&query)
                    || action_en.contains(&query)
                    || key.contains(&query);
            }
            true
        })
        .collect();

    let match_count = filtered.len();

    html! {
        <div class="shortcut-dictionary">
            <div class="section shortcut-app-section">
                <h3>{i18n.t("shortcut_dictionary.select_app")}</h3>
                <div class="shortcut-app-grid">
                    { for AppType::all().iter().map(|app| {
                        let is_active = *selected_app == *app;
                        let on_click = {
                            let on_app_change = on_app_change.clone();
                            let app = *app;
                            Callback::from(move |_| on_app_change.emit(app))
                        };
                        let label = i18n.t(app.translation_key());
                        html! {
                            <button
                                class={classes!("shortcut-app-btn", is_active.then_some("active"))}
                                onclick={on_click}
                            >
                                <span class="shortcut-app-icon">{app.icon()}</span>
                                <span class="shortcut-app-label">{label}</span>
                            </button>
                        }
                    })}
                </div>
            </div>

            <div class="section shortcut-filters-section">
                <div class="shortcut-filters-row">
                    <div class="shortcut-search-wrapper">
                        <input
                            type="text"
                            class="form-input shortcut-search-input"
                            placeholder={i18n.t("shortcut_dictionary.search_placeholder")}
                            value={(*search_query).clone()}
                            oninput={on_search_change}
                        />
                        if !search_query.is_empty() {
                            <button class="shortcut-search-clear" onclick={on_clear_search}>
                                {"\u{2715}"}
                            </button>
                        }
                    </div>
                    <div class="shortcut-os-select">
                        <label class="shortcut-os-label">{i18n.t("shortcut_dictionary.os_label")}</label>
                        <select class="form-select" onchange={on_os_change}>
                            { for OsType::all().iter().map(|os| {
                                let value = match os {
                                    OsType::Mac => "mac",
                                    OsType::Windows => "windows",
                                    OsType::Linux => "linux",
                                };
                                html! {
                                    <option value={value} selected={*selected_os == *os}>
                                        {i18n.t(os.translation_key())}
                                    </option>
                                }
                            })}
                        </select>
                    </div>
                </div>

                <div class="shortcut-category-tabs">
                    { for ShortcutCategory::all().iter().map(|cat| {
                        let is_active = *selected_category == *cat;
                        let on_click = {
                            let on_category_change = on_category_change.clone();
                            let cat = *cat;
                            Callback::from(move |_| on_category_change.emit(cat))
                        };
                        let label = i18n.t(cat.translation_key());
                        html! {
                            <button
                                class={classes!("shortcut-category-btn", is_active.then_some("active"))}
                                onclick={on_click}
                            >
                                {label}
                            </button>
                        }
                    })}
                </div>
            </div>

            <div class="section shortcut-results-section">
                <div class="shortcut-results-header">
                    <span class="shortcut-results-count">
                        {i18n.t("shortcut_dictionary.results_count").replace("{count}", &match_count.to_string())}
                    </span>
                </div>
                <div class="shortcut-table-wrapper">
                    <table class="shortcut-table">
                        <thead>
                            <tr>
                                <th class="shortcut-th-action">{i18n.t("shortcut_dictionary.col_action")}</th>
                                <th class="shortcut-th-key">{i18n.t("shortcut_dictionary.col_shortcut")}</th>
                                <th class="shortcut-th-category">{i18n.t("shortcut_dictionary.col_category")}</th>
                                <th class="shortcut-th-copy"></th>
                            </tr>
                        </thead>
                        <tbody>
                            { for filtered.iter().map(|(idx, entry)| {
                                let action = get_action_for_lang(entry, current_lang);
                                let key = get_key_for_os(entry, &selected_os);
                                let cat_label = i18n.t(entry.category.translation_key());
                                let is_copied = *copied_index == Some(*idx);
                                let on_copy = {
                                    let copied_index = copied_index.clone();
                                    let key_str = key.to_string();
                                    let idx = *idx;
                                    Callback::from(move |_| {
                                        let key_str = key_str.clone();
                                        let copied_index = copied_index.clone();
                                        if let Some(win) = window() {
                                            let clipboard = win.navigator().clipboard();
                                            let copied_index_clone = copied_index.clone();
                                            spawn_local(async move {
                                                let _ = wasm_bindgen_futures::JsFuture::from(
                                                    clipboard.write_text(&key_str),
                                                ).await;
                                                copied_index_clone.set(Some(idx));
                                                let copied_reset = copied_index.clone();
                                                gloo_timers::callback::Timeout::new(2000, move || {
                                                    copied_reset.set(None);
                                                }).forget();
                                            });
                                        }
                                    })
                                };
                                html! {
                                    <tr class="shortcut-row">
                                        <td class="shortcut-td-action">{action}</td>
                                        <td class="shortcut-td-key">
                                            <kbd class="shortcut-kbd">{key}</kbd>
                                        </td>
                                        <td class="shortcut-td-category">
                                            <span class="shortcut-cat-badge">{cat_label}</span>
                                        </td>
                                        <td class="shortcut-td-copy">
                                            <button
                                                class={classes!("copy-btn", "shortcut-copy-btn", is_copied.then_some("copied"))}
                                                onclick={on_copy}
                                            >
                                                if is_copied {
                                                    {"\u{2713}"}
                                                } else {
                                                    {"\u{1f4cb}"}
                                                }
                                            </button>
                                        </td>
                                    </tr>
                                }
                            })}
                        </tbody>
                    </table>
                    if filtered.is_empty() {
                        <div class="shortcut-no-results">
                            {i18n.t("shortcut_dictionary.no_results")}
                        </div>
                    }
                </div>
            </div>
        </div>
    }
}
