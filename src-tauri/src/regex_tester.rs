use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegexFlags {
    pub global: bool,
    pub case_insensitive: bool,
    pub multiline: bool,
    pub dot_all: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchInfo {
    pub full_match: String,
    pub start: usize,
    pub end: usize,
    pub groups: Vec<GroupInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupInfo {
    pub index: usize,
    pub name: Option<String>,
    pub value: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegexResult {
    pub success: bool,
    pub matches: Vec<MatchInfo>,
    pub match_count: usize,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceResult {
    pub success: bool,
    pub result: String,
    pub replacements: usize,
    pub error: Option<String>,
}

fn build_regex(pattern: &str, flags: RegexFlags) -> Result<Regex, String> {
    RegexBuilder::new(pattern)
        .case_insensitive(flags.case_insensitive)
        .multi_line(flags.multiline)
        .dot_matches_new_line(flags.dot_all)
        .build()
        .map_err(|e| e.to_string())
}

pub fn test_regex(pattern: &str, test_text: &str, flags: RegexFlags) -> RegexResult {
    let re = match build_regex(pattern, flags) {
        Ok(r) => r,
        Err(e) => {
            return RegexResult {
                success: false,
                matches: vec![],
                match_count: 0,
                error: Some(e),
            };
        }
    };

    let mut matches = Vec::new();
    let group_names: Vec<Option<&str>> = re.capture_names().collect();

    if flags.global {
        // Global flag: find all matches
        for caps in re.captures_iter(test_text) {
            let m = caps.get(0).unwrap();
            let mut groups = Vec::new();

            // Iterate through capture groups (skip index 0 which is the full match)
            for (i, name) in group_names.iter().enumerate().skip(1) {
                if let Some(group_match) = caps.get(i) {
                    groups.push(GroupInfo {
                        index: i,
                        name: name.map(|n| n.to_string()),
                        value: group_match.as_str().to_string(),
                        start: group_match.start(),
                        end: group_match.end(),
                    });
                }
            }

            matches.push(MatchInfo {
                full_match: m.as_str().to_string(),
                start: m.start(),
                end: m.end(),
                groups,
            });
        }
    } else {
        // Non-global: find first match only
        if let Some(caps) = re.captures(test_text) {
            let m = caps.get(0).unwrap();
            let mut groups = Vec::new();

            for (i, name) in group_names.iter().enumerate().skip(1) {
                if let Some(group_match) = caps.get(i) {
                    groups.push(GroupInfo {
                        index: i,
                        name: name.map(|n| n.to_string()),
                        value: group_match.as_str().to_string(),
                        start: group_match.start(),
                        end: group_match.end(),
                    });
                }
            }

            matches.push(MatchInfo {
                full_match: m.as_str().to_string(),
                start: m.start(),
                end: m.end(),
                groups,
            });
        }
    }

    let match_count = matches.len();

    RegexResult {
        success: true,
        matches,
        match_count,
        error: None,
    }
}

pub fn replace_regex(
    pattern: &str,
    test_text: &str,
    replacement: &str,
    flags: RegexFlags,
) -> ReplaceResult {
    let re = match build_regex(pattern, flags) {
        Ok(r) => r,
        Err(e) => {
            return ReplaceResult {
                success: false,
                result: String::new(),
                replacements: 0,
                error: Some(e),
            };
        }
    };

    let mut replacement_count = 0;

    let result = if flags.global {
        // Count replacements
        replacement_count = re.find_iter(test_text).count();
        re.replace_all(test_text, replacement).to_string()
    } else {
        if re.is_match(test_text) {
            replacement_count = 1;
        }
        re.replace(test_text, replacement).to_string()
    };

    ReplaceResult {
        success: true,
        result,
        replacements: replacement_count,
        error: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_match() {
        let flags = RegexFlags {
            global: true,
            case_insensitive: false,
            multiline: false,
            dot_all: false,
        };

        let result = test_regex(r"\d+", "abc 123 def 456", flags);
        assert!(result.success);
        assert_eq!(result.match_count, 2);
        assert_eq!(result.matches[0].full_match, "123");
        assert_eq!(result.matches[1].full_match, "456");
    }

    #[test]
    fn test_capture_groups() {
        let flags = RegexFlags {
            global: true,
            case_insensitive: false,
            multiline: false,
            dot_all: false,
        };

        let result = test_regex(r"(\w+)@(\w+)\.(\w+)", "test@example.com", flags);
        assert!(result.success);
        assert_eq!(result.match_count, 1);
        assert_eq!(result.matches[0].groups.len(), 3);
        assert_eq!(result.matches[0].groups[0].value, "test");
        assert_eq!(result.matches[0].groups[1].value, "example");
        assert_eq!(result.matches[0].groups[2].value, "com");
    }

    #[test]
    fn test_named_groups() {
        let flags = RegexFlags {
            global: true,
            case_insensitive: false,
            multiline: false,
            dot_all: false,
        };

        let result = test_regex(r"(?P<user>\w+)@(?P<domain>\w+)", "test@example", flags);
        assert!(result.success);
        assert_eq!(result.matches[0].groups[0].name, Some("user".to_string()));
        assert_eq!(result.matches[0].groups[1].name, Some("domain".to_string()));
    }

    #[test]
    fn test_replace() {
        let flags = RegexFlags {
            global: true,
            case_insensitive: false,
            multiline: false,
            dot_all: false,
        };

        let result = replace_regex(r"\d+", "abc 123 def 456", "XXX", flags);
        assert!(result.success);
        assert_eq!(result.result, "abc XXX def XXX");
        assert_eq!(result.replacements, 2);
    }

    #[test]
    fn test_replace_with_groups() {
        let flags = RegexFlags {
            global: true,
            case_insensitive: false,
            multiline: false,
            dot_all: false,
        };

        let result = replace_regex(r"(\w+)@(\w+)", "test@example", "$2@$1", flags);
        assert!(result.success);
        assert_eq!(result.result, "example@test");
    }

    #[test]
    fn test_invalid_pattern() {
        let flags = RegexFlags {
            global: true,
            case_insensitive: false,
            multiline: false,
            dot_all: false,
        };

        let result = test_regex(r"[", "test", flags);
        assert!(!result.success);
        assert!(result.error.is_some());
    }
}
