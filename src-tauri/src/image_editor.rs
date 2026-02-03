use image::{DynamicImage, ImageBuffer, ImageFormat, ImageReader, Rgba};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Cursor;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageEditorInfo {
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub file_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditResult {
    pub success: bool,
    pub output_path: String,
    pub original_size: u64,
    pub new_size: u64,
    pub new_width: u32,
    pub new_height: u32,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RotationAngle {
    Rotate90,
    Rotate180,
    Rotate270,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ImageFilter {
    Grayscale,
    Sepia,
    Invert,
    Blur,
    Sharpen,
}

pub fn get_editor_image_info(path: &str) -> Result<ImageEditorInfo, String> {
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

    Ok(ImageEditorInfo {
        width: img.width(),
        height: img.height(),
        format,
        file_size,
    })
}

fn load_image(path: &str) -> Result<(DynamicImage, u64), String> {
    let input = Path::new(path);
    let original_size = fs::metadata(input)
        .map_err(|e| format!("Failed to read file: {}", e))?
        .len();

    let img = ImageReader::open(input)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    Ok((img, original_size))
}

fn save_image(img: &DynamicImage, output_path: &str) -> Result<(), String> {
    let output = Path::new(output_path);
    let format = output
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "png".to_string());

    match format.as_str() {
        "jpg" | "jpeg" => {
            let rgb = img.to_rgb8();
            let mut buffer = Cursor::new(Vec::new());
            let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, 90);
            rgb.write_with_encoder(encoder)
                .map_err(|e| format!("Failed to encode JPEG: {}", e))?;
            fs::write(output, buffer.into_inner())
                .map_err(|e| format!("Failed to write file: {}", e))?;
        }
        _ => {
            img.save_with_format(output, ImageFormat::Png)
                .map_err(|e| format!("Failed to save image: {}", e))?;
        }
    }

    Ok(())
}

fn create_result(
    success: bool,
    output_path: &str,
    original_size: u64,
    img: Option<&DynamicImage>,
    error: Option<String>,
) -> EditResult {
    let (new_size, new_width, new_height) = if success {
        let new_size = fs::metadata(output_path).map(|m| m.len()).unwrap_or(0);
        let (w, h) = img.map(|i| (i.width(), i.height())).unwrap_or((0, 0));
        (new_size, w, h)
    } else {
        (0, 0, 0)
    };

    EditResult {
        success,
        output_path: output_path.to_string(),
        original_size,
        new_size,
        new_width,
        new_height,
        error,
    }
}

pub fn resize_image(
    input_path: &str,
    output_path: &str,
    width: u32,
    height: u32,
    maintain_aspect: bool,
) -> EditResult {
    let (img, original_size) = match load_image(input_path) {
        Ok(result) => result,
        Err(e) => return create_result(false, output_path, 0, None, Some(e)),
    };

    let resized = if maintain_aspect {
        img.resize(width, height, image::imageops::FilterType::Lanczos3)
    } else {
        img.resize_exact(width, height, image::imageops::FilterType::Lanczos3)
    };

    if let Err(e) = save_image(&resized, output_path) {
        return create_result(false, output_path, original_size, None, Some(e));
    }

    create_result(true, output_path, original_size, Some(&resized), None)
}

pub fn rotate_image(input_path: &str, output_path: &str, angle: RotationAngle) -> EditResult {
    let (img, original_size) = match load_image(input_path) {
        Ok(result) => result,
        Err(e) => return create_result(false, output_path, 0, None, Some(e)),
    };

    let rotated = match angle {
        RotationAngle::Rotate90 => img.rotate90(),
        RotationAngle::Rotate180 => img.rotate180(),
        RotationAngle::Rotate270 => img.rotate270(),
    };

    if let Err(e) = save_image(&rotated, output_path) {
        return create_result(false, output_path, original_size, None, Some(e));
    }

    create_result(true, output_path, original_size, Some(&rotated), None)
}

pub fn crop_image(
    input_path: &str,
    output_path: &str,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> EditResult {
    let (img, original_size) = match load_image(input_path) {
        Ok(result) => result,
        Err(e) => return create_result(false, output_path, 0, None, Some(e)),
    };

    if x + width > img.width() || y + height > img.height() {
        return create_result(
            false,
            output_path,
            original_size,
            None,
            Some("Crop area exceeds image bounds".to_string()),
        );
    }

    let cropped = img.crop_imm(x, y, width, height);

    if let Err(e) = save_image(&cropped, output_path) {
        return create_result(false, output_path, original_size, None, Some(e));
    }

    create_result(true, output_path, original_size, Some(&cropped), None)
}

pub fn adjust_brightness(input_path: &str, output_path: &str, value: i32) -> EditResult {
    let (img, original_size) = match load_image(input_path) {
        Ok(result) => result,
        Err(e) => return create_result(false, output_path, 0, None, Some(e)),
    };

    let adjusted = image::imageops::brighten(&img, value);
    let dynamic_img = DynamicImage::ImageRgba8(adjusted);

    if let Err(e) = save_image(&dynamic_img, output_path) {
        return create_result(false, output_path, original_size, None, Some(e));
    }

    create_result(true, output_path, original_size, Some(&dynamic_img), None)
}

pub fn adjust_contrast(input_path: &str, output_path: &str, value: f32) -> EditResult {
    let (img, original_size) = match load_image(input_path) {
        Ok(result) => result,
        Err(e) => return create_result(false, output_path, 0, None, Some(e)),
    };

    let adjusted = image::imageops::contrast(&img, value);
    let dynamic_img = DynamicImage::ImageRgba8(adjusted);

    if let Err(e) = save_image(&dynamic_img, output_path) {
        return create_result(false, output_path, original_size, None, Some(e));
    }

    create_result(true, output_path, original_size, Some(&dynamic_img), None)
}

pub fn apply_filter(input_path: &str, output_path: &str, filter: ImageFilter) -> EditResult {
    let (img, original_size) = match load_image(input_path) {
        Ok(result) => result,
        Err(e) => return create_result(false, output_path, 0, None, Some(e)),
    };

    let filtered: DynamicImage = match filter {
        ImageFilter::Grayscale => DynamicImage::ImageLuma8(img.to_luma8()),
        ImageFilter::Sepia => apply_sepia(&img),
        ImageFilter::Invert => {
            let mut inverted = img.clone();
            inverted.invert();
            inverted
        }
        ImageFilter::Blur => img.blur(3.0),
        ImageFilter::Sharpen => img.unsharpen(1.0, 5),
    };

    if let Err(e) = save_image(&filtered, output_path) {
        return create_result(false, output_path, original_size, None, Some(e));
    }

    create_result(true, output_path, original_size, Some(&filtered), None)
}

fn apply_sepia(img: &DynamicImage) -> DynamicImage {
    let rgba = img.to_rgba8();
    let (width, height) = (rgba.width(), rgba.height());

    let mut output: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (x, y, pixel) in rgba.enumerate_pixels() {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;

        let new_r = (0.393 * r + 0.769 * g + 0.189 * b).min(255.0) as u8;
        let new_g = (0.349 * r + 0.686 * g + 0.168 * b).min(255.0) as u8;
        let new_b = (0.272 * r + 0.534 * g + 0.131 * b).min(255.0) as u8;

        output.put_pixel(x, y, Rgba([new_r, new_g, new_b, pixel[3]]));
    }

    DynamicImage::ImageRgba8(output)
}

pub fn flip_horizontal(input_path: &str, output_path: &str) -> EditResult {
    let (img, original_size) = match load_image(input_path) {
        Ok(result) => result,
        Err(e) => return create_result(false, output_path, 0, None, Some(e)),
    };

    let flipped = img.fliph();

    if let Err(e) = save_image(&flipped, output_path) {
        return create_result(false, output_path, original_size, None, Some(e));
    }

    create_result(true, output_path, original_size, Some(&flipped), None)
}

pub fn flip_vertical(input_path: &str, output_path: &str) -> EditResult {
    let (img, original_size) = match load_image(input_path) {
        Ok(result) => result,
        Err(e) => return create_result(false, output_path, 0, None, Some(e)),
    };

    let flipped = img.flipv();

    if let Err(e) = save_image(&flipped, output_path) {
        return create_result(false, output_path, original_size, None, Some(e));
    }

    create_result(true, output_path, original_size, Some(&flipped), None)
}
