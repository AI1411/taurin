use serde::{Deserialize, Serialize};
use similar::{ChangeTag, TextDiff};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DiffMode {
    Line,
    Word,
    Character,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffChange {
    pub tag: String,
    pub value: String,
    pub old_index: Option<usize>,
    pub new_index: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineDiff {
    pub line_number_old: Option<usize>,
    pub line_number_new: Option<usize>,
    pub tag: String,
    pub content: String,
    pub inline_changes: Vec<InlineChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineChange {
    pub tag: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffResult {
    pub success: bool,
    pub lines: Vec<LineDiff>,
    pub stats: DiffStats,
    pub unified_diff: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffStats {
    pub additions: usize,
    pub deletions: usize,
    pub modifications: usize,
    pub unchanged: usize,
    pub total_lines_old: usize,
    pub total_lines_new: usize,
}

pub fn compute_diff(old_text: &str, new_text: &str, mode: DiffMode) -> DiffResult {
    let diff = TextDiff::from_lines(old_text, new_text);

    let mut lines: Vec<LineDiff> = Vec::new();
    let mut old_line_num = 1usize;
    let mut new_line_num = 1usize;
    let mut additions = 0usize;
    let mut deletions = 0usize;
    let mut unchanged = 0usize;

    for change in diff.iter_all_changes() {
        let (tag_str, line_old, line_new) = match change.tag() {
            ChangeTag::Delete => {
                deletions += 1;
                let ln = old_line_num;
                old_line_num += 1;
                ("delete", Some(ln), None)
            }
            ChangeTag::Insert => {
                additions += 1;
                let ln = new_line_num;
                new_line_num += 1;
                ("insert", None, Some(ln))
            }
            ChangeTag::Equal => {
                unchanged += 1;
                let lo = old_line_num;
                let ln = new_line_num;
                old_line_num += 1;
                new_line_num += 1;
                ("equal", Some(lo), Some(ln))
            }
        };

        let content = change.value().to_string();
        let _ = &mode; // Suppress unused warning
        let inline_changes = Vec::new();

        lines.push(LineDiff {
            line_number_old: line_old,
            line_number_new: line_new,
            tag: tag_str.to_string(),
            content,
            inline_changes,
        });
    }

    let total_lines_old = old_text.lines().count().max(1);
    let total_lines_new = new_text.lines().count().max(1);

    let unified_diff = generate_unified_diff(old_text, new_text);

    DiffResult {
        success: true,
        lines,
        stats: DiffStats {
            additions,
            deletions,
            modifications: 0,
            unchanged,
            total_lines_old,
            total_lines_new,
        },
        unified_diff,
        error: None,
    }
}

#[allow(dead_code)]
pub fn compute_inline_diff(old_line: &str, new_line: &str, mode: DiffMode) -> Vec<InlineChange> {
    let changes: Vec<InlineChange> = match mode {
        DiffMode::Word => {
            let diff = TextDiff::from_words(old_line, new_line);
            diff.iter_all_changes()
                .map(|change| {
                    let tag = match change.tag() {
                        ChangeTag::Delete => "delete",
                        ChangeTag::Insert => "insert",
                        ChangeTag::Equal => "equal",
                    };
                    InlineChange {
                        tag: tag.to_string(),
                        value: change.value().to_string(),
                    }
                })
                .collect()
        }
        DiffMode::Character => {
            let diff = TextDiff::from_chars(old_line, new_line);
            diff.iter_all_changes()
                .map(|change| {
                    let tag = match change.tag() {
                        ChangeTag::Delete => "delete",
                        ChangeTag::Insert => "insert",
                        ChangeTag::Equal => "equal",
                    };
                    InlineChange {
                        tag: tag.to_string(),
                        value: change.value().to_string(),
                    }
                })
                .collect()
        }
        DiffMode::Line => Vec::new(),
    };
    changes
}

fn generate_unified_diff(old_text: &str, new_text: &str) -> String {
    let diff = TextDiff::from_lines(old_text, new_text);
    let mut output = String::new();

    output.push_str("--- a/original\n");
    output.push_str("+++ b/modified\n");

    for hunk in diff.unified_diff().context_radius(3).iter_hunks() {
        output.push_str(&format!("{}", hunk));
    }

    output
}

pub fn read_text_file(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub content: String,
}

pub fn get_file_info(path: &str) -> Result<FileInfo, String> {
    let metadata =
        std::fs::metadata(path).map_err(|e| format!("Failed to get file info: {}", e))?;
    let content = read_text_file(path)?;
    let name = std::path::Path::new(path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string());

    Ok(FileInfo {
        path: path.to_string(),
        name,
        size: metadata.len(),
        content,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_diff_no_changes() {
        let text = "line1\nline2\nline3\n";
        let result = compute_diff(text, text, DiffMode::Line);

        assert!(result.success);
        assert_eq!(result.stats.additions, 0);
        assert_eq!(result.stats.deletions, 0);
        assert_eq!(result.stats.unchanged, 3);
    }

    #[test]
    fn test_compute_diff_with_additions() {
        let old = "line1\nline2\n";
        let new = "line1\nline2\nline3\n";
        let result = compute_diff(old, new, DiffMode::Line);

        assert!(result.success);
        assert_eq!(result.stats.additions, 1);
        assert_eq!(result.stats.deletions, 0);
        assert_eq!(result.stats.unchanged, 2);
    }

    #[test]
    fn test_compute_diff_with_deletions() {
        let old = "line1\nline2\nline3\n";
        let new = "line1\nline3\n";
        let result = compute_diff(old, new, DiffMode::Line);

        assert!(result.success);
        assert_eq!(result.stats.deletions, 1);
        assert_eq!(result.stats.unchanged, 2);
    }

    #[test]
    fn test_compute_diff_with_modifications() {
        let old = "line1\nline2\nline3\n";
        let new = "line1\nmodified\nline3\n";
        let result = compute_diff(old, new, DiffMode::Line);

        assert!(result.success);
        assert_eq!(result.stats.additions, 1);
        assert_eq!(result.stats.deletions, 1);
        assert_eq!(result.stats.unchanged, 2);
    }

    #[test]
    fn test_unified_diff_format() {
        let old = "line1\nline2\n";
        let new = "line1\nline3\n";
        let result = compute_diff(old, new, DiffMode::Line);

        assert!(result.unified_diff.contains("--- a/original"));
        assert!(result.unified_diff.contains("+++ b/modified"));
        assert!(result.unified_diff.contains("-line2"));
        assert!(result.unified_diff.contains("+line3"));
    }

    #[test]
    fn test_empty_texts() {
        let result = compute_diff("", "", DiffMode::Line);

        assert!(result.success);
        assert_eq!(result.stats.additions, 0);
        assert_eq!(result.stats.deletions, 0);
        assert_eq!(result.stats.unchanged, 0);
    }
}
