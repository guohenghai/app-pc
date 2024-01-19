// 引入日志相关的库
use log::{info, warn};

// 这是一个外部属性，应用于 main 函数
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[tauri::command]
fn greet(name: &str) -> String {
    info!("greet 命令被调用，名字为：{}", name); // 使用 info! 宏记录信息级别的日志
    format!("你好，{}！你已经被 Rust 问候！", name)
}

fn main() {
    // 初始化 env_logger
    env_logger::init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
