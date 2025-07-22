use crate::db::{models::{Tip, TipType, Category}, operations, manager::UnifiedDbManager};
use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{Emitter, Manager};
use uuid::Uuid;
use image::{DynamicImage, GenericImageView, ImageFormat};
use std::io::Cursor;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::stream::{self, StreamExt, TryStreamExt};
use std::sync::atomic::{AtomicBool, Ordering};
use once_cell::sync::Lazy;
use git2::Repository;
use tempfile::tempdir;
use crate::api::settings;

// 导入取消标志
static IMPORT_CANCELLATION_TOKEN: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));

// 新增：取消导入命令
#[tauri::command]
pub async fn cancel_import() -> Result<(), String> {
    IMPORT_CANCELLATION_TOKEN.store(true, Ordering::SeqCst);
    Ok(())
}

// 新增：导入进度状态
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ImportStatus {
    Starting,
    Cloning,
    Scanning,
    InProgress,
    Completed,
    Error,
}

// 新增：导入进度事件的有效载荷
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImportProgress {
    status: ImportStatus,
    total_files: usize,
    processed_files: usize,
    current_file: String,
    result: Option<ImportResult>,
}

// 图片压缩选项
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageCompressionOptions {
    pub enabled: bool,
    pub quality: u8,
    pub max_width: u32,
    pub max_height: u32,
}

// 导入选项
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportOptions {
    pub conflict_resolution: ConflictResolution, // 冲突解决策略
    pub include_subdirs: bool,                   // 是否包含子目录
    pub process_images: bool,                    // 是否处理图片
    pub image_compression: ImageCompressionOptions, // 图片压缩选项
}

// 冲突解决策略
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConflictResolution {
    Skip,   // 跳过
    Rename, // 重命名
    Merge,  // 合并（仅用于笔记本）
}

// 导入结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportResult {
    pub success: bool,
    pub notebooks_created: u32,
    pub notes_imported: u32,
    pub images_processed: u32,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

// GitHub 导入选项
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GithubImportOptions {
    pub repo_url: String,
    pub branch: Option<String>,
    pub subdirectory: Option<String>,
    pub token: Option<String>,
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

// 新增：从GitHub导入
#[tauri::command]
pub async fn import_from_github(
    app: tauri::AppHandle,
    github_options: GithubImportOptions,
    target_notebook_id: Option<String>,
    options: ImportOptions,
) -> Result<(), String> {
    tauri::async_runtime::spawn(async move {
        let emit_progress = |status: ImportStatus, total: usize, processed: usize, file: &str, res: Option<ImportResult>| {
            let _ = app.emit("import-progress", ImportProgress {
                status,
                total_files: total,
                processed_files: processed,
                current_file: file.to_string(),
                result: res,
            });
        };

        // 1. 立刻发送克隆状态
        let clone_msg = format!("Cloning repository: {}", github_options.repo_url);
        emit_progress(ImportStatus::Cloning, 0, 0, &clone_msg, None);

        // 2. 获取数据库和网络设置
        let manager = match app.try_state::<UnifiedDbManager>() {
            Some(m) => m.inner().clone(),
            None => {
                let mut result = ImportResult::default();
                result.errors.push("Database manager not initialized.".to_string());
                emit_progress(ImportStatus::Error, 0, 0, "Error", Some(result));
                return;
            }
        };
        let conn = match manager.get_conn().await {
            Ok(c) => c,
            Err(e) => {
                let mut result = ImportResult::default();
                result.errors.push(format!("Failed to get db connection: {}", e));
                emit_progress(ImportStatus::Error, 0, 0, "Error", Some(result));
                return;
            }
        };
        let network_settings = settings::get_network_settings_from_db(&conn).await.unwrap_or_default();

        // 3. 执行克隆
        let temp_dir = match tempdir() {
            Ok(dir) => dir,
            Err(e) => {
                let mut result = ImportResult::default();
                result.errors.push(format!("Failed to create temporary directory: {}", e));
                emit_progress(ImportStatus::Error, 0, 0, "Error", Some(result));
                return;
            }
        };
        let temp_path = temp_dir.path().to_path_buf();
        let repo_url = github_options.repo_url.clone();
        let token = github_options.token.clone();

        let clone_result = tokio::task::spawn_blocking(move || {
            let mut fetch_opts = git2::FetchOptions::new();
            if network_settings.proxy.enabled {
                let proxy_url = format!(
                    "{}://{}:{}",
                    network_settings.proxy.protocol,
                    network_settings.proxy.host,
                    network_settings.proxy.port
                );
                let mut proxy_opts = git2::ProxyOptions::new();
                proxy_opts.url(&proxy_url);
                fetch_opts.proxy_options(proxy_opts);
            }
            if let Some(token) = token {
                let mut callbacks = git2::RemoteCallbacks::new();
                callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
                    git2::Cred::userpass_plaintext(&token, "")
                });
                fetch_opts.remote_callbacks(callbacks);
            }
            let mut repo_builder = git2::build::RepoBuilder::new();
            repo_builder.fetch_options(fetch_opts);
            repo_builder.clone(&repo_url, &temp_path)
                .map(|_| temp_path)
                .map_err(|e| format!("Failed to clone GitHub repository: {}", e))
        }).await.map_err(|e| format!("Clone task panicked: {}", e));

        let repo_path = match clone_result {
            Ok(Ok(path)) => path,
            Ok(Err(e)) | Err(e) => {
                let mut result = ImportResult::default();
                result.errors.push(e);
                emit_progress(ImportStatus::Error, 0, 0, "Error", Some(result));
                let _ = temp_dir.close();
                return;
            }
        };

        // 4. 确定导入路径并执行导入
        let import_path = if let Some(subdir) = &github_options.subdirectory {
            repo_path.join(subdir)
        } else {
            repo_path
        };

        if !import_path.exists() {
            let mut result = ImportResult::default();
            result.errors.push(format!("Subdirectory '{}' does not exist in the repository.", github_options.subdirectory.unwrap_or_default()));
            emit_progress(ImportStatus::Error, 0, 0, "Error", Some(result));
            let _ = temp_dir.close();
            return;
        }

        let import_result = do_import_from_directory(
            app.clone(),
            manager,
            import_path.to_str().unwrap().to_string(),
            target_notebook_id,
            options,
        ).await;

        // 5. 清理临时目录
        if let Err(e) = temp_dir.close() {
            if let Ok(mut res) = import_result {
                res.warnings.push(format!("Failed to clean up temporary directory: {}", e));
                // 重新发送最终结果，包含新的警告信息
                emit_progress(ImportStatus::Completed, res.notes_imported as usize, res.notes_imported as usize, "Completed", Some(res));
            }
        }
    });
    
    Ok(())
}


// 从目录导入（修改为异步API）
#[tauri::command]
pub async fn import_from_directory(
    app: tauri::AppHandle,
    directory_path: String,
    target_notebook_id: Option<String>,
    options: ImportOptions,
) -> Result<(), String> {
    let manager = app.state::<UnifiedDbManager>().inner().clone();
    
    tauri::async_runtime::spawn(async move {
        let _ = do_import_from_directory(
            app,
            manager,
            directory_path,
            target_notebook_id,
            options
        ).await;
    });

    Ok(())
}

// 提取核心导入逻辑到一个新的辅助函数中
async fn do_import_from_directory(
    app: tauri::AppHandle,
    manager: UnifiedDbManager,
    directory_path: String,
    target_notebook_id: Option<String>,
    options: ImportOptions,
) -> Result<ImportResult, String> {
    
    // 每次导入开始时重置取消标志
    IMPORT_CANCELLATION_TOKEN.store(false, Ordering::SeqCst);

    let root_path = PathBuf::from(directory_path);
    
    let emit_progress = |status: ImportStatus, total: usize, processed: usize, file: &str, res: Option<ImportResult>| {
        let _ = app.emit("import-progress", ImportProgress {
            status,
            total_files: total,
            processed_files: processed,
            current_file: file.to_string(),
            result: res,
        });
    };

    emit_progress(ImportStatus::Scanning, 0, 0, "Scanning files...", None);

    let files_to_import = match scan_directory_for_files(&root_path, options.include_subdirs) {
        Ok(files) => files,
        Err(e) => {
            let mut result = ImportResult::default();
            result.errors.push(format!("Scanning directory failed: {}", e));
            emit_progress(ImportStatus::Error, 0, 0, "", Some(result.clone()));
            return Err(e.to_string());
        }
    };

    if files_to_import.is_empty() {
        let mut result = ImportResult::default();
        result.warnings.push("No markdown files found to import.".to_string());
        result.success = true;
        emit_progress(ImportStatus::Completed, 0, 0, "", Some(result.clone()));
        return Ok(result);
    }

    let total_files = files_to_import.len();
    
    let result = Arc::new(Mutex::new(ImportResult::default()));
    let notebook_cache = Arc::new(Mutex::new(HashMap::<PathBuf, String>::new()));
    let processed_files_count = Arc::new(Mutex::new(0_usize));
    
    let concurrency = 10; // 设置并发处理的文件数量

    let import_result_future = stream::iter(files_to_import)
        .map(Ok) // for try_for_each_concurrent
        .try_for_each_concurrent(concurrency, |file| {
            let file_path_display = file.path.display().to_string();
            let result = Arc::clone(&result);
            let notebook_cache = Arc::clone(&notebook_cache);
            let manager = manager.clone();
            let root_path = root_path.clone();
            let target_notebook_id = target_notebook_id.clone();
            let options = options.clone();
            let app = app.clone();
            let processed_files_count = Arc::clone(&processed_files_count);

            async move {
                if IMPORT_CANCELLATION_TOKEN.load(Ordering::SeqCst) {
                    return Err(anyhow!("Import cancelled by user"));
                }

                let import_one_file_result = async {
                    let conn = manager.get_conn().await?;
                    let parent_dir = file.relative_path.parent().unwrap_or(Path::new(""));
                    
                    let parent_notebook_id = get_or_create_parent_notebook_concurrent(
                        &conn,
                        &root_path,
                        parent_dir,
                        &target_notebook_id,
                        &result,
                        &notebook_cache,
                    ).await?;

                    import_single_file_to_notebook_concurrent(
                        &conn,
                        &file.path,
                        &parent_notebook_id,
                        &options,
                        &result,
                    ).await
                }.await;

                if let Err(e) = import_one_file_result {
                    let error_msg = format!("Failed to import file {}: {}", file_path_display, e);
                    result.lock().await.errors.push(error_msg.clone());
                    tracing::error!("{}", error_msg);
                }
                
                let mut processed = processed_files_count.lock().await;
                *processed += 1;

                let _ = app.emit("import-progress", ImportProgress {
                    status: ImportStatus::InProgress,
                    total_files: total_files,
                    processed_files: *processed,
                    current_file: file_path_display,
                    result: None,
                });
                
                Ok(())
            }
        })
        .await;

    let mut final_result = Arc::try_unwrap(result).unwrap().into_inner();
    let final_processed_count = *processed_files_count.lock().await;
    
    if let Err(e) = import_result_future {
        if e.to_string() == "Import cancelled by user" {
            final_result.warnings.push("Import was cancelled.".to_string());
        } else {
            final_result.errors.push(e.to_string());
        }
    }

    final_result.success = final_result.errors.is_empty();
    emit_progress(ImportStatus::Completed, total_files, final_processed_count, "Completed", Some(final_result.clone()));

    Ok(final_result)
}

// 从单个文件导入（修改为同步API）
#[tauri::command]
pub fn import_markdown_file(
    app: tauri::AppHandle,
    file_path: String,
    target_notebook_id: String,
) -> Result<ImportResult, String> {
    let path = Path::new(&file_path);
    if !path.exists() {
        return Err("指定的文件不存在".to_string());
    }

    let manager = app.state::<UnifiedDbManager>().inner().clone();
    let path_buf = path.to_path_buf();
    let options = ImportOptions {
        conflict_resolution: ConflictResolution::Rename, // 单文件导入的默认策略
        include_subdirs: false,
        process_images: true, // 单文件导入总是处理图片
        image_compression: ImageCompressionOptions { // 使用默认压缩设置
            enabled: true,
            quality: 80,
            max_width: 1920,
            max_height: 1920,
        },
    };

    let import_handle = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async {
            let mut result = ImportResult::default();
            let conn = match manager.get_conn().await {
                Ok(conn) => conn,
                Err(e) => {
                    result.errors.push(format!("数据库连接失败: {}", e));
                    return result;
                }
            };

            let import_logic = async {
                let content = fs::read_to_string(&path_buf).map_err(|e| anyhow!("读取文件失败: {}", e))?;
                let title = path_buf.file_stem().and_then(|s| s.to_str()).unwrap_or("未命名").to_string();
                let (processed_content, image_data) = process_markdown_images(&content, &path_buf, &options.image_compression)?;

                let tip_id = Uuid::new_v4().to_string();
                let now = Utc::now().timestamp_millis();
                let tip = Tip {
                    id: tip_id.clone(),
                    title,
                    content: processed_content,
                    tip_type: TipType::Markdown,
                    language: None,
                    category_id: Some(target_notebook_id),
                    created_at: now,
                    updated_at: now,
                    version: Some(1),
                    last_synced_at: Some(0),
                    sync_hash: None,
                    is_encrypted: Some(false),
                    encryption_key_id: None,
                    encrypted_content: None,
                };

                // 首先创建 Tip
                operations::create_tip(&conn, &tip).await?;
                result.notes_imported += 1;

                // 然后处理图片
                for (image_id, base64_data) in image_data {
                    if let Err(e) = operations::save_tip_image(&conn, &tip.id, &image_id, &base64_data).await {
                        result.warnings.push(format!("保存图片 {} 失败: {}", image_id, e));
                        tracing::warn!("Failed to save image {} for tip {}: {}", image_id, &tip.id, e);
                    } else {
                        result.images_processed += 1;
                    }
                }
                Ok::<(), anyhow::Error>(())
            };

            match import_logic.await {
                Ok(_) => {
                    result.success = true;
                }
                Err(e) => {
                    result.errors.push(e.to_string());
                }
            }
            result
        })
    });

    match import_handle.join() {
        Ok(import_result) => Ok(import_result),
        Err(_) => {
            let mut result = ImportResult::default();
            result.errors.push("Import task panicked".to_string());
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

// 处理Markdown中的图片引用
fn process_markdown_images(
    content: &str,
    file_path: &Path,
    compression_options: &ImageCompressionOptions,
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

            // 图片处理（压缩和调整大小）
            let (final_image_bytes, image_format_str) = if compression_options.enabled {
                match process_image(&image_bytes, compression_options) {
                    Ok((processed_bytes, format)) => (processed_bytes, format!("{:?}", format).to_lowercase()),
                    Err(e) => {
                        // 如果处理失败，使用原始图片并记录警告
                        (image_bytes, "png".to_string()) // 假设为 png
                    }
                }
            } else {
                (image_bytes, "png".to_string()) // 未启用压缩，假设为 png
            };
            
            // 生成完整的Data URL格式
            let base64_data = general_purpose::STANDARD.encode(&final_image_bytes);
            let data_url = format!("data:image/{};base64,{}", image_format_str, base64_data);
            
            // 替换Markdown中的图片引用
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

// 新增：图片处理函数
fn process_image(
    image_bytes: &[u8],
    options: &ImageCompressionOptions,
) -> Result<(Vec<u8>, ImageFormat)> {
    // 尝试解码图片
    let mut img = image::load_from_memory(image_bytes)?;
    let original_format = image::guess_format(image_bytes)?;

    // 先根据最大宽高进行缩放
    let (width, height) = img.dimensions();
    if width > options.max_width || height > options.max_height {
        img = img.resize(options.max_width, options.max_height, image::imageops::FilterType::Triangle);
    }

    // 重新编码为 JPEG 以获得更好的压缩率（对 PNG 等无损格式尤其有效）
    let mut compressed_bytes: Vec<u8> = Vec::new();
    {
        let mut cursor = Cursor::new(&mut compressed_bytes);
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cursor, options.quality);
        img.write_with_encoder(encoder)?;
    }

    // 与原始大小进行对比，如果压缩后反而更大，则保留原始字节
    let (final_bytes, final_format) = if compressed_bytes.len() < image_bytes.len() {
        (compressed_bytes, ImageFormat::Jpeg)
    } else {
        (image_bytes.to_vec(), original_format)
    };

    Ok((final_bytes, final_format))
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

#[derive(Debug, Clone)]
struct FileToImport {
    path: PathBuf,
    relative_path: PathBuf,
}

// 扫描目录以获取文件列表
fn scan_directory_for_files(
    dir_path: &Path,
    include_subdirs: bool,
) -> Result<Vec<FileToImport>> {
    let mut files_to_import = Vec::new();
    let mut dirs_to_visit = vec![dir_path.to_path_buf()];

    while let Some(current_dir) = dirs_to_visit.pop() {
        for entry in fs::read_dir(&current_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() && include_subdirs {
                if let Ok(metadata) = entry.metadata() {
                    if !metadata.is_symlink() {
                        dirs_to_visit.push(path);
                    }
                }
            } else if path.is_file() && is_markdown_file(&path) {
                let relative_path = path.strip_prefix(dir_path)?.to_path_buf();
                files_to_import.push(FileToImport {
                    path,
                    relative_path,
                });
            }
        }
    }
    Ok(files_to_import)
}

// 辅助函数: 逐级创建或获取父笔记本
async fn get_or_create_parent_notebook_concurrent(
    conn: &crate::db::DbConnection,
    root_path: &Path,
    relative_dir: &Path,
    target_notebook_id: &Option<String>,
    result: &Arc<Mutex<ImportResult>>,
    cache: &Arc<Mutex<HashMap<PathBuf, String>>>,
) -> Result<String> {
    // 案例1: 指定了目标笔记本
    if let Some(target_id) = target_notebook_id {
        let mut current_parent_id = target_id.clone();
        // 使用一个虚拟的基础路径来构建缓存键，以避免与文件系统根目录冲突
        let mut cache_path_key = PathBuf::from("__target__");

        for component in relative_dir.components() {
            let component_str = component.as_os_str().to_string_lossy().to_string();
            cache_path_key.push(&component_str);
            
            let notebook_id = create_or_get_notebook_concurrent(
                conn,
                &component_str,
                Some(current_parent_id.clone()),
                result,
                cache,
                &cache_path_key,
            )
            .await?;
            current_parent_id = notebook_id;
        }
        return Ok(current_parent_id);
    }

    // 案例2: 未指定目标笔记本，从根目录开始创建
    let root_dir_name = root_path.file_name().and_then(|s| s.to_str()).unwrap_or("Imported Notes").to_string();
    let mut current_path_key = root_path.to_path_buf();
    let mut current_parent_id = create_or_get_notebook_concurrent(conn, &root_dir_name, None, result, cache, &current_path_key).await?;

    for component in relative_dir.components() {
        let component_str = component.as_os_str().to_string_lossy().to_string();
        current_path_key.push(&component_str);

        let notebook_id = create_or_get_notebook_concurrent(
            conn,
            &component_str,
            Some(current_parent_id.clone()),
            result,
            cache,
            &current_path_key,
        )
        .await?;
        current_parent_id = notebook_id;
    }

    Ok(current_parent_id)
}

// 辅助函数: 创建或获取笔记本ID
async fn create_or_get_notebook_concurrent(
    conn: &crate::db::DbConnection,
    name: &str,
    parent_id: Option<String>,
    result: &Arc<Mutex<ImportResult>>,
    cache: &Arc<Mutex<HashMap<PathBuf, String>>>,
    path_key: &PathBuf,
) -> Result<String> {
    // 检查缓存中是否已存在
    {
        let cache_guard = cache.lock().await;
        if let Some(id) = cache_guard.get(path_key) {
            return Ok(id.clone());
        }
    } // 在这里释放锁

    // 再次锁定以进行创建
    let mut cache_guard = cache.lock().await;
    // 双重检查，防止在释放锁和再次获取锁之间有其他线程创建了该笔记本
    if let Some(id) = cache_guard.get(path_key) {
        return Ok(id.clone());
    }
    
    let now = Utc::now().timestamp_millis();
    let category = Category {
        id: Uuid::new_v4().to_string(),
        name: name.to_string(),
        parent_id,
        created_at: now,
        updated_at: now,
        version: Some(1),
        last_synced_at: Some(0),
        sync_hash: None,
        is_encrypted: Some(false),
        encryption_key_id: None,
    };

    let cat_id = category.id.clone();
    operations::create_category(conn, &category).await?;
    
    result.lock().await.notebooks_created += 1;
    cache_guard.insert(path_key.clone(), cat_id.clone());
    Ok(cat_id)
}

// 新辅助函数: 导入单个文件到指定笔记本
async fn import_single_file_to_notebook_concurrent(
    conn: &crate::db::DbConnection,
    file_path: &Path,
    notebook_id: &str,
    options: &ImportOptions,
    result: &Arc<Mutex<ImportResult>>,
) -> Result<()> {
    let content = fs::read_to_string(file_path).map_err(|e| anyhow!("Failed to read file {}: {}", file_path.display(), e))?;
    let title = file_path.file_stem().and_then(|s| s.to_str()).unwrap_or("Untitled").to_string();
    
    let (processed_content, image_data) = if options.process_images {
        process_markdown_images(&content, file_path, &options.image_compression)?
    } else {
        (content, Vec::new())
    };

    let tip_id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp_millis();
    let tip = Tip {
        id: tip_id.clone(),
        title,
        content: processed_content,
        tip_type: TipType::Markdown,
        language: None,
        category_id: Some(notebook_id.to_string()),
        created_at: now,
        updated_at: now,
        version: Some(1),
        last_synced_at: Some(0),
        sync_hash: None,
        is_encrypted: Some(false),
        encryption_key_id: None,
        encrypted_content: None,
    };

    operations::create_tip(conn, &tip).await?;
    result.lock().await.notes_imported += 1;
    
    let mut images_processed_count = 0;
    let mut warnings_list = vec![];
    for (image_id, base64_data) in image_data {
        if operations::save_tip_image(conn, &tip.id, &image_id, &base64_data).await.is_ok() {
            images_processed_count += 1;
        } else {
            let warning_msg = format!("Failed to save image for note '{}' from file {}", tip.title, file_path.display());
            warnings_list.push(warning_msg);
        }
    }

    if images_processed_count > 0 || !warnings_list.is_empty() {
        let mut result_guard = result.lock().await;
        result_guard.images_processed += images_processed_count;
        result_guard.warnings.extend(warnings_list);
    }

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

