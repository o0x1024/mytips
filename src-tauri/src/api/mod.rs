// API modules
pub mod ai;
pub mod audio;
pub mod categories;
pub mod clipboard_api;
pub mod database;
pub mod database_manager;
pub mod encryption;
pub mod export;
pub mod import;
pub mod settings;
pub mod shortcuts;
pub mod tags;
pub mod templates;
pub mod tips;
pub mod updater;
pub mod certificates;

// Re-export commonly used functions from database_manager
pub use database_manager::{
    switch_database_mode, get_database_status, sync_database, test_database_connection,
    switch_to_local_mode, switch_to_remote_mode, switch_to_embedded_replica_mode,
    get_supported_database_modes, DatabaseModeManager, get_legacy_connection,
};

// 重新导出所有公共API函数，方便在main.rs中注册
pub use ai::*;
pub use audio::*;
pub use categories::*;
pub use clipboard_api::*;
pub use database::*;
pub use export::*;
pub use import::*;
pub use settings::*;
pub use shortcuts::*;
pub use tags::*;
pub use tips::*;
pub use updater::*;
pub use templates::*;
