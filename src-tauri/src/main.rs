// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // 原来的代码
    //tauri::Builder::default()
    //    .invoke_handler(tauri::generate_handler![greet])
    //    .run(tauri::generate_context!())
    //    .expect("error while running tauri application");

    // 新的带更新调试的代码
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .build(tauri::generate_context!()) //notice build instead of run
        .expect("error preparing tauri app");
    //app.run(|_app_handle, event| {
    //    if let tauri::RunEvent::Updater(event) = event {
    //        dbg!(event);
    //    }
    //});

    // 增强版本
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
