use crate::api::database::{DatabaseInfo};
use crate::db::{DbManager, ConflictResolutionStrategy};
use crate::sync::{
    SyncManager, SyncStats, ConflictData, BatchConflictResolutionResult,
    EnhancedConflictResolutionResult, BuiltinSyncConfig, BuiltinSyncStatus, BuiltinSyncStats,
    IncrementalSyncConfig, HealthCheckResult
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager}; 