use pulldown_cmark::{Options, Parser};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownInfo {
    pub file_name: String,
    pub file_size: u64,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownToPdfResult {
    pub success: bool,
    pub output_path: String,
    pub page_count: u32,
    pub file_size: u64,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownToHtmlResult {
    pub success: bool,
    pub html: String,
    pub error: Option<String>,
}

pub fn read_markdown(path: &str) -> Result<MarkdownInfo, String> {
    let metadata =
        fs::metadata(path).map_err(|e| format!("Failed to read file metadata: {}", e))?;
    let file_size = metadata.len();

    let file_name = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    Ok(MarkdownInfo {
        file_name,
        file_size,
        content,
    })
}

pub fn markdown_to_html(markdown: &str) -> MarkdownToHtmlResult {
    let options = Options::all();
    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();

    pulldown_cmark::html::push_html(&mut html_output, parser);

    MarkdownToHtmlResult {
        success: true,
        html: html_output,
        error: None,
    }
}

fn generate_full_html(markdown: &str, base_path: Option<&str>) -> String {
    let options = Options::all();
    let parser = Parser::new_ext(markdown, options);
    let mut html_body = String::new();
    pulldown_cmark::html::push_html(&mut html_body, parser);

    // base_pathがあれば画像の相対パスを絶対パスに変換
    let html_body = if let Some(base) = base_path {
        convert_relative_paths(&html_body, base)
    } else {
        html_body
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="ja">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        * {{
            box-sizing: border-box;
        }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Hiragino Sans', 'Hiragino Kaku Gothic ProN', 'Noto Sans JP', sans-serif;
            font-size: 14px;
            line-height: 1.8;
            color: #333;
            max-width: 800px;
            margin: 0 auto;
            padding: 40px 20px;
            background: #fff;
        }}
        h1 {{
            font-size: 2em;
            font-weight: 700;
            margin: 0 0 1em 0;
            padding-bottom: 0.3em;
            border-bottom: 2px solid #eee;
            color: #111;
        }}
        h2 {{
            font-size: 1.5em;
            font-weight: 600;
            margin: 1.5em 0 0.5em 0;
            padding-bottom: 0.2em;
            border-bottom: 1px solid #eee;
            color: #222;
        }}
        h3 {{
            font-size: 1.25em;
            font-weight: 600;
            margin: 1.2em 0 0.4em 0;
            color: #333;
        }}
        h4, h5, h6 {{
            font-size: 1.1em;
            font-weight: 600;
            margin: 1em 0 0.3em 0;
            color: #444;
        }}
        p {{
            margin: 0 0 1em 0;
        }}
        ul, ol {{
            margin: 0 0 1em 0;
            padding-left: 2em;
        }}
        li {{
            margin-bottom: 0.3em;
        }}
        code {{
            font-family: 'SF Mono', 'Monaco', 'Menlo', 'Consolas', monospace;
            font-size: 0.9em;
            background: #f5f5f5;
            padding: 0.2em 0.4em;
            border-radius: 3px;
        }}
        pre {{
            background: #f5f5f5;
            padding: 1em;
            border-radius: 6px;
            overflow-x: auto;
            margin: 0 0 1em 0;
        }}
        pre code {{
            background: none;
            padding: 0;
            font-size: 0.85em;
        }}
        blockquote {{
            margin: 0 0 1em 0;
            padding: 0.5em 1em;
            border-left: 4px solid #007aff;
            background: #f8f9fa;
            color: #555;
        }}
        blockquote p {{
            margin: 0;
        }}
        hr {{
            border: none;
            border-top: 1px solid #ddd;
            margin: 2em 0;
        }}
        a {{
            color: #007aff;
            text-decoration: none;
        }}
        a:hover {{
            text-decoration: underline;
        }}
        img {{
            max-width: 100%;
            height: auto;
            display: block;
            margin: 1em 0;
            border-radius: 4px;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            margin: 0 0 1em 0;
        }}
        th, td {{
            border: 1px solid #ddd;
            padding: 0.5em 0.75em;
            text-align: left;
        }}
        th {{
            background: #f5f5f5;
            font-weight: 600;
        }}
        tr:nth-child(even) {{
            background: #fafafa;
        }}
        strong {{
            font-weight: 600;
        }}
        em {{
            font-style: italic;
        }}
        @media print {{
            body {{
                padding: 0;
                max-width: none;
            }}
            pre {{
                white-space: pre-wrap;
                word-wrap: break-word;
            }}
        }}
    </style>
</head>
<body>
{html_body}
</body>
</html>"#
    )
}

fn convert_relative_paths(html: &str, base_path: &str) -> String {
    let base_dir = Path::new(base_path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    // img srcの相対パスを絶対パスに変換
    let re = regex::Regex::new(r#"src="([^"]+)""#).unwrap();
    re.replace_all(html, |caps: &regex::Captures| {
        let src = &caps[1];
        if src.starts_with("http://") || src.starts_with("https://") || src.starts_with("file://") {
            format!(r#"src="{}""#, src)
        } else {
            let abs_path = Path::new(&base_dir).join(src);
            if abs_path.exists() {
                format!(r#"src="file://{}""#, abs_path.to_string_lossy())
            } else {
                format!(r#"src="{}""#, src)
            }
        }
    })
    .to_string()
}

fn find_pdf_converter() -> Option<String> {
    // wkhtmltopdfをチェック
    if Command::new("wkhtmltopdf")
        .arg("--version")
        .output()
        .is_ok()
    {
        return Some("wkhtmltopdf".to_string());
    }

    // Chromeをチェック (macOS)
    let chrome_paths = [
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/Applications/Chromium.app/Contents/MacOS/Chromium",
        "google-chrome",
        "chromium",
    ];

    for path in chrome_paths {
        if Command::new(path).arg("--version").output().is_ok() {
            return Some(path.to_string());
        }
    }

    None
}

pub fn convert_markdown_to_pdf(
    markdown: &str,
    output_path: &str,
    source_path: Option<&str>,
) -> MarkdownToPdfResult {
    let converter = find_pdf_converter();

    match converter {
        Some(tool) if tool == "wkhtmltopdf" => {
            convert_with_wkhtmltopdf(markdown, output_path, source_path)
        }
        Some(tool) => convert_with_chrome(&tool, markdown, output_path, source_path),
        None => MarkdownToPdfResult {
            success: false,
            output_path: String::new(),
            page_count: 0,
            file_size: 0,
            error: Some(
                "PDF converter not found. Please install wkhtmltopdf or Google Chrome.".to_string(),
            ),
        },
    }
}

fn convert_with_wkhtmltopdf(
    markdown: &str,
    output_path: &str,
    source_path: Option<&str>,
) -> MarkdownToPdfResult {
    let html = generate_full_html(markdown, source_path);

    // 一時HTMLファイルを作成
    let temp_dir = std::env::temp_dir();
    let temp_html = temp_dir.join(format!("md_to_pdf_{}.html", std::process::id()));

    if let Err(e) = fs::write(&temp_html, &html) {
        return MarkdownToPdfResult {
            success: false,
            output_path: String::new(),
            page_count: 0,
            file_size: 0,
            error: Some(format!("Failed to create temp file: {}", e)),
        };
    }

    let result = Command::new("wkhtmltopdf")
        .args([
            "--enable-local-file-access",
            "--encoding",
            "UTF-8",
            "--page-size",
            "A4",
            "--margin-top",
            "15mm",
            "--margin-bottom",
            "15mm",
            "--margin-left",
            "15mm",
            "--margin-right",
            "15mm",
            temp_html.to_str().unwrap(),
            output_path,
        ])
        .output();

    // 一時ファイルを削除
    let _ = fs::remove_file(&temp_html);

    match result {
        Ok(output) => {
            if output.status.success() {
                let file_size = fs::metadata(output_path).map(|m| m.len()).unwrap_or(0);
                MarkdownToPdfResult {
                    success: true,
                    output_path: output_path.to_string(),
                    page_count: 1, // wkhtmltopdfはページ数を返さないため
                    file_size,
                    error: None,
                }
            } else {
                MarkdownToPdfResult {
                    success: false,
                    output_path: String::new(),
                    page_count: 0,
                    file_size: 0,
                    error: Some(String::from_utf8_lossy(&output.stderr).to_string()),
                }
            }
        }
        Err(e) => MarkdownToPdfResult {
            success: false,
            output_path: String::new(),
            page_count: 0,
            file_size: 0,
            error: Some(format!("Failed to run wkhtmltopdf: {}", e)),
        },
    }
}

fn convert_with_chrome(
    chrome_path: &str,
    markdown: &str,
    output_path: &str,
    source_path: Option<&str>,
) -> MarkdownToPdfResult {
    let html = generate_full_html(markdown, source_path);

    // 一時HTMLファイルを作成
    let temp_dir = std::env::temp_dir();
    let temp_html = temp_dir.join(format!("md_to_pdf_{}.html", std::process::id()));

    if let Err(e) = fs::write(&temp_html, &html) {
        return MarkdownToPdfResult {
            success: false,
            output_path: String::new(),
            page_count: 0,
            file_size: 0,
            error: Some(format!("Failed to create temp file: {}", e)),
        };
    }

    let result = Command::new(chrome_path)
        .args([
            "--headless",
            "--disable-gpu",
            "--no-sandbox",
            "--print-to-pdf-no-header",
            &format!("--print-to-pdf={}", output_path),
            &format!("file://{}", temp_html.to_string_lossy()),
        ])
        .output();

    // 一時ファイルを削除
    let _ = fs::remove_file(&temp_html);

    match result {
        Ok(output) => {
            if output.status.success() || Path::new(output_path).exists() {
                let file_size = fs::metadata(output_path).map(|m| m.len()).unwrap_or(0);
                MarkdownToPdfResult {
                    success: true,
                    output_path: output_path.to_string(),
                    page_count: 1,
                    file_size,
                    error: None,
                }
            } else {
                MarkdownToPdfResult {
                    success: false,
                    output_path: String::new(),
                    page_count: 0,
                    file_size: 0,
                    error: Some(String::from_utf8_lossy(&output.stderr).to_string()),
                }
            }
        }
        Err(e) => MarkdownToPdfResult {
            success: false,
            output_path: String::new(),
            page_count: 0,
            file_size: 0,
            error: Some(format!("Failed to run Chrome: {}", e)),
        },
    }
}
