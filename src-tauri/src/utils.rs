use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;

// Sistemdeki veri klasörünü bulur
pub fn app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let yol = app
        .path()
        .app_local_data_dir()
        .map_err(|_| "Sistem klasörüne erişilemiyor!".to_string())?;

    if !yol.exists() {
        fs::create_dir_all(&yol).map_err(|_| "Veri klasörü oluşturulamadı!".to_string())?;
    }
    Ok(yol)
}

// Şarkıların kaydedileceği klasörü bulur
pub fn songs_klasoru_bul(app: &AppHandle) -> Result<PathBuf, String> {
    let mut yol = app_data_dir(app)?;
    yol.push("songs");
    if !yol.exists() {
        fs::create_dir_all(&yol).map_err(|_| "Şarkı klasörü oluşturulamadı!".to_string())?;
    }
    Ok(yol)
}
