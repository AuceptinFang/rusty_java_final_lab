use crate::server::*;

pub mod seat;
pub mod server;
pub mod log;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            use tracing_subscriber::prelude::*;
            let logger_layer = log::FrontendLogger{
                app: app.handle().clone(),
            };

            tracing_subscriber::registry()
                .with(tracing_subscriber::fmt::layer()) // 保持终端也能看到
                .with(logger_layer)                     // 新增前端转发
                .init();

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![server::server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
