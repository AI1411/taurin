use image::{DynamicImage, ImageFormat, ImageReader};
use ravif::{Encoder, Img};
use rgb::RGBA8;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Cursor;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionOptions {
    pub quality: u8,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub output_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionResult {
    pub success: bool,
    pub output_path: String,
    pub original_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f64,
    pub error: Option<String>,
}

pub fn compress_image(
    input_path: &str,
    output_path: &str,
    options: CompressionOptions,
) -> CompressionResult {
    let input = Path::new(input_path);
    let output = Path::new(output_path);

    let original_size = match fs::metadata(input) {
        Ok(meta) => meta.len(),
        Err(e) => {
            return CompressionResult {
                success: false,
                output_path: output_path.to_string(),
                original_size: 0,
                compressed_size: 0,
                compression_ratio: 0.0,
                error: Some(format!("Failed to read input file: {}", e)),
            };
        }
    };

    let img = match ImageReader::open(input) {
        Ok(reader) => match reader.decode() {
            Ok(img) => img,
            Err(e) => {
                return CompressionResult {
                    success: false,
                    output_path: output_path.to_string(),
                    original_size,
                    compressed_size: 0,
                    compression_ratio: 0.0,
                    error: Some(format!("Failed to decode image: {}", e)),
                };
            }
        },
        Err(e) => {
            return CompressionResult {
                success: false,
                output_path: output_path.to_string(),
                original_size,
                compressed_size: 0,
                compression_ratio: 0.0,
                error: Some(format!("Failed to open image: {}", e)),
            };
        }
    };

    let img = resize_if_needed(img, options.width, options.height);

    let result = match options.output_format.to_lowercase().as_str() {
        "avif" => save_as_avif(&img, output, options.quality),
        "webp" => save_as_webp(&img, output, options.quality),
        "jpeg" | "jpg" => save_as_jpeg(&img, output, options.quality),
        "png" => save_as_png(&img, output),
        _ => Err(format!("Unsupported format: {}", options.output_format)),
    };

    match result {
        Ok(_) => {
            let compressed_size = fs::metadata(output).map(|m| m.len()).unwrap_or(0);
            let compression_ratio = if original_size > 0 {
                (1.0 - (compressed_size as f64 / original_size as f64)) * 100.0
            } else {
                0.0
            };

            CompressionResult {
                success: true,
                output_path: output_path.to_string(),
                original_size,
                compressed_size,
                compression_ratio,
                error: None,
            }
        }
        Err(e) => CompressionResult {
            success: false,
            output_path: output_path.to_string(),
            original_size,
            compressed_size: 0,
            compression_ratio: 0.0,
            error: Some(e),
        },
    }
}

fn resize_if_needed(img: DynamicImage, width: Option<u32>, height: Option<u32>) -> DynamicImage {
    match (width, height) {
        (Some(w), Some(h)) => img.resize_exact(w, h, image::imageops::FilterType::Lanczos3),
        (Some(w), None) => {
            let ratio = w as f64 / img.width() as f64;
            let h = (img.height() as f64 * ratio) as u32;
            img.resize_exact(w, h, image::imageops::FilterType::Lanczos3)
        }
        (None, Some(h)) => {
            let ratio = h as f64 / img.height() as f64;
            let w = (img.width() as f64 * ratio) as u32;
            img.resize_exact(w, h, image::imageops::FilterType::Lanczos3)
        }
        (None, None) => img,
    }
}

fn save_as_avif(img: &DynamicImage, output: &Path, quality: u8) -> Result<(), String> {
    let rgba = img.to_rgba8();
    let width = rgba.width() as usize;
    let height = rgba.height() as usize;

    let pixels: Vec<RGBA8> = rgba
        .pixels()
        .map(|p| RGBA8::new(p[0], p[1], p[2], p[3]))
        .collect();

    let img_ref = Img::new(&pixels[..], width, height);

    let encoder = Encoder::new().with_quality(quality as f32).with_speed(6);

    let encoded = encoder
        .encode_rgba(img_ref)
        .map_err(|e| format!("AVIF encoding failed: {}", e))?;

    fs::write(output, encoded.avif_file).map_err(|e| format!("Failed to write AVIF file: {}", e))
}

fn save_as_webp(img: &DynamicImage, output: &Path, quality: u8) -> Result<(), String> {
    let rgba = img.to_rgba8();
    let width = rgba.width();
    let height = rgba.height();

    let encoder = webp::Encoder::from_rgba(&rgba, width, height);
    let webp_data = encoder.encode(quality as f32);

    fs::write(output, &*webp_data).map_err(|e| format!("Failed to write WebP file: {}", e))
}

fn save_as_jpeg(img: &DynamicImage, output: &Path, quality: u8) -> Result<(), String> {
    let rgb = img.to_rgb8();
    let mut buffer = Cursor::new(Vec::new());

    let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, quality);
    rgb.write_with_encoder(encoder)
        .map_err(|e| format!("JPEG encoding failed: {}", e))?;

    fs::write(output, buffer.into_inner()).map_err(|e| format!("Failed to write JPEG file: {}", e))
}

fn save_as_png(img: &DynamicImage, output: &Path) -> Result<(), String> {
    img.save_with_format(output, ImageFormat::Png)
        .map_err(|e| format!("PNG encoding failed: {}", e))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub file_size: u64,
}

pub fn get_image_info(path: &str) -> Result<ImageInfo, String> {
    let input = Path::new(path);

    let file_size = fs::metadata(input)
        .map_err(|e| format!("Failed to read file: {}", e))?
        .len();

    let img = ImageReader::open(input)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    let format = input
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("unknown")
        .to_uppercase();

    Ok(ImageInfo {
        width: img.width(),
        height: img.height(),
        format,
        file_size,
    })
}
