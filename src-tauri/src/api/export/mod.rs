use crate::db::{manager::UnifiedDbManager, models::{Category, Tip}, operations};
use tauri::{AppHandle, command, Manager, State};
use tauri_plugin_dialog::{DialogExt, FilePath};
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Result, anyhow};
use printpdf::{PdfDocument, Mm, Point, Line, IndirectFontRef, BuiltinFont, PdfDocumentReference};
use docx_rs::{AlignmentType, BorderType, BreakType, Docx, LineSpacing, PageMargin, Paragraph, Run, RunFonts, RunProperty, SectionProperty, Shading, Style, StyleType, Table, TableBorders, TableCell, TableRow};
use pulldown_cmark::{Parser, Event, Tag, Options};

// 将字体文件嵌入到二进制文件中
const FONT_BYTES: &[u8] = include_bytes!("../../../assets/SourceHanSansSC-Regular.otf");

// 新增: 调整 Markdown 标题层级的工具函数
/// Shift all Markdown heading levels inside `markdown` by `shift` levels (e.g. 1 means `#` -> `##`).
/// Lines inside fenced code blocks are ignored.
fn shift_markdown_headings(markdown: &str, shift: usize) -> String {
    if shift == 0 {
        return markdown.to_string();
    }
    let mut result = String::with_capacity(markdown.len());
    let mut in_code_block = false;
    for (idx, line) in markdown.lines().enumerate() {
        // 保留换行符（lines() 会去掉换行，需要手动补回）
        if idx > 0 {
            result.push('\n');
        }
        let trimmed = line.trim_start();
        // 进入或退出代码块（``` 或 ~~~）
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_code_block = !in_code_block;
            result.push_str(line);
            continue;
        }
        if !in_code_block {
            // 统计开头的 '#'
            let mut num_hash = 0;
            for ch in line.chars() {
                if ch == '#' {
                    num_hash += 1;
                } else {
                    break;
                }
            }
            if num_hash > 0 && num_hash <= 6 {
                if line.chars().nth(num_hash) == Some(' ') {
                    let new_level = (num_hash + shift).min(6);
                    let new_hashes = "#".repeat(new_level);
                    let rest = &line[num_hash..];
                    result.push_str(&format!("{}{}", new_hashes, rest));
                    continue;
                }
            }
        }
        // 默认直接写入
        result.push_str(line);
    }
    result
}

// 导出为Markdown
#[tauri::command]
pub async fn export_as_markdown(
    app: tauri::AppHandle,
    note_ids: Option<Vec<String>>,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<String, String> {
    use std::fs;

    // 获取笔记
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    let tips = if let Some(ids) = note_ids {
        // 如果提供了笔记ID列表，只导出指定的笔记
        let mut result = Vec::new();
        for id in ids {
            if let Ok(Some(tip)) = operations::get_tip_by_id(&conn, &id).await {
                result.push(tip);
            }
        }
        result
    } else {
        // 否则导出所有笔记
        operations::list_tips(&conn).await.map_err(|e| e.to_string())?
    };

    if tips.is_empty() {
        return Err("No notes found to export".to_string());
    }

    // 成功导出的笔记数量
    let mut exported_count = 0;
    let total_count = tips.len();

    // 导出每个笔记
    for tip in tips {
        // 生成默认文件名
        let mut file_name = tip.title.clone();
        // 替换文件名中的非法字符
        file_name = file_name
            .replace("/", "-")
            .replace("\\", "-")
            .replace(":", "-")
            .replace("*", "-")
            .replace("?", "-")
            .replace("\"", "-")
            .replace("<", "-")
            .replace(">", "-")
            .replace("|", "-");

        // 显示保存文件对话框，让用户选择保存位置和文件名
        let file_path = match app
            .dialog()
            .file()
            .add_filter("Markdown file", &["md"])
            .set_file_name(&format!("{}.md", file_name))
            .blocking_save_file()
        {
            Some(path) => match path.as_path() {
                Some(p) => p.to_path_buf(),
                None => continue, // 如果用户取消了对话框，跳过当前笔记
            },
            None => continue, // 如果用户取消了对话框，跳过当前笔记
        };

        // 写入Markdown格式的内容
        let mut content = format!("# {}\n\n", tip.title);

        // 添加分类信息（如果有）
        if let Some(category_id) = &tip.category_id {
            if let Ok(Some(category)) = operations::get_category_by_id(&conn, category_id).await {
                content.push_str(&format!("**Category**: {}\n\n", category.name));
            }
        }

        // 添加标签信息（暂时跳过，因为没有tag关联功能）
        let tags: Vec<String> = Vec::new(); // TODO: 实现tag功能后替换
        if !tags.is_empty() {
            content.push_str("**Tags**: ");
            for (i, tag) in tags.iter().enumerate() {
                if i > 0 {
                    content.push_str(", ");
                }
                content.push_str(tag);
            }
            content.push_str("\n\n");
        }

        // 添加笔记内容，并将原有标题级别下移一级
        let adjusted_content = shift_markdown_headings(&tip.content, 1);
        content.push_str(&adjusted_content);

        // 写入文件
        fs::write(&file_path, content).map_err(|e| format!("Failed to write file: {}", e))?;

        exported_count += 1;
    }

    if exported_count == 0 {
        return Err("No notes were exported".to_string());
    } else if exported_count < total_count {
        Ok(format!(
            "Successfully exported {} out of {} notes",
            exported_count, total_count
        ))
    } else {
        Ok(format!("Successfully exported {} notes", exported_count))
    }
}

// 备份数据库
#[tauri::command]
pub async fn backup_database(app: tauri::AppHandle) -> Result<String, String> {
    use chrono::Local;
    use std::fs;

    // 获取数据库路径
    let db_path = dirs::data_dir()
        .ok_or_else(|| "Could not get app data directory".to_string())?
        .join("mytips/mytips.db");

    if !db_path.exists() {
        return Err("Database file does not exist".to_string());
    }

    // 生成默认备份文件名（带时间戳）
    let date_time = Local::now().format("%Y%m%d_%H%M%S");
    let default_filename = format!("mytips_backup_{}.db", date_time);

    // 打开文件保存对话框
    let file_path = match app
        .dialog()
        .file()
        .add_filter("Database file", &["db"])
        .set_file_name(&default_filename)
        .blocking_save_file()
    {
        Some(path) => match path.as_path() {
            Some(p) => p.to_path_buf(),
            None => return Err("Could not get file path to save".to_string()),
        },
        None => return Err("User cancelled the backup operation".to_string()),
    };

    // 复制数据库文件到选择的位置
    fs::copy(&db_path, &file_path).map_err(|e| format!("Failed to backup database: {}", e))?;

    Ok(format!("Successfully backed up database to {}", file_path.display()))
}

// 恢复数据库
#[tauri::command]
pub async fn restore_database(app: tauri::AppHandle) -> Result<String, String> {
    // 打开文件选择对话框
    let file_path = match app
        .dialog()
        .file()
        .add_filter("Database file", &["db"])
        .blocking_pick_file()
    {
        Some(path) => match path.as_path() {
            Some(p) => p.to_path_buf(),
            None => return Err("Could not get selected file path".to_string()),
        },
        None => return Err("Restore cancelled".to_string()),
    };

    // 确保是数据库文件
    if let Some(ext) = file_path.extension() {
        if ext != "db" {
            return Err("Please select a valid database backup file (.db)".to_string());
        }
    } else {
        return Err("Please select a valid database backup file (.db)".to_string());
    }

    // 获取目标数据库路径
    let db_dir = dirs::data_dir()
        .ok_or_else(|| "Could not get app data directory".to_string())?
        .join("mytips");

    let db_path = db_dir.join("mytips.db");

    // 确保目录存在
    std::fs::create_dir_all(&db_dir).map_err(|e| format!("Failed to create data directory: {}", e))?;

    // 复制备份文件到数据库位置
    std::fs::copy(&file_path, &db_path).map_err(|e| format!("Restore failed: {}", e))?;

    Ok("Database has been restored. Please restart the application to load the latest data.".to_string())
}

// 导出为HTML
#[tauri::command]
pub async fn export_as_html(
    app: tauri::AppHandle,
    note_ids: Option<Vec<String>>,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<String, String> {
    use std::fs;

    // 获取笔记
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    let tips = if let Some(ids) = note_ids {
        // 如果提供了笔记ID列表，只导出指定的笔记
        let mut result = Vec::new();
        for id in ids {
            if let Ok(Some(tip)) = operations::get_tip_by_id(&conn, &id).await {
                result.push(tip);
            }
        }
        result
    } else {
        // 否则导出所有笔记
        operations::list_tips(&conn).await.map_err(|e| e.to_string())?
    };

    if tips.is_empty() {
        return Err("No notes found to export".to_string());
    }

    // 成功导出的笔记数量
    let mut exported_count = 0;
    let total_count = tips.len();

    // 导出每个笔记
    for tip in tips {
        // 生成默认文件名
        let mut file_name = tip.title.clone();
        // 替换文件名中的非法字符
        file_name = file_name
            .replace("/", "-")
            .replace("\\", "-")
            .replace(":", "-")
            .replace("*", "-")
            .replace("?", "-")
            .replace("\"", "-")
            .replace("<", "-")
            .replace(">", "-")
            .replace("|", "-");

        // 显示保存文件对话框，让用户选择保存位置和文件名
        let file_path = match app
            .dialog()
            .file()
            .add_filter("HTML file", &["html"])
            .set_file_name(&format!("{}.html", file_name))
            .blocking_save_file()
        {
            Some(path) => match path.as_path() {
                Some(p) => p.to_path_buf(),
                None => continue, // 如果用户取消了对话框，跳过当前笔记
            },
            None => continue, // 如果用户取消了对话框，跳过当前笔记
        };

        // 获取标签信息（暂时跳过）
        let tags: Vec<String> = Vec::new(); // TODO: 实现tag功能后替换
        let tags_str = tags.join(", ");

        // 获取分类信息
        let category_name = if let Some(category_id) = &tip.category_id {
            if let Ok(Some(category)) = operations::get_category_by_id(&conn, category_id).await {
                category.name
            } else {
                "Uncategorized".to_string()
            }
        } else {
            "Uncategorized".to_string()
        };

        // 将Markdown转换为HTML，并将原有标题级别下移一级
        let shifted_markdown = shift_markdown_headings(&tip.content, 1);
        let parser = pulldown_cmark::Parser::new(&shifted_markdown);
        let mut html_content = String::new();
        pulldown_cmark::html::push_html(&mut html_content, parser);

        // 构建HTML文档
        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
        }}
        h1 {{
            border-bottom: 1px solid #eaecef;
            padding-bottom: 0.3em;
        }}
        .meta {{
            color: #6c757d;
            margin-bottom: 20px;
        }}
        pre {{
            background-color: #f6f8fa;
            padding: 16px;
            border-radius: 6px;
            overflow: auto;
        }}
        code {{
            font-family: SFMono-Regular, Consolas, "Liberation Mono", Menlo, monospace;
            background-color: rgba(27, 31, 35, 0.05);
            padding: 0.2em 0.4em;
            border-radius: 3px;
        }}
        pre code {{
            background-color: transparent;
            padding: 0;
        }}
        blockquote {{
            border-left: 4px solid #dfe2e5;
            padding-left: 16px;
            color: #6a737d;
            margin-left: 0;
        }}
        img {{
            max-width: 100%;
        }}
        table {{
            border-collapse: collapse;
            width: 100%;
            margin-bottom: 16px;
        }}
        table, th, td {{
            border: 1px solid #dfe2e5;
        }}
        th, td {{
            padding: 8px 16px;
        }}
        th {{
            background-color: #f6f8fa;
        }}
    </style>
</head>
<body>
    <h1>{title}</h1>
    <div class="meta">
        <p><strong>Category</strong>: {category}<br>
        <strong>Tags</strong>: {tags}</p>
    </div>
    <div class="content">
        {content}
    </div>
</body>
</html>"#,
            title = tip.title,
            category = category_name,
            tags = tags_str,
            content = html_content
        );

        // 写入文件
        fs::write(&file_path, html).map_err(|e| format!("Failed to write file: {}", e))?;

        exported_count += 1;
    }

    if exported_count == 0 {
        return Err("No notes were exported".to_string());
    } else if exported_count < total_count {
        Ok(format!(
            "Successfully exported {} out of {} notes",
            exported_count, total_count
        ))
    } else {
        Ok(format!("Successfully exported {} notes", exported_count))
    }
}

// 导出为PDF
#[tauri::command]
pub async fn export_as_pdf(
    app: tauri::AppHandle,
    note_ids: Option<Vec<String>>,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<String, String> {
    use std::fs;
    use std::io::Write;

    // 获取笔记
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    let tips = if let Some(ids) = note_ids {
        // 如果提供了笔记ID列表，只导出指定的笔记
        let mut result = Vec::new();
        for id in ids {
            if let Ok(Some(tip)) = operations::get_tip_by_id(&conn, &id).await {
                result.push(tip);
            }
        }
        result
    } else {
        // 否则导出所有笔记
        operations::list_tips(&conn).await.map_err(|e| e.to_string())?
    };

    if tips.is_empty() {
        return Err("No notes found to export".to_string());
    }

    // 成功导出的笔记数量
    let mut exported_count = 0;
    let total_count = tips.len();

    for tip in tips {
        // 生成默认文件名
        let mut file_name = tip.title.clone();
        // 替换文件名中的非法字符
        file_name = file_name
            .replace("/", "-")
            .replace("\\", "-")
            .replace(":", "-")
            .replace("*", "-")
            .replace("?", "-")
            .replace("\"", "-")
            .replace("<", "-")
            .replace(">", "-")
            .replace("|", "-");

        // 显示保存文件对话框，让用户选择保存位置和文件名
        let file_path = match app
            .dialog()
            .file()
            .add_filter("PDF file", &["pdf"])
            .set_file_name(&format!("{}.pdf", file_name))
            .blocking_save_file()
        {
            Some(path) => match path.as_path() {
                Some(p) => p.to_path_buf(),
                None => continue, // 如果用户取消了对话框，跳过当前笔记
            },
            None => continue, // 如果用户取消了对话框，跳过当前笔记
        };

        // 获取标签信息（暂时跳过）
        let tags: Vec<String> = Vec::new(); // TODO: 实现tag功能后替换
        let tags_str = tags.join(", ");

        // 获取分类信息
        let category_name = if let Some(category_id) = &tip.category_id {
            if let Ok(Some(category)) = operations::get_category_by_id(&conn, category_id).await {
                category.name
            } else {
                "Uncategorized".to_string()
            }
        } else {
            "Uncategorized".to_string()
        };

        // 将Markdown转换为HTML，并将原有标题级别下移一级
        let shifted_markdown = shift_markdown_headings(&tip.content, 1);
        let parser = pulldown_cmark::Parser::new(&shifted_markdown);
        let mut html_content = String::new();
        pulldown_cmark::html::push_html(&mut html_content, parser);

        // 构建完整的HTML文档，支持中文字体
        let html_document = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <meta name="keywords" content="{title}">
    <style>
        @page {{
            size: A4;
            margin: 2cm;
        }}
        body {{
            font-family: "PingFang SC", "Microsoft YaHei", "SimHei", "Arial Unicode MS", Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            font-size: 12pt;
            word-wrap: break-word;
        }}
        h1 {{
            font-size: 20pt;
            font-weight: bold;
            margin: 0 0 10pt 0;
            border-bottom: 2px solid #333;
            padding-bottom: 5pt;
        }}
        .meta {{
            color: #666;
            font-size: 10pt;
            margin-bottom: 20pt;
            padding: 10pt;
            background-color: #f5f5f5;
            border-left: 4px solid #007acc;
        }}
        .content {{
            margin-top: 15pt;
        }}
        h2 {{ font-size: 16pt; margin: 15pt 0 8pt 0; }}
        h3 {{ font-size: 14pt; margin: 12pt 0 6pt 0; }}
        h4 {{ font-size: 12pt; margin: 10pt 0 5pt 0; }}
        h5, h6 {{ font-size: 11pt; margin: 8pt 0 4pt 0; }}
        p {{ margin: 8pt 0; }}
        pre {{
            background-color: #f8f8f8;
            padding: 10pt;
            border: 1px solid #ddd;
            border-radius: 4pt;
            overflow-wrap: break-word;
            white-space: pre-wrap;
            font-size: 10pt;
        }}
        code {{
            font-family: "Courier New", Consolas, monospace;
            background-color: #f0f0f0;
            padding: 2pt 4pt;
            border-radius: 2pt;
            font-size: 10pt;
        }}
        pre code {{
            background-color: transparent;
            padding: 0;
        }}
        blockquote {{
            border-left: 4px solid #ddd;
            margin: 10pt 0;
            padding-left: 15pt;
            color: #666;
            font-style: italic;
        }}
        ul, ol {{
            margin: 8pt 0;
            padding-left: 20pt;
        }}
        li {{ margin: 4pt 0; }}
        table {{
            border-collapse: collapse;
            width: 100%;
            margin: 10pt 0;
            font-size: 10pt;
        }}
        table, th, td {{
            border: 1px solid #ddd;
        }}
        th, td {{
            padding: 6pt 8pt;
            text-align: left;
        }}
        th {{
            background-color: #f5f5f5;
            font-weight: bold;
        }}
        img {{
            max-width: 100%;
            height: auto;
            display: block;
            margin: 10pt auto;
        }}
        a {{
            color: #007acc;
            text-decoration: none;
        }}
        a:hover {{
            text-decoration: underline;
        }}
        .page-break {{
            page-break-before: always;
        }}
    </style>
</head>
<body>
    <h1>{title}</h1>
    <div class="meta">
        <p><strong>Category:</strong>{category}</p>
        <p><strong>Tags:</strong>{tags}</p>
        <p><strong>Export Time:</strong>{export_time}</p>
    </div>
    <div class="content">
        {content}
    </div>
</body>
</html>"#,
            title = tip.title,
            category = category_name,
            tags = tags_str,
            export_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            content = html_content
        );

        // 创建临时HTML文件
        let temp_dir = std::env::temp_dir();
        let temp_html = temp_dir.join(format!("mytips_temp_{}.html", uuid::Uuid::new_v4()));
        
        // 写入HTML内容
        let mut html_file = fs::File::create(&temp_html)
            .map_err(|e| format!("Failed to create temporary HTML file: {}", e))?;
        html_file.write_all(html_document.as_bytes())
            .map_err(|e| format!("Failed to write HTML content: {}", e))?;

        // 尝试使用不同的PDF生成方法
        let pdf_generation_success = 
            // 方法1: 尝试使用系统的wkhtmltopdf
            try_wkhtmltopdf(&temp_html, &file_path) ||
            // 方法2: 尝试使用Chrome/Chromium的headless模式
            try_chrome_pdf(&temp_html, &file_path) ||
            // 方法3: 降级到简单的printpdf方案（纯文本）
            try_simple_pdf_fallback(&tip, &file_path, &category_name, &tags_str);

        // 清理临时文件
        let _ = fs::remove_file(&temp_html);

        if pdf_generation_success {
            exported_count += 1;
        } else {
            return Err(format!("PDF generation failed. Please make sure that either Chrome browser or wkhtmltopdf tool is installed.\n\nAlternatives:\n1. Install Chrome browser\n2. Install wkhtmltopdf: brew install wkhtmltopdf (macOS)\n3. Or choose to export as HTML format and then print as PDF using a browser."));
        }
    }

    if exported_count == 0 {
        return Err("No notes were exported".to_string());
    } else if exported_count < total_count {
        Ok(format!(
            "Successfully exported {} out of {} notes",
            exported_count, total_count
        ))
    } else {
        Ok(format!("Successfully exported {} notes", exported_count))
    }
}

// 尝试使用wkhtmltopdf生成PDF
fn try_wkhtmltopdf(html_path: &std::path::Path, pdf_path: &std::path::Path) -> bool {
    let output = std::process::Command::new("wkhtmltopdf")
        .arg("--page-size").arg("A4")
        .arg("--margin-top").arg("2cm")
        .arg("--margin-bottom").arg("2cm")
        .arg("--margin-left").arg("2cm")
        .arg("--margin-right").arg("2cm")
        .arg("--encoding").arg("UTF-8")
        .arg("--disable-smart-shrinking")
        .arg("--print-media-type")
        .arg(html_path)
        .arg(pdf_path)
        .output();

    match output {
        Ok(result) => result.status.success(),
        Err(_) => false,
    }
}

// 尝试使用Chrome headless模式生成PDF
fn try_chrome_pdf(html_path: &std::path::Path, pdf_path: &std::path::Path) -> bool {
    // 转换路径为file:// URL
    let file_url = format!("file://{}", html_path.to_string_lossy());
    
    // 尝试不同的Chrome可执行文件路径
    let chrome_paths = [
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome", // macOS
        "/Applications/Chromium.app/Contents/MacOS/Chromium", // macOS Chromium
        "google-chrome", // Linux
        "chromium-browser", // Linux
        "chrome", // 通用
        "chromium", // 通用
    ];

    for chrome_path in &chrome_paths {
        let output = std::process::Command::new(chrome_path)
            .arg("--headless")
            .arg("--disable-gpu")
            .arg("--print-to-pdf-no-header")
            .arg("--no-margins")
            .arg("--disable-web-security")
            .arg("--disable-features=VizDisplayCompositor")
            .arg(format!("--print-to-pdf={}", pdf_path.to_string_lossy()))
            .arg(&file_url)
            .output();

        if let Ok(result) = output {
            if result.status.success() && pdf_path.exists() {
                return true;
            }
        }
    }
    
    false
}

// 简单的PDF生成降级方案（支持中文但格式简单）
fn try_simple_pdf_fallback(
    tip: &crate::db::Tip,
    pdf_path: &std::path::Path,
    category_name: &str,
    tags_str: &str,
) -> bool {
    use printpdf::*;
    use std::fs;
    use std::io::BufWriter;

    // 创建新的PDF文档
    let (doc, page1, layer1) = PdfDocument::new("mytips Export", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // 使用内置字体（仅支持ASCII）
    let font = match doc.add_builtin_font(BuiltinFont::Helvetica) {
        Ok(font) => font,
        Err(_) => return false,
    };

    // 提示用户字体限制
    current_layer.begin_text_section();
    current_layer.set_font(&font, 12.0);
    current_layer.set_text_cursor(Mm(10.0), Mm(280.0));
    current_layer.write_text("Note: PDF fallback mode - Chinese characters may not be displayed correctly.", &font);
    current_layer.end_text_section();

    // 转换中文到拼音或保留原文（简化处理）
    let safe_title = convert_to_safe_text(&tip.title);
    let safe_category = convert_to_safe_text(category_name);
    let safe_tags = convert_to_safe_text(tags_str);
    let safe_content = convert_to_safe_text(&tip.content);

    // 添加标题
    current_layer.begin_text_section();
    current_layer.set_font(&font, 18.0);
    current_layer.set_text_cursor(Mm(10.0), Mm(260.0));
    current_layer.write_text(format!("Title: {}", safe_title), &font);
    current_layer.end_text_section();

    // 添加分类
    current_layer.begin_text_section();
    current_layer.set_font(&font, 12.0);
    current_layer.set_text_cursor(Mm(10.0), Mm(250.0));
    current_layer.write_text(format!("Category: {}", safe_category), &font);
    current_layer.end_text_section();

    // 添加标签
    current_layer.begin_text_section();
    current_layer.set_font(&font, 12.0);
    current_layer.set_text_cursor(Mm(10.0), Mm(240.0));
    current_layer.write_text(format!("Tags: {}", safe_tags), &font);
    current_layer.end_text_section();

    // 添加内容
    current_layer.begin_text_section();
    current_layer.set_font(&font, 10.0);
    let mut y_pos = 220.0;
    let chars_per_line = 80;

    for chunk in safe_content.as_bytes().chunks(chars_per_line) {
        if y_pos < 20.0 {
            break;
        }
        
        if let Ok(text) = std::str::from_utf8(chunk) {
            current_layer.set_text_cursor(Mm(10.0), Mm(y_pos));
            current_layer.write_text(text, &font);
            y_pos -= 5.0;
        }
    }

    current_layer.end_text_section();

    // 保存PDF
    match fs::File::create(pdf_path) {
        Ok(file) => {
            let mut buf_writer = BufWriter::new(file);
            doc.save(&mut buf_writer).is_ok()
        }
        Err(_) => false,
    }
}

// 将文本转换为安全的ASCII文本（简化处理）
fn convert_to_safe_text(text: &str) -> String {
    // 保留ASCII字符，移除控制字符，对中文字符进行标记
    let mut result = String::new();
    let mut has_chinese = false;
    
    for char in text.chars() {
        if char.is_ascii() && !char.is_control() {
            result.push(char);
        } else if char as u32 >= 0x4E00 && char as u32 <= 0x9FFF {
            // 检测到中文字符
            has_chinese = true;
            result.push('?'); // 用?代替中文字符
        } else {
            result.push(' '); // 其他字符用空格代替
        }
    }
    
    if has_chinese {
        format!("[Contains Chinese Text] {}", result.trim())
    } else {
        result
    }
}

#[tauri::command]
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub async fn export_notebook_to_folder(app: AppHandle, notebook_id: String) -> Result<(), String> {
    // 1. 弹出对话框
    let dialog = app.dialog();
    let destination = match dialog.file().blocking_pick_folder() {
        Some(path) => path,
        None => return Ok(()),
    };

    // 2. 数据库和笔记本信息
    let manager = app.state::<UnifiedDbManager>();
    let conn = manager.get_conn().await.map_err(|e| e.to_string())?;
    let notebook = operations::get_category_by_id(&conn, &notebook_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Notebook not found".to_string())?;

    // 3. 创建目录
    let destination_path = destination.as_path().ok_or_else(|| "Invalid path".to_string())?;
    let root_path = destination_path.join(sanitize_filename(&notebook.name));
    fs::create_dir_all(&root_path).map_err(|e| e.to_string())?;

    // 4. 递归导出
    export_category_recursive(&conn, &notebook, &root_path).await.map_err(|e| e.to_string())
}

#[derive(Debug)]
struct ExportCategoryNode {
    name: String,
    tips: Vec<Tip>,
    sub_categories: Vec<ExportCategoryNode>,
}

async fn build_export_tree(
    conn: &crate::db::DbConnection,
    category: &Category,
) -> Result<ExportCategoryNode> {
    let mut sub_nodes = Vec::new();
    let sub_categories = operations::get_subcategories(conn, &category.id).await?;
    for sub_cat in sub_categories {
        let sub_node = Box::pin(build_export_tree(conn, &sub_cat)).await?;
        sub_nodes.push(sub_node);
    }

    let tips = operations::get_tips_by_category(conn, &category.id).await?;

    Ok(ExportCategoryNode {
        name: category.name.clone(),
        tips,
        sub_categories: sub_nodes,
    })
}

fn build_html_for_notebook_recursive(
    node: &ExportCategoryNode,
    level: usize,
    html: &mut String,
) {
    // 笔记本标题
    html.push_str(&format!("<h{}>{}</h{}>", level, &node.name, level));

    // 笔记本下的笔记列表
    if !node.tips.is_empty() {
        html.push_str("<ul>");
        for tip in &node.tips {
            html.push_str("<li>");
            // 笔记标题
            html.push_str(&format!("<strong>{}</strong>", &tip.title));
            
            // 笔记内容，根据嵌套层级调整标题
            let shifted_content = shift_markdown_headings(&tip.content, level);
            let parser = pulldown_cmark::Parser::new(&shifted_content);
            let mut tip_html = String::new();
            pulldown_cmark::html::push_html(&mut tip_html, parser);
            html.push_str(&format!("<div class='tip-content'>{}</div>", tip_html));

            html.push_str("</li>");
        }
        html.push_str("</ul>");
    }

    // 递归处理子笔记本
    for sub_node in &node.sub_categories {
        build_html_for_notebook_recursive(sub_node, level + 1, html);
    }
}

fn collect_tip_titles_recursive(node: &ExportCategoryNode, titles: &mut Vec<String>) {
    for tip in &node.tips {
        titles.push(tip.title.clone());
    }
    for sub_node in &node.sub_categories {
        collect_tip_titles_recursive(sub_node, titles);
    }
}


#[tauri::command]
pub async fn export_notebook_to_pdf(app: AppHandle, notebook_id: String) -> Result<(), String> {
    // 1. 弹出对话框让用户选择保存位置
    let dialog = app.dialog();
    let destination_path = match dialog.file().add_filter("PDF", &["pdf"]).blocking_save_file() {
        Some(path) => match path.as_path() {
            Some(p) => p.to_path_buf(),
            None => return Ok(()),
        },
        None => return Ok(()),
    };

    // 2. 数据库和笔记本信息
    let manager = app.state::<UnifiedDbManager>();
    let conn = manager.get_conn().await.map_err(|e| e.to_string())?;
    let notebook = operations::get_category_by_id(&conn, &notebook_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Notebook not found".to_string())?;

    // 3. 异步构建数据树
    let export_tree = build_export_tree(&conn, &notebook).await.map_err(|e| e.to_string())?;

    // 4. 为关键词收集所有笔记标题
    let mut tip_titles = Vec::new();
    collect_tip_titles_recursive(&export_tree, &mut tip_titles);
    let keywords = tip_titles.join(", ");

    // 5. 生成HTML内容
    let mut notebook_html_content = String::new();
    build_html_for_notebook_recursive(&export_tree, 1, &mut notebook_html_content);
    
    // 6. 构建完整的HTML文档
    let html_document = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>{title}</title>
    <meta name="keywords" content="{keywords}">
    <style>
        @page {{
            size: A4;
            margin: 2cm;
        }}
        body {{
            font-family: "PingFang SC", "Microsoft YaHei", "SimHei", "Arial Unicode MS", Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            font-size: 10pt;
            word-wrap: break-word;
        }}
        h1, h2, h3, h4, h5, h6 {{
            font-weight: bold;
            margin: 1.2em 0 0.8em 0;
            page-break-after: avoid;
        }}
        h1 {{ font-size: 22pt; border-bottom: 2px solid #333; padding-bottom: 5pt; }}
        h2 {{ font-size: 18pt; border-bottom: 1px solid #ccc; padding-bottom: 4pt; }}
        h3 {{ font-size: 14pt; }}
        h4, h5, h6 {{ font-size: 12pt; }}
        p {{ margin: 8pt 0; }}
        ul {{ padding-left: 20pt; }}
        li {{ margin-bottom: 12pt; }}
        .tip-content {{
            border-left: 3px solid #eee;
            padding-left: 15pt;
            margin-top: 5pt;
            color: #444;
        }}
        pre {{
            background-color: #f8f8f8;
            padding: 10pt;
            border: 1px solid #ddd;
            border-radius: 4pt;
            overflow-wrap: break-word;
            white-space: pre-wrap;
            font-size: 9pt;
        }}
        code {{
            font-family: "Courier New", Consolas, monospace;
            background-color: #f0f0f0;
            padding: 2pt 4pt;
            border-radius: 2pt;
        }}
        pre code {{
            background-color: transparent; padding: 0;
        }}
        blockquote {{
            border-left: 4px solid #ddd;
            margin: 10pt 0;
            padding-left: 15pt;
            color: #666;
            font-style: italic;
        }}
        table {{
            border-collapse: collapse;
            width: 100%;
            margin: 10pt 0;
        }}
        table, th, td {{ border: 1px solid #ddd; }}
        th, td {{ padding: 6pt 8pt; text-align: left; }}
        th {{ background-color: #f5f5f5; font-weight: bold; }}
        img {{
            max-width: 100%; height: auto; display: block; margin: 10pt auto;
        }}
        .page-break {{ page-break-before: always; }}
    </style>
</head>
<body>
    <h1>{title}</h1>
    <div class="meta">
        <p><strong>Export Time:</strong>{export_time}</p>
    </div>
    <div class="content">
        {content}
    </div>
</body>
</html>"#,
        title = notebook.name,
        keywords = keywords,
        export_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        content = notebook_html_content
    );

    // 7. 创建临时HTML文件并生成PDF
    use std::fs;
    use std::io::Write;

    let temp_dir = std::env::temp_dir();
    let temp_html = temp_dir.join(format!("mytips_notebook_temp_{}.html", uuid::Uuid::new_v4()));
    
    let mut html_file = fs::File::create(&temp_html)
        .map_err(|e| format!("Failed to create temporary HTML file: {}", e))?;
    html_file.write_all(html_document.as_bytes())
        .map_err(|e| format!("Failed to write HTML content: {}", e))?;

    let pdf_generation_success = 
        try_wkhtmltopdf(&temp_html, &destination_path) ||
        try_chrome_pdf(&temp_html, &destination_path);

    let _ = fs::remove_file(&temp_html);

    if !pdf_generation_success {
        return Err("PDF generation failed. Please ensure that either Google Chrome or wkhtmltopdf is installed on your system.".to_string());
    }
    
    Ok(())
}


#[tauri::command]
pub async fn export_notebook_to_word(app: AppHandle, notebook_id: String) -> Result<(), String> {
    // 1. 弹出对话框
    let dialog = app.dialog();
    let destination_path = match dialog.file().add_filter("Word Document", &["docx"]).blocking_save_file() {
        Some(path) => match path.as_path() {
            Some(p) => p.to_path_buf(),
            None => return Ok(()),
        },
        None => return Ok(()),
    };

    // 2. 数据库和笔记本信息
    let manager = app.state::<UnifiedDbManager>();
    let conn = manager.get_conn().await.map_err(|e| e.to_string())?;
    let notebook = operations::get_category_by_id(&conn, &notebook_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Notebook not found".to_string())?;

    // 3. 创建 Word 文档
    let mut docx = Docx::new();
    
    // 4. 递归写入内容
    write_category_to_word_recursive(&conn, &notebook, &mut docx, 1).await.map_err(|e| e.to_string())?;

    // 5. 保存
    let file = fs::File::create(destination_path).map_err(|e| e.to_string())?;
    docx.build().pack(&file).map_err(|e| e.to_string())?;

    Ok(())
}

fn append_markdown_to_docx(mut docx: Docx, markdown: &str) -> Result<Docx> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown, options);

    let mut paragraph = Paragraph::new();
    let mut is_bold = false;
    let mut is_italic = false;
    let mut is_strike = false;
    let mut is_in_code_block = false;
    let mut code_buffer = String::new();

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::CodeBlock(_) => {
                    if !paragraph.children.is_empty() {
                        docx = docx.add_paragraph(paragraph);
                    }
                    paragraph = Paragraph::new();
                    is_in_code_block = true;
                }
                Tag::Heading(level, _, _) => {
                    if !paragraph.children.is_empty() {
                        docx = docx.add_paragraph(paragraph);
                    }
                    paragraph = Paragraph::new().style(&format!("Heading{}", level as u32));
                }
                Tag::Paragraph => {
                    paragraph = Paragraph::new();
                }
                Tag::Strong => is_bold = true,
                Tag::Emphasis => is_italic = true,
                Tag::Strikethrough => is_strike = true,
                Tag::Item => {
                    if !paragraph.children.is_empty() {
                        docx = docx.add_paragraph(paragraph);
                    }
                    paragraph = Paragraph::new().add_run(Run::new().add_text("•\t"));
                }
                _ => {}
            },
            Event::End(tag) => match tag {
                Tag::CodeBlock(_) => {
                    is_in_code_block = false;

                    let mut code_run = Run::new().add_text(&code_buffer);
                    code_run.run_property = code_run.run_property.fonts(RunFonts::new().east_asia("Courier New"));
                    let code_paragraph = Paragraph::new().add_run(code_run);
                    
                    let shading = Shading::new().color("auto").fill("F0F0F0");
                    let cell = TableCell::new()
                        .add_paragraph(code_paragraph)
                        .shading(shading);

                    let table = Table::new(vec![TableRow::new(vec![cell])]);

                    docx = docx.add_table(table);
                    code_buffer.clear();
                    paragraph = Paragraph::new();
                }
                Tag::Heading(..) | Tag::Paragraph | Tag::Item => {
                    docx = docx.add_paragraph(paragraph.clone());
                    paragraph = Paragraph::new();
                }
                Tag::Strong => is_bold = false,
                Tag::Emphasis => is_italic = false,
                Tag::Strikethrough => is_strike = false,
                _ => {}
            },
            Event::Text(text) => {
                if is_in_code_block {
                    code_buffer.push_str(&text);
                } else {
                    let mut run = Run::new().add_text(&*text);
                    if is_bold { run = run.bold(); }
                    if is_italic { run = run.italic(); }
                    if is_strike {
                        run.run_property = run.run_property.strike();
                    }
                    paragraph = paragraph.add_run(run);
                }
            },
            Event::Code(text) => {
                let mut run = Run::new().add_text(&*text);
                run.run_property = run.run_property.highlight("lightGray");
                paragraph = paragraph.add_run(run);
            },
            Event::HardBreak => {
                paragraph = paragraph.add_run(Run::new().add_break(BreakType::TextWrapping));
            }
            Event::Rule => {
                 let p = Paragraph::new().add_run(Run::new().add_text("---")); // Simplistic rule
                 docx = docx.add_paragraph(p);
            }
            _ => {}
        }
    }
    if !paragraph.children.is_empty() {
        docx = docx.add_paragraph(paragraph);
    }
    Ok(docx)
}

async fn write_category_to_word_recursive(
    conn: &crate::db::DbConnection,
    category: &Category,
    docx: &mut Docx,
    level: usize,
) -> Result<()> {
    let mut temp_docx = std::mem::take(docx);
    // 写入笔记本标题
    let heading_style = format!("Heading{}", level.min(5));
    temp_docx = temp_docx.add_paragraph(
        Paragraph::new()
            .add_run(Run::new().add_text(&category.name))
            .style(&heading_style),
    );
    
    // 写入笔记
    let tips = operations::get_tips_by_category(conn, &category.id).await?;
    for tip in tips {
        temp_docx = temp_docx.add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text(&tip.title))
                .style(&format!("Heading{}", (level + 1).min(5))),
        );
        let shifted_markdown = shift_markdown_headings(&tip.content, 1);
        temp_docx = append_markdown_to_docx(temp_docx, &shifted_markdown)?;
    }
    
    *docx = temp_docx;

    // 递归
    let sub_categories = operations::get_subcategories(conn, &category.id).await?;
    for sub_category in sub_categories {
        Box::pin(write_category_to_word_recursive(conn, &sub_category, docx, level + 1)).await?;
    }
    
    Ok(())
}

async fn export_category_recursive(conn: &crate::db::DbConnection, category: &Category, current_path: &Path) -> Result<()> {
    // 1. 导出当前笔记本下的所有笔记
    let tips = operations::get_tips_by_category(conn, &category.id).await?;
    for tip in tips {
        let file_path = current_path.join(format!("{}.md", sanitize_filename(&tip.title)));
        fs::write(file_path, &tip.content)?;
    }

    // 2. 递归导出子笔记本
    let sub_categories = operations::get_subcategories(conn, &category.id).await?;
    for sub_category in sub_categories {
        let sub_path = current_path.join(sanitize_filename(&sub_category.name));
        fs::create_dir_all(&sub_path)?;
        Box::pin(export_category_recursive(conn, &sub_category, &sub_path)).await?;
    }

    Ok(())
}

fn sanitize_filename(name: &str) -> String {
    name.chars().map(|c| {
        match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c
        }
    }).collect()
}
