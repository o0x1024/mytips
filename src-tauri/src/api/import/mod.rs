use crate::db::{DbManager, Tip, TipType};
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

// 导入选项
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportOptions {
    pub conflict_resolution: ConflictResolution, // 冲突解决策略
    pub include_subdirs: bool,                   // 是否包含子目录
    pub process_images: bool,                    // 是否处理图片
}

// 冲突解决策略
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConflictResolution {
    Skip,   // 跳过
    Rename, // 重命名
    Merge,  // 合并（仅用于笔记本）
}

// 导入结果
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportResult {
    pub success: bool,
    pub notebooks_created: u32,
    pub notes_imported: u32,
    pub images_processed: u32,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

// 预览结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportPreview {
    pub notebooks: Vec<NotebookPreview>,
    pub total_notes: u32,
    pub total_images: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotebookPreview {
    pub name: String,
    pub path: String,
    pub notes: Vec<NotePreview>,
    pub children: Vec<NotebookPreview>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotePreview {
    pub title: String,
    pub path: String,
    pub images: Vec<String>,
}

impl Default for ImportResult {
    fn default() -> Self {
        Self {
            success: false,
            notebooks_created: 0,
            notes_imported: 0,
            images_processed: 0,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

// 从目录导入（修改为同步API）
#[tauri::command]
pub fn import_from_directory(
    _app: tauri::AppHandle,
    directory_path: String,
    target_notebook_id: Option<String>,
    options: ImportOptions,
    db_state: tauri::State<'_, std::sync::Mutex<DbManager>>,
) -> Result<ImportResult, String> {
    let db = db_state
        .lock()
        .map_err(|e| format!("数据库锁定失败: {}", e))?;
    let mut result = ImportResult::default();

    let path = Path::new(&directory_path);
    if !path.exists() {
        return Err("指定的目录不存在".to_string());
    }

    match process_directory_sync(&*db, &path, target_notebook_id, &options, &mut result) {
        Ok(_) => {
            result.success = true;
            Ok(result)
        }
        Err(e) => {
            result.errors.push(e.to_string());
            Ok(result)
        }
    }
}

// 从单个文件导入（修改为同步API）
#[tauri::command]
pub fn import_markdown_file(
    _app: tauri::AppHandle,
    file_path: String,
    target_notebook_id: String,
    db_state: tauri::State<'_, std::sync::Mutex<DbManager>>,
) -> Result<ImportResult, String> {
    let db = db_state
        .lock()
        .map_err(|e| format!("数据库锁定失败: {}", e))?;
    let mut result = ImportResult::default();

    let path = Path::new(&file_path);
    if !path.exists() {
        return Err("指定的文件不存在".to_string());
    }

    match import_single_file_sync(&*db, &path, &target_notebook_id, &mut result) {
        Ok(_) => {
            result.success = true;
            Ok(result)
        }
        Err(e) => {
            result.errors.push(e.to_string());
            Ok(result)
        }
    }
}

// 获取导入预览（修改为同步API）
#[tauri::command]
pub fn get_import_preview(directory_path: String) -> Result<ImportPreview, String> {
    let path = Path::new(&directory_path);
    if !path.exists() {
        return Err("指定的目录不存在".to_string());
    }

    let mut preview = ImportPreview {
        notebooks: Vec::new(),
        total_notes: 0,
        total_images: 0,
    };

    scan_directory_preview_sync(&path, &mut preview).map_err(|e| e.to_string())?;

    Ok(preview)
}

// 同步处理目录
fn process_directory_sync(
    db: &DbManager,
    dir_path: &Path,
    parent_notebook_id: Option<String>,
    options: &ImportOptions,
    result: &mut ImportResult,
) -> Result<String> {
    // 创建或获取笔记本
    let dir_name = dir_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("未命名文件夹")
        .to_string();

    let notebook_id = if let Some(parent_id) = parent_notebook_id {
        // 为子目录创建新的子笔记本，parent_id作为父笔记本ID
        let category = db
            .create_category(&dir_name, Some(&parent_id))
            .map_err(|e| anyhow!("创建子笔记本失败: {}", e))?;
        result.notebooks_created += 1;
        category.id
    } else {
        // 创建顶级笔记本
        let category = db
            .create_category(&dir_name, None)
            .map_err(|e| anyhow!("创建笔记本失败: {}", e))?;
        result.notebooks_created += 1;
        category.id
    };

    // 处理当前目录下的Markdown文件
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() && is_markdown_file(&path) {
                if let Err(e) = import_single_file_sync(db, &path, &notebook_id, result) {
                    result
                        .errors
                        .push(format!("导入文件 {} 失败: {}", path.display(), e));
                }
            } else if path.is_dir() && options.include_subdirs {
                // 递归处理子目录，将当前笔记本ID作为父ID传递
                if let Err(e) =
                    process_directory_sync(db, &path, Some(notebook_id.clone()), options, result)
                {
                    result
                        .errors
                        .push(format!("处理目录 {} 失败: {}", path.display(), e));
                }
            }
        }
    }

    Ok(notebook_id)
}

// 导入单个文件（同步版本）
fn import_single_file_sync(
    db: &DbManager,
    file_path: &Path,
    notebook_id: &str,
    result: &mut ImportResult,
) -> Result<()> {
    // 读取文件内容
    let content = fs::read_to_string(file_path).map_err(|e| anyhow!("读取文件失败: {}", e))?;

    // 获取文件名作为标题
    let title = file_path
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("未命名")
        .to_string();

    // 创建笔记
    let tip_id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp_millis();

    // 处理图片引用并获取图片数据
    let (processed_content, image_data) = process_markdown_images(&content, file_path)?;

    let tip = Tip {
        id: tip_id.clone(),
        title: title.clone(),
        content: processed_content,
        tip_type: TipType::Markdown,
        language: None,
        category_id: Some(notebook_id.to_string()),
        created_at: now,
        updated_at: now,
    };

    // 保存笔记
    db.save_tip(tip)?;

    // 保存图片
    for (image_id, base64_data) in image_data {
        if let Err(e) = db.save_image(&tip_id, &image_id, &base64_data) {
            result
                .warnings
                .push(format!("保存图片 {} 失败: {}", image_id, e));
        } else {
            result.images_processed += 1;
        }
    }

    result.notes_imported += 1;
    Ok(())
}

// 处理Markdown中的图片引用
fn process_markdown_images(
    content: &str,
    file_path: &Path,
) -> Result<(String, Vec<(String, String)>)> {
    let mut processed_content = content.to_string();
    let mut image_data = Vec::new();

    // 查找图片引用：![alt](path) 格式
    let re = regex::Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").unwrap();
    let mut offset = 0i32;

    for mat in re.find_iter(content) {
        let match_str = mat.as_str();
        let caps = re.captures(match_str).unwrap();
        let alt_text = &caps[1];
        let image_path = &caps[2];

        // 跳过网络图片
        if image_path.starts_with("http://") || image_path.starts_with("https://") {
            continue;
        }

        // 解析相对路径
        let image_file_path = if image_path.starts_with('/') {
            PathBuf::from(image_path)
        } else {
            file_path
                .parent()
                .unwrap_or(Path::new("."))
                .join(image_path)
        };

        // 读取图片文件
        if let Ok(image_bytes) = fs::read(&image_file_path) {
            let image_id = Uuid::new_v4().to_string();

            // 检测图片格式
            let image_format = detect_image_format(&image_file_path, &image_bytes)?;

            // 生成完整的Data URL格式，与前端粘贴功能保持一致
            let base64_data = general_purpose::STANDARD.encode(&image_bytes);
            let data_url = format!("data:image/{};base64,{}", image_format, base64_data);

            // 替换Markdown中的图片引用，使用local://前缀以匹配前端处理逻辑
            let new_image_ref = format!("![{}](local://{})", alt_text, image_id);

            let start = mat.start() as i32 + offset;
            let end = mat.end() as i32 + offset;

            // 计算绝对位置
            let abs_start = start as usize;
            let abs_end = end as usize;

            if abs_end <= processed_content.len() {
                processed_content.replace_range(abs_start..abs_end, &new_image_ref);
                offset += new_image_ref.len() as i32 - (abs_end - abs_start) as i32;
            }

            image_data.push((image_id, data_url));
        }
    }

    Ok((processed_content, image_data))
}

// 检查是否为Markdown文件
fn is_markdown_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        match ext.to_str() {
            Some("md") | Some("markdown") => true,
            _ => false,
        }
    } else {
        false
    }
}

// 扫描目录预览（同步版本）
fn scan_directory_preview_sync(dir_path: &Path, preview: &mut ImportPreview) -> Result<()> {
    let dir_name = dir_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("未命名文件夹")
        .to_string();

    let mut notebook_preview = NotebookPreview {
        name: dir_name,
        path: dir_path.to_string_lossy().to_string(),
        notes: Vec::new(),
        children: Vec::new(),
    };

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() && is_markdown_file(&path) {
                let title = path
                    .file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or("未命名")
                    .to_string();

                // 扫描图片引用
                let mut images = Vec::new();
                if let Ok(content) = fs::read_to_string(&path) {
                    let re = regex::Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").unwrap();
                    for caps in re.captures_iter(&content) {
                        let image_path = &caps[2];
                        if !image_path.starts_with("http://") && !image_path.starts_with("https://")
                        {
                            images.push(image_path.to_string());
                        }
                    }
                }

                let note_preview = NotePreview {
                    title,
                    path: path.to_string_lossy().to_string(),
                    images: images.clone(),
                };

                notebook_preview.notes.push(note_preview);
                preview.total_notes += 1;
                preview.total_images += images.len() as u32;
            } else if path.is_dir() {
                // 为子目录创建单独的预览
                let mut child_preview = ImportPreview {
                    notebooks: Vec::new(),
                    total_notes: 0,
                    total_images: 0,
                };

                if let Ok(_) = scan_directory_preview_sync(&path, &mut child_preview) {
                    // 将子目录的笔记本添加到当前笔记本的children中
                    notebook_preview.children.extend(child_preview.notebooks);
                    // 累加笔记和图片总数
                    preview.total_notes += child_preview.total_notes;
                    preview.total_images += child_preview.total_images;
                }
            }
        }
    }

    preview.notebooks.push(notebook_preview);
    Ok(())
}

// 检测图片格式的辅助函数
fn detect_image_format(file_path: &Path, file_data: &[u8]) -> Result<&'static str> {
    // 首先通过文件头（魔数）检测真实格式
    if file_data.len() >= 8 {
        // PNG: 89 50 4E 47 0D 0A 1A 0A
        if file_data.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
            return Ok("png");
        }
        // JPEG: FF D8 FF
        if file_data.starts_with(&[0xFF, 0xD8, 0xFF]) {
            return Ok("jpeg");
        }
        // GIF: GIF87a 或 GIF89a
        if file_data.starts_with(b"GIF87a") || file_data.starts_with(b"GIF89a") {
            return Ok("gif");
        }
        // WEBP: RIFF....WEBP
        if file_data.len() >= 12 && file_data.starts_with(b"RIFF") && &file_data[8..12] == b"WEBP" {
            return Ok("webp");
        }
        // BMP: BM
        if file_data.starts_with(b"BM") {
            return Ok("bmp");
        }
        // TIFF: II*\0 或 MM\0*
        if file_data.starts_with(&[0x49, 0x49, 0x2A, 0x00])
            || file_data.starts_with(&[0x4D, 0x4D, 0x00, 0x2A])
        {
            return Ok("tiff");
        }
    }

    // 如果魔数检测失败，回退到文件扩展名检测
    if let Some(ext) = file_path.extension() {
        if let Some(ext_str) = ext.to_str() {
            let ext_lower = ext_str.to_lowercase();
            match ext_lower.as_str() {
                "png" | "apng" => Ok("png"),
                "jpg" | "jpeg" => Ok("jpeg"),
                "gif" => Ok("gif"),
                "webp" => Ok("webp"),
                "bmp" | "dib" => Ok("bmp"),
                "tiff" | "tif" => Ok("tiff"),
                "ico" => Ok("x-icon"),
                "svg" => Ok("svg+xml"),
                _ => Err(anyhow!("不支持的图片格式: {}", ext_str)),
            }
        } else {
            Err(anyhow!("无法解析文件扩展名"))
        }
    } else {
        Err(anyhow!("文件没有扩展名，无法确定图片格式"))
    }
}
