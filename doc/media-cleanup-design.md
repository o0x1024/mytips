# 媒体文件清理机制设计

## 问题分析

当前系统在删除笔记中的图片和音频内容时存在以下问题：

1. **删除笔记时的不完整清理**：
   - ✅ 删除图片：`DELETE FROM tip_images WHERE tip_id = ?1`
   - ❌ 缺少音频文件清理：未删除 `tip_audio_files` 表中的记录
   - ❌ 缺少孤儿文件检测：没有检查是否有未被引用的媒体文件

2. **编辑笔记内容时的不完整清理**：
   - 用户删除笔记中的图片/音频引用时，数据库中的文件记录仍然存在
   - 没有自动检测和清理不再被引用的媒体文件

3. **数据一致性问题**：
   - 媒体文件占用存储空间但不再被使用
   - 可能导致数据库膨胀

## 解决方案设计

### 1. 完善删除笔记时的清理逻辑

修改 `delete_tip_with_dependencies` 函数，添加音频文件清理：

```rust
async fn delete_tip_with_dependencies(conn: &DbConnection, tip_id: &str) -> Result<()> {
    // 1. 查询并记录要删除的媒体文件数量
    let image_count: i64 = conn.query(
        "SELECT COUNT(*) FROM tip_images WHERE tip_id = ?1",
        params![tip_id]
    ).await?.next().await?.unwrap().get(0)?;
    
    let audio_count: i64 = conn.query(
        "SELECT COUNT(*) FROM tip_audio_files WHERE tip_id = ?1",
        params![tip_id]
    ).await?.next().await?.unwrap().get(0)?;
    
    println!("Deleting tip {} with {} images and {} audio files", tip_id, image_count, audio_count);
    
    // 2. 删除相关的图片
    let deleted_images = conn.execute(
        "DELETE FROM tip_images WHERE tip_id = ?1",
        params![tip_id]
    ).await?;
    
    // 3. 删除相关的音频文件
    let deleted_audio = conn.execute(
        "DELETE FROM tip_audio_files WHERE tip_id = ?1",
        params![tip_id]
    ).await?;
    
    // 4. 删除相关的标签关联
    let deleted_tags = conn.execute(
        "DELETE FROM tip_tags WHERE tip_id = ?1", 
        params![tip_id]
    ).await?;
    
    // 5. 删除笔记本身
    let rows_affected = conn.execute(
        "DELETE FROM tips WHERE id = ?1", 
        params![tip_id]
    ).await?;
    
    println!("Successfully deleted tip {} with {} images, {} audio files, and {} tag associations", 
             tip_id, deleted_images, deleted_audio, deleted_tags);
    
    Ok(())
}
```

### 2. 实现孤儿媒体文件清理机制

#### 2.1 检测孤儿文件的函数

```rust
/// 检测并清理孤儿图片文件
pub async fn cleanup_orphaned_images(conn: &DbConnection) -> Result<u64> {
    // 查找在tip_images表中但不在任何笔记内容中被引用的图片
    let mut rows = conn.query(
        "SELECT ti.image_id, ti.tip_id 
         FROM tip_images ti 
         JOIN tips t ON ti.tip_id = t.id 
         WHERE t.content NOT LIKE '%' || ti.image_id || '%'",
        ()
    ).await?;
    
    let mut orphaned_count = 0;
    while let Some(row) = rows.next().await? {
        let image_id: String = row.get(0)?;
        let tip_id: String = row.get(1)?;
        
        // 删除孤儿图片
        conn.execute(
            "DELETE FROM tip_images WHERE image_id = ?1",
            params![image_id]
        ).await?;
        
        orphaned_count += 1;
        println!("Cleaned up orphaned image {} from tip {}", image_id, tip_id);
    }
    
    Ok(orphaned_count)
}

/// 检测并清理孤儿音频文件
pub async fn cleanup_orphaned_audio_files(conn: &DbConnection) -> Result<u64> {
    // 查找在tip_audio_files表中但不在任何笔记内容中被引用的音频文件
    let mut rows = conn.query(
        "SELECT taf.audio_id, taf.tip_id 
         FROM tip_audio_files taf 
         JOIN tips t ON taf.tip_id = t.id 
         WHERE t.content NOT LIKE '%' || taf.audio_id || '%'",
        ()
    ).await?;
    
    let mut orphaned_count = 0;
    while let Some(row) = rows.next().await? {
        let audio_id: String = row.get(0)?;
        let tip_id: String = row.get(1)?;
        
        // 删除孤儿音频文件
        conn.execute(
            "DELETE FROM tip_audio_files WHERE audio_id = ?1",
            params![audio_id]
        ).await?;
        
        orphaned_count += 1;
        println!("Cleaned up orphaned audio file {} from tip {}", audio_id, tip_id);
    }
    
    Ok(orphaned_count)
}

/// 综合清理函数
pub async fn cleanup_orphaned_media_files(conn: &DbConnection) -> Result<(u64, u64)> {
    let orphaned_images = cleanup_orphaned_images(conn).await?;
    let orphaned_audio = cleanup_orphaned_audio_files(conn).await?;
    
    println!("Media cleanup completed: {} orphaned images, {} orphaned audio files removed", 
             orphaned_images, orphaned_audio);
    
    Ok((orphaned_images, orphaned_audio))
}
```

#### 2.2 定期清理机制

```rust
/// 定期清理任务
pub async fn schedule_media_cleanup(db_manager: &UnifiedDbManager) -> Result<()> {
    let conn = db_manager.get_conn().await?;
    
    // 可以在以下时机触发清理：
    // 1. 应用启动时
    // 2. 定期任务（如每天一次）
    // 3. 手动触发
    
    let (images_cleaned, audio_cleaned) = cleanup_orphaned_media_files(&conn).await?;
    
    if images_cleaned > 0 || audio_cleaned > 0 {
        println!("Scheduled cleanup completed: {} images, {} audio files removed", 
                 images_cleaned, audio_cleaned);
    }
    
    Ok(())
}
```

### 3. 前端触发清理的接口

#### 3.1 Tauri命令

```rust
#[tauri::command]
pub async fn cleanup_orphaned_media(
    state: tauri::State<'_, AppState>,
) -> Result<(u64, u64), String> {
    let db_manager = &state.db_manager;
    let conn = db_manager.get_conn().await
        .map_err(|e| format!("Database connection failed: {}", e))?;
    
    cleanup_orphaned_media_files(&conn).await
        .map_err(|e| format!("Media cleanup failed: {}", e))
}

#[tauri::command]
pub async fn get_media_statistics(
    state: tauri::State<'_, AppState>,
) -> Result<MediaStatistics, String> {
    let db_manager = &state.db_manager;
    let conn = db_manager.get_conn().await
        .map_err(|e| format!("Database connection failed: {}", e))?;
    
    // 统计媒体文件信息
    let total_images: i64 = conn.query(
        "SELECT COUNT(*) FROM tip_images", ()
    ).await.map_err(|e| format!("Query failed: {}", e))?
    .next().await.map_err(|e| format!("Row read failed: {}", e))?
    .unwrap().get(0).map_err(|e| format!("Value get failed: {}", e))?;
    
    let total_audio: i64 = conn.query(
        "SELECT COUNT(*) FROM tip_audio_files", ()
    ).await.map_err(|e| format!("Query failed: {}", e))?
    .next().await.map_err(|e| format!("Row read failed: {}", e))?
    .unwrap().get(0).map_err(|e| format!("Value get failed: {}", e))?;
    
    // 计算可能的孤儿文件数量
    let orphaned_images: i64 = conn.query(
        "SELECT COUNT(*) FROM tip_images ti 
         JOIN tips t ON ti.tip_id = t.id 
         WHERE t.content NOT LIKE '%' || ti.image_id || '%'", ()
    ).await.map_err(|e| format!("Query failed: {}", e))?
    .next().await.map_err(|e| format!("Row read failed: {}", e))?
    .unwrap().get(0).map_err(|e| format!("Value get failed: {}", e))?;
    
    let orphaned_audio: i64 = conn.query(
        "SELECT COUNT(*) FROM tip_audio_files taf 
         JOIN tips t ON taf.tip_id = t.id 
         WHERE t.content NOT LIKE '%' || taf.audio_id || '%'", ()
    ).await.map_err(|e| format!("Query failed: {}", e))?
    .next().await.map_err(|e| format!("Row read failed: {}", e))?
    .unwrap().get(0).map_err(|e| format!("Value get failed: {}", e))?;
    
    Ok(MediaStatistics {
        total_images: total_images as u64,
        total_audio: total_audio as u64,
        orphaned_images: orphaned_images as u64,
        orphaned_audio: orphaned_audio as u64,
    })
}
```

#### 3.2 前端设置页面

在设置页面添加媒体文件管理功能：

```vue
<template>
  <div class="media-management">
    <h3>媒体文件管理</h3>
    
    <div class="stats-section">
      <h4>存储统计</h4>
      <div class="stats-grid">
        <div class="stat-item">
          <span class="label">图片文件：</span>
          <span class="value">{{ mediaStats.total_images }}</span>
        </div>
        <div class="stat-item">
          <span class="label">音频文件：</span>
          <span class="value">{{ mediaStats.total_audio }}</span>
        </div>
        <div class="stat-item warning" v-if="mediaStats.orphaned_images > 0">
          <span class="label">孤儿图片：</span>
          <span class="value">{{ mediaStats.orphaned_images }}</span>
        </div>
        <div class="stat-item warning" v-if="mediaStats.orphaned_audio > 0">
          <span class="label">孤儿音频：</span>
          <span class="value">{{ mediaStats.orphaned_audio }}</span>
        </div>
      </div>
    </div>
    
    <div class="actions-section">
      <button 
        class="btn btn-primary" 
        @click="refreshStats"
        :disabled="loading"
      >
        刷新统计
      </button>
      
      <button 
        class="btn btn-warning" 
        @click="cleanupOrphanedMedia"
        :disabled="loading || (mediaStats.orphaned_images === 0 && mediaStats.orphaned_audio === 0)"
      >
        清理孤儿文件
      </button>
    </div>
    
    <div v-if="lastCleanupResult" class="cleanup-result">
      <p>上次清理结果：删除了 {{ lastCleanupResult.images }} 个图片文件和 {{ lastCleanupResult.audio }} 个音频文件</p>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const mediaStats = ref({
  total_images: 0,
  total_audio: 0,
  orphaned_images: 0,
  orphaned_audio: 0
})

const loading = ref(false)
const lastCleanupResult = ref(null)

const refreshStats = async () => {
  loading.value = true
  try {
    mediaStats.value = await invoke('get_media_statistics')
  } catch (error) {
    console.error('获取媒体统计失败:', error)
  } finally {
    loading.value = false
  }
}

const cleanupOrphanedMedia = async () => {
  const confirmed = confirm('确定要清理孤儿媒体文件吗？此操作不可撤销。')
  if (!confirmed) return
  
  loading.value = true
  try {
    const result = await invoke('cleanup_orphaned_media')
    lastCleanupResult.value = {
      images: result[0],
      audio: result[1]
    }
    await refreshStats() // 刷新统计
  } catch (error) {
    console.error('清理孤儿文件失败:', error)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  refreshStats()
})
</script>
```

### 4. 实施计划

#### 阶段1：修复删除笔记时的清理逻辑
1. 修改 `delete_tip_with_dependencies` 函数
2. 添加音频文件删除逻辑
3. 测试删除笔记功能

#### 阶段2：实现孤儿文件检测和清理
1. 实现孤儿文件检测函数
2. 添加Tauri命令接口
3. 创建前端管理界面

#### 阶段3：添加自动清理机制
1. 在应用启动时执行清理
2. 添加定期清理任务
3. 在笔记保存时检测并清理孤儿文件

#### 阶段4：优化和监控
1. 添加清理日志
2. 性能优化
3. 错误处理改进

## 注意事项

1. **数据安全**：在清理前应该备份重要数据
2. **性能考虑**：大量媒体文件的清理可能耗时较长，需要考虑异步处理
3. **用户体验**：提供清理进度反馈和结果统计
4. **错误处理**：确保清理过程中的错误不会影响正常功能
5. **测试覆盖**：需要充分测试各种边界情况