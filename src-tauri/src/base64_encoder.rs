use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Base64EncodeResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Base64DecodeResult {
    pub success: bool,
    pub output: String,
    pub is_valid_utf8: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Base64ImageResult {
    pub success: bool,
    pub output: String,
    pub mime_type: String,
    pub data_url: String,
    pub size_bytes: usize,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Base64DecodeImageResult {
    pub success: bool,
    pub mime_type: Option<String>,
    pub size_bytes: usize,
    pub preview_data_url: Option<String>,
    pub error: Option<String>,
}

/// Encode a string to Base64
pub fn encode_base64(input: &str, url_safe: bool) -> Base64EncodeResult {
    use base64::{engine::general_purpose, Engine};

    if input.is_empty() {
        return Base64EncodeResult {
            success: false,
            output: String::new(),
            error: Some("Input is empty".to_string()),
        };
    }

    let encoded = if url_safe {
        general_purpose::URL_SAFE.encode(input.as_bytes())
    } else {
        general_purpose::STANDARD.encode(input.as_bytes())
    };

    Base64EncodeResult {
        success: true,
        output: encoded,
        error: None,
    }
}

/// Decode a Base64 string
pub fn decode_base64(input: &str, url_safe: bool) -> Base64DecodeResult {
    use base64::{engine::general_purpose, Engine};

    if input.is_empty() {
        return Base64DecodeResult {
            success: false,
            output: String::new(),
            is_valid_utf8: false,
            error: Some("Input is empty".to_string()),
        };
    }

    // Remove whitespace from input
    let cleaned_input: String = input.chars().filter(|c| !c.is_whitespace()).collect();

    let decode_result = if url_safe {
        general_purpose::URL_SAFE.decode(&cleaned_input)
    } else {
        // Try standard first, then URL safe
        general_purpose::STANDARD
            .decode(&cleaned_input)
            .or_else(|_| general_purpose::URL_SAFE.decode(&cleaned_input))
    };

    match decode_result {
        Ok(bytes) => {
            let is_valid_utf8 = String::from_utf8(bytes.clone()).is_ok();
            let output = if is_valid_utf8 {
                String::from_utf8(bytes).unwrap()
            } else {
                // Convert to hex representation for binary data
                bytes
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ")
            };

            Base64DecodeResult {
                success: true,
                output,
                is_valid_utf8,
                error: None,
            }
        }
        Err(e) => Base64DecodeResult {
            success: false,
            output: String::new(),
            is_valid_utf8: false,
            error: Some(format!("Invalid Base64: {}", e)),
        },
    }
}

/// Encode an image file to Base64
pub fn encode_image_to_base64(path: &str) -> Base64ImageResult {
    use base64::{engine::general_purpose, Engine};

    let path = Path::new(path);

    if !path.exists() {
        return Base64ImageResult {
            success: false,
            output: String::new(),
            mime_type: String::new(),
            data_url: String::new(),
            size_bytes: 0,
            error: Some("File not found".to_string()),
        };
    }

    // Determine MIME type from extension
    let mime_type = match path.extension().and_then(|e| e.to_str()) {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("bmp") => "image/bmp",
        Some("avif") => "image/avif",
        _ => "application/octet-stream",
    };

    match fs::read(path) {
        Ok(bytes) => {
            let size_bytes = bytes.len();
            let encoded = general_purpose::STANDARD.encode(&bytes);
            let data_url = format!("data:{};base64,{}", mime_type, encoded);

            Base64ImageResult {
                success: true,
                output: encoded,
                mime_type: mime_type.to_string(),
                data_url,
                size_bytes,
                error: None,
            }
        }
        Err(e) => Base64ImageResult {
            success: false,
            output: String::new(),
            mime_type: String::new(),
            data_url: String::new(),
            size_bytes: 0,
            error: Some(format!("Failed to read file: {}", e)),
        },
    }
}

/// Decode Base64 and detect if it's an image
pub fn decode_base64_image(input: &str) -> Base64DecodeImageResult {
    use base64::{engine::general_purpose, Engine};

    if input.is_empty() {
        return Base64DecodeImageResult {
            success: false,
            mime_type: None,
            size_bytes: 0,
            preview_data_url: None,
            error: Some("Input is empty".to_string()),
        };
    }

    // Handle data URL format
    let (mime_type, base64_data) = if input.starts_with("data:") {
        // Parse data URL: data:image/png;base64,xxxxx
        if let Some(comma_pos) = input.find(',') {
            let header = &input[5..comma_pos]; // Skip "data:"
            let mime = if let Some(semi_pos) = header.find(';') {
                Some(header[..semi_pos].to_string())
            } else {
                Some(header.to_string())
            };
            (mime, &input[comma_pos + 1..])
        } else {
            (None, input)
        }
    } else {
        (None, input)
    };

    // Remove whitespace from base64 data
    let cleaned_input: String = base64_data.chars().filter(|c| !c.is_whitespace()).collect();

    match general_purpose::STANDARD.decode(&cleaned_input) {
        Ok(bytes) => {
            let size_bytes = bytes.len();

            // Detect image type from magic bytes
            let detected_mime = detect_image_type(&bytes);
            let final_mime = mime_type.or(detected_mime);

            // Generate preview data URL if it's an image
            let preview_data_url = if let Some(ref mime) = final_mime {
                if mime.starts_with("image/") {
                    Some(format!("data:{};base64,{}", mime, cleaned_input))
                } else {
                    None
                }
            } else {
                None
            };

            Base64DecodeImageResult {
                success: true,
                mime_type: final_mime,
                size_bytes,
                preview_data_url,
                error: None,
            }
        }
        Err(e) => Base64DecodeImageResult {
            success: false,
            mime_type: None,
            size_bytes: 0,
            preview_data_url: None,
            error: Some(format!("Invalid Base64: {}", e)),
        },
    }
}

/// Detect image type from magic bytes
fn detect_image_type(bytes: &[u8]) -> Option<String> {
    if bytes.len() < 4 {
        return None;
    }

    // PNG: 89 50 4E 47
    if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
        return Some("image/png".to_string());
    }

    // JPEG: FF D8 FF
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return Some("image/jpeg".to_string());
    }

    // GIF: 47 49 46 38
    if bytes.starts_with(&[0x47, 0x49, 0x46, 0x38]) {
        return Some("image/gif".to_string());
    }

    // WebP: 52 49 46 46 ... 57 45 42 50
    if bytes.len() >= 12
        && bytes.starts_with(&[0x52, 0x49, 0x46, 0x46])
        && bytes[8..12] == [0x57, 0x45, 0x42, 0x50]
    {
        return Some("image/webp".to_string());
    }

    // BMP: 42 4D
    if bytes.starts_with(&[0x42, 0x4D]) {
        return Some("image/bmp".to_string());
    }

    // ICO: 00 00 01 00
    if bytes.starts_with(&[0x00, 0x00, 0x01, 0x00]) {
        return Some("image/x-icon".to_string());
    }

    // AVIF: Check for "ftyp" box with AVIF brand
    if bytes.len() >= 12 && &bytes[4..8] == b"ftyp" {
        let brand = &bytes[8..12];
        if brand == b"avif" || brand == b"avis" || brand == b"mif1" {
            return Some("image/avif".to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_base64() {
        let result = encode_base64("Hello, World!", false);
        assert!(result.success);
        assert_eq!(result.output, "SGVsbG8sIFdvcmxkIQ==");
    }

    #[test]
    fn test_decode_base64() {
        let result = decode_base64("SGVsbG8sIFdvcmxkIQ==", false);
        assert!(result.success);
        assert_eq!(result.output, "Hello, World!");
        assert!(result.is_valid_utf8);
    }

    #[test]
    fn test_encode_url_safe() {
        let input = "Hello+World/Test";
        let result = encode_base64(input, true);
        assert!(result.success);
        // URL safe should not contain + or /
        assert!(!result.output.contains('+') || result.output.contains('-'));
    }

    #[test]
    fn test_decode_invalid() {
        let result = decode_base64("not valid base64!!!", false);
        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_empty_input() {
        let encode_result = encode_base64("", false);
        assert!(!encode_result.success);

        let decode_result = decode_base64("", false);
        assert!(!decode_result.success);
    }
}
