pub mod commands;
pub mod models;

/// Initializes and runs the Tauri application.
/// Command handlers are registered here before the event loop starts.
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::ping,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Paperplane");
}
