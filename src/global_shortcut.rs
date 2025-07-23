#![cfg(not(any(target_os = "android", target_os = "ios")))]
use crate::db::models::ShortcutHandler;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut}; 