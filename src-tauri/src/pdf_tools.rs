use lopdf::{dictionary, Document, Object, ObjectId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfInfo {
    pub page_count: u32,
    pub file_size: u64,
    pub file_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfSplitResult {
    pub success: bool,
    pub output_paths: Vec<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfMergeResult {
    pub success: bool,
    pub output_path: String,
    pub page_count: u32,
    pub file_size: u64,
    pub error: Option<String>,
}

pub fn get_pdf_info(path: &str) -> Result<PdfInfo, String> {
    let metadata =
        fs::metadata(path).map_err(|e| format!("Failed to read file metadata: {}", e))?;
    let file_size = metadata.len();

    let file_name = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let doc = Document::load(path).map_err(|e| format!("Failed to load PDF: {}", e))?;
    let page_count = doc.get_pages().len() as u32;

    Ok(PdfInfo {
        page_count,
        file_size,
        file_name,
    })
}

pub fn split_pdf_by_pages(input_path: &str, output_dir: &str) -> PdfSplitResult {
    let doc = match Document::load(input_path) {
        Ok(d) => d,
        Err(e) => {
            return PdfSplitResult {
                success: false,
                output_paths: vec![],
                error: Some(format!("Failed to load PDF: {}", e)),
            }
        }
    };

    let pages = doc.get_pages();
    let page_ids: Vec<ObjectId> = pages.values().copied().collect();
    let mut output_paths = Vec::new();

    let input_stem = Path::new(input_path)
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("page");

    for (i, &page_id) in page_ids.iter().enumerate() {
        let output_path = format!("{}/{}_page_{}.pdf", output_dir, input_stem, i + 1);

        match extract_pages(&doc, &[page_id], &output_path) {
            Ok(_) => output_paths.push(output_path),
            Err(e) => {
                return PdfSplitResult {
                    success: false,
                    output_paths: vec![],
                    error: Some(format!("Failed to extract page {}: {}", i + 1, e)),
                }
            }
        }
    }

    PdfSplitResult {
        success: true,
        output_paths,
        error: None,
    }
}

pub fn split_pdf_by_range(
    input_path: &str,
    output_path: &str,
    start_page: u32,
    end_page: u32,
) -> PdfSplitResult {
    let doc = match Document::load(input_path) {
        Ok(d) => d,
        Err(e) => {
            return PdfSplitResult {
                success: false,
                output_paths: vec![],
                error: Some(format!("Failed to load PDF: {}", e)),
            }
        }
    };

    let pages = doc.get_pages();
    let page_count = pages.len() as u32;

    if start_page < 1 || end_page > page_count || start_page > end_page {
        return PdfSplitResult {
            success: false,
            output_paths: vec![],
            error: Some(format!(
                "Invalid page range: {}-{} (document has {} pages)",
                start_page, end_page, page_count
            )),
        };
    }

    let page_ids: Vec<ObjectId> = pages.values().copied().collect();
    let selected_pages: Vec<ObjectId> =
        page_ids[(start_page as usize - 1)..(end_page as usize)].to_vec();

    match extract_pages(&doc, &selected_pages, output_path) {
        Ok(_) => PdfSplitResult {
            success: true,
            output_paths: vec![output_path.to_string()],
            error: None,
        },
        Err(e) => PdfSplitResult {
            success: false,
            output_paths: vec![],
            error: Some(format!("Failed to extract pages: {}", e)),
        },
    }
}

pub fn merge_pdfs(input_paths: &[String], output_path: &str) -> PdfMergeResult {
    if input_paths.is_empty() {
        return PdfMergeResult {
            success: false,
            output_path: String::new(),
            page_count: 0,
            file_size: 0,
            error: Some("No input files provided".to_string()),
        };
    }

    // Load the first document as the base
    let mut merged_doc = match Document::load(&input_paths[0]) {
        Ok(d) => d,
        Err(e) => {
            return PdfMergeResult {
                success: false,
                output_path: String::new(),
                page_count: 0,
                file_size: 0,
                error: Some(format!("Failed to load {}: {}", input_paths[0], e)),
            }
        }
    };

    // For each additional document, merge its pages
    for path in input_paths.iter().skip(1) {
        let doc = match Document::load(path) {
            Ok(d) => d,
            Err(e) => {
                return PdfMergeResult {
                    success: false,
                    output_path: String::new(),
                    page_count: 0,
                    file_size: 0,
                    error: Some(format!("Failed to load {}: {}", path, e)),
                }
            }
        };

        if let Err(e) = append_document(&mut merged_doc, &doc) {
            return PdfMergeResult {
                success: false,
                output_path: String::new(),
                page_count: 0,
                file_size: 0,
                error: Some(format!("Failed to merge {}: {}", path, e)),
            };
        }
    }

    if let Err(e) = merged_doc.save(output_path) {
        return PdfMergeResult {
            success: false,
            output_path: String::new(),
            page_count: 0,
            file_size: 0,
            error: Some(format!("Failed to save merged PDF: {}", e)),
        };
    }

    let file_size = fs::metadata(output_path).map(|m| m.len()).unwrap_or(0);
    let page_count = merged_doc.get_pages().len() as u32;

    PdfMergeResult {
        success: true,
        output_path: output_path.to_string(),
        page_count,
        file_size,
        error: None,
    }
}

fn extract_pages(
    src_doc: &Document,
    page_ids: &[ObjectId],
    output_path: &str,
) -> Result<(), String> {
    let mut new_doc = Document::with_version("1.5");
    let mut object_map: BTreeMap<ObjectId, ObjectId> = BTreeMap::new();

    // Create the pages array
    let mut kids: Vec<Object> = Vec::new();

    for &page_id in page_ids {
        let new_page_id = clone_object_recursive(src_doc, &mut new_doc, page_id, &mut object_map)?;
        kids.push(Object::Reference(new_page_id));
    }

    // Create the Pages dictionary
    let pages_dict = dictionary! {
        "Type" => "Pages",
        "Kids" => kids.clone(),
        "Count" => kids.len() as i64,
    };
    let pages_id = new_doc.add_object(Object::Dictionary(pages_dict));

    // Update parent references for all pages
    for kid in &kids {
        if let Object::Reference(page_id) = kid {
            if let Ok(Object::Dictionary(ref mut page_dict)) = new_doc.get_object_mut(*page_id) {
                page_dict.set("Parent", Object::Reference(pages_id));
            }
        }
    }

    // Create the Catalog
    let catalog_dict = dictionary! {
        "Type" => "Catalog",
        "Pages" => Object::Reference(pages_id),
    };
    let catalog_id = new_doc.add_object(Object::Dictionary(catalog_dict));
    new_doc.trailer.set("Root", Object::Reference(catalog_id));

    new_doc
        .save(output_path)
        .map_err(|e| format!("Failed to save: {}", e))?;
    Ok(())
}

fn append_document(dest_doc: &mut Document, src_doc: &Document) -> Result<(), String> {
    let mut object_map: BTreeMap<ObjectId, ObjectId> = BTreeMap::new();

    // Get the destination pages object
    let dest_pages_id = get_pages_id(dest_doc)?;

    // Clone all pages from source and add to destination
    let src_page_ids: Vec<ObjectId> = src_doc.get_pages().values().copied().collect();

    for page_id in src_page_ids {
        let new_page_id = clone_object_recursive(src_doc, dest_doc, page_id, &mut object_map)?;

        // Add to Kids array and update Count
        if let Ok(Object::Dictionary(ref mut pages_dict)) = dest_doc.get_object_mut(dest_pages_id) {
            if let Ok(Object::Array(ref mut kids)) = pages_dict.get_mut(b"Kids") {
                kids.push(Object::Reference(new_page_id));
            }

            let count = if let Ok(count_obj) = pages_dict.get(b"Count") {
                count_obj.as_i64().unwrap_or(0)
            } else {
                0
            } + 1;
            pages_dict.set("Count", Object::Integer(count));
        }

        // Update parent reference
        if let Ok(Object::Dictionary(ref mut page_dict)) = dest_doc.get_object_mut(new_page_id) {
            page_dict.set("Parent", Object::Reference(dest_pages_id));
        }
    }

    Ok(())
}

fn get_pages_id(doc: &Document) -> Result<ObjectId, String> {
    let catalog = doc
        .catalog()
        .map_err(|e| format!("Failed to get catalog: {}", e))?;

    match catalog.get(b"Pages") {
        Ok(Object::Reference(id)) => Ok(*id),
        _ => Err("Failed to get Pages reference".to_string()),
    }
}

fn clone_object_recursive(
    src_doc: &Document,
    dest_doc: &mut Document,
    object_id: ObjectId,
    object_map: &mut BTreeMap<ObjectId, ObjectId>,
) -> Result<ObjectId, String> {
    if let Some(&new_id) = object_map.get(&object_id) {
        return Ok(new_id);
    }

    let obj = src_doc
        .get_object(object_id)
        .map_err(|e| format!("Failed to get object {:?}: {}", object_id, e))?;

    let new_obj = clone_object_value(src_doc, dest_doc, obj, object_map)?;
    let new_id = dest_doc.add_object(new_obj);
    object_map.insert(object_id, new_id);

    Ok(new_id)
}

fn clone_object_value(
    src_doc: &Document,
    dest_doc: &mut Document,
    obj: &Object,
    object_map: &mut BTreeMap<ObjectId, ObjectId>,
) -> Result<Object, String> {
    match obj {
        Object::Reference(ref_id) => {
            let new_id = clone_object_recursive(src_doc, dest_doc, *ref_id, object_map)?;
            Ok(Object::Reference(new_id))
        }
        Object::Array(arr) => {
            let mut new_arr = Vec::new();
            for item in arr {
                new_arr.push(clone_object_value(src_doc, dest_doc, item, object_map)?);
            }
            Ok(Object::Array(new_arr))
        }
        Object::Dictionary(dict) => {
            let mut new_dict = lopdf::Dictionary::new();
            for (key, value) in dict.iter() {
                // Skip Parent reference to avoid circular references
                if key == b"Parent" {
                    continue;
                }
                let new_value = clone_object_value(src_doc, dest_doc, value, object_map)?;
                new_dict.set(key.clone(), new_value);
            }
            Ok(Object::Dictionary(new_dict))
        }
        Object::Stream(stream) => {
            let mut new_dict = lopdf::Dictionary::new();
            for (key, value) in stream.dict.iter() {
                if key == b"Parent" {
                    continue;
                }
                let new_value = clone_object_value(src_doc, dest_doc, value, object_map)?;
                new_dict.set(key.clone(), new_value);
            }
            Ok(Object::Stream(lopdf::Stream::new(
                new_dict,
                stream.content.clone(),
            )))
        }
        _ => Ok(obj.clone()),
    }
}
