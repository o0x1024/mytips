use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::db::UnifiedDbManager;
use super::storage;

/// 音频搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSearchResult {
    pub audio_id: String,
    pub tip_id: String,
    pub file_name: String,
    pub transcription: Option<String>,
    pub matched_segments: Vec<SearchMatchSegment>,
    pub relevance_score: f32,
    pub created_at: i64,
}

/// 搜索匹配片段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMatchSegment {
    pub start_pos: usize,
    pub end_pos: usize,
    pub matched_text: String,
    pub context: String,
    pub score: f32,
}

/// 音频搜索请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSearchRequest {
    pub query: String,
    pub tip_ids: Option<Vec<String>>, // 限制搜索范围
    pub language: Option<String>,
    pub include_transcription: bool,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// 搜索音频内容
pub async fn search_audio_content(
    db_manager: &UnifiedDbManager,
    request: AudioSearchRequest,
) -> Result<Vec<AudioSearchResult>, String> {
    if request.query.trim().is_empty() {
        return Ok(Vec::new());
    }

    // 获取所有音频文件
    let all_audio_files = if let Some(tip_ids) = &request.tip_ids {
        let mut files = Vec::new();
        for tip_id in tip_ids {
            let tip_files = storage::get_tip_audio_files(db_manager, tip_id).await?;
            files.extend(tip_files);
        }
        files
    } else {
        get_all_audio_files(db_manager).await?
    };

    // 筛选有转录文本的音频文件
    let audio_files_with_transcription: Vec<_> = all_audio_files
        .into_iter()
        .filter(|f| f.transcription.is_some())
        .collect();

    if audio_files_with_transcription.is_empty() {
        return Ok(Vec::new());
    }

    // 执行搜索
    let search_terms = tokenize_search_query(&request.query);
    let mut results = Vec::new();

    for audio_file in audio_files_with_transcription {
        if let Some(ref transcription) = audio_file.transcription {
            let matches = find_text_matches(transcription, &search_terms);
            if !matches.is_empty() {
                let relevance_score = calculate_relevance_score(&matches, transcription, &request.query);
                
                let result = AudioSearchResult {
                    audio_id: audio_file.audio_id,
                    tip_id: audio_file.tip_id,
                    file_name: audio_file.file_name,
                    transcription: audio_file.transcription,
                    matched_segments: matches,
                    relevance_score,
                    created_at: audio_file.created_at,
                };
                
                results.push(result);
            }
        }
    }

    // 按相关性排序
    results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());

    // 应用分页
    let offset = request.offset.unwrap_or(0);
    let limit = request.limit.unwrap_or(50);
    
    let end = std::cmp::min(offset + limit, results.len());
    if offset >= results.len() {
        Ok(Vec::new())
    } else {
        Ok(results[offset..end].to_vec())
    }
}

/// 获取所有音频文件
async fn get_all_audio_files(
    db_manager: &UnifiedDbManager,
) -> Result<Vec<super::AudioFileInfo>, String> {
    let conn = db_manager.get_conn().await
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let query = "SELECT id, tip_id, audio_id, file_name, file_format, duration, transcription, created_at, updated_at FROM tip_audio_files ORDER BY created_at DESC";
    
    let mut rows = conn.query(query, ()).await
        .map_err(|e| format!("Failed to execute query: {}", e))?;

    let mut audio_files = Vec::new();
    while let Some(row) = rows.next().await.map_err(|e| format!("Failed to read row: {}", e))? {
        let audio_file = super::AudioFileInfo {
            id: row.get::<String>(0).map_err(|e| format!("Failed to get id: {}", e))?,
            tip_id: row.get::<String>(1).map_err(|e| format!("Failed to get tip_id: {}", e))?,
            audio_id: row.get::<String>(2).map_err(|e| format!("Failed to get audio_id: {}", e))?,
            file_name: row.get::<String>(3).map_err(|e| format!("Failed to get file_name: {}", e))?,
            file_format: row.get::<String>(4).map_err(|e| format!("Failed to get file_format: {}", e))?,
            duration: row.get::<Option<i64>>(5).map_err(|e| format!("Failed to get duration: {}", e))?,
            file_size: 0, // 不在这个查询中获取文件大小
            transcription: row.get::<Option<String>>(6).map_err(|e| format!("Failed to get transcription: {}", e))?,
            transcription_confidence: None,
            created_at: row.get::<i64>(7).map_err(|e| format!("Failed to get created_at: {}", e))?,
            updated_at: row.get::<i64>(8).map_err(|e| format!("Failed to get updated_at: {}", e))?,
        };
        audio_files.push(audio_file);
    }

    Ok(audio_files)
}

/// 将搜索查询分词
fn tokenize_search_query(query: &str) -> Vec<String> {
    query
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.trim_matches(|c: char| c.is_ascii_punctuation()))
        .filter(|s| !s.is_empty() && s.len() > 1)
        .map(|s| s.to_string())
        .collect()
}

/// 在文本中查找匹配项
fn find_text_matches(text: &str, search_terms: &[String]) -> Vec<SearchMatchSegment> {
    let mut matches = Vec::new();
    let text_lower = text.to_lowercase();

    for term in search_terms {
        let mut start = 0;
        while let Some(pos) = text_lower[start..].find(term) {
            let actual_pos = start + pos;
            let context_start = actual_pos.saturating_sub(50);
            let context_end = std::cmp::min(actual_pos + term.len() + 50, text.len());
            
            let matched_text = text[actual_pos..actual_pos + term.len()].to_string();
            let context = text[context_start..context_end].to_string();
            
            let score = calculate_term_score(term, &matched_text, actual_pos, text.len());
            
            matches.push(SearchMatchSegment {
                start_pos: actual_pos,
                end_pos: actual_pos + term.len(),
                matched_text,
                context,
                score,
            });
            
            start = actual_pos + 1;
        }
    }

    // 合并重叠的匹配项
    merge_overlapping_matches(matches)
}

/// 合并重叠的匹配项
fn merge_overlapping_matches(mut matches: Vec<SearchMatchSegment>) -> Vec<SearchMatchSegment> {
    if matches.is_empty() {
        return matches;
    }

    matches.sort_by_key(|m| m.start_pos);
    
    let mut merged = Vec::new();
    let mut current = matches[0].clone();
    
    for match_item in matches.into_iter().skip(1) {
        if match_item.start_pos <= current.end_pos + 10 { // 允许10个字符的间隔
            // 合并匹配项
            current.end_pos = std::cmp::max(current.end_pos, match_item.end_pos);
            current.score = (current.score + match_item.score) / 2.0;
            
            // 扩展上下文
            if match_item.context.len() > current.context.len() {
                current.context = match_item.context;
            }
        } else {
            merged.push(current);
            current = match_item;
        }
    }
    merged.push(current);
    
    merged
}

/// 计算词项评分
fn calculate_term_score(term: &str, matched_text: &str, position: usize, total_length: usize) -> f32 {
    let mut score = 1.0;
    
    // 完全匹配加分
    if term == matched_text.to_lowercase() {
        score += 0.5;
    }
    
    // 位置加分（开头的匹配更重要）
    let position_ratio = position as f32 / total_length as f32;
    score += (1.0 - position_ratio) * 0.3;
    
    // 长度加分
    score += (term.len() as f32 / 10.0).min(0.5);
    
    score
}

/// 计算相关性评分
fn calculate_relevance_score(
    matches: &[SearchMatchSegment], 
    text: &str, 
    query: &str
) -> f32 {
    if matches.is_empty() {
        return 0.0;
    }

    let mut total_score = 0.0;
    let query_terms = tokenize_search_query(query);
    
    // 基础匹配评分
    for match_item in matches {
        total_score += match_item.score;
    }
    
    // 匹配项数量加分
    total_score += matches.len() as f32 * 0.2;
    
    // 查询覆盖率加分
    let unique_terms_matched = matches
        .iter()
        .map(|m| m.matched_text.to_lowercase())
        .collect::<HashSet<_>>();
    
    let coverage_ratio = unique_terms_matched.len() as f32 / query_terms.len() as f32;
    total_score += coverage_ratio * 1.0;
    
    // 文本长度调整（较短文本的匹配更有意义）
    let text_length_factor = 1.0 / (1.0 + (text.len() as f32 / 1000.0));
    total_score *= text_length_factor;
    
    total_score
}

/// 构建音频内容搜索索引
pub async fn build_audio_search_index(
    db_manager: &UnifiedDbManager,
) -> Result<AudioSearchIndex, String> {
    let audio_files = get_all_audio_files(db_manager).await?;
    let mut index = AudioSearchIndex::new();
    
    for audio_file in audio_files {
        if let Some(transcription) = audio_file.transcription {
            index.add_document(audio_file.audio_id, &transcription);
        }
    }
    
    Ok(index)
}

/// 音频搜索索引
#[derive(Debug, Clone)]
pub struct AudioSearchIndex {
    // 倒排索引：词 -> 文档ID列表
    inverted_index: HashMap<String, Vec<String>>,
    // 文档内容：文档ID -> 原始文本
    documents: HashMap<String, String>,
    // 词频统计：文档ID -> 词 -> 频率
    term_frequencies: HashMap<String, HashMap<String, usize>>,
}

impl AudioSearchIndex {
    pub fn new() -> Self {
        Self {
            inverted_index: HashMap::new(),
            documents: HashMap::new(),
            term_frequencies: HashMap::new(),
        }
    }
    
    /// 添加文档到索引
    pub fn add_document(&mut self, doc_id: String, content: &str) {
        let terms = tokenize_search_query(content);
        
        // 存储原始文档
        self.documents.insert(doc_id.clone(), content.to_string());
        
        // 计算词频
        let mut term_freq = HashMap::new();
        for term in &terms {
            *term_freq.entry(term.clone()).or_insert(0) += 1;
        }
        self.term_frequencies.insert(doc_id.clone(), term_freq);
        
        // 更新倒排索引
        for term in terms {
            self.inverted_index
                .entry(term)
                .or_insert_with(Vec::new)
                .push(doc_id.clone());
        }
    }
    
    /// 搜索文档
    pub fn search(&self, query: &str) -> Vec<(String, f32)> {
        let query_terms = tokenize_search_query(query);
        if query_terms.is_empty() {
            return Vec::new();
        }
        
        // 找到包含查询词的文档
        let mut candidate_docs = HashSet::new();
        for term in &query_terms {
            if let Some(doc_ids) = self.inverted_index.get(term) {
                candidate_docs.extend(doc_ids.iter().cloned());
            }
        }
        
        // 计算每个候选文档的评分
        let mut scored_docs = Vec::new();
        for doc_id in candidate_docs {
            let score = self.calculate_tf_idf_score(&doc_id, &query_terms);
            if score > 0.0 {
                scored_docs.push((doc_id, score));
            }
        }
        
        // 按评分排序
        scored_docs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        scored_docs
    }
    
    /// 计算TF-IDF评分
    fn calculate_tf_idf_score(&self, doc_id: &str, query_terms: &[String]) -> f32 {
        let total_docs = self.documents.len() as f32;
        if total_docs == 0.0 {
            return 0.0;
        }
        
        let term_freq = match self.term_frequencies.get(doc_id) {
            Some(tf) => tf,
            None => return 0.0,
        };
        
        let mut score = 0.0;
        for term in query_terms {
            // TF (Term Frequency)
            let tf = *term_freq.get(term).unwrap_or(&0) as f32;
            if tf == 0.0 {
                continue;
            }
            
            // IDF (Inverse Document Frequency)
            let docs_with_term = self.inverted_index
                .get(term)
                .map(|docs| docs.len())
                .unwrap_or(0) as f32;
            
            if docs_with_term == 0.0 {
                continue;
            }
            
            let idf = (total_docs / docs_with_term).ln();
            score += tf * idf;
        }
        
        score
    }
}

/// 获取音频搜索统计信息
pub async fn get_audio_search_stats(
    db_manager: &UnifiedDbManager,
) -> Result<AudioSearchStats, String> {
    let conn = db_manager.get_conn().await
        .map_err(|e| format!("Database connection failed: {}", e))?;

    // 统计总音频文件数
    let mut rows = conn.query("SELECT COUNT(*) FROM tip_audio_files", ()).await
        .map_err(|e| format!("Failed to count audio files: {}", e))?;
    let total_audio_files: u32 = if let Some(row) = rows.next().await.map_err(|e| format!("Failed to read row: {}", e))? {
        row.get::<u32>(0).map_err(|e| format!("Failed to get count: {}", e))?
    } else { 0 };

    // 统计有转录的音频文件数
    let mut rows = conn.query("SELECT COUNT(*) FROM tip_audio_files WHERE transcription IS NOT NULL AND transcription != ''", ()).await
        .map_err(|e| format!("Failed to count transcribed files: {}", e))?;
    let transcribed_files: u32 = if let Some(row) = rows.next().await.map_err(|e| format!("Failed to read row: {}", e))? {
        row.get::<u32>(0).map_err(|e| format!("Failed to get count: {}", e))?
    } else { 0 };

    // 统计总转录文本长度
    let mut rows = conn.query("SELECT SUM(LENGTH(transcription)) FROM tip_audio_files WHERE transcription IS NOT NULL", ()).await
        .map_err(|e| format!("Failed to calculate text length: {}", e))?;
    let total_text_length: i64 = if let Some(row) = rows.next().await.map_err(|e| format!("Failed to read row: {}", e))? {
        row.get::<Option<i64>>(0).map_err(|e| format!("Failed to get sum: {}", e))?.unwrap_or(0)
    } else { 0 };

    Ok(AudioSearchStats {
        total_audio_files,
        transcribed_files,
        searchable_files: transcribed_files,
        total_text_length: total_text_length as u64,
        index_size_estimate: estimate_index_size(total_text_length as u64),
    })
}

/// 音频搜索统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSearchStats {
    pub total_audio_files: u32,
    pub transcribed_files: u32,
    pub searchable_files: u32,
    pub total_text_length: u64,
    pub index_size_estimate: u64, // 估计的索引大小（字节）
}

/// 估算索引大小
fn estimate_index_size(text_length: u64) -> u64 {
    // 粗略估计：每个字符产生约2-3字节的索引数据
    text_length * 3
} 