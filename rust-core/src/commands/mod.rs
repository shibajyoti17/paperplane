/// Health-check command — verifies IPC bridge is working.
#[tauri::command]
pub fn ping() -> &'static str {
    "pong"
}
