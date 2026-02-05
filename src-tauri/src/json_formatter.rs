use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonFormatResult {
    pub success: bool,
    pub formatted: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonValidateResult {
    pub valid: bool,
    pub error: Option<String>,
    pub error_position: Option<ErrorPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorPosition {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonTreeNode {
    pub key: String,
    pub value_type: JsonValueType,
    pub value: Option<String>,
    pub path: String,
    pub children: Vec<JsonTreeNode>,
    pub expanded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum JsonValueType {
    Object,
    Array,
    String,
    Number,
    Boolean,
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonParseResult {
    pub success: bool,
    pub tree: Option<JsonTreeNode>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonMinifyResult {
    pub success: bool,
    pub minified: String,
    pub original_size: usize,
    pub minified_size: usize,
    pub savings_percent: f64,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSearchResult {
    pub success: bool,
    pub matches: Vec<JsonSearchMatch>,
    pub total_count: usize,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSearchMatch {
    pub path: String,
    pub key: String,
    pub value: String,
    pub value_type: JsonValueType,
}

pub fn format_json(input: &str, indent_size: usize) -> JsonFormatResult {
    match serde_json::from_str::<Value>(input) {
        Ok(value) => {
            let indent = " ".repeat(indent_size);
            let formatter = serde_json::ser::PrettyFormatter::with_indent(indent.as_bytes());
            let mut buf = Vec::new();
            let mut serializer = serde_json::Serializer::with_formatter(&mut buf, formatter);

            match value.serialize(&mut serializer) {
                Ok(_) => {
                    let formatted = String::from_utf8(buf).unwrap_or_default();
                    JsonFormatResult {
                        success: true,
                        formatted,
                        error: None,
                    }
                }
                Err(e) => JsonFormatResult {
                    success: false,
                    formatted: String::new(),
                    error: Some(e.to_string()),
                },
            }
        }
        Err(e) => JsonFormatResult {
            success: false,
            formatted: String::new(),
            error: Some(format!("Parse error: {}", e)),
        },
    }
}

pub fn validate_json(input: &str) -> JsonValidateResult {
    match serde_json::from_str::<Value>(input) {
        Ok(_) => JsonValidateResult {
            valid: true,
            error: None,
            error_position: None,
        },
        Err(e) => {
            let error_position = Some(ErrorPosition {
                line: e.line(),
                column: e.column(),
            });
            JsonValidateResult {
                valid: false,
                error: Some(e.to_string()),
                error_position,
            }
        }
    }
}

pub fn minify_json(input: &str) -> JsonMinifyResult {
    let original_size = input.len();

    match serde_json::from_str::<Value>(input) {
        Ok(value) => match serde_json::to_string(&value) {
            Ok(minified) => {
                let minified_size = minified.len();
                let savings = if original_size > 0 {
                    ((original_size - minified_size) as f64 / original_size as f64) * 100.0
                } else {
                    0.0
                };
                JsonMinifyResult {
                    success: true,
                    minified,
                    original_size,
                    minified_size,
                    savings_percent: (savings * 100.0).round() / 100.0,
                    error: None,
                }
            }
            Err(e) => JsonMinifyResult {
                success: false,
                minified: String::new(),
                original_size,
                minified_size: 0,
                savings_percent: 0.0,
                error: Some(e.to_string()),
            },
        },
        Err(e) => JsonMinifyResult {
            success: false,
            minified: String::new(),
            original_size,
            minified_size: 0,
            savings_percent: 0.0,
            error: Some(format!("Parse error: {}", e)),
        },
    }
}

pub fn parse_to_tree(input: &str) -> JsonParseResult {
    match serde_json::from_str::<Value>(input) {
        Ok(value) => {
            let tree = value_to_tree(&value, "root".to_string(), "$".to_string());
            JsonParseResult {
                success: true,
                tree: Some(tree),
                error: None,
            }
        }
        Err(e) => JsonParseResult {
            success: false,
            tree: None,
            error: Some(format!("Parse error: {}", e)),
        },
    }
}

fn value_to_tree(value: &Value, key: String, path: String) -> JsonTreeNode {
    match value {
        Value::Object(map) => {
            let children: Vec<JsonTreeNode> = map
                .iter()
                .map(|(k, v)| {
                    let child_path = format!("{}.{}", path, k);
                    value_to_tree(v, k.clone(), child_path)
                })
                .collect();

            JsonTreeNode {
                key,
                value_type: JsonValueType::Object,
                value: Some(format!("{{{} keys}}", map.len())),
                path,
                children,
                expanded: true,
            }
        }
        Value::Array(arr) => {
            let children: Vec<JsonTreeNode> = arr
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let child_path = format!("{}[{}]", path, i);
                    value_to_tree(v, format!("[{}]", i), child_path)
                })
                .collect();

            JsonTreeNode {
                key,
                value_type: JsonValueType::Array,
                value: Some(format!("[{} items]", arr.len())),
                path,
                children,
                expanded: true,
            }
        }
        Value::String(s) => JsonTreeNode {
            key,
            value_type: JsonValueType::String,
            value: Some(format!("\"{}\"", s)),
            path,
            children: vec![],
            expanded: false,
        },
        Value::Number(n) => JsonTreeNode {
            key,
            value_type: JsonValueType::Number,
            value: Some(n.to_string()),
            path,
            children: vec![],
            expanded: false,
        },
        Value::Bool(b) => JsonTreeNode {
            key,
            value_type: JsonValueType::Boolean,
            value: Some(b.to_string()),
            path,
            children: vec![],
            expanded: false,
        },
        Value::Null => JsonTreeNode {
            key,
            value_type: JsonValueType::Null,
            value: Some("null".to_string()),
            path,
            children: vec![],
            expanded: false,
        },
    }
}

pub fn search_json(
    input: &str,
    query: &str,
    search_keys: bool,
    search_values: bool,
) -> JsonSearchResult {
    if query.is_empty() {
        return JsonSearchResult {
            success: true,
            matches: vec![],
            total_count: 0,
            error: None,
        };
    }

    match serde_json::from_str::<Value>(input) {
        Ok(value) => {
            let mut matches = Vec::new();
            let query_lower = query.to_lowercase();
            search_value(
                &value,
                "$".to_string(),
                &query_lower,
                search_keys,
                search_values,
                &mut matches,
            );

            let total_count = matches.len();
            JsonSearchResult {
                success: true,
                matches,
                total_count,
                error: None,
            }
        }
        Err(e) => JsonSearchResult {
            success: false,
            matches: vec![],
            total_count: 0,
            error: Some(format!("Parse error: {}", e)),
        },
    }
}

fn search_value(
    value: &Value,
    path: String,
    query: &str,
    search_keys: bool,
    search_values: bool,
    matches: &mut Vec<JsonSearchMatch>,
) {
    match value {
        Value::Object(map) => {
            for (k, v) in map {
                let child_path = format!("{}.{}", path, k);

                if search_keys && k.to_lowercase().contains(query) {
                    matches.push(JsonSearchMatch {
                        path: child_path.clone(),
                        key: k.clone(),
                        value: value_to_string(v),
                        value_type: get_value_type(v),
                    });
                }

                search_value(v, child_path, query, search_keys, search_values, matches);
            }
        }
        Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                let child_path = format!("{}[{}]", path, i);
                search_value(v, child_path, query, search_keys, search_values, matches);
            }
        }
        Value::String(s) => {
            if search_values && s.to_lowercase().contains(query) {
                let key = path.split('.').last().unwrap_or("").to_string();
                matches.push(JsonSearchMatch {
                    path,
                    key,
                    value: format!("\"{}\"", s),
                    value_type: JsonValueType::String,
                });
            }
        }
        Value::Number(n) => {
            if search_values && n.to_string().contains(query) {
                let key = path.split('.').last().unwrap_or("").to_string();
                matches.push(JsonSearchMatch {
                    path,
                    key,
                    value: n.to_string(),
                    value_type: JsonValueType::Number,
                });
            }
        }
        Value::Bool(b) => {
            if search_values && b.to_string().to_lowercase().contains(query) {
                let key = path.split('.').last().unwrap_or("").to_string();
                matches.push(JsonSearchMatch {
                    path,
                    key,
                    value: b.to_string(),
                    value_type: JsonValueType::Boolean,
                });
            }
        }
        Value::Null => {
            if search_values && "null".contains(query) {
                let key = path.split('.').last().unwrap_or("").to_string();
                matches.push(JsonSearchMatch {
                    path,
                    key,
                    value: "null".to_string(),
                    value_type: JsonValueType::Null,
                });
            }
        }
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => format!("\"{}\"", s),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        Value::Object(map) => format!("{{{} keys}}", map.len()),
        Value::Array(arr) => format!("[{} items]", arr.len()),
    }
}

fn get_value_type(value: &Value) -> JsonValueType {
    match value {
        Value::Object(_) => JsonValueType::Object,
        Value::Array(_) => JsonValueType::Array,
        Value::String(_) => JsonValueType::String,
        Value::Number(_) => JsonValueType::Number,
        Value::Bool(_) => JsonValueType::Boolean,
        Value::Null => JsonValueType::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_json() {
        let input = r#"{"name":"test","value":123}"#;
        let result = format_json(input, 2);
        assert!(result.success);
        assert!(result.formatted.contains("\"name\": \"test\""));
    }

    #[test]
    fn test_validate_json_valid() {
        let input = r#"{"name": "test"}"#;
        let result = validate_json(input);
        assert!(result.valid);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_validate_json_invalid() {
        let input = r#"{"name": }"#;
        let result = validate_json(input);
        assert!(!result.valid);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_minify_json() {
        let input = r#"{
            "name": "test",
            "value": 123
        }"#;
        let result = minify_json(input);
        assert!(result.success);
        assert_eq!(result.minified, r#"{"name":"test","value":123}"#);
        assert!(result.savings_percent > 0.0);
    }

    #[test]
    fn test_parse_to_tree() {
        let input = r#"{"name": "test", "nested": {"value": 123}}"#;
        let result = parse_to_tree(input);
        assert!(result.success);
        assert!(result.tree.is_some());
        let tree = result.tree.unwrap();
        assert_eq!(tree.value_type, JsonValueType::Object);
    }

    #[test]
    fn test_search_json() {
        let input = r#"{"name": "test", "description": "testing value"}"#;
        let result = search_json(input, "test", true, true);
        assert!(result.success);
        assert!(result.total_count > 0);
    }
}
