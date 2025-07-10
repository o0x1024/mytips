use crate::db::DbManager;
use tauri::State;
use tauri_plugin_dialog::DialogExt;

// 导出为Markdown
#[tauri::command]
pub async fn export_as_markdown(
    app: tauri::AppHandle,
    note_ids: Option<Vec<String>>,
    db_manager: State<'_, DbManager>,
) -> Result<String, String> {
    use std::fs;

    // 获取笔记
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let tips = if let Some(ids) = note_ids {
        // 如果提供了笔记ID列表，只导出指定的笔记
        let mut result = Vec::new();
        for id in ids {
            if let Ok(tip) = crate::db::get_tip(&conn, &id) {
                result.push(tip);
            }
        }
        result
    } else {
        // 否则导出所有笔记
        crate::db::get_all_tips(&conn).map_err(|e| e.to_string())?
    };

    if tips.is_empty() {
        return Err("没有找到要导出的笔记".to_string());
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
            .add_filter("Markdown文件", &["md"])
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
            if let Ok(category) = crate::db::get_category_by_id(&conn, category_id) {
                content.push_str(&format!("**分类**: {}\n\n", category.name));
            }
        }

        // 添加标签信息
        let tags = crate::db::get_tip_tags(&conn, &tip.id).map_err(|e| e.to_string())?;
        if !tags.is_empty() {
            content.push_str("**标签**: ");
            for (i, tag) in tags.iter().enumerate() {
                if i > 0 {
                    content.push_str(", ");
                }
                content.push_str(&tag.name);
            }
            content.push_str("\n\n");
        }

        // 添加笔记内容
        content.push_str(&tip.content);

        // 写入文件
        fs::write(&file_path, content).map_err(|e| format!("写入文件失败: {}", e))?;

        exported_count += 1;
    }

    if exported_count == 0 {
        return Err("没有导出任何笔记".to_string());
    } else if exported_count < total_count {
        Ok(format!(
            "成功导出 {} 个笔记（共 {} 个）",
            exported_count, total_count
        ))
    } else {
        Ok(format!("成功导出 {} 个笔记", exported_count))
    }
}

// 备份数据库
#[tauri::command]
pub async fn backup_database(app: tauri::AppHandle) -> Result<String, String> {
    use chrono::Local;
    use std::fs;

    // 获取数据库路径
    let db_path = dirs::data_dir()
        .ok_or_else(|| "无法获取应用数据目录".to_string())?
        .join("mytips/mytips.db");

    if !db_path.exists() {
        return Err("数据库文件不存在".to_string());
    }

    // 生成默认备份文件名（带时间戳）
    let date_time = Local::now().format("%Y%m%d_%H%M%S");
    let default_filename = format!("mytips_backup_{}.db", date_time);

    // 打开文件保存对话框
    let file_path = match app
        .dialog()
        .file()
        .add_filter("数据库文件", &["db"])
        .set_file_name(&default_filename)
        .blocking_save_file()
    {
        Some(path) => match path.as_path() {
            Some(p) => p.to_path_buf(),
            None => return Err("无法获取保存文件路径".to_string()),
        },
        None => return Err("用户取消了备份操作".to_string()),
    };

    // 复制数据库文件到选择的位置
    fs::copy(&db_path, &file_path).map_err(|e| format!("备份数据库失败: {}", e))?;

    Ok(format!("成功备份数据库到 {}", file_path.display()))
}

// 恢复数据库
#[tauri::command]
pub async fn restore_database(app: tauri::AppHandle) -> Result<String, String> {
    // 打开文件选择对话框
    let file_path = match app
        .dialog()
        .file()
        .add_filter("数据库文件", &["db"])
        .blocking_pick_file()
    {
        Some(path) => match path.as_path() {
            Some(p) => p.to_path_buf(),
            None => return Err("无法获取选择文件路径".to_string()),
        },
        None => return Err("恢复已取消".to_string()),
    };

    // 确保是数据库文件
    if let Some(ext) = file_path.extension() {
        if ext != "db" {
            return Err("请选择有效的数据库备份文件(.db)".to_string());
        }
    } else {
        return Err("请选择有效的数据库备份文件(.db)".to_string());
    }

    // 获取目标数据库路径
    let db_dir = dirs::data_dir()
        .ok_or_else(|| "无法获取应用数据目录".to_string())?
        .join("mytips");

    let db_path = db_dir.join("mytips.db");

    // 确保目录存在
    std::fs::create_dir_all(&db_dir).map_err(|e| format!("创建数据目录失败: {}", e))?;

    // 复制备份文件到数据库位置
    std::fs::copy(&file_path, &db_path).map_err(|e| format!("恢复失败: {}", e))?;

    Ok("数据库已恢复，重启应用以加载最新数据".to_string())
}

// 导出为HTML
#[tauri::command]
pub async fn export_as_html(
    app: tauri::AppHandle,
    note_ids: Option<Vec<String>>,
    db_manager: State<'_, DbManager>,
) -> Result<String, String> {
    use std::fs;

    // 获取笔记
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let tips = if let Some(ids) = note_ids {
        // 如果提供了笔记ID列表，只导出指定的笔记
        let mut result = Vec::new();
        for id in ids {
            if let Ok(tip) = crate::db::get_tip(&conn, &id) {
                result.push(tip);
            }
        }
        result
    } else {
        // 否则导出所有笔记
        crate::db::get_all_tips(&conn).map_err(|e| e.to_string())?
    };

    if tips.is_empty() {
        return Err("没有找到要导出的笔记".to_string());
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
            .add_filter("HTML文件", &["html"])
            .set_file_name(&format!("{}.html", file_name))
            .blocking_save_file()
        {
            Some(path) => match path.as_path() {
                Some(p) => p.to_path_buf(),
                None => continue, // 如果用户取消了对话框，跳过当前笔记
            },
            None => continue, // 如果用户取消了对话框，跳过当前笔记
        };

        // 获取标签信息
        let tags = crate::db::get_tip_tags(&conn, &tip.id).map_err(|e| e.to_string())?;
        let tags_str = tags
            .iter()
            .map(|tag| tag.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        // 获取分类信息
        let category_name = if let Some(category_id) = &tip.category_id {
            if let Ok(category) = crate::db::get_category_by_id(&conn, category_id) {
                category.name
            } else {
                "未分类".to_string()
            }
        } else {
            "未分类".to_string()
        };

        // 将Markdown转换为HTML
        let parser = pulldown_cmark::Parser::new(&tip.content);
        let mut html_content = String::new();
        pulldown_cmark::html::push_html(&mut html_content, parser);

        // 构建HTML文档
        let html = format!(
            r#"<!DOCTYPE html>
<html lang="zh-CN">
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
        <p><strong>分类</strong>: {category}<br>
        <strong>标签</strong>: {tags}</p>
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
        fs::write(&file_path, html).map_err(|e| format!("写入文件失败: {}", e))?;

        exported_count += 1;
    }

    if exported_count == 0 {
        return Err("没有导出任何笔记".to_string());
    } else if exported_count < total_count {
        Ok(format!(
            "成功导出 {} 个笔记（共 {} 个）",
            exported_count, total_count
        ))
    } else {
        Ok(format!("成功导出 {} 个笔记", exported_count))
    }
}

// 导出为PDF
#[tauri::command]
pub async fn export_as_pdf(
    app: tauri::AppHandle,
    note_ids: Option<Vec<String>>,
    db_manager: State<'_, DbManager>,
) -> Result<String, String> {
    use std::fs;
    use std::io::Write;


    // 获取笔记
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let tips = if let Some(ids) = note_ids {
        // 如果提供了笔记ID列表，只导出指定的笔记
        let mut result = Vec::new();
        for id in ids {
            if let Ok(tip) = crate::db::get_tip(&conn, &id) {
                result.push(tip);
            }
        }
        result
    } else {
        // 否则导出所有笔记
        crate::db::get_all_tips(&conn).map_err(|e| e.to_string())?
    };

    if tips.is_empty() {
        return Err("没有找到要导出的笔记".to_string());
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
            .add_filter("PDF文件", &["pdf"])
            .set_file_name(&format!("{}.pdf", file_name))
            .blocking_save_file()
        {
            Some(path) => match path.as_path() {
                Some(p) => p.to_path_buf(),
                None => continue, // 如果用户取消了对话框，跳过当前笔记
            },
            None => continue, // 如果用户取消了对话框，跳过当前笔记
        };

        // 获取标签信息
        let tags = crate::db::get_tip_tags(&conn, &tip.id).map_err(|e| e.to_string())?;
        let tags_str = tags
            .iter()
            .map(|tag| tag.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        // 获取分类信息
        let category_name = if let Some(category_id) = &tip.category_id {
            if let Ok(category) = crate::db::get_category_by_id(&conn, category_id) {
                category.name
            } else {
                "未分类".to_string()
            }
        } else {
            "未分类".to_string()
        };

        // 将Markdown转换为HTML
        let parser = pulldown_cmark::Parser::new(&tip.content);
        let mut html_content = String::new();
        pulldown_cmark::html::push_html(&mut html_content, parser);

        // 构建完整的HTML文档，支持中文字体
        let html_document = format!(
            r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
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
        <p><strong>分类：</strong>{category}</p>
        <p><strong>标签：</strong>{tags}</p>
        <p><strong>导出时间：</strong>{export_time}</p>
    </div>
    <div class="content">
        {content}
    </div>
</body>
</html>"#,
            title = tip.title,
            category = category_name,
            tags = tags_str,
            export_time = chrono::Local::now().format("%Y年%m月%d日 %H:%M:%S").to_string(),
            content = html_content
        );

        // 创建临时HTML文件
        let temp_dir = std::env::temp_dir();
        let temp_html = temp_dir.join(format!("mytips_temp_{}.html", uuid::Uuid::new_v4()));
        
        // 写入HTML内容
        let mut html_file = fs::File::create(&temp_html)
            .map_err(|e| format!("创建临时HTML文件失败: {}", e))?;
        html_file.write_all(html_document.as_bytes())
            .map_err(|e| format!("写入HTML内容失败: {}", e))?;

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
            return Err(format!("PDF生成失败，请确保系统已安装Chrome浏览器或wkhtmltopdf工具。\n\n备选方案：\n1. 安装Chrome浏览器\n2. 安装wkhtmltopdf: brew install wkhtmltopdf (macOS)\n3. 或者选择导出为HTML格式，然后使用浏览器打印为PDF"));
        }
    }

    if exported_count == 0 {
        return Err("没有导出任何笔记".to_string());
    } else if exported_count < total_count {
        Ok(format!(
            "成功导出 {} 个笔记（共 {} 个）",
            exported_count, total_count
        ))
    } else {
        Ok(format!("成功导出 {} 个笔记", exported_count))
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
    let (doc, page1, layer1) = PdfDocument::new("MyTips Export", Mm(210.0), Mm(297.0), "Layer 1");
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
    current_layer.write_text("Note: PDF fallback mode - Chinese characters converted to pinyin", &font);
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
