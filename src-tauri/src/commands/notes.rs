use crate::settings::{get_settings, write_settings};
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;

/// The notes folder currently in effect (the user's choice, else the default).
#[tauri::command]
#[specta::specta]
pub fn get_notes_dir(app: AppHandle) -> Result<String, String> {
    Ok(crate::notes::notes_dir(&app)?.to_string_lossy().to_string())
}

/// Set the notes folder. `None` (or an empty path) restores the default.
#[tauri::command]
#[specta::specta]
pub fn change_notes_dir_setting(app: AppHandle, path: Option<String>) -> Result<(), String> {
    let mut settings = get_settings(&app);
    settings.notes_dir = path.map(|p| p.trim().to_string()).filter(|p| !p.is_empty());
    write_settings(&app, settings);
    Ok(())
}

/// Open the notes folder in the system file manager, creating it if needed.
#[tauri::command]
#[specta::specta]
pub fn open_notes_dir(app: AppHandle) -> Result<(), String> {
    let dir = crate::notes::notes_dir(&app)?;
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Failed to create notes directory: {}", e))?;
    app.opener()
        .open_path(dir.to_string_lossy().to_string(), None::<String>)
        .map_err(|e| format!("Failed to open notes directory: {}", e))
}
