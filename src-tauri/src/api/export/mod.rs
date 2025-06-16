use std::io::Write;
use tauri_plugin_dialog::DialogExt;

// 导出为Markdown
#[tauri::command]
pub async fn export_as_markdown(
    app: tauri::AppHandle,
    note_ids: Option<Vec<String>>,
) -> Result<String, String> {
    use crate::db::DbManager;
    use std::fs;

    // 获取笔记
    let db = DbManager::init().map_err(|e| e.to_string())?;
    let tips = if let Some(ids) = note_ids {
        // 如果提供了笔记ID列表，只导出指定的笔记
        let mut result = Vec::new();
        for id in ids {
            if let Ok(tip) = db.get_tip(&id) {
                result.push(tip);
            }
        }
        result
    } else {
        // 否则导出所有笔记
        db.get_all_tips().map_err(|e| e.to_string())?
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
            if let Ok(category) = db.get_category_by_id(category_id) {
                content.push_str(&format!("**分类**: {}\n\n", category.name));
            }
        }

        // 添加标签信息
        let tags = db.get_tip_tags(&tip.id).map_err(|e| e.to_string())?;
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
) -> Result<String, String> {
    use crate::db::DbManager;
    use std::fs;

    // 获取笔记
    let db = DbManager::init().map_err(|e| e.to_string())?;
    let tips = if let Some(ids) = note_ids {
        // 如果提供了笔记ID列表，只导出指定的笔记
        let mut result = Vec::new();
        for id in ids {
            if let Ok(tip) = db.get_tip(&id) {
                result.push(tip);
            }
        }
        result
    } else {
        // 否则导出所有笔记
        db.get_all_tips().map_err(|e| e.to_string())?
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
        let tags = db.get_tip_tags(&tip.id).map_err(|e| e.to_string())?;
        let tags_str = tags
            .iter()
            .map(|tag| tag.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        // 获取分类信息
        let category_name = if let Some(category_id) = &tip.category_id {
            if let Ok(category) = db.get_category_by_id(category_id) {
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
) -> Result<String, String> {
    use crate::db::DbManager;
    use printpdf::*;
    use std::fs;
    use std::io::{BufWriter, Write};

    // 获取笔记
    let db = DbManager::init().map_err(|e| e.to_string())?;
    let tips = if let Some(ids) = note_ids {
        // 如果提供了笔记ID列表，只导出指定的笔记
        let mut result = Vec::new();
        for id in ids {
            if let Ok(tip) = db.get_tip(&id) {
                result.push(tip);
            }
        }
        result
    } else {
        // 否则导出所有笔记
        db.get_all_tips().map_err(|e| e.to_string())?
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
        let tags = db.get_tip_tags(&tip.id).map_err(|e| e.to_string())?;
        let tags_str = tags
            .iter()
            .map(|tag| tag.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        // 获取分类信息
        let category_name = if let Some(category_id) = &tip.category_id {
            if let Ok(category) = db.get_category_by_id(category_id) {
                category.name
            } else {
                "未分类".to_string()
            }
        } else {
            "未分类".to_string()
        };

        // 创建一个新的PDF文档
        let (doc, page1, layer1) =
            PdfDocument::new("MyTips Export", Mm(210.0), Mm(297.0), "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);

        // 添加内置字体
        let font = doc
            .add_builtin_font(BuiltinFont::Helvetica)
            .map_err(|e| e.to_string())?;

        // 简单添加笔记标题 - 仅使用ASCII字符
        let ascii_title: String = tip
            .title
            .chars()
            .filter(|c| c.is_ascii() && !c.is_control())
            .collect();

        current_layer.begin_text_section();
        current_layer.set_font(&font, 24.0);
        current_layer.set_text_cursor(Mm(10.0), Mm(280.0));
        current_layer.write_text(ascii_title, &font);
        current_layer.end_text_section();

        // 添加分类和标签信息 - 仅使用ASCII字符
        let ascii_category: String = category_name
            .chars()
            .filter(|c| c.is_ascii() && !c.is_control())
            .collect();

        let ascii_tags: String = tags_str
            .chars()
            .filter(|c| c.is_ascii() && !c.is_control())
            .collect();

        current_layer.begin_text_section();
        current_layer.set_font(&font, 12.0);
        current_layer.set_text_cursor(Mm(10.0), Mm(270.0));
        current_layer.write_text(format!("Category: {}", ascii_category), &font);
        current_layer.end_text_section();

        current_layer.begin_text_section();
        current_layer.set_font(&font, 12.0);
        current_layer.set_text_cursor(Mm(10.0), Mm(265.0));
        current_layer.write_text(format!("Tags: {}", ascii_tags), &font);
        current_layer.end_text_section();

        // 添加空行作为分隔
        current_layer.begin_text_section();
        current_layer.set_font(&font, 10.0);
        current_layer.set_text_cursor(Mm(10.0), Mm(255.0));
        current_layer.write_text("", &font);
        current_layer.end_text_section();

        // 只添加纯文本内容，避免处理Markdown
        current_layer.begin_text_section();
        current_layer.set_font(&font, 10.0);
        current_layer.set_text_cursor(Mm(10.0), Mm(250.0));

        // 处理内容，仅使用ASCII字符避免中文乱码
        // 移除Markdown标记
        let content = tip
            .content
            .replace("# ", "")
            .replace("## ", "")
            .replace("### ", "")
            .replace("#### ", "")
            .replace("##### ", "")
            .replace("###### ", "")
            .replace("*", "")
            .replace("```", "")
            .replace("`", "");

        // 由于printpdf库处理中文字符有限制，我们只保留ASCII字符
        let filtered_content: String = content
            .chars()
            .filter(|c| c.is_ascii() && !c.is_control())
            .collect();

        // 分行显示，每行最多80字符
        let mut y_pos = 250.0;
        let chars_per_line = 80;

        for chunk in filtered_content
            .as_bytes()
            .chunks(chars_per_line)
            .map(|c| std::str::from_utf8(c).unwrap_or(""))
        {
            if y_pos < 20.0 {
                break; // 避免超出页面
            }

            current_layer.set_text_cursor(Mm(10.0), Mm(y_pos));
            current_layer.write_text(chunk.to_string(), &font);
            y_pos -= 5.0;
        }

        current_layer.end_text_section();

        // 保存PDF文件
        let file = fs::File::create(&file_path).map_err(|e| format!("创建文件失败: {}", e))?;
        let mut buf_writer = BufWriter::new(file);
        doc.save(&mut buf_writer)
            .map_err(|e| format!("保存PDF失败: {}", e))?;

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
