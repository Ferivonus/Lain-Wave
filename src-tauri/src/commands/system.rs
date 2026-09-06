use crate::{
    commands::music::row_to_sarki, // Şarkıları okumak için
    models::{Ayarlar, Playlist, Sarki}, // Sadece veri modellerini çağırıyoruz
    db::{DbState, DiscordState}, // State (Durum) yapılarını db modülünden çağırıyoruz (DÜZELTİLDİ)
    utils::app_data_dir,
};
use rusqlite::params;
use std::process::Command;
use tauri::{AppHandle, Manager};

/// Uygulamanın sistemdeki veri klasörünün yolunu döndürür.
#[tauri::command]
pub fn get_app_data_dir(app: AppHandle) -> Result<String, String> {
    let path = app_data_dir(&app)?;
    Ok(path.to_string_lossy().to_string())
}

/// Uygulamanın veri klasörünü işletim sisteminin dosya yöneticisinde açar.
#[tauri::command]
pub fn open_data_folder(app: AppHandle) -> Result<(), String> {
    let path = app_data_dir(&app)?;
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Discord Rich Presence (Zengin Durum) bilgisini günceller.
#[tauri::command]
pub fn update_discord_status(
    state: tauri::State<'_, DiscordState>,
    detay: String,
    durum: String,
    start_timestamp: Option<i64>,
    end_timestamp: Option<i64>,
) -> Result<(), String> {
    let mut drpc = state
        .0
        .lock()
        .map_err(|_| "Discord işlem kilitlendi!".to_string())?;
    drpc.set_activity(|mut a| {
        a = a
            .details(detay)
            .state(durum)
            .assets(|ass| ass.large_image("icon"));
        a = match (start_timestamp, end_timestamp) {
            (Some(start), Some(end)) => a.timestamps(|t| t.start(start as u64).end(end as u64)),
            (Some(start), None) => a.timestamps(|t| t.start(start as u64)),
            (None, Some(end)) => a.timestamps(|t| t.end(end as u64)),
            (None, None) => a,
        };
        a
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Discord Rich Presence bilgisini temizler.
#[tauri::command]
pub fn clear_discord_status(state: tauri::State<'_, DiscordState>) -> Result<(), String> {
    let mut drpc = state
        .0
        .lock()
        .map_err(|_| "Discord işlem kilitlendi!".to_string())?;
    drpc.clear_activity()
        .map_err(|e: discord_presence::DiscordError| e.to_string())?;
    Ok(())
}

/// Veritabanından kullanıcı ayarlarını okur.
#[tauri::command]
pub async fn ayarlari_getir(app: AppHandle) -> Result<Ayarlar, String> {
    let db_arc = app.state::<DbState>().0.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;
        let ayarlar = conn.query_row("SELECT kullanici_adi, discord_aktif, medya_tuslari_aktif, tema FROM ayarlar WHERE id = 1", [], |row| {
            let discord: i32 = row.get(1)?;
            let medya: i32 = row.get(2)?;
            Ok(Ayarlar {
                kullanici_adi: row.get(0)?,
                discord_aktif: discord == 1,
                medya_tuslari_aktif: medya == 1,
                tema: row.get(3)?
            })
        }).unwrap_or_default();
        Ok(ayarlar)
    }).await.map_err(|e| e.to_string())?
}

/// Kullanıcı ayarlarını veritabanına kaydeder.
#[tauri::command]
pub async fn ayarlari_kaydet(app: AppHandle, ayarlar: Ayarlar) -> Result<(), String> {
    let db_arc = app.state::<DbState>().0.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;
        conn.execute(
            "UPDATE ayarlar SET kullanici_adi = ?1, discord_aktif = ?2, medya_tuslari_aktif = ?3, tema = ?4 WHERE id = 1",
            params![ayarlar.kullanici_adi, if ayarlar.discord_aktif {1} else {0}, if ayarlar.medya_tuslari_aktif {1} else {0}, ayarlar.tema]
        ).map_err(|e| e.to_string())?;
        Ok(())
    }).await.map_err(|e| e.to_string())?
}

/// Tüm veritabanının (şarkılar, listeler, ayarlar) yedeğini alıp belirtilen JSON dosyasına yazar.
#[tauri::command]
pub async fn yedek_al(app: AppHandle, hedef_yol: String) -> Result<(), String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;
        let mut yedek_verisi = serde_json::Map::new();

        // 1. Şarkıları Yedekle
        let mut stmt = conn.prepare("SELECT * FROM sarkilar ORDER BY sira ASC").map_err(|e| e.to_string())?;
        let sarki_iter = stmt.query_map([], row_to_sarki).map_err(|e| e.to_string())?;
        let sarkilar: Vec<Sarki> = sarki_iter.filter_map(Result::ok).collect();
        yedek_verisi.insert("kutuphane".to_string(), serde_json::to_value(&sarkilar).unwrap_or_default());

        // 2. Playlistleri Yedekle
        let mut stmt_pl = conn.prepare("SELECT id, isim FROM playlistler").map_err(|e| e.to_string())?;
        let pl_iter = stmt_pl.query_map([], |row| Ok(Playlist { id: row.get(0)?, isim: row.get(1)?, sarkilar: Vec::new() })).map_err(|e| e.to_string())?;
        let mut listeler: Vec<Playlist> = pl_iter.filter_map(Result::ok).collect();

        for pl in listeler.iter_mut() {
            let mut s_stmt = conn.prepare("SELECT sarki_id FROM playlist_sarkilar WHERE playlist_id = ? ORDER BY sira ASC").map_err(|e| e.to_string())?;
            let ids = s_stmt.query_map([&pl.id], |row| row.get::<_, String>(0)).map_err(|e| e.to_string())?;
            for s in ids.flatten() { pl.sarkilar.push(s); }
        }
        yedek_verisi.insert("playlistler".to_string(), serde_json::to_value(&listeler).unwrap_or_default());

        // 3. Favorileri Yedekle
        let mut stmt_fav = conn.prepare("SELECT sarki_id FROM favoriler").map_err(|e| e.to_string())?;
        let favs: Vec<String> = stmt_fav.query_map([], |row| row.get(0)).map_err(|e| e.to_string())?.filter_map(Result::ok).collect();
        yedek_verisi.insert("favoriler".to_string(), serde_json::to_value(&favs).unwrap_or_default());

        // 4. Ayarları Yedekle
        let ayarlar = conn.query_row("SELECT kullanici_adi, discord_aktif, medya_tuslari_aktif, tema FROM ayarlar WHERE id = 1", [], |row| {
            Ok(Ayarlar { kullanici_adi: row.get(0)?, discord_aktif: row.get::<_, i32>(1)? == 1, medya_tuslari_aktif: row.get::<_, i32>(2)? == 1, tema: row.get(3)? })
        }).unwrap_or_default();
        yedek_verisi.insert("ayarlar".to_string(), serde_json::to_value(&ayarlar).unwrap_or_default());

        // JSON'a çevir ve dosyaya yaz
        let json_cikti = serde_json::to_string_pretty(&yedek_verisi).map_err(|e| e.to_string())?;
        std::fs::write(hedef_yol, json_cikti).map_err(|e| e.to_string())?;

        Ok(())
    }).await.map_err(|e| e.to_string())?
}

/// JSON yedek dosyasını okuyup veritabanını bu yedekle değiştirir.
#[tauri::command]
pub async fn yedekten_don(app: AppHandle, kaynak_yol: String) -> Result<(), String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        // Dosyayı oku ve JSON objesine dönüştür
        let icerik = std::fs::read_to_string(kaynak_yol).map_err(|e| e.to_string())?;
        let yedek_verisi: serde_json::Value = serde_json::from_str(&icerik).map_err(|e| format!("Geçersiz yedek dosyası: {}", e))?;

        let mut conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;
        
        // Hata durumunda işlemi geri alabilmek için transaction başlatıyoruz
        let tx = conn.transaction().map_err(|e| e.to_string())?;

        // 1. Şarkıları Geri Yükle
        if let Some(kutuphane) = yedek_verisi.get("kutuphane").and_then(|v| v.as_array()) {
            tx.execute("DELETE FROM sarkilar", []).ok();
            let mut sira = 0;
            for s in kutuphane {
                if let Ok(sarki) = serde_json::from_value::<Sarki>(s.clone()) {
                    tx.execute("INSERT INTO sarkilar (id, isim, sarkici, album, yol, kapak_yolu, sozler_yolu, tarz, kalite, sure, dinlenme_sayisi, son_dinlenme_tarihi, yil, notlar, sira) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
                    params![sarki.id, sarki.isim, sarki.sarkici, sarki.album, sarki.yol, sarki.kapak_yolu, sarki.sozler_yolu, sarki.tarz, sarki.kalite, sarki.sure, sarki.dinlenme_sayisi, sarki.son_dinlenme_tarihi.map(|v| v as i64), sarki.yil, sarki.notlar, sira]).ok();
                    sira += 1;
                }
            }
        }

        // 2. Playlistleri Geri Yükle
        if let Some(playlistler) = yedek_verisi.get("playlistler").and_then(|v| v.as_array()) {
            tx.execute("DELETE FROM playlistler", []).ok();
            tx.execute("DELETE FROM playlist_sarkilar", []).ok();
            for p in playlistler {
                if let Ok(pl) = serde_json::from_value::<Playlist>(p.clone()) {
                    tx.execute("INSERT INTO playlistler (id, isim) VALUES (?1, ?2)", params![pl.id, pl.isim]).ok();
                    for (index, s_id) in pl.sarkilar.iter().enumerate() {
                        tx.execute("INSERT INTO playlist_sarkilar (playlist_id, sarki_id, sira) VALUES (?1, ?2, ?3)", params![pl.id, s_id, index as i32]).ok();
                    }
                }
            }
        }

        // 3. Favorileri Geri Yükle
        if let Some(favoriler) = yedek_verisi.get("favoriler").and_then(|v| v.as_array()) {
            tx.execute("DELETE FROM favoriler", []).ok();
            for f in favoriler {
                if let Some(fav_id) = f.as_str() {
                    tx.execute("INSERT INTO favoriler (sarki_id) VALUES (?)", params![fav_id]).ok();
                }
            }
        }

        // 4. Ayarları Geri Yükle
        if let Some(ayarlar) = yedek_verisi.get("ayarlar") {
            if let Ok(ayar) = serde_json::from_value::<Ayarlar>(ayarlar.clone()) {
                tx.execute("UPDATE ayarlar SET kullanici_adi = ?1, discord_aktif = ?2, medya_tuslari_aktif = ?3, tema = ?4 WHERE id = 1",
                params![ayar.kullanici_adi, if ayar.discord_aktif {1} else {0}, if ayar.medya_tuslari_aktif {1} else {0}, ayar.tema]).ok();
            }
        }

        // İşlemleri güvenli bir şekilde onayla ve bitir
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }).await.map_err(|e| e.to_string())?
}