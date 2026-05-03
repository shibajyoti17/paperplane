// IPC command handlers exposed to the frontend via Tauri's invoke bridge.
// Each function tagged with #[tauri::command] becomes callable from JS/TS.

/// Health-check command — verifies IPC bridge is working.
#[tauri::command]
pub fn ping() -> &'static str {
    "pong"
}
