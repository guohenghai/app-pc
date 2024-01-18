/*
 * @Author       : 郭恒海
 * @Date         : 2024-01-17 14:01:18
 * @LastEditors  : 郭恒海
 * @LastEditTime : 2024-01-18 17:13:06
 * @FilePath     : /app-pc/src-tauri/src/main.rs
 * @Description  :
 * guohenghai@126.com
 */
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .build(tauri::generate_context!()) //notice build instead of run
        .expect("error preparing tauri app");
    app.run(|_app_handle, event| {
        if let tauri::RunEvent::Updater(event) = event {
            dbg!(event);
        }
    });
}
