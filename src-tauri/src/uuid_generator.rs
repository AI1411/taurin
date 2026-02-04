use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UuidVersion {
    V4,
    V7,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UuidFormat {
    Standard,
    NoHyphens,
    Uppercase,
    UppercaseNoHyphens,
    Braces,
    Urn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UuidGenerateOptions {
    pub version: UuidVersion,
    pub format: UuidFormat,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UuidGenerateResult {
    pub success: bool,
    pub uuids: Vec<String>,
    pub error: Option<String>,
}

fn format_uuid(uuid: &Uuid, format: &UuidFormat) -> String {
    match format {
        UuidFormat::Standard => uuid.to_string(),
        UuidFormat::NoHyphens => uuid.simple().to_string(),
        UuidFormat::Uppercase => uuid.to_string().to_uppercase(),
        UuidFormat::UppercaseNoHyphens => uuid.simple().to_string().to_uppercase(),
        UuidFormat::Braces => format!("{{{uuid}}}"),
        UuidFormat::Urn => uuid.urn().to_string(),
    }
}

pub fn generate_uuids(options: UuidGenerateOptions) -> UuidGenerateResult {
    let count = options.count.clamp(1, 1000);

    let uuids: Vec<String> = (0..count)
        .map(|_| {
            let uuid = match options.version {
                UuidVersion::V4 => Uuid::new_v4(),
                UuidVersion::V7 => Uuid::now_v7(),
            };
            format_uuid(&uuid, &options.format)
        })
        .collect();

    UuidGenerateResult {
        success: true,
        uuids,
        error: None,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UuidValidateResult {
    pub valid: bool,
    pub version: Option<String>,
    pub variant: Option<String>,
    pub error: Option<String>,
}

pub fn validate_uuid(input: &str) -> UuidValidateResult {
    match Uuid::parse_str(input.trim()) {
        Ok(uuid) => {
            let version = match uuid.get_version_num() {
                1 => "v1 (Time-based)",
                2 => "v2 (DCE Security)",
                3 => "v3 (MD5 Name-based)",
                4 => "v4 (Random)",
                5 => "v5 (SHA-1 Name-based)",
                6 => "v6 (Reordered Time-based)",
                7 => "v7 (Unix Epoch Time-based)",
                8 => "v8 (Custom)",
                _ => "Unknown",
            };

            let variant = match uuid.get_variant() {
                uuid::Variant::NCS => "NCS",
                uuid::Variant::RFC4122 => "RFC 4122",
                uuid::Variant::Microsoft => "Microsoft",
                uuid::Variant::Future => "Future",
                _ => "Unknown",
            };

            UuidValidateResult {
                valid: true,
                version: Some(version.to_string()),
                variant: Some(variant.to_string()),
                error: None,
            }
        }
        Err(e) => UuidValidateResult {
            valid: false,
            version: None,
            variant: None,
            error: Some(e.to_string()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_uuid_v4() {
        let options = UuidGenerateOptions {
            version: UuidVersion::V4,
            format: UuidFormat::Standard,
            count: 5,
        };
        let result = generate_uuids(options);
        assert!(result.success);
        assert_eq!(result.uuids.len(), 5);
    }

    #[test]
    fn test_uuid_formats() {
        let uuid = Uuid::new_v4();

        let standard = format_uuid(&uuid, &UuidFormat::Standard);
        assert!(standard.contains('-'));

        let no_hyphens = format_uuid(&uuid, &UuidFormat::NoHyphens);
        assert!(!no_hyphens.contains('-'));
        assert_eq!(no_hyphens.len(), 32);

        let uppercase = format_uuid(&uuid, &UuidFormat::Uppercase);
        assert_eq!(uppercase, uppercase.to_uppercase());

        let braces = format_uuid(&uuid, &UuidFormat::Braces);
        assert!(braces.starts_with('{') && braces.ends_with('}'));

        let urn = format_uuid(&uuid, &UuidFormat::Urn);
        assert!(urn.starts_with("urn:uuid:"));
    }

    #[test]
    fn test_validate_uuid() {
        let valid_uuid = "550e8400-e29b-41d4-a716-446655440000";
        let result = validate_uuid(valid_uuid);
        assert!(result.valid);
        assert!(result.version.is_some());

        let invalid_uuid = "not-a-uuid";
        let result = validate_uuid(invalid_uuid);
        assert!(!result.valid);
        assert!(result.error.is_some());
    }
}
