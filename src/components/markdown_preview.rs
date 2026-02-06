use i18nrs::yew::use_translation;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum PreviewTheme {
    Dark,
    Light,
}

#[derive(Clone, PartialEq)]
enum ViewMode {
    Split,
    Editor,
    Preview,
}

fn markdown_to_html(input: &str) -> String {
    let mut html = String::new();
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_content = String::new();
    let mut in_table = false;
    let mut in_list = false;
    let mut in_ordered_list = false;
    let mut in_blockquote = false;
    let mut toc_entries: Vec<(usize, String, String)> = Vec::new();

    let lines: Vec<&str> = input.lines().collect();

    // First pass: collect TOC entries
    for line in &lines {
        if let Some(level) = heading_level(line) {
            let text = line.trim_start_matches('#').trim();
            let id = text
                .to_lowercase()
                .replace(' ', "-")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-')
                .collect::<String>();
            toc_entries.push((level, id, text.to_string()));
        }
    }

    // Build TOC if there are headings
    if toc_entries.len() >= 2 {
        html.push_str("<nav class=\"md-toc\">");
        html.push_str("<details open><summary class=\"md-toc-title\">Table of Contents</summary>");
        html.push_str("<ul>");
        for (level, id, text) in &toc_entries {
            let indent = if *level > 1 {
                format!("style=\"margin-left: {}em\"", (level - 1))
            } else {
                String::new()
            };
            html.push_str(&format!(
                "<li {}><a href=\"#{}\">{}</a></li>",
                indent,
                escape_html(id),
                escape_html(text)
            ));
        }
        html.push_str("</ul></details></nav>");
    }

    for line in &lines {
        // Code blocks
        if line.starts_with("```") {
            if in_code_block {
                html.push_str("</code></pre>");
                in_code_block = false;
                code_lang.clear();
                code_content.clear();
                continue;
            } else {
                close_lists(&mut html, &mut in_list, &mut in_ordered_list);
                close_blockquote(&mut html, &mut in_blockquote);
                code_lang = line.trim_start_matches('`').trim().to_string();
                let lang_class = if code_lang.is_empty() {
                    String::new()
                } else {
                    format!(" class=\"language-{}\"", escape_html(&code_lang))
                };
                html.push_str(&format!(
                    "<pre class=\"md-code-block\"><code{}>",
                    lang_class
                ));
                in_code_block = true;
                continue;
            }
        }

        if in_code_block {
            html.push_str(&escape_html(line));
            html.push('\n');
            continue;
        }

        // Tables
        if line.contains('|') && line.trim().starts_with('|') {
            close_lists(&mut html, &mut in_list, &mut in_ordered_list);
            close_blockquote(&mut html, &mut in_blockquote);
            let trimmed = line.trim();

            // Check if separator line
            if trimmed
                .chars()
                .all(|c| c == '|' || c == '-' || c == ':' || c == ' ')
                && trimmed.contains('-')
            {
                continue;
            }

            if !in_table {
                html.push_str("<table class=\"md-table\">");
                html.push_str("<thead><tr>");
                in_table = true;
                let cells: Vec<&str> = trimmed
                    .trim_matches('|')
                    .split('|')
                    .map(|s| s.trim())
                    .collect();
                for cell in &cells {
                    html.push_str(&format!("<th>{}</th>", inline_format(cell)));
                }
                html.push_str("</tr></thead><tbody>");
                continue;
            }

            html.push_str("<tr>");
            let cells: Vec<&str> = trimmed
                .trim_matches('|')
                .split('|')
                .map(|s| s.trim())
                .collect();
            for cell in &cells {
                html.push_str(&format!("<td>{}</td>", inline_format(cell)));
            }
            html.push_str("</tr>");
            continue;
        } else if in_table {
            html.push_str("</tbody></table>");
            in_table = false;
        }

        let trimmed = line.trim();

        // Empty line
        if trimmed.is_empty() {
            close_lists(&mut html, &mut in_list, &mut in_ordered_list);
            close_blockquote(&mut html, &mut in_blockquote);
            continue;
        }

        // Headings
        if let Some(level) = heading_level(line) {
            close_lists(&mut html, &mut in_list, &mut in_ordered_list);
            close_blockquote(&mut html, &mut in_blockquote);
            let text = line.trim_start_matches('#').trim();
            let id = text
                .to_lowercase()
                .replace(' ', "-")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-')
                .collect::<String>();
            html.push_str(&format!(
                "<h{} id=\"{}\">{}</h{}>",
                level,
                escape_html(&id),
                inline_format(text),
                level
            ));
            continue;
        }

        // Horizontal rule
        if trimmed == "---" || trimmed == "***" || trimmed == "___" {
            close_lists(&mut html, &mut in_list, &mut in_ordered_list);
            close_blockquote(&mut html, &mut in_blockquote);
            html.push_str("<hr class=\"md-hr\">");
            continue;
        }

        // Blockquote
        if trimmed.starts_with("> ") || trimmed == ">" {
            close_lists(&mut html, &mut in_list, &mut in_ordered_list);
            if !in_blockquote {
                html.push_str("<blockquote class=\"md-blockquote\">");
                in_blockquote = true;
            }
            let content = trimmed.strip_prefix("> ").unwrap_or("").trim();
            html.push_str(&format!("<p>{}</p>", inline_format(content)));
            continue;
        }

        // Task lists
        if trimmed.starts_with("- [x] ") || trimmed.starts_with("- [ ] ") {
            close_blockquote(&mut html, &mut in_blockquote);
            if !in_list {
                html.push_str("<ul class=\"md-task-list\">");
                in_list = true;
            }
            let checked = trimmed.starts_with("- [x]");
            let text = &trimmed[6..];
            let checkbox = if checked {
                "<input type=\"checkbox\" checked disabled>"
            } else {
                "<input type=\"checkbox\" disabled>"
            };
            html.push_str(&format!(
                "<li class=\"md-task-item\">{} {}</li>",
                checkbox,
                inline_format(text.trim())
            ));
            continue;
        }

        // Unordered list
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") {
            close_blockquote(&mut html, &mut in_blockquote);
            if in_ordered_list {
                html.push_str("</ol>");
                in_ordered_list = false;
            }
            if !in_list {
                html.push_str("<ul class=\"md-list\">");
                in_list = true;
            }
            let text = &trimmed[2..];
            html.push_str(&format!("<li>{}</li>", inline_format(text)));
            continue;
        }

        // Ordered list
        if let Some(rest) = parse_ordered_list(trimmed) {
            close_blockquote(&mut html, &mut in_blockquote);
            if in_list && !in_ordered_list {
                html.push_str("</ul>");
                in_list = false;
            }
            if !in_ordered_list {
                html.push_str("<ol class=\"md-list\">");
                in_ordered_list = true;
            }
            html.push_str(&format!("<li>{}</li>", inline_format(rest)));
            continue;
        }

        // Regular paragraph
        close_lists(&mut html, &mut in_list, &mut in_ordered_list);
        close_blockquote(&mut html, &mut in_blockquote);
        html.push_str(&format!("<p>{}</p>", inline_format(trimmed)));
    }

    // Close any remaining open elements
    if in_code_block {
        html.push_str("</code></pre>");
    }
    if in_table {
        html.push_str("</tbody></table>");
    }
    close_lists(&mut html, &mut in_list, &mut in_ordered_list);
    close_blockquote(&mut html, &mut in_blockquote);

    html
}

fn heading_level(line: &str) -> Option<usize> {
    let trimmed = line.trim_start();
    if trimmed.starts_with("######") {
        Some(6)
    } else if trimmed.starts_with("#####") {
        Some(5)
    } else if trimmed.starts_with("####") {
        Some(4)
    } else if trimmed.starts_with("###") {
        Some(3)
    } else if trimmed.starts_with("##") {
        Some(2)
    } else if trimmed.starts_with("# ") {
        Some(1)
    } else {
        None
    }
}

fn parse_ordered_list(line: &str) -> Option<&str> {
    let mut chars = line.chars().peekable();
    let mut has_digit = false;
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            has_digit = true;
            chars.next();
        } else {
            break;
        }
    }
    if !has_digit {
        return None;
    }
    if chars.next() == Some('.') && chars.next() == Some(' ') {
        let rest: String = chars.collect();
        let prefix_len = line.len() - rest.len();
        Some(&line[prefix_len..])
    } else {
        None
    }
}

fn close_lists(html: &mut String, in_list: &mut bool, in_ordered_list: &mut bool) {
    if *in_list {
        html.push_str("</ul>");
        *in_list = false;
    }
    if *in_ordered_list {
        html.push_str("</ol>");
        *in_ordered_list = false;
    }
}

fn close_blockquote(html: &mut String, in_blockquote: &mut bool) {
    if *in_blockquote {
        html.push_str("</blockquote>");
        *in_blockquote = false;
    }
}

fn inline_format(text: &str) -> String {
    let mut result = escape_html(text);

    // Bold + italic (***text***)
    result = replace_pattern(&result, "***", "***", "<strong><em>", "</em></strong>");
    result = replace_pattern(&result, "___", "___", "<strong><em>", "</em></strong>");

    // Bold (**text**)
    result = replace_pattern(&result, "**", "**", "<strong>", "</strong>");
    result = replace_pattern(&result, "__", "__", "<strong>", "</strong>");

    // Italic (*text*)
    result = replace_single_pattern(&result, "*", "<em>", "</em>");
    result = replace_single_pattern(&result, "_", "<em>", "</em>");

    // Strikethrough (~~text~~)
    result = replace_pattern(&result, "~~", "~~", "<del>", "</del>");

    // Inline code (`text`)
    result = replace_single_pattern(&result, "`", "<code class=\"md-inline-code\">", "</code>");

    // Links [text](url)
    result = replace_links(&result);

    // Images ![alt](url)
    result = replace_images(&result);

    // Emoji shortcodes
    result = replace_emojis(&result);

    result
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn replace_pattern(
    text: &str,
    open: &str,
    close: &str,
    html_open: &str,
    html_close: &str,
) -> String {
    let mut result = String::new();
    let mut remaining = text;

    while let Some(start) = remaining.find(open) {
        result.push_str(&remaining[..start]);
        let after_open = &remaining[start + open.len()..];
        if let Some(end) = after_open.find(close) {
            result.push_str(html_open);
            result.push_str(&after_open[..end]);
            result.push_str(html_close);
            remaining = &after_open[end + close.len()..];
        } else {
            result.push_str(open);
            remaining = after_open;
        }
    }
    result.push_str(remaining);
    result
}

fn replace_single_pattern(text: &str, delim: &str, html_open: &str, html_close: &str) -> String {
    let mut result = String::new();
    let mut remaining = text;
    let mut open = false;

    while let Some(pos) = remaining.find(delim) {
        result.push_str(&remaining[..pos]);
        if !open {
            let after = &remaining[pos + delim.len()..];
            if after.contains(delim) {
                result.push_str(html_open);
                open = true;
            } else {
                result.push_str(delim);
            }
        } else {
            result.push_str(html_close);
            open = false;
        }
        remaining = &remaining[pos + delim.len()..];
    }
    result.push_str(remaining);
    result
}

fn replace_links(text: &str) -> String {
    let mut result = String::new();
    let mut remaining = text;

    while let Some(bracket_start) = remaining.find('[') {
        if bracket_start > 0 && remaining.as_bytes()[bracket_start - 1] == b'!' {
            result.push_str(&remaining[..=bracket_start]);
            remaining = &remaining[bracket_start + 1..];
            continue;
        }

        result.push_str(&remaining[..bracket_start]);
        let after_bracket = &remaining[bracket_start + 1..];

        if let Some(bracket_end) = after_bracket.find(']') {
            let link_text = &after_bracket[..bracket_end];
            let after_close = &after_bracket[bracket_end + 1..];

            if after_close.starts_with('(') {
                if let Some(paren_end) = after_close.find(')') {
                    let url = &after_close[1..paren_end];
                    result.push_str(&format!(
                        "<a href=\"{}\" target=\"_blank\" rel=\"noopener\">{}</a>",
                        url, link_text
                    ));
                    remaining = &after_close[paren_end + 1..];
                    continue;
                }
            }
            result.push('[');
            remaining = after_bracket;
        } else {
            result.push('[');
            remaining = after_bracket;
        }
    }
    result.push_str(remaining);
    result
}

fn replace_images(text: &str) -> String {
    let mut result = String::new();
    let mut remaining = text;

    while let Some(pos) = remaining.find("![") {
        result.push_str(&remaining[..pos]);
        let after = &remaining[pos + 2..];

        if let Some(bracket_end) = after.find(']') {
            let alt_text = &after[..bracket_end];
            let after_close = &after[bracket_end + 1..];

            if after_close.starts_with('(') {
                if let Some(paren_end) = after_close.find(')') {
                    let url = &after_close[1..paren_end];
                    result.push_str(&format!(
                        "<img src=\"{}\" alt=\"{}\" class=\"md-image\">",
                        url, alt_text
                    ));
                    remaining = &after_close[paren_end + 1..];
                    continue;
                }
            }
            result.push_str("![");
            remaining = after;
        } else {
            result.push_str("![");
            remaining = after;
        }
    }
    result.push_str(remaining);
    result
}

fn replace_emojis(text: &str) -> String {
    text.replace(":smile:", "\u{1F604}")
        .replace(":laughing:", "\u{1F606}")
        .replace(":thumbsup:", "\u{1F44D}")
        .replace(":thumbsdown:", "\u{1F44E}")
        .replace(":heart:", "\u{2764}\u{FE0F}")
        .replace(":star:", "\u{2B50}")
        .replace(":fire:", "\u{1F525}")
        .replace(":rocket:", "\u{1F680}")
        .replace(":warning:", "\u{26A0}\u{FE0F}")
        .replace(":check:", "\u{2705}")
        .replace(":x:", "\u{274C}")
        .replace(":info:", "\u{2139}\u{FE0F}")
        .replace(":bulb:", "\u{1F4A1}")
        .replace(":memo:", "\u{1F4DD}")
        .replace(":tada:", "\u{1F389}")
        .replace(":eyes:", "\u{1F440}")
        .replace(":thinking:", "\u{1F914}")
        .replace(":wave:", "\u{1F44B}")
        .replace(":clap:", "\u{1F44F}")
        .replace(":100:", "\u{1F4AF}")
}

fn generate_preview_styles(theme: &PreviewTheme) -> String {
    let (bg, text, text_secondary, border, code_bg, link_color, blockquote_border, table_stripe) =
        match theme {
            PreviewTheme::Dark => (
                "#1a1a2e",
                "#e0e0e0",
                "#a0a0a0",
                "#333355",
                "#16213e",
                "#00d4ff",
                "#00d4ff",
                "rgba(255,255,255,0.03)",
            ),
            PreviewTheme::Light => (
                "#ffffff",
                "#1a1a2e",
                "#555555",
                "#e0e0e0",
                "#f5f5f5",
                "#0066cc",
                "#0066cc",
                "rgba(0,0,0,0.03)",
            ),
        };

    format!(
        r#"
        .md-preview-content {{
            font-family: 'Geist', -apple-system, BlinkMacSystemFont, sans-serif;
            font-size: 15px;
            line-height: 1.8;
            color: {text};
            background: {bg};
            padding: 24px;
            word-wrap: break-word;
            overflow-wrap: break-word;
        }}
        .md-preview-content h1 {{ font-size: 2em; font-weight: 700; margin: 1.2em 0 0.6em; padding-bottom: 0.3em; border-bottom: 2px solid {border}; }}
        .md-preview-content h2 {{ font-size: 1.5em; font-weight: 700; margin: 1em 0 0.5em; padding-bottom: 0.2em; border-bottom: 1px solid {border}; }}
        .md-preview-content h3 {{ font-size: 1.25em; font-weight: 600; margin: 0.8em 0 0.4em; }}
        .md-preview-content h4 {{ font-size: 1.1em; font-weight: 600; margin: 0.6em 0 0.3em; }}
        .md-preview-content h5, .md-preview-content h6 {{ font-size: 1em; font-weight: 600; margin: 0.5em 0 0.3em; color: {text_secondary}; }}
        .md-preview-content p {{ margin: 0 0 1em; }}
        .md-preview-content a {{ color: {link_color}; text-decoration: none; }}
        .md-preview-content a:hover {{ text-decoration: underline; }}
        .md-preview-content strong {{ font-weight: 700; }}
        .md-preview-content em {{ font-style: italic; }}
        .md-preview-content del {{ text-decoration: line-through; opacity: 0.7; }}
        .md-preview-content .md-inline-code {{
            font-family: 'JetBrains Mono', 'SF Mono', monospace;
            background: {code_bg};
            padding: 2px 6px;
            border-radius: 4px;
            font-size: 0.9em;
            border: 1px solid {border};
        }}
        .md-preview-content .md-code-block {{
            font-family: 'JetBrains Mono', 'SF Mono', monospace;
            background: {code_bg};
            padding: 16px;
            border-radius: 8px;
            font-size: 0.9em;
            overflow-x: auto;
            margin: 1em 0;
            border: 1px solid {border};
            line-height: 1.6;
        }}
        .md-preview-content .md-code-block code {{
            background: none;
            padding: 0;
            border: none;
        }}
        .md-preview-content .md-blockquote {{
            border-left: 4px solid {blockquote_border};
            padding: 0.5em 1em;
            margin: 1em 0;
            color: {text_secondary};
            background: {code_bg};
            border-radius: 0 8px 8px 0;
        }}
        .md-preview-content .md-blockquote p {{ margin: 0.3em 0; }}
        .md-preview-content .md-list {{
            padding-left: 2em;
            margin: 0.5em 0 1em;
        }}
        .md-preview-content .md-list li {{
            margin: 0.3em 0;
        }}
        .md-preview-content .md-task-list {{
            list-style: none;
            padding-left: 0.5em;
        }}
        .md-preview-content .md-task-item {{
            display: flex;
            align-items: center;
            gap: 8px;
            margin: 0.3em 0;
        }}
        .md-preview-content .md-task-item input[type="checkbox"] {{
            width: 16px;
            height: 16px;
            accent-color: {link_color};
        }}
        .md-preview-content .md-table {{
            width: 100%;
            border-collapse: collapse;
            margin: 1em 0;
            font-size: 0.95em;
        }}
        .md-preview-content .md-table th,
        .md-preview-content .md-table td {{
            border: 1px solid {border};
            padding: 8px 12px;
            text-align: left;
        }}
        .md-preview-content .md-table th {{
            font-weight: 600;
            background: {code_bg};
        }}
        .md-preview-content .md-table tr:nth-child(even) {{
            background: {table_stripe};
        }}
        .md-preview-content .md-hr {{
            border: none;
            border-top: 2px solid {border};
            margin: 2em 0;
        }}
        .md-preview-content .md-image {{
            max-width: 100%;
            border-radius: 8px;
            margin: 1em 0;
        }}
        .md-preview-content .md-toc {{
            background: {code_bg};
            border: 1px solid {border};
            border-radius: 8px;
            padding: 16px;
            margin-bottom: 2em;
        }}
        .md-preview-content .md-toc-title {{
            font-weight: 600;
            cursor: pointer;
            margin-bottom: 8px;
        }}
        .md-preview-content .md-toc ul {{
            list-style: none;
            padding-left: 0;
            margin: 0;
        }}
        .md-preview-content .md-toc li {{
            margin: 4px 0;
        }}
        .md-preview-content .md-toc a {{
            color: {link_color};
            text-decoration: none;
            font-size: 0.9em;
        }}
        .md-preview-content .md-toc a:hover {{
            text-decoration: underline;
        }}
    "#,
        text = text,
        bg = bg,
        text_secondary = text_secondary,
        border = border,
        code_bg = code_bg,
        link_color = link_color,
        blockquote_border = blockquote_border,
        table_stripe = table_stripe,
    )
}

const DEFAULT_MARKDOWN: &str = r#"# Markdown Preview

Welcome to the **Markdown Preview** tool! This supports GitHub Flavored Markdown (GFM).

## Features

- **Bold**, *italic*, and ~~strikethrough~~ text
- [Links](https://example.com) and images
- Code blocks with syntax highlighting
- Tables, task lists, and more

## Code Block

```rust
fn main() {
    println!("Hello, Markdown!");
}
```

## Table

| Feature | Status |
|---------|--------|
| GFM | :check: |
| Tables | :check: |
| Task Lists | :check: |
| Emoji | :check: |

## Task List

- [x] Basic Markdown rendering
- [x] GFM support
- [x] Table of Contents
- [ ] Mermaid diagrams

## Blockquote

> This is a blockquote.
> It can span multiple lines.

## Emoji

:rocket: :fire: :star: :heart: :tada:

---

*Enjoy writing Markdown!* :smile:
"#;

#[function_component(MarkdownPreview)]
pub fn markdown_preview() -> Html {
    let (i18n, _) = use_translation();
    let input = use_state(|| DEFAULT_MARKDOWN.to_string());
    let theme = use_state(|| PreviewTheme::Dark);
    let view_mode = use_state(|| ViewMode::Split);
    let copied_html = use_state(|| false);
    let word_count = use_state(|| 0usize);
    let line_count = use_state(|| 0usize);
    let char_count = use_state(|| 0usize);

    // Update stats when input changes
    {
        let word_count = word_count.clone();
        let line_count = line_count.clone();
        let char_count = char_count.clone();
        let input_val = (*input).clone();
        use_effect_with(input_val.clone(), move |input_val| {
            let words = input_val
                .split_whitespace()
                .filter(|w| !w.is_empty())
                .count();
            let lines = if input_val.is_empty() {
                0
            } else {
                input_val.lines().count()
            };
            let chars = input_val.len();
            word_count.set(words);
            line_count.set(lines);
            char_count.set(chars);
            || {}
        });
    }

    let preview_html = markdown_to_html(&input);
    let styles = generate_preview_styles(&theme);

    let on_input_change = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            input.set(textarea.value());
        })
    };

    let on_theme_toggle = {
        let theme = theme.clone();
        Callback::from(move |_| {
            theme.set(if *theme == PreviewTheme::Dark {
                PreviewTheme::Light
            } else {
                PreviewTheme::Dark
            });
        })
    };

    let on_view_split = {
        let view_mode = view_mode.clone();
        Callback::from(move |_| view_mode.set(ViewMode::Split))
    };

    let on_view_editor = {
        let view_mode = view_mode.clone();
        Callback::from(move |_| view_mode.set(ViewMode::Editor))
    };

    let on_view_preview = {
        let view_mode = view_mode.clone();
        Callback::from(move |_| view_mode.set(ViewMode::Preview))
    };

    let on_clear = {
        let input = input.clone();
        Callback::from(move |_| {
            input.set(String::new());
        })
    };

    let on_export_html = {
        let input_val = (*input).clone();
        let theme_val = (*theme).clone();
        let copied_html = copied_html.clone();
        Callback::from(move |_| {
            let html_content = markdown_to_html(&input_val);
            let styles = generate_preview_styles(&theme_val);
            let full_html = format!(
                "<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n<title>Markdown Export</title>\n<style>{}</style>\n</head>\n<body>\n<div class=\"md-preview-content\">{}</div>\n</body>\n</html>",
                styles, html_content
            );
            let copied_html = copied_html.clone();
            if let Some(win) = window() {
                let clipboard = win.navigator().clipboard();
                spawn_local(async move {
                    let _ = wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&full_html))
                        .await;
                    copied_html.set(true);
                    let copied_reset = copied_html.clone();
                    gloo_timers::callback::Timeout::new(2000, move || {
                        copied_reset.set(false);
                    })
                    .forget();
                });
            }
        })
    };

    let editor_class = match *view_mode {
        ViewMode::Split => "markdown-preview-editor",
        ViewMode::Editor => "markdown-preview-editor full-width",
        ViewMode::Preview => "markdown-preview-editor hidden",
    };

    let preview_class = match *view_mode {
        ViewMode::Split => "markdown-preview-result",
        ViewMode::Preview => "markdown-preview-result full-width",
        ViewMode::Editor => "markdown-preview-result hidden",
    };

    let styled_preview = format!(
        "<style>{}</style><div class=\"md-preview-content\">{}</div>",
        styles, preview_html
    );

    html! {
        <div class="markdown-preview">
            <div class="section markdown-preview-header">
                <div class="markdown-preview-title-row">
                    <h3>{i18n.t("markdown_preview.title")}</h3>
                    <div class="markdown-preview-stats">
                        <span class="stat-badge">{format!("{} {}", *line_count, i18n.t("common.lines"))}</span>
                        <span class="stat-badge">{format!("{} {}", *word_count, i18n.t("markdown_preview.words"))}</span>
                        <span class="stat-badge">{format!("{} {}", *char_count, i18n.t("common.characters"))}</span>
                    </div>
                </div>
                <div class="markdown-preview-toolbar">
                    <div class="markdown-preview-view-toggle">
                        <button
                            class={classes!("view-btn", (*view_mode == ViewMode::Split).then_some("active"))}
                            onclick={on_view_split}
                        >
                            {i18n.t("markdown_preview.view_split")}
                        </button>
                        <button
                            class={classes!("view-btn", (*view_mode == ViewMode::Editor).then_some("active"))}
                            onclick={on_view_editor}
                        >
                            {i18n.t("markdown_preview.view_editor")}
                        </button>
                        <button
                            class={classes!("view-btn", (*view_mode == ViewMode::Preview).then_some("active"))}
                            onclick={on_view_preview}
                        >
                            {i18n.t("markdown_preview.view_preview")}
                        </button>
                    </div>
                    <div class="markdown-preview-actions">
                        <button class="secondary-btn" onclick={on_theme_toggle}>
                            if *theme == PreviewTheme::Dark {
                                {format!("{} {}", "\u{263D}", i18n.t("markdown_preview.theme_light"))}
                            } else {
                                {format!("{} {}", "\u{2600}", i18n.t("markdown_preview.theme_dark"))}
                            }
                        </button>
                        <button
                            class={classes!("secondary-btn", (*copied_html).then_some("copied"))}
                            onclick={on_export_html}
                        >
                            if *copied_html {
                                {format!("{} {}", "\u{2713}", i18n.t("common.copied"))}
                            } else {
                                {i18n.t("markdown_preview.export_html")}
                            }
                        </button>
                        <button class="secondary-btn" onclick={on_clear}>
                            {i18n.t("common.clear")}
                        </button>
                    </div>
                </div>
            </div>

            <div class="markdown-preview-panes">
                <div class={editor_class}>
                    <div class="pane-header">
                        <span class="pane-label">{i18n.t("markdown_preview.editor_label")}</span>
                    </div>
                    <textarea
                        class="markdown-preview-textarea"
                        placeholder={i18n.t("markdown_preview.placeholder")}
                        value={(*input).clone()}
                        oninput={on_input_change}
                        spellcheck="false"
                    />
                </div>
                <div class={preview_class}>
                    <div class="pane-header">
                        <span class="pane-label">{i18n.t("markdown_preview.preview_label")}</span>
                    </div>
                    <div class="markdown-preview-rendered">
                        <div class="markdown-preview-inner">
                            {Html::from_html_unchecked(AttrValue::from(styled_preview))}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
