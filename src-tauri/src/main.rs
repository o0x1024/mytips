// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "macos")]
    std::env::set_var("OS_ACTIVITY_MODE", "disable");
    
   let _ = mytips_lib::run();
}
