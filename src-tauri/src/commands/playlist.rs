use rusqlite::params;
use tauri::{AppHandle, Manager};

// Modelleri ve yardımcı fonksiyonları doğru konumlarından çağırıyoruz
use crate::db::DbState;
use crate::models::Playlist;
use crate::utils::app_data_dir;

/// Yeni bir çalma listesi oluşturur ve veritabanına kaydeder.
#[tauri::command]
pub async fn playlist_olustur(app: AppHandle, isim: String) -> Result<Playlist, String> {
    let db_arc = app.state::<DbState>().0.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;
        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM playlistler", [], |row| row.get(0))
            .unwrap_or(0);
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| "Zaman hatası!")?
            .as_nanos();
        let id = format!("pl_{}_{}", count + 1, timestamp);

        conn.execute(
            "INSERT INTO playlistler (id, isim) VALUES (?1, ?2)",
            params![id, isim],
        )
        .map_err(|e| format!("Playlist oluşturulamadı: {}", e))?;

        Ok(Playlist {
            id,
            isim,
            sarkilar: Vec::new(),
        })
    })
    .await
    .map_err(|e| format!("İşlem hatası: {}", e))?
}

/// Tüm çalma listelerini ve içerdikleri şarkıların ID'lerini getirir.
#[tauri::command]
pub async fn playlistleri_getir(app: AppHandle) -> Result<Vec<Playlist>, String> {
    let db_arc = app.state::<DbState>().0.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;
        let mut stmt = conn.prepare("SELECT id, isim FROM playlistler").map_err(|e| e.to_string())?;

        let pl_iter = stmt.query_map([], |row| {
            Ok(Playlist { id: row.get(0)?, isim: row.get(1)?, sarkilar: Vec::new() })
        }).map_err(|e| e.to_string())?;

        let mut listeler = Vec::new();
        for pl_res in pl_iter {
            if let Ok(mut pl) = pl_res {
                let mut stmt_sarki = conn.prepare("SELECT sarki_id FROM playlist_sarkilar WHERE playlist_id = ? ORDER BY sira ASC").map_err(|e| e.to_string())?;
                let sarkilar = stmt_sarki.query_map([&pl.id], |row| row.get::<_, String>(0)).map_err(|e| e.to_string())?;
                for s in sarkilar.flatten() {
                    pl.sarkilar.push(s);
                }
                listeler.push(pl);
            }
        }
        Ok(listeler)
    }).await.map_err(|e| format!("İşlem hatası: {}", e))?
}

/// Belirtilen çalma listesine yeni bir şarkı ekler.
#[tauri::command]
pub async fn playliste_sarki_ekle(
    app: AppHandle,
    playlist_id: String,
    sarki_id: String,
) -> Result<Playlist, String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;

        // Şarkı zaten listede var mı kontrol et
        let var_mi: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM playlist_sarkilar WHERE playlist_id = ? AND sarki_id = ?",
                params![playlist_id, sarki_id],
                |row| row.get(0),
            )
            .unwrap_or(0);

        if var_mi == 0 {
            // Şarkı yoksa en sona ekle
            let max_sira: i32 = conn.query_row(
                "SELECT COALESCE(MAX(sira), 0) + 1 FROM playlist_sarkilar WHERE playlist_id = ?",
                params![playlist_id],
                |row| row.get(0)
            ).unwrap_or(1);

            conn.execute(
                "INSERT INTO playlist_sarkilar (playlist_id, sarki_id, sira) VALUES (?1, ?2, ?3)",
                params![playlist_id, sarki_id, max_sira],
            )
            .map_err(|e| e.to_string())?;
        }

        // Güncellenmiş playlist bilgisini döndür
        let isim: String = conn
            .query_row(
                "SELECT isim FROM playlistler WHERE id = ?",
                params![playlist_id],
                |row| row.get(0),
            )
            .map_err(|_| "Playlist bulunamadı")?;

        let mut stmt = conn
            .prepare(
                "SELECT sarki_id FROM playlist_sarkilar WHERE playlist_id = ? ORDER BY sira ASC",
            )
            .map_err(|e| e.to_string())?;

        let sarkilar_iter = stmt
            .query_map([&playlist_id], |row| row.get(0))
            .map_err(|e| e.to_string())?;
        let mut sarkilar = Vec::new();
        for s in sarkilar_iter.flatten() {
            sarkilar.push(s);
        }

        Ok(Playlist {
            id: playlist_id,
            isim,
            sarkilar,
        })
    })
    .await
    .map_err(|e| format!("İşlem hatası: {}", e))?
}

/// Çalma listesinden belirtilen şarkıyı çıkarır.
#[tauri::command]
pub async fn playlistten_sarki_cikar(
    app: AppHandle,
    playlist_id: String,
    sarki_id: String,
) -> Result<Playlist, String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;

        conn.execute(
            "DELETE FROM playlist_sarkilar WHERE playlist_id = ? AND sarki_id = ?",
            params![playlist_id, sarki_id],
        )
        .map_err(|e| e.to_string())?;

        let isim: String = conn
            .query_row(
                "SELECT isim FROM playlistler WHERE id = ?",
                params![playlist_id],
                |row| row.get(0),
            )
            .map_err(|_| "Playlist bulunamadı")?;

        let mut stmt = conn
            .prepare(
                "SELECT sarki_id FROM playlist_sarkilar WHERE playlist_id = ? ORDER BY sira ASC",
            )
            .map_err(|e| e.to_string())?;

        let sarkilar_iter = stmt
            .query_map([&playlist_id], |row| row.get(0))
            .map_err(|e| e.to_string())?;
        let mut sarkilar = Vec::new();
        for s in sarkilar_iter.flatten() {
            sarkilar.push(s);
        }

        Ok(Playlist {
            id: playlist_id,
            isim,
            sarkilar,
        })
    })
    .await
    .map_err(|e| format!("İşlem hatası: {}", e))?
}

/// Çalma listesinin sırasını günceller.
#[tauri::command]
pub async fn playlist_sirasi_guncelle(
    app: AppHandle,
    playlist_id: String,
    yeni_sarki_siralari: Vec<String>,
) -> Result<(), String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let mut conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;
        let tx = conn.transaction().map_err(|e| e.to_string())?;

        tx.execute(
            "DELETE FROM playlist_sarkilar WHERE playlist_id = ?",
            params![playlist_id],
        )
        .map_err(|e| e.to_string())?;

        for (index, sarki_id) in yeni_sarki_siralari.iter().enumerate() {
            tx.execute(
                "INSERT INTO playlist_sarkilar (playlist_id, sarki_id, sira) VALUES (?1, ?2, ?3)",
                params![playlist_id, sarki_id, index as i32],
            )
            .ok();
        }

        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Çalma listesini tamamen siler.
#[tauri::command]
pub async fn playlist_sil(app: AppHandle, playlist_id: String) -> Result<(), String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let mut conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;
        let tx = conn.transaction().map_err(|e| e.to_string())?;

        tx.execute("DELETE FROM playlistler WHERE id = ?", params![playlist_id])
            .map_err(|e| e.to_string())?;
        tx.execute(
            "DELETE FROM playlist_sarkilar WHERE playlist_id = ?",
            params![playlist_id],
        )
        .map_err(|e| e.to_string())?;

        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    })
    .await
    .map_err(|e| format!("İşlem hatası: {}", e))?
}

/// Çalma listesini JSON dosyası olarak dışa aktarır.
#[tauri::command]
pub async fn playlist_disa_aktar(
    app: tauri::AppHandle,
    playlist_adi: String,
    icerik: String,
) -> Result<String, String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let app_dir = app_data_dir(&app)?;
        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;

        let kullanici_adi: String = conn
            .query_row(
                "SELECT kullanici_adi FROM ayarlar WHERE id = 1",
                [],
                |row| row.get(0),
            )
            .unwrap_or_else(|_| "Anonim".to_string());

        let guvenli_kullanici = if kullanici_adi.trim().is_empty() {
            "Anonim".to_string()
        } else {
            kullanici_adi.replace(&['/', '\\', ':', '*', '?', '"', '<', '>', '|'][..], "_")
        };
        let guvenli_isim =
            playlist_adi.replace(&['/', '\\', ':', '*', '?', '"', '<', '>', '|'][..], "_");

        let export_dir = app_dir.join("paylasima_uygun_playlistler");
        if !export_dir.exists() {
            std::fs::create_dir_all(&export_dir).map_err(|e| e.to_string())?;
        }

        let dosya_yolu = export_dir.join(format!("{} - {}.json", guvenli_kullanici, guvenli_isim));
        std::fs::write(&dosya_yolu, icerik).map_err(|e| e.to_string())?;

        Ok(dosya_yolu.to_string_lossy().to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Favori şarkıların ID'lerini bir liste olarak getirir.
#[tauri::command]
pub async fn favorileri_getir(app: AppHandle) -> Result<Vec<String>, String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;
        let mut stmt = conn
            .prepare("SELECT sarki_id FROM favoriler")
            .map_err(|e| e.to_string())?;

        let mut favoriler = Vec::new();
        let rows = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| e.to_string())?;

        for r in rows.flatten() {
            favoriler.push(r);
        }

        Ok(favoriler)
    })
    .await
    .map_err(|e| format!("İşlem hatası: {}", e))?
}

/// Bir şarkıyı favorilere ekler veya favorilerden çıkarır (Toggle mantığı).
#[tauri::command]
pub async fn favori_degistir(app: AppHandle, sarki_id: String) -> Result<Vec<String>, String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;

        let var_mi: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM favoriler WHERE sarki_id = ?",
                params![sarki_id],
                |row| row.get(0),
            )
            .unwrap_or(0);

        if var_mi > 0 {
            // Varsa sil
            conn.execute(
                "DELETE FROM favoriler WHERE sarki_id = ?",
                params![sarki_id],
            )
            .ok();
        } else {
            // Yoksa ekle
            conn.execute(
                "INSERT INTO favoriler (sarki_id) VALUES (?)",
                params![sarki_id],
            )
            .ok();
        }

        // Güncel favori listesini döndür
        let mut stmt = conn
            .prepare("SELECT sarki_id FROM favoriler")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| e.to_string())?;
        let mut favoriler = Vec::new();

        for r in rows.flatten() {
            favoriler.push(r);
        }

        Ok(favoriler)
    })
    .await
    .map_err(|e| format!("İşlem hatası: {}", e))?
}
