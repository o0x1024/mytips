mod sync;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod global_shortcut;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::api::database_manager::DatabaseManager;
use crate::api::settings::Settings;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use ta