/*
 * @Author       : 郭恒海
 * @Date         : 2024-01-17 14:01:18
 * @LastEditors  : 郭恒海
 * @LastEditTime : 2024-01-23 11:22:18
 * @FilePath     : /app-pc/src-tauri/src/main.rs
 * @Description  :
 * guohenghai@126.com
 */
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
    env_logger::init();

    // 生产版本
    //tauri::Builder::default()
    //    .invoke_handler(tauri::generate_handler![greet])
    //    .run(tauri::generate_context!())
    //    .expect("error while running tauri application");

    // 升级可调试版本
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!()) //notice build instead of run
        .expect("error preparing tauri app");

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::Ready => println!("Application is ready"),
        tauri::RunEvent::Resumed => println!("Application is resumed"),
        tauri::RunEvent::ExitRequested { .. } => println!("Exit was requested"),
        tauri::RunEvent::Updater(event) => {
            println!("Updater event: {:?}", event);
            if let tauri::UpdaterEvent::Error(error) = event {
                println!("Updater error details: {:?}", error);
            }
        }
        _ => {}
    });
}
