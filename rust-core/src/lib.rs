pub mod commands;
pub mod models;

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::ping,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Paperplane");
}
