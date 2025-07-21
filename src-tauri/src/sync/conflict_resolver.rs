use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use tracing::{info, warn, debug};
use super::monitoring::{PerformanceMonitor, StructuredLogger};
use crate::db::{Tip, Category, Tag};
use super::{SyncStatusRecord, ConflictData, SyncOperation, ConflictResolutionStrategy};

/// 增强的冲突解决器
pub struct EnhancedConflictResolver {
    /// 默认解决策略
    default_strategy: ConflictResolutionStrategy,
    /// 表级别的特定策略
    table_strategies: HashMap<String, ConflictResolutionStrategy>,
    /// 字段级别的优先级配置
    field_priorities: HashMap<String, FieldPriorityConfig>,
    /// 性能监控器
    performance_monitor: Arc<PerformanceMonitor>,
    /// 日志记录器
    structured_logger: Arc<StructuredLogger>,
}

/// 字段优先级配置
#[derive(Debug, Clone)]
pub struct FieldPriorityConfig {
    /// 字段名称
    pub field_name: String,
    /// 优先级权重（0-100）
    pub priority_weight: u8,
    /// 合并策略
    pub merge_strategy: FieldMergeStrategy,
    /// 是否为关键字段
    pub is_critical: bool,
}

/// 字段级合并策略
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldMergeStrategy {
    /// 使用本地值
    LocalWins,
    /// 使用远程值
    RemoteWins,
    /// 使用较新的值（基于时间戳）
    NewerWins,
    /// 使用较长的值（适用于文本字段）
    LongerWins,
    /// 合并列表（适用于数组字段）
    MergeLists,
    /// 用户选择
    UserChoice,
    /// 自定义合并逻辑
    Custom,
}

/// 增强的冲突数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedConflictData {
    /// 基础冲突信息
    pub base_conflict: ConflictData,
    /// 字段级别的冲突详情
    pub field_conflicts: Vec<FieldConflict>,
    /// 冲突严重级别
    pub severity: ConflictSeverity,
    /// 建议的解决策略
    pub suggested_strategy: ConflictResolutionStrategy,
    /// 上下文信息
    pub context: ConflictContext,
}

/// 字段冲突详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldConflict {
    /// 字段名称
    pub field_name: String,
    /// 本地值
    pub local_value: serde_json::Value,
    /// 远程值
    pub remote_value: serde_json::Value,
    /// 冲突类型
    pub conflict_type: FieldConflictType,
    /// 建议的解决方案
    pub suggested_resolution: FieldResolution,
}

/// 冲突严重级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictSeverity {
    /// 低：可自动解决
    Low,
    /// 中：建议用户确认
    Medium,
    /// 高：需要用户干预
    High,
    /// 严重：数据完整性风险
    Critical,
}

/// 字段冲突类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldConflictType {
    /// 值不同
    ValueDifference,
    /// 类型不匹配
    TypeMismatch,
    /// 一个为空另一个不为空
    NullConflict,
    /// 格式冲突
    FormatConflict,
}

/// 字段解决方案
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldResolution {
    /// 解决策略
    pub strategy: FieldMergeStrategy,
    /// 解决后的值
    pub resolved_value: serde_json::Value,
    /// 置信度（0-100）
    pub confidence: u8,
}

/// 冲突上下文
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictContext {
    /// 表名
    pub table_name: String,
    /// 记录ID
    pub record_id: String,
    /// 操作类型
    pub operation: SyncOperation,
    /// 最后修改用户
    pub last_modified_by: Option<String>,
    /// 修改时间差（秒）
    pub time_diff_seconds: i64,
    /// 相关的外键约束
    pub foreign_keys: Vec<String>,
}

/// 增强的冲突解决结果
#[derive(Debug, Serialize, Deserialize)]
pub struct EnhancedConflictResolutionResult {
    /// 使用的解决策略
    pub strategy: ConflictResolutionStrategy,
    /// 解决后的内容
    pub resolved_content: String,
    /// 增强的冲突数据
    pub enhanced_conflict: EnhancedConflictData,
    /// 解决方案的置信度（0-100）
    pub resolution_confidence: u8,
    /// 应用的字段解决方案
    pub applied_field_resolutions: Vec<FieldConflict>,
    /// 解决者标识
    pub resolved_by: String,
}

/// 批量冲突解决结果
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchConflictResolutionResult {
    /// 总冲突数量
    pub total_conflicts: usize,
    /// 解决成功数量
    pub resolved_count: usize,
    /// 解决失败数量
    pub failed_count: usize,
    /// 解决结果列表
    pub resolution_results: Vec<EnhancedConflictResolutionResult>,
    /// 总耗时（毫秒）
    pub duration_ms: u64,
}

impl EnhancedConflictResolver {
    /// 创建新的增强冲突解决器
    pub fn new(
        performance_monitor: Arc<PerformanceMonitor>,
        structured_logger: Arc<StructuredLogger>,
    ) -> Self {
        let mut resolver = Self {
            default_strategy: ConflictResolutionStrategy::Merge,
            table_strategies: HashMap::new(),
            field_priorities: HashMap::new(),
            performance_monitor,
            structured_logger,
        };

        // 初始化默认的表级策略
        resolver.init_default_strategies();
        
        // 初始化默认的字段优先级
        resolver.init_default_field_priorities();

        resolver
    }

    /// 初始化默认策略
    fn init_default_strategies(&mut self) {
        // 不同表的默认策略
        self.table_strategies.insert("tips".to_string(), ConflictResolutionStrategy::Merge);
        self.table_strategies.insert("categories".to_string(), ConflictResolutionStrategy::RemoteWins);
        self.table_strategies.insert("tags".to_string(), ConflictResolutionStrategy::Merge);
        self.table_strategies.insert("ai_conversations".to_string(), ConflictResolutionStrategy::LocalWins);
        self.table_strategies.insert("ai_messages".to_string(), ConflictResolutionStrategy::LocalWins);
    }

    /// 初始化默认字段优先级
    fn init_default_field_priorities(&mut self) {
        // Tips 表的字段优先级
        self.field_priorities.insert("tips.title".to_string(), FieldPriorityConfig {
            field_name: "title".to_string(),
            priority_weight: 90,
            merge_strategy: FieldMergeStrategy::LongerWins,
            is_critical: true,
        });

        self.field_priorities.insert("tips.content".to_string(), FieldPriorityConfig {
            field_name: "content".to_string(),
            priority_weight: 100,
            merge_strategy: FieldMergeStrategy::LongerWins,
            is_critical: true,
        });

        self.field_priorities.insert("tips.updated_at".to_string(), FieldPriorityConfig {
            field_name: "updated_at".to_string(),
            priority_weight: 80,
            merge_strategy: FieldMergeStrategy::NewerWins,
            is_critical: false,
        });

        self.field_priorities.insert("tips.tags".to_string(), FieldPriorityConfig {
            field_name: "tags".to_string(),
            priority_weight: 70,
            merge_strategy: FieldMergeStrategy::MergeLists,
            is_critical: false,
        });

        // Categories 表的字段优先级
        self.field_priorities.insert("categories.name".to_string(), FieldPriorityConfig {
            field_name: "name".to_string(),
            priority_weight: 95,
            merge_strategy: FieldMergeStrategy::RemoteWins, // 分类名称以远程为准
            is_critical: true,
        });

        // Tags 表的字段优先级
        self.field_priorities.insert("tags.name".to_string(), FieldPriorityConfig {
            field_name: "name".to_string(),
            priority_weight: 85,
            merge_strategy: FieldMergeStrategy::RemoteWins,
            is_critical: true,
        });
    }

    /// 智能解决冲突
    pub async fn resolve_conflict_intelligently(
        &self,
        record: &SyncStatusRecord,
        conflict: &ConflictData,
    ) -> Result<EnhancedConflictResolutionResult> {
        let start_time = Instant::now();
        
        info!("Starting intelligent conflict resolution for record: {} in table: {}", 
              record.record_id, record.table_name);

        // 1. 进行详细的冲突分析
        let enhanced_conflict = self.analyze_conflict_details(record, conflict).await?;

        // 2. 根据冲突严重级别选择策略
        let resolution_strategy = self.determine_resolution_strategy(&enhanced_conflict);

        // 3. 执行智能合并
        let merged_content = self.execute_intelligent_merge(&enhanced_conflict, resolution_strategy.clone()).await?;

        // 4. 验证合并结果
        self.validate_merge_result(&enhanced_conflict, &merged_content).await?;

        let duration = start_time.elapsed();
        
        // 记录同步操作日志
        self.structured_logger.log_sync_operation(
            "conflict_resolved_intelligently",
            1,
            1,
            0,
            duration.as_millis() as u64,
        );

        info!("Intelligent conflict resolution completed in {:?} for record: {}", 
              duration, record.record_id);

        let resolution_confidence = self.calculate_resolution_confidence(&enhanced_conflict, resolution_strategy.clone());

        Ok(EnhancedConflictResolutionResult {
            strategy: resolution_strategy,
            resolved_content: merged_content,
            enhanced_conflict,
            resolution_confidence,
            applied_field_resolutions: Vec::new(), // 将在执行过程中填充
            resolved_by: "INTELLIGENT_RESOLVER".to_string(),
        })
    }

    /// 分析冲突详情
    pub async fn analyze_conflict_details(
        &self,
        record: &SyncStatusRecord,
        conflict: &ConflictData,
    ) -> Result<EnhancedConflictData> {
        info!("Analyzing conflict details for {}.{}", record.table_name, record.record_id);

        // 解析本地和远程内容为JSON
        let local_json: serde_json::Value = serde_json::from_str(&conflict.local_content)
            .map_err(|e| anyhow!("Failed to parse local content as JSON: {}", e))?;
        let remote_json: serde_json::Value = serde_json::from_str(&conflict.remote_content)
            .map_err(|e| anyhow!("Failed to parse remote content as JSON: {}", e))?;

        // 检测字段级冲突
        let field_conflicts = self.detect_field_conflicts(&local_json, &remote_json)?;

        // 计算冲突严重级别
        let severity = self.calculate_conflict_severity(&field_conflicts, &record.table_name);

        // 生成建议策略
        let suggested_strategy = self.suggest_resolution_strategy(&field_conflicts, &record.table_name, severity.clone());

        // 创建上下文信息
        let context = ConflictContext {
            table_name: record.table_name.clone(),
            record_id: record.record_id.clone(),
            operation: record.operation.clone(),
            last_modified_by: self.extract_last_modified_by(&local_json, &remote_json),
            time_diff_seconds: self.calculate_time_difference(&local_json, &remote_json),
            foreign_keys: self.extract_foreign_keys(&record.table_name, &local_json, &remote_json),
        };

        Ok(EnhancedConflictData {
            base_conflict: conflict.clone(),
            field_conflicts,
            severity,
            suggested_strategy,
            context,
        })
    }

    /// 检测字段级冲突
    fn detect_field_conflicts(
        &self,
        local_json: &serde_json::Value,
        remote_json: &serde_json::Value,
    ) -> Result<Vec<FieldConflict>> {
        let mut field_conflicts = Vec::new();

        // 获取两个JSON对象的所有字段
        let local_obj = local_json.as_object().ok_or_else(|| anyhow!("Local content is not a JSON object"))?;
        let remote_obj = remote_json.as_object().ok_or_else(|| anyhow!("Remote content is not a JSON object"))?;

        // 收集所有字段名
        let mut all_fields: std::collections::HashSet<String> = local_obj.keys().cloned().collect();
        all_fields.extend(remote_obj.keys().cloned());

        // 检查每个字段的冲突
        for field_name in all_fields {
            let local_value = local_obj.get(&field_name).cloned().unwrap_or(serde_json::Value::Null);
            let remote_value = remote_obj.get(&field_name).cloned().unwrap_or(serde_json::Value::Null);

            if local_value != remote_value {
                let conflict_type = self.determine_field_conflict_type(&local_value, &remote_value);
                let suggested_resolution = self.suggest_field_resolution(&field_name, &local_value, &remote_value, conflict_type.clone());

                field_conflicts.push(FieldConflict {
                    field_name: field_name.clone(),
                    local_value,
                    remote_value,
                    conflict_type,
                    suggested_resolution,
                });
            }
        }

        Ok(field_conflicts)
    }

    /// 确定字段冲突类型
    fn determine_field_conflict_type(
        &self,
        local_value: &serde_json::Value,
        remote_value: &serde_json::Value,
    ) -> FieldConflictType {
        match (local_value, remote_value) {
            (serde_json::Value::Null, serde_json::Value::Null) => FieldConflictType::ValueDifference,
            (serde_json::Value::Null, _) | (_, serde_json::Value::Null) => FieldConflictType::NullConflict,
            (local, remote) if std::mem::discriminant(local) != std::mem::discriminant(remote) => {
                FieldConflictType::TypeMismatch
            }
            _ => FieldConflictType::ValueDifference,
        }
    }

    /// 建议字段解决方案
    fn suggest_field_resolution(
        &self,
        field_name: &str,
        local_value: &serde_json::Value,
        remote_value: &serde_json::Value,
        conflict_type: FieldConflictType,
    ) -> FieldResolution {
        // 查找字段特定的配置
        let field_config = self.field_priorities.get(field_name);

        let (strategy, resolved_value, confidence) = match field_config {
            Some(config) => {
                let (value, conf) = self.apply_field_merge_strategy(&config.merge_strategy, local_value, remote_value);
                (config.merge_strategy.clone(), value, conf)
            }
            None => {
                // 使用默认策略
                let strategy = self.get_default_field_strategy(field_name, conflict_type);
                let (value, conf) = self.apply_field_merge_strategy(&strategy, local_value, remote_value);
                (strategy, value, conf)
            }
        };

        FieldResolution {
            strategy,
            resolved_value,
            confidence,
        }
    }

    /// 应用字段合并策略
    fn apply_field_merge_strategy(
        &self,
        strategy: &FieldMergeStrategy,
        local_value: &serde_json::Value,
        remote_value: &serde_json::Value,
    ) -> (serde_json::Value, u8) {
        match strategy {
            FieldMergeStrategy::LocalWins => (local_value.clone(), 85),
            FieldMergeStrategy::RemoteWins => (remote_value.clone(), 85),
            FieldMergeStrategy::NewerWins => {
                // 尝试解析时间戳字段
                self.resolve_newer_wins(local_value, remote_value)
            }
            FieldMergeStrategy::LongerWins => {
                self.resolve_longer_wins(local_value, remote_value)
            }
            FieldMergeStrategy::MergeLists => {
                self.resolve_merge_lists(local_value, remote_value)
            }
            FieldMergeStrategy::UserChoice => {
                // 需要用户选择，暂时使用本地值
                (local_value.clone(), 50)
            }
            FieldMergeStrategy::Custom => {
                // 自定义逻辑，暂时使用智能选择
                self.resolve_smart_choice(local_value, remote_value)
            }
        }
    }

    /// 解决"较新获胜"策略
    fn resolve_newer_wins(
        &self,
        local_value: &serde_json::Value,
        remote_value: &serde_json::Value,
    ) -> (serde_json::Value, u8) {
        // 尝试解析为时间戳
        if let (Some(local_ts), Some(remote_ts)) = (
            local_value.as_i64(),
            remote_value.as_i64(),
        ) {
            if local_ts > remote_ts {
                (local_value.clone(), 90)
            } else {
                (remote_value.clone(), 90)
            }
        } else {
            // 无法解析为时间戳，使用本地值
            (local_value.clone(), 60)
        }
    }

    /// 解决"较长获胜"策略
    fn resolve_longer_wins(
        &self,
        local_value: &serde_json::Value,
        remote_value: &serde_json::Value,
    ) -> (serde_json::Value, u8) {
        let local_len = self.get_value_length(local_value);
        let remote_len = self.get_value_length(remote_value);

        if local_len > remote_len {
            (local_value.clone(), 80)
        } else if remote_len > local_len {
            (remote_value.clone(), 80)
        } else {
            // 长度相同，使用本地值
            (local_value.clone(), 70)
        }
    }

    /// 解决"合并列表"策略
    fn resolve_merge_lists(
        &self,
        local_value: &serde_json::Value,
        remote_value: &serde_json::Value,
    ) -> (serde_json::Value, u8) {
        match (local_value, remote_value) {
            (serde_json::Value::Array(local_arr), serde_json::Value::Array(remote_arr)) => {
                let mut merged = local_arr.clone();
                for item in remote_arr {
                    if !merged.contains(item) {
                        merged.push(item.clone());
                    }
                }
                (serde_json::Value::Array(merged), 85)
            }
            _ => {
                // 不是数组，无法合并，使用本地值
                (local_value.clone(), 50)
            }
        }
    }

    /// 智能选择策略
    fn resolve_smart_choice(
        &self,
        local_value: &serde_json::Value,
        remote_value: &serde_json::Value,
    ) -> (serde_json::Value, u8) {
        // 实现智能选择逻辑
        match (local_value, remote_value) {
            (serde_json::Value::Null, remote) => (remote.clone(), 80),
            (local, serde_json::Value::Null) => (local.clone(), 80),
            (local, remote) => {
                // 比较值的复杂度，选择更丰富的内容
                let local_complexity = self.calculate_value_complexity(local);
                let remote_complexity = self.calculate_value_complexity(remote);
                
                if local_complexity > remote_complexity {
                    (local.clone(), 75)
                } else {
                    (remote.clone(), 75)
                }
            }
        }
    }

    /// 获取值的长度
    fn get_value_length(&self, value: &serde_json::Value) -> usize {
        match value {
            serde_json::Value::String(s) => s.len(),
            serde_json::Value::Array(arr) => arr.len(),
            serde_json::Value::Object(obj) => obj.len(),
            _ => 0,
        }
    }

    /// 计算值的复杂度
    fn calculate_value_complexity(&self, value: &serde_json::Value) -> usize {
        match value {
            serde_json::Value::Null => 0,
            serde_json::Value::Bool(_) => 1,
            serde_json::Value::Number(_) => 1,
            serde_json::Value::String(s) => s.len(),
            serde_json::Value::Array(arr) => {
                arr.iter().map(|v| self.calculate_value_complexity(v)).sum::<usize>() + arr.len()
            }
            serde_json::Value::Object(obj) => {
                obj.values().map(|v| self.calculate_value_complexity(v)).sum::<usize>() + obj.len() * 2
            }
        }
    }

    /// 获取默认字段策略
    fn get_default_field_strategy(&self, field_name: &str, conflict_type: FieldConflictType) -> FieldMergeStrategy {
        match field_name {
            "id" | "created_at" => FieldMergeStrategy::LocalWins, // ID和创建时间保持本地
            "updated_at" | "modified_at" => FieldMergeStrategy::NewerWins, // 更新时间使用较新的
            "title" | "name" | "content" | "description" => FieldMergeStrategy::LongerWins, // 文本字段使用较长的
            "tags" | "categories" => FieldMergeStrategy::MergeLists, // 列表字段合并
            _ => match conflict_type {
                FieldConflictType::NullConflict => FieldMergeStrategy::RemoteWins, // 优先填充空值
                _ => FieldMergeStrategy::NewerWins, // 默认使用较新的
            }
        }
    }

    /// 计算冲突严重级别
    fn calculate_conflict_severity(&self, field_conflicts: &[FieldConflict], table_name: &str) -> ConflictSeverity {
        let critical_conflicts = field_conflicts.iter().filter(|fc| {
            let field_key = format!("{}.{}", table_name, fc.field_name);
            self.field_priorities.get(&field_key)
                .map(|config| config.is_critical)
                .unwrap_or(false)
        }).count();

        let total_conflicts = field_conflicts.len();

        if critical_conflicts > 0 {
            ConflictSeverity::High
        } else if total_conflicts > 3 {
            ConflictSeverity::Medium
        } else {
            ConflictSeverity::Low
        }
    }

    /// 建议解决策略
    fn suggest_resolution_strategy(&self, field_conflicts: &[FieldConflict], table_name: &str, severity: ConflictSeverity) -> ConflictResolutionStrategy {
        match severity {
            ConflictSeverity::Critical => ConflictResolutionStrategy::UserChoice,
            ConflictSeverity::High => {
                // 检查是否有表级别的特定策略
                self.table_strategies.get(table_name)
                    .cloned()
                    .unwrap_or(ConflictResolutionStrategy::UserChoice)
            }
            ConflictSeverity::Medium | ConflictSeverity::Low => {
                // 检查字段冲突的类型，决定是否可以自动合并
                let can_auto_merge = field_conflicts.iter().all(|fc| {
                    matches!(fc.conflict_type, FieldConflictType::ValueDifference | FieldConflictType::NullConflict)
                });

                if can_auto_merge {
                    ConflictResolutionStrategy::Merge
                } else {
                    self.table_strategies.get(table_name)
                        .cloned()
                        .unwrap_or(self.default_strategy.clone())
                }
            }
        }
    }

    /// 确定解决策略
    fn determine_resolution_strategy(&self, enhanced_conflict: &EnhancedConflictData) -> ConflictResolutionStrategy {
        // 如果建议的策略是合并且严重级别允许，使用合并策略
        match (enhanced_conflict.suggested_strategy.clone(), enhanced_conflict.severity.clone()) {
            (ConflictResolutionStrategy::Merge, ConflictSeverity::Low | ConflictSeverity::Medium) => {
                ConflictResolutionStrategy::Merge
            }
            (strategy, ConflictSeverity::High | ConflictSeverity::Critical) => {
                // 高严重级别的冲突，可能需要用户干预
                strategy
            }
            (strategy, _) => strategy,
        }
    }

    /// 执行智能合并
    async fn execute_intelligent_merge(
        &self,
        enhanced_conflict: &EnhancedConflictData,
        strategy: ConflictResolutionStrategy,
    ) -> Result<String> {
        match strategy {
            ConflictResolutionStrategy::Merge => {
                self.execute_field_level_merge(enhanced_conflict).await
            }
            ConflictResolutionStrategy::LocalWins => {
                Ok(enhanced_conflict.base_conflict.local_content.clone())
            }
            ConflictResolutionStrategy::RemoteWins => {
                Ok(enhanced_conflict.base_conflict.remote_content.clone())
            }
            ConflictResolutionStrategy::UserChoice => {
                // 用户选择策略，暂时返回错误，需要UI处理
                Err(anyhow!("User choice required for conflict resolution"))
            }
        }
    }

    /// 执行字段级合并
    async fn execute_field_level_merge(&self, enhanced_conflict: &EnhancedConflictData) -> Result<String> {
        let mut local_json: serde_json::Value = serde_json::from_str(&enhanced_conflict.base_conflict.local_content)?;
        let remote_json: serde_json::Value = serde_json::from_str(&enhanced_conflict.base_conflict.remote_content)?;

        let local_obj = local_json.as_object_mut().ok_or_else(|| anyhow!("Local content is not a JSON object"))?;

        // 应用每个字段的解决方案
        for field_conflict in &enhanced_conflict.field_conflicts {
            local_obj.insert(
                field_conflict.field_name.clone(),
                field_conflict.suggested_resolution.resolved_value.clone(),
            );
        }

        // 确保所有远程字段都被考虑
        if let Some(remote_obj) = remote_json.as_object() {
            for (key, value) in remote_obj {
                if !local_obj.contains_key(key) {
                    // 远程有但本地没有的字段，直接添加
                    local_obj.insert(key.clone(), value.clone());
                }
            }
        }

        Ok(serde_json::to_string(local_obj)?)
    }

    /// 验证合并结果
    async fn validate_merge_result(&self, enhanced_conflict: &EnhancedConflictData, merged_content: &str) -> Result<()> {
        // 验证合并结果是否为有效的JSON
        let _: serde_json::Value = serde_json::from_str(merged_content)
            .map_err(|e| anyhow!("Merge result is not valid JSON: {}", e))?;

        // 根据表类型进行特定验证
        self.validate_table_specific_constraints(&enhanced_conflict.context.table_name, merged_content).await?;

        Ok(())
    }

    /// 验证表特定约束
    async fn validate_table_specific_constraints(&self, table_name: &str, content: &str) -> Result<()> {
        match table_name {
            "tips" => {
                let tip: Tip = serde_json::from_str(content)
                    .map_err(|e| anyhow!("Merged tip content is invalid: {}", e))?;
                
                if tip.title.trim().is_empty() {
                    return Err(anyhow!("Tip title cannot be empty after merge"));
                }
                
                if tip.content.trim().is_empty() {
                    return Err(anyhow!("Tip content cannot be empty after merge"));
                }
            }
            "categories" => {
                let category: Category = serde_json::from_str(content)
                    .map_err(|e| anyhow!("Merged category content is invalid: {}", e))?;
                
                if category.name.trim().is_empty() {
                    return Err(anyhow!("Category name cannot be empty after merge"));
                }
            }
            "tags" => {
                let tag: Tag = serde_json::from_str(content)
                    .map_err(|e| anyhow!("Merged tag content is invalid: {}", e))?;
                
                if tag.name.trim().is_empty() {
                    return Err(anyhow!("Tag name cannot be empty after merge"));
                }
            }
            _ => {
                // 其他表的通用验证
                debug!("No specific validation rules for table: {}", table_name);
            }
        }

        Ok(())
    }

    /// 计算解决方案置信度
    fn calculate_resolution_confidence(&self, enhanced_conflict: &EnhancedConflictData, strategy: ConflictResolutionStrategy) -> u8 {
        match strategy {
            ConflictResolutionStrategy::Merge => {
                // 计算所有字段解决方案的平均置信度
                if enhanced_conflict.field_conflicts.is_empty() {
                    80
                } else {
                    let total_confidence: u32 = enhanced_conflict.field_conflicts
                        .iter()
                        .map(|fc| fc.suggested_resolution.confidence as u32)
                        .sum();
                    (total_confidence / enhanced_conflict.field_conflicts.len() as u32) as u8
                }
            }
            ConflictResolutionStrategy::LocalWins | ConflictResolutionStrategy::RemoteWins => 90,
            ConflictResolutionStrategy::UserChoice => 100, // 用户选择总是最高置信度
        }
    }

    /// 提取最后修改用户
    fn extract_last_modified_by(&self, local_json: &serde_json::Value, remote_json: &serde_json::Value) -> Option<String> {
        // 尝试从两个版本中提取 "modified_by" 或 "updated_by" 字段
        local_json.get("modified_by")
            .or_else(|| local_json.get("updated_by"))
            .or_else(|| remote_json.get("modified_by"))
            .or_else(|| remote_json.get("updated_by"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    /// 计算时间差异
    fn calculate_time_difference(&self, local_json: &serde_json::Value, remote_json: &serde_json::Value) -> i64 {
        let local_time = local_json.get("updated_at")
            .or_else(|| local_json.get("modified_at"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0);

        let remote_time = remote_json.get("updated_at")
            .or_else(|| remote_json.get("modified_at"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0);

        (local_time - remote_time).abs() / 1000 // 转换为秒
    }

    /// 提取外键信息
    fn extract_foreign_keys(&self, table_name: &str, local_json: &serde_json::Value, remote_json: &serde_json::Value) -> Vec<String> {
        let mut foreign_keys = Vec::new();

        match table_name {
            "tips" => {
                if let Some(category_id) = local_json.get("category_id").or_else(|| remote_json.get("category_id")) {
                    if let Some(id) = category_id.as_str() {
                        foreign_keys.push(format!("categories.{}", id));
                    }
                }
            }
            "tip_tags" => {
                if let Some(tip_id) = local_json.get("tip_id").or_else(|| remote_json.get("tip_id")) {
                    if let Some(id) = tip_id.as_str() {
                        foreign_keys.push(format!("tips.{}", id));
                    }
                }
                if let Some(tag_id) = local_json.get("tag_id").or_else(|| remote_json.get("tag_id")) {
                    if let Some(id) = tag_id.as_str() {
                        foreign_keys.push(format!("tags.{}", id));
                    }
                }
            }
            _ => {}
        }

        foreign_keys
    }
} 