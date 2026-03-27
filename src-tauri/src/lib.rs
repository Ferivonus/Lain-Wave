
use discord_presence::Client as DiscordClient;
use id3::{Tag, TagLike};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use tauri::Emitter;
use tauri::tray::TrayIconEvent;
use tauri::{AppHandle, Manager};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};
use std::fs::File;
use std::io::Write;

use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Shortcut, ShortcutState};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};



#[derive(Serialize, Deserialize, Clone)]
pub struct Sarki {
    pub id: String,
    pub isim: String,
    pub sarkici: String,
    pub album: String,
    pub yol: String,
    pub kapak_yolu: Option<String>,
    pub sozler_yolu: Option<String>,
    pub tarz: Option<String>,
    pub kalite: Option<String>,
    pub sure: Option<u32>,
    pub dinlenme_sayisi: Option<u32>,
    pub son_dinlenme_tarihi: Option<u64>,
    pub yil: Option<u32>,
    pub notlar: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Playlist {
    pub id: String,
    pub isim: String,
    pub sarkilar: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct YouTubeSonuc {
    pub title: String,
    pub channel: String,
    pub duration_string: String,
    pub thumbnail: String,
    pub webpage_url: String,
}

#[derive(Serialize)]
pub struct MetadataBilgisi {
    pub isim: Option<String>,
    pub sarkici: Option<String>,
    pub album: Option<String>,
    pub tarz: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ayarlar {
    pub kullanici_adi: String,
    pub discord_aktif: bool,
    pub medya_tuslari_aktif: bool,
    pub tema: String,
}

impl Default for Ayarlar {
    fn default() -> Self {
        Self {
            kullanici_adi: String::new(),
            discord_aktif: true,
            medya_tuslari_aktif: true,
            tema: "theme-modern".to_string(),
        }
    }
}

struct DiscordState(Arc<Mutex<DiscordClient>>);
struct DbState(Arc<Mutex<Connection>>);

fn app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let yol = app
        .path()
        .app_local_data_dir()
        .map_err(|_| "Sistem klasörüne erişilemiyor!".to_string())?;
    if !yol.exists() {
        fs::create_dir_all(&yol).map_err(|_| "Veri klasörü oluşturulamadı!".to_string())?;
    }
    Ok(yol)
}

fn songs_klasoru_bul(app: &AppHandle) -> Result<PathBuf, String> {
    let mut yol = app_data_dir(app)?;
    yol.push("songs");
    if !yol.exists() {
        fs::create_dir_all(&yol).map_err(|_| "Şarkı klasörü oluşturulamadı!".to_string())?;
    }
    Ok(yol)
}

fn init_db(app: &AppHandle) -> Result<Connection, String> {
    let mut db_path = app_data_dir(app)?;
    db_path.push("lainwave.db");

    let conn = Connection::open(db_path).map_err(|e| format!("Veritabanı açılamadı: {}", e))?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS sarkilar (
            id TEXT PRIMARY KEY,
            isim TEXT NOT NULL,
            sarkici TEXT NOT NULL,
            album TEXT NOT NULL,
            yol TEXT NOT NULL,
            kapak_yolu TEXT,
            sozler_yolu TEXT,
            tarz TEXT,
            kalite TEXT,
            sure INTEGER,
            dinlenme_sayisi INTEGER DEFAULT 0,
            son_dinlenme_tarihi INTEGER,
            yil INTEGER,
            notlar TEXT,
            sira INTEGER DEFAULT 0
        );
        CREATE TABLE IF NOT EXISTS playlistler (
            id TEXT PRIMARY KEY,
            isim TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS playlist_sarkilar (
            playlist_id TEXT,
            sarki_id TEXT,
            sira INTEGER,
            PRIMARY KEY (playlist_id, sarki_id)
        );
        CREATE TABLE IF NOT EXISTS favoriler (
            sarki_id TEXT PRIMARY KEY
        );
        CREATE TABLE IF NOT EXISTS ayarlar (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            kullanici_adi TEXT,
            discord_aktif INTEGER,
            medya_tuslari_aktif INTEGER,
            tema TEXT
        );
        ",
    )
    .map_err(|e| format!("Tablolar oluşturulamadı: {}", e))?;

    conn.execute(
        "INSERT OR IGNORE INTO ayarlar (id, kullanici_adi, discord_aktif, medya_tuslari_aktif, tema)
         VALUES (1, '', 1, 1, 'theme-modern')",
        [],
    )
    .ok();

    Ok(conn)
}

fn row_to_sarki(row: &rusqlite::Row) -> rusqlite::Result<Sarki> {
    Ok(Sarki {
        id: row.get(0)?,
        isim: row.get(1)?,
        sarkici: row.get(2)?,
        album: row.get(3)?,
        yol: row.get(4)?,
        kapak_yolu: row.get(5)?,
        sozler_yolu: row.get(6)?,
        tarz: row.get(7)?,
        kalite: row.get(8)?,
        sure: row.get(9)?,
        dinlenme_sayisi: row.get(10)?,
        son_dinlenme_tarihi: row.get::<_, Option<i64>>(11)?.map(|v| v as u64),
        yil: row.get(12)?,
        notlar: row.get(13)?,
    })
}

#[tauri::command]
async fn sarki_metadata_oku(yol: String) -> Result<MetadataBilgisi, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let path = Path::new(&yol);
        if !path.exists() {
            return Err("Dosya bulunamadı".into());
        }

        if let Ok(tag) = Tag::read_from_path(path) {
            Ok(MetadataBilgisi {
                isim: tag.title().map(|s| s.to_string()),
                sarkici: tag.artist().map(|s| s.to_string()),
                album: tag.album().map(|s| s.to_string()),
                tarz: tag.genre().map(|s| s.to_string()),
            })
        } else {
            Ok(MetadataBilgisi {
                isim: None,
                sarkici: None,
                album: None,
                tarz: None,
            })
        }
    })
    .await
    .map_err(|e| format!("İşlem hatası: {}", e))?
}

#[tauri::command]
async fn sarki_kaydet(
    app: AppHandle,
    mut isim: String,
    sarkici: String,
    album: String,
    yol: String,
    manuel_tarz: Option<String>,
    yil: Option<u32>,
    notlar: Option<String>,
) -> Result<Sarki, String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let songs_klasoru = songs_klasoru_bul(&app)?;
        let orijinal_path = Path::new(&yol);

        if !orijinal_path.exists() {
            return Err("Kaynak dosya bulunamadı!".into());
        }

        let uzanti = orijinal_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("mp3");

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| "Zaman hesaplama hatası!")?
            .as_nanos();

        let id = format!("song_{}", timestamp);

        let yeni_dosya_adi = format!("{}.{}", id, uzanti);
        let mut hedef_yol = songs_klasoru.clone();
        hedef_yol.push(&yeni_dosya_adi);

        fs::copy(orijinal_path, &hedef_yol).map_err(|e| format!("Kopyalama hatası: {}", e))?;

        let mut kapak_yolu = None;
        let mut final_tarz = manuel_tarz;
        let mut sure = None;

        if let Ok(tag) = Tag::read_from_path(orijinal_path) {
            if isim.trim().is_empty() {
                if let Some(t) = tag.title() { isim = t.to_string(); }
            }
            if final_tarz.is_none() {
                final_tarz = tag.genre().map(|g| g.to_string());
            }
            if let Some(tlen) = tag.get("TLEN").and_then(|f| f.content().text()) {
                if let Ok(ms) = tlen.parse::<u32>() { sure = Some(ms / 1000); }
            }
            if let Some(pic) = tag.pictures().next() {
                let pic_ext = if pic.mime_type == "image/png" { "png" } else { "jpg" };
                let kapak_adi = format!("{}_cover.{}", id, pic_ext);
                let mut kapak_hedef = songs_klasoru.clone();
                kapak_hedef.push(&kapak_adi);

                if fs::write(&kapak_hedef, &pic.data).is_ok() {
                    kapak_yolu = Some(kapak_hedef.to_string_lossy().to_string());
                }
            }
        }

        let yeni_sarki = Sarki {
            id: id.clone(),
            isim: isim.clone(),
            sarkici: sarkici.clone(),
            album: album.clone(),
            yol: hedef_yol.to_string_lossy().to_string(),
            kapak_yolu: kapak_yolu.clone(),
            sozler_yolu: None,
            tarz: final_tarz.clone(),
            kalite: Some(uzanti.to_uppercase()),
            sure,
            dinlenme_sayisi: Some(0),
            son_dinlenme_tarihi: None,
            yil,
            notlar: notlar.clone(),
        };

        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;

        let sira_sorgu: i32 = conn.query_row("SELECT COALESCE(MAX(sira), 0) + 1 FROM sarkilar", [], |r| r.get(0)).unwrap_or(0);

        conn.execute(
            "INSERT INTO sarkilar (id, isim, sarkici, album, yol, kapak_yolu, sozler_yolu, tarz, kalite, sure, dinlenme_sayisi, son_dinlenme_tarihi, yil, notlar, sira)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![
                yeni_sarki.id, yeni_sarki.isim, yeni_sarki.sarkici, yeni_sarki.album, yeni_sarki.yol,
                yeni_sarki.kapak_yolu, yeni_sarki.sozler_yolu, yeni_sarki.tarz, yeni_sarki.kalite, yeni_sarki.sure,
                yeni_sarki.dinlenme_sayisi, yeni_sarki.son_dinlenme_tarihi.map(|v| v as i64), yeni_sarki.yil, yeni_sarki.notlar, sira_sorgu
            ],
        ).map_err(|e| format!("Veritabanına eklenemedi: {}", e))?;

        Ok(yeni_sarki)
    })
    .await
    .map_err(|e| format!("İşlem hatası: {}", e))?
}

#[tauri::command]
async fn sarkilari_getir(app: AppHandle) -> Result<Vec<Sarki>, String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;
        let mut stmt = conn
            .prepare("SELECT * FROM sarkilar ORDER BY sira ASC")
            .map_err(|e| e.to_string())?;

        let sarki_iter = stmt
            .query_map([], row_to_sarki)
            .map_err(|e| e.to_string())?;

        let mut sarkilar = Vec::new();
        for sarki in sarki_iter {
            if let Ok(s) = sarki {
                sarkilar.push(s);
            }
        }
        Ok(sarkilar)
    })
    .await
    .map_err(|e| format!("İşlem hatası: {}", e))?
}

#[tauri::command]
async fn playlist_olustur(app: AppHandle, isim: String) -> Result<Playlist, String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;

        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM playlistler", [], |row| row.get(0))
            .unwrap_or(0);

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| "Zaman hesaplama hatası!")?
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

#[tauri::command]
async fn playlistleri_getir(app: AppHandle) -> Result<Vec<Playlist>, String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;
        let mut stmt = conn.prepare("SELECT id, isim FROM playlistler").map_err(|e| e.to_string())?;

        let pl_iter = stmt.query_map([], |row| {
            Ok(Playlist {
                id: row.get(0)?,
                isim: row.get(1)?,
                sarkilar: Vec::new(),
            })
        }).map_err(|e| e.to_string())?;

        let mut listeler = Vec::new();
        for pl_res in pl_iter {
            if let Ok(mut pl) = pl_res {
                let mut stmt_sarki = conn.prepare("SELECT sarki_id FROM playlist_sarkilar WHERE playlist_id = ? ORDER BY sira ASC").map_err(|e| e.to_string())?;
let sarkilar = stmt_sarki.query_map([&pl.id], |row| row.get::<_, String>(0)).map_err(|e| e.to_string())?;
                for s in sarkilar {
                    if let Ok(id) = s {
                        pl.sarkilar.push(id);
                    }
                }
                listeler.push(pl);
            }
        }
        Ok(listeler)
    })
    .await
    .map_err(|e| format!("İşlem hatası: {}", e))?
}

#[tauri::command]
async fn playliste_sarki_ekle(
    app: AppHandle,
    playlist_id: String,
    sarki_id: String,
) -> Result<Playlist, String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;

        let var_mi: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM playlist_sarkilar WHERE playlist_id = ? AND sarki_id = ?",
                params![playlist_id, sarki_id],
                |row| row.get(0),
            )
            .unwrap_or(0);

        if var_mi == 0 {
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

        let isim: String = conn
            .query_row(
                "SELECT isim FROM playlistler WHERE id = ?",
                params![playlist_id],
                |row| row.get(0),
            )
            .map_err(|_| "Playlist bulunamadı")?;

        let mut stmt = conn
    .prepare("SELECT sarki_id FROM playlist_sarkilar WHERE playlist_id = ? ORDER BY sira ASC")
    .map_err(|e| e.to_string())?;
let sarkilar_iter = stmt.query_map([&playlist_id], |row| row.get(0)).map_err(|e| e.to_string())?;
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

#[tauri::command]
async fn favorileri_getir(app: AppHandle) -> Result<Vec<String>, String> {
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

#[tauri::command]
async fn favori_degistir(app: AppHandle, sarki_id: String) -> Result<Vec<String>, String> {
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
            conn.execute(
                "DELETE FROM favoriler WHERE sarki_id = ?",
                params![sarki_id],
            )
            .ok();
        } else {
            conn.execute(
                "INSERT INTO favoriler (sarki_id) VALUES (?)",
                params![sarki_id],
            )
            .ok();
        }

       let mut stmt = conn.prepare("SELECT sarki_id FROM favoriler").map_err(|e| e.to_string())?;
let rows = stmt.query_map([], |row| row.get::<_, String>(0)).map_err(|e| e.to_string())?;let mut favoriler = Vec::new();
        for r in rows.flatten() {
            favoriler.push(r);
        }

        Ok(favoriler)
    })
    .await
    .map_err(|e| format!("İşlem hatası: {}", e))?
}

#[tauri::command]
async fn playlistten_sarki_cikar(
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
    .prepare("SELECT sarki_id FROM playlist_sarkilar WHERE playlist_id = ? ORDER BY sira ASC")
    .map_err(|e| e.to_string())?;
let sarkilar_iter = stmt.query_map([&playlist_id], |row| row.get(0)).map_err(|e| e.to_string())?;
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

#[tauri::command]
async fn sarki_sil(app: AppHandle, sarki_id: String) -> Result<(), String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let mut conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;

        let (yol, kapak_yolu, sozler_yolu): (String, Option<String>, Option<String>) = conn
            .query_row(
                "SELECT yol, kapak_yolu, sozler_yolu FROM sarkilar WHERE id = ?",
                params![sarki_id],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .map_err(|_| "Şarkı veritabanında bulunamadı")?;

        let _ = fs::remove_file(&yol);
        if let Some(kapak) = kapak_yolu {
            let _ = fs::remove_file(&kapak);
        }
        if let Some(sozler) = sozler_yolu {
            let _ = fs::remove_file(&sozler);
        }

        let tx = conn.transaction().map_err(|e| e.to_string())?;
        tx.execute("DELETE FROM sarkilar WHERE id = ?", params![sarki_id])
            .map_err(|e| e.to_string())?;
        tx.execute(
            "DELETE FROM playlist_sarkilar WHERE sarki_id = ?",
            params![sarki_id],
        )
        .map_err(|e| e.to_string())?;
        tx.execute(
            "DELETE FROM favoriler WHERE sarki_id = ?",
            params![sarki_id],
        )
        .map_err(|e| e.to_string())?;
        tx.commit().map_err(|e| e.to_string())?;

        Ok(())
    })
    .await
    .map_err(|e| format!("İşlem hatası: {}", e))?
}

#[tauri::command]
async fn sarki_sirasi_guncelle(app: AppHandle, yeni_liste: Vec<Sarki>) -> Result<(), String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let mut conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;
        let tx = conn.transaction().map_err(|e| e.to_string())?;

        for (index, sarki) in yeni_liste.iter().enumerate() {
            tx.execute(
                "UPDATE sarkilar SET sira = ? WHERE id = ?",
                params![index as i32, sarki.id],
            )
            .ok();
        }

        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    })
    .await
    .map_err(|e| format!("İşlem hatası: {}", e))?
}

#[tauri::command]
fn get_app_data_dir(app: AppHandle) -> Result<String, String> {
    let path = app_data_dir(&app)?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
fn open_data_folder(app: AppHandle) -> Result<(), String> {
    let path = app_data_dir(&app)?;
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer").arg(path).spawn().map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(path).spawn().map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open").arg(path).spawn().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn update_discord_status(
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

#[tauri::command]
fn clear_discord_status(state: tauri::State<'_, DiscordState>) -> Result<(), String> {
    let mut drpc = state
        .0
        .lock()
        .map_err(|_| "Discord işlem kilitlendi!".to_string())?;
    drpc.clear_activity().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn dinlenme_sayisi_artir(
    app: AppHandle,
    sarki_id: String,
    tarih: u64,
) -> Result<(u32, u64), String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;

        conn.execute(
            "UPDATE sarkilar SET dinlenme_sayisi = dinlenme_sayisi + 1, son_dinlenme_tarihi = ? WHERE id = ?",
            params![tarih as i64, sarki_id]
        ).map_err(|e| e.to_string())?;

        let yeni_sayi: u32 = conn.query_row("SELECT dinlenme_sayisi FROM sarkilar WHERE id = ?", params![sarki_id], |row| row.get(0)).unwrap_or(1);
        Ok((yeni_sayi, tarih))
    })
    .await
    .map_err(|e| format!("İşlem hatası: {}", e))?
}

#[tauri::command]
async fn playlist_sil(app: AppHandle, playlist_id: String) -> Result<(), String> {
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


#[tauri::command]
async fn sarki_guncelle(
    app: AppHandle,
    id: String,
    isim: String,
    sarkici: String,
    album: String,
    tarz: Option<String>,
    yil: Option<u32>,
) -> Result<Sarki, String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;

        conn.execute(
            "UPDATE sarkilar SET isim = ?1, sarkici = ?2, album = ?3, tarz = ?4, yil = ?5 WHERE id = ?6",
            params![isim, sarkici, album, tarz, yil, id]
        ).map_err(|e| e.to_string())?;

let mut stmt = conn.prepare("SELECT * FROM sarkilar WHERE id = ?").map_err(|e| e.to_string())?;
let sarki = stmt.query_row(params![id], row_to_sarki).map_err(|_| "Güncellenen şarkı bulunamadı")?;
        Ok(sarki)
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn playlist_disa_aktar(
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

#[tauri::command]
async fn ayarlari_getir(app: AppHandle) -> Result<Ayarlar, String> {
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

#[tauri::command]
async fn ayarlari_kaydet(app: AppHandle, ayarlar: Ayarlar) -> Result<(), String> {
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

#[tauri::command]
async fn playlist_sirasi_guncelle(
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

#[tauri::command]
async fn yedek_al(app: tauri::AppHandle, hedef_yol: String) -> Result<(), String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;
        let mut yedek_verisi = serde_json::Map::new();

        // Şarkılar
        let mut stmt = conn.prepare("SELECT * FROM sarkilar ORDER BY sira ASC").map_err(|e| e.to_string())?;
        let sarki_iter = stmt.query_map([], row_to_sarki).map_err(|e| e.to_string())?;
        let sarkilar: Vec<Sarki> = sarki_iter.filter_map(Result::ok).collect();
        yedek_verisi.insert("kutuphane".to_string(), serde_json::to_value(&sarkilar).unwrap_or_default());

        // Playlistler
        let mut stmt_pl = conn.prepare("SELECT id, isim FROM playlistler").map_err(|e| e.to_string())?;
        let pl_iter = stmt_pl.query_map([], |row| Ok(Playlist { id: row.get(0)?, isim: row.get(1)?, sarkilar: Vec::new() })).map_err(|e| e.to_string())?;
        let mut listeler: Vec<Playlist> = pl_iter.filter_map(Result::ok).collect();

        for pl in listeler.iter_mut() {
            let mut s_stmt = conn.prepare("SELECT sarki_id FROM playlist_sarkilar WHERE playlist_id = ? ORDER BY sira ASC").map_err(|e| e.to_string())?;
            let ids = s_stmt.query_map([&pl.id], |row| row.get::<_, String>(0)).map_err(|e| e.to_string())?;
            for s in ids.flatten() { pl.sarkilar.push(s); }
        }
        yedek_verisi.insert("playlistler".to_string(), serde_json::to_value(&listeler).unwrap_or_default());

        // Favoriler
        let mut stmt_fav = conn.prepare("SELECT sarki_id FROM favoriler").map_err(|e| e.to_string())?;
        let favs: Vec<String> = stmt_fav.query_map([], |row| row.get(0)).map_err(|e| e.to_string())?.filter_map(Result::ok).collect();
        yedek_verisi.insert("favoriler".to_string(), serde_json::to_value(&favs).unwrap_or_default());

        // Ayarlar
        let ayarlar = conn.query_row("SELECT kullanici_adi, discord_aktif, medya_tuslari_aktif, tema FROM ayarlar WHERE id = 1", [], |row| {
            Ok(Ayarlar { kullanici_adi: row.get(0)?, discord_aktif: row.get::<_, i32>(1)? == 1, medya_tuslari_aktif: row.get::<_, i32>(2)? == 1, tema: row.get(3)? })
        }).unwrap_or_default();
        yedek_verisi.insert("ayarlar".to_string(), serde_json::to_value(&ayarlar).unwrap_or_default());

        let json_cikti = serde_json::to_string_pretty(&yedek_verisi).map_err(|e| e.to_string())?;
        fs::write(hedef_yol, json_cikti).map_err(|e| e.to_string())?;

        Ok(())
    }).await.map_err(|e| e.to_string())?
}
#[tauri::command]
async fn yedekten_don(app: tauri::AppHandle, kaynak_yol: String) -> Result<(), String> {
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let icerik = fs::read_to_string(kaynak_yol).map_err(|e| e.to_string())?;
        let yedek_verisi: serde_json::Value = serde_json::from_str(&icerik).map_err(|e| format!("Geçersiz yedek dosyası: {}", e))?;

        let mut conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;
        let tx = conn.transaction().map_err(|e| e.to_string())?;

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

        if let Some(favoriler) = yedek_verisi.get("favoriler").and_then(|v| v.as_array()) {
            tx.execute("DELETE FROM favoriler", []).ok();
            for f in favoriler {
                if let Some(fav_id) = f.as_str() {
                    tx.execute("INSERT INTO favoriler (sarki_id) VALUES (?)", params![fav_id]).ok();
                }
            }
        }

        if let Some(ayarlar) = yedek_verisi.get("ayarlar") {
            if let Ok(ayar) = serde_json::from_value::<Ayarlar>(ayarlar.clone()) {
                tx.execute("UPDATE ayarlar SET kullanici_adi = ?1, discord_aktif = ?2, medya_tuslari_aktif = ?3, tema = ?4 WHERE id = 1",
                params![ayar.kullanici_adi, if ayar.discord_aktif {1} else {0}, if ayar.medya_tuslari_aktif {1} else {0}, ayar.tema]).ok();
            }
        }

        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }).await.map_err(|e| e.to_string())?
}


#[tauri::command]
async fn sarki_sozu_oku(yol: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        std::fs::read_to_string(&yol).map_err(|e| format!("Dosya okuma hatası: {}", e))
    })
    .await
    .map_err(|e| format!("Thread hatası: {}", e))?
}

#[derive(Serialize)]
pub struct AvailableLanguage {
    pub dil: String,
    pub yol: String,
}

#[tauri::command]
async fn mevcut_soz_dillerini_bul(yol: String) -> Result<Vec<AvailableLanguage>, String> {
    let path = Path::new(&yol);
    let parent = path.parent().ok_or("Klasör bulunamadı")?;
    let file_stem = path.file_stem()
        .and_then(|s| s.to_str())
        .and_then(|s| s.split('.').next()) // "yt_123.tr" -> "yt_123" kısmını alır
        .ok_or("Dosya adı geçersiz")?;

    let mut diller = Vec::new();
    if let Ok(entries) = std::fs::read_dir(parent) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with(file_stem) && name.ends_with(".srt") {
                // Dil kodunu ayıkla (Örn: yt_123.tr.srt -> TR)
                let dil_kodu = name.replace(file_stem, "")
                    .replace(".srt", "")
                    .replace(".", "")
                    .to_uppercase();
                
                let dil_adi = if dil_kodu.is_empty() { "ORİJİNAL".to_string() } else { dil_kodu };
                
                diller.push(AvailableLanguage {
                    dil: dil_adi,
                    yol: entry.path().to_string_lossy().to_string(),
                });
            }
        }
    }
    
    // Alfabetik sırala (TR, EN vb.)
    diller.sort_by(|a, b| a.dil.cmp(&b.dil));
    Ok(diller)
}

#[tauri::command]
async fn youtube_indir(
    app: tauri::AppHandle, 
    url: String, 
    tarz: String, 
    dil: String,
    youtube_cevirisi_kullan: bool, 
    yapay_zeka_kullan: bool
) -> Result<Sarki, String> {
    let app_clone = app.clone();
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let songs_klasoru = songs_klasoru_bul(&app_clone)?;
        let local_data = app_data_dir(&app_clone)?;
        let cache_dir = local_data.join("yt_cache");

        if !cache_dir.exists() {
            std::fs::create_dir_all(&cache_dir).map_err(|e| format!("Cache hatası: {}", e))?;
        }

        let mut temiz_url = url.clone();
        if let Some(pos) = temiz_url.find("&list=") { temiz_url.truncate(pos); }
        if let Some(pos) = temiz_url.find("?list=") { temiz_url.truncate(pos); }
        if let Some(pos) = temiz_url.find("&index=") { temiz_url.truncate(pos); }

        let (yt_dlp_path, binaries_dir) = {
            let exe_path = std::env::current_exe().map(|p| p.parent().map(|p| p.to_path_buf()).unwrap_or_default()).unwrap_or_default();
            let resource_dir = app_clone.path().resource_dir().unwrap_or_default();
            let current_dir = std::env::current_dir().unwrap_or_default();

            let olasi_yollar = vec![
                exe_path.join("binaries").join("yt-dlp.exe"),
                resource_dir.join("binaries").join("yt-dlp.exe"),
                current_dir.join("src-tauri").join("binaries").join("yt-dlp.exe"),
                current_dir.join("binaries").join("yt-dlp.exe"),
            ];

            let yt_path = olasi_yollar.into_iter().find(|p| p.exists()).ok_or_else(|| "Araç bulunamadı (yt-dlp)".to_string())?;
            let bin_dir = yt_path.parent().unwrap_or(Path::new("")).to_path_buf();
            (yt_path, bin_dir)
        };

        let ffmpeg_path = binaries_dir.join("ffmpeg.exe");
        if !ffmpeg_path.exists() { return Err(format!("Araç bulunamadı (ffmpeg): {:?}", ffmpeg_path)); }

        let model_yolu = binaries_dir.join("ggml-base.bin");

        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|_| "Zaman hatası!")?.as_nanos();
        let id = format!("yt_{}", timestamp);

        let yt_dlp_hedef = songs_klasoru.join(format!("{}.%(ext)s", id));
        let hedef_ses_yolu = songs_klasoru.join(format!("{}.flac", id));
        let hedef_kapak_yolu = songs_klasoru.join(format!("{}.jpg", id));

        let mut cmd = std::process::Command::new(&yt_dlp_path);
        cmd.env("PYTHONIOENCODING", "utf-8");
        cmd.env("PATH", "C:\\Windows\\System32;C:\\Windows");
        cmd.current_dir(&songs_klasoru);

        // Dinamik Argüman Oluşturma
        cmd.arg("--no-warnings").arg("--no-playlist").arg("--newline").arg("--progress").arg("--no-simulate").arg("--ignore-errors")
            .arg("--cache-dir").arg(&cache_dir).arg("-f").arg("bestaudio/best").arg("-x").arg("--audio-format").arg("flac")
            .arg("--audio-quality").arg("0");

        // SADECE kullanıcı YouTube çevirisi istiyorsa altyazı indirme argümanlarını ekle
        if youtube_cevirisi_kullan {
            cmd.arg("--write-sub").arg("--write-auto-sub").arg("--sub-langs").arg(&dil).arg("--convert-subs").arg("srt");
        }

        // Parametrelerin karıştırılmaması için en sona "--" ekliyoruz.
cmd.arg("--print").arg("%(title)s|*|%(uploader)s|*|%(duration)s").arg("-o").arg(&yt_dlp_hedef).arg("--").arg(&temiz_url)
            .stdin(std::process::Stdio::null()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped());

        #[cfg(target_os = "windows")]
        use std::os::windows::process::CommandExt;
        #[cfg(target_os = "windows")]
        cmd.creation_flags(0x08000000);

        let mut child = cmd.spawn().map_err(|e| format!("yt-dlp başlatma hatası: {}", e))?;
        let stdout = child.stdout.take().ok_or("Stdout okuma hatası")?;
        
        let mut reader = std::io::BufReader::new(stdout);
        let mut metadata_line = String::new();
        let mut buf = Vec::new();

        use std::io::BufRead;
        while let Ok(bytes_read) = reader.read_until(b'\n', &mut buf) {
            if bytes_read == 0 { break; }
            let l = String::from_utf8_lossy(&buf).to_string();
            if l.contains("|*|") { metadata_line = l.clone(); }
            if l.contains("%") && l.contains("[download]") {
                let parts: Vec<&str> = l.split_whitespace().collect();
                if let Some(pct_str) = parts.get(1) {
                    if let Ok(pct) = pct_str.replace("%", "").parse::<f32>() {
                        let _ = app_clone.emit("download-progress", serde_json::json!({ "percentage": pct, "speed": parts.get(7).unwrap_or(&"0KiB/s"), "eta": parts.get(9).unwrap_or(&"00:00") }));
                    }
                }
            }
            buf.clear();
        }

        let status = child.wait().map_err(|e| format!("İndirme bekleme hatası: {}", e))?;

        if !status.success() && !hedef_ses_yolu.exists() { 
            return Err(format!("İndirme tamamlanamadı. Çıkış Kodu: {:?}", status.code())); 
        }

        // --- AKILLI ALTYAZI KONTROLÜ VE WHISPER ---
        let mut final_sozler_yolu = None;
        
        // EĞER İKİSİ DE FALSE İSE HİÇBİR ALTYAZI İŞLEMİ YAPMA (Hızlı Mod)
        if youtube_cevirisi_kullan || yapay_zeka_kullan {
            // 1. Eğer Youtube çevirisi istendiyse, inen SRT'leri kontrol et
            if youtube_cevirisi_kullan {
                let mut bulunan_srtler = Vec::new();
                if let Ok(entries) = std::fs::read_dir(&songs_klasoru) {
                    for entry in entries.flatten() {
                        let file_name = entry.file_name().to_string_lossy().to_string();
                        if file_name.starts_with(&id) && file_name.ends_with(".srt") {
                            bulunan_srtler.push(entry.path());
                        }
                    }
                }

                let aranan_uzanti = format!(".{}.srt", dil); // Örn: .tr.srt
                if let Some(srt_yolu) = bulunan_srtler.iter().find(|p| p.to_string_lossy().contains(&aranan_uzanti)) {
                    final_sozler_yolu = Some(srt_yolu.to_string_lossy().to_string());
                } else if !bulunan_srtler.is_empty() {
                    final_sozler_yolu = Some(bulunan_srtler[0].to_string_lossy().to_string());
                }
            }

            // 2. Youtube çevirisi başarısız olduysa VEYA istenmediyse VE yapay zeka istendiyse -> Whisper Çalıştır
            if final_sozler_yolu.is_none() && yapay_zeka_kullan && hedef_ses_yolu.exists() {
                let _ = app_clone.emit("download-progress", serde_json::json!({ "percentage": 99.0, "speed": "AI PROCESSING", "eta": "Whisper" }));
                let yapay_zeka_srt_yolu = songs_klasoru.join(format!("{}.srt", id));
                
                // Kullanıcının seçtiği dili Whisper'a paslıyoruz
                if let Ok(_) = whisper_altyazi_uret(&hedef_ses_yolu, &yapay_zeka_srt_yolu, &model_yolu, &ffmpeg_path, &dil) {
                    final_sozler_yolu = Some(yapay_zeka_srt_yolu.to_string_lossy().to_string());
                }
            }
        } else {
            // İkisi de false ise sadece log basıp hızlıca geç
            println!("Altyazı veya AI istenmedi. Şarkı hızlı modda kaydediliyor.");
        }

        let parcalar: Vec<&str> = metadata_line.split("|*|").collect();
        let isim = parcalar.get(0).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| "Bilinmeyen Parça".to_string());
        let sarkici = parcalar.get(1).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| "YouTube".to_string());
        let sure: Option<u32> = parcalar.get(2).and_then(|s| s.trim().parse::<f64>().ok()).map(|v| v as u32);

        let mut kapak_yolu = None;
        if hedef_kapak_yolu.exists() {
            kapak_yolu = Some(hedef_kapak_yolu.to_string_lossy().to_string());
        }

        let yeni_sarki = Sarki {
            id: id.clone(), isim, sarkici, album: "YouTube Arşivi".to_string(),
            yol: hedef_ses_yolu.to_string_lossy().to_string(), 
            kapak_yolu, 
            sozler_yolu: final_sozler_yolu, 
            tarz: Some(tarz),
            kalite: Some("FLAC (Kayıpsız)".to_string()), 
            sure, 
            dinlenme_sayisi: Some(0), 
            son_dinlenme_tarihi: None,
            yil: None, 
            notlar: Some(temiz_url),
        };

        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;
        let sira_sorgu: i32 = conn.query_row("SELECT COALESCE(MAX(sira), 0) + 1 FROM sarkilar", [], |r| r.get(0)).unwrap_or(0);

        conn.execute(
            "INSERT INTO sarkilar (id, isim, sarkici, album, yol, kapak_yolu, sozler_yolu, tarz, kalite, sure, dinlenme_sayisi, son_dinlenme_tarihi, yil, notlar, sira)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![yeni_sarki.id, yeni_sarki.isim, yeni_sarki.sarkici, yeni_sarki.album, yeni_sarki.yol, yeni_sarki.kapak_yolu, yeni_sarki.sozler_yolu, yeni_sarki.tarz, yeni_sarki.kalite, yeni_sarki.sure, yeni_sarki.dinlenme_sayisi, yeni_sarki.son_dinlenme_tarihi.map(|v| v as i64), yeni_sarki.yil, yeni_sarki.notlar, sira_sorgu],
        ).map_err(|e| format!("Veritabanına eklenemedi: {}", e))?;

        Ok(yeni_sarki)
    }).await.map_err(|e| format!("Arka plan işlemi hatası: {}", e))?
}
#[tauri::command]
async fn youtube_arama(app: tauri::AppHandle, sorgu: String) -> Result<Vec<YouTubeSonuc>, String> {
    let app_clone = app.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let cache_dir = app_data_dir(&app_clone)?.join("yt_cache");
        if !cache_dir.exists() {
            std::fs::create_dir_all(&cache_dir).map_err(|e| e.to_string())?;
        }

        let arama_kodu = format!("ytsearch20:{}", sorgu);
        let (yt_dlp_path, binaries_dir) = {
            let exe_path = std::env::current_exe()
                .map(|p| p.parent().map(|p| p.to_path_buf()).unwrap_or_default())
                .unwrap_or_default();
            let resource_dir = app_clone.path().resource_dir().unwrap_or_default();
            let current_dir = std::env::current_dir().unwrap_or_default();
            let olasi_yollar = vec![
                exe_path.join("binaries").join("yt-dlp.exe"),
                resource_dir.join("binaries").join("yt-dlp.exe"),
                current_dir
                    .join("src-tauri")
                    .join("binaries")
                    .join("yt-dlp.exe"),
                current_dir.join("binaries").join("yt-dlp.exe"),
            ];
            let yt_path = olasi_yollar
                .into_iter()
                .find(|p| p.exists())
                .ok_or_else(|| "Araç bulunamadı".to_string())?;
            let bin_dir = yt_path.parent().unwrap_or(Path::new("")).to_path_buf();
            (yt_path, bin_dir)
        };

        let mut cmd = std::process::Command::new(&yt_dlp_path);
        cmd.env("PYTHONIOENCODING", "utf-8")
            .env("PATH", "C:\\Windows\\System32;C:\\Windows")
            .current_dir(&cache_dir);
        cmd.args([
            "--no-warnings",
            "--cache-dir",
            cache_dir.to_str().unwrap_or(""),
            "--ffmpeg-location",
            binaries_dir.to_str().unwrap_or(""),
            "--dump-json",
            "--default-search",
            "ytsearch",
            "--no-playlist",
            "--",
            &arama_kodu,
        ])
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped());

        #[cfg(target_os = "windows")]
        use std::os::windows::process::CommandExt;
        #[cfg(target_os = "windows")]
        cmd.creation_flags(0x08000000);

        let output = cmd.output().map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err(format!(
                "Arama başarısız: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let mut sonuclar = Vec::new();
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                sonuclar.push(YouTubeSonuc {
                    title: json["title"].as_str().unwrap_or("Bilinmeyen").to_string(),
                    channel: json["uploader"]
                        .as_str()
                        .unwrap_or("Bilinmeyen")
                        .to_string(),
                    duration_string: json["duration_string"]
                        .as_str()
                        .unwrap_or("0:00")
                        .to_string(),
                    thumbnail: json["thumbnail"].as_str().unwrap_or("").to_string(),
                    webpage_url: json["webpage_url"].as_str().unwrap_or("").to_string(),
                });
            }
        }
        Ok(sonuclar)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn youtube_playlist_getir(
    app: tauri::AppHandle,
    url: String,
) -> Result<Vec<YouTubeSonuc>, String> {
    let app_clone = app.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let cache_dir = app_data_dir(&app_clone)?.join("yt_cache");
        if !cache_dir.exists() {
            std::fs::create_dir_all(&cache_dir).map_err(|e| e.to_string())?;
        }

        let (yt_dlp_path, binaries_dir) = {
            let exe_path = std::env::current_exe()
                .map(|p| p.parent().map(|p| p.to_path_buf()).unwrap_or_default())
                .unwrap_or_default();
            let resource_dir = app_clone.path().resource_dir().unwrap_or_default();
            let current_dir = std::env::current_dir().unwrap_or_default();
            let olasi_yollar = vec![
                exe_path.join("binaries").join("yt-dlp.exe"),
                resource_dir.join("binaries").join("yt-dlp.exe"),
                current_dir
                    .join("src-tauri")
                    .join("binaries")
                    .join("yt-dlp.exe"),
                current_dir.join("binaries").join("yt-dlp.exe"),
            ];
            let yt_path = olasi_yollar
                .into_iter()
                .find(|p| p.exists())
                .ok_or_else(|| "Araç bulunamadı".to_string())?;
            let bin_dir = yt_path.parent().unwrap_or(Path::new("")).to_path_buf();
            (yt_path, bin_dir)
        };

        let mut cmd = std::process::Command::new(&yt_dlp_path);
        cmd.env("PYTHONIOENCODING", "utf-8")
            .env("PATH", "C:\\Windows\\System32;C:\\Windows")
            .current_dir(&cache_dir);
        cmd.args([
            "--no-warnings",
            "--cache-dir",
            cache_dir.to_str().unwrap_or(""),
            "--ffmpeg-location",
            binaries_dir.to_str().unwrap_or(""),
            "--dump-json",
            "--flat-playlist",
            "--",
            &url,
        ])
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped());

        #[cfg(target_os = "windows")]
        use std::os::windows::process::CommandExt;
        #[cfg(target_os = "windows")]
        cmd.creation_flags(0x08000000);

        let output = cmd.output().map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err(format!(
                "Arama başarısız: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let mut sonuclar = Vec::new();
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                if json["url"].is_string() || json["id"].is_string() {
                    let video_url =
                        json["url"]
                            .as_str()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| {
                                format!(
                                    "https://www.youtube.com/watch?v={}",
                                    json["id"].as_str().unwrap_or("")
                                )
                            });
                    sonuclar.push(YouTubeSonuc {
                        title: json["title"].as_str().unwrap_or("Bilinmeyen").to_string(),
                        channel: json["uploader"]
                            .as_str()
                            .unwrap_or("Bilinmeyen")
                            .to_string(),
                        duration_string: json["duration_string"]
                            .as_str()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| {
                                json["duration"]
                                    .as_f64()
                                    .map(|d| format!("{}:{:02.0}", (d / 60.0).floor(), d % 60.0))
                                    .unwrap_or_else(|| "0:00".to_string())
                            }),
                        thumbnail: json["thumbnails"]
                            .as_array()
                            .and_then(|t| t.last())
                            .and_then(|t| t["url"].as_str())
                            .unwrap_or("")
                            .to_string(),
                        webpage_url: video_url,
                    });
                }
            }
        }
        Ok(sonuclar)
    })
    .await
    .map_err(|e| e.to_string())?
}

// 1. Yardımcı Fonksiyon: Saniyeyi SRT zaman damgasına çevirir
fn srt_zaman_damgasi(saniye: i64, milisaniye: i64) -> String {
    let saat = saniye / 3600;
    let dakika = (saniye % 3600) / 60;
    let sn = saniye % 60;
    format!("{:02}:{:02}:{:02},{:03}", saat, dakika, sn, milisaniye)
}
// 2. Ana Fonksiyon: Yapay zekayı çalıştırıp altyazı üretir
// 2. Ana Fonksiyon: Yapay zekayı çalıştırıp altyazı üretir
fn whisper_altyazi_uret(
    orijinal_ses_yolu: &PathBuf,
    hedef_srt_yolu: &PathBuf,
    model_yolu: &PathBuf,
    ffmpeg_yolu: &PathBuf,
    dil: &str, // KULLANICIDAN GELEN DİL ("tr", "en", "ja", "es")
) -> Result<(), String> {
    
    let temp_dir = std::env::temp_dir();
    let temp_wav = temp_dir.join(format!("temp_whisper_{}.wav", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros()));

    let mut cmd = std::process::Command::new(ffmpeg_yolu);
    cmd.args([
        "-i", orijinal_ses_yolu.to_str().unwrap(),
        "-ar", "16000",
        "-ac", "1",
        "-c:a", "pcm_s16le",
        "-y",
        temp_wav.to_str().unwrap()
    ]);

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000);
    }

    let status = cmd.status().map_err(|e| format!("FFMPEG başlatılamadı: {}", e))?;
    if !status.success() {
        return Err("FFMPEG ses dönüştürme işlemi başarısız oldu.".to_string());
    }

    let mut reader = hound::WavReader::open(&temp_wav).map_err(|e| format!("Wav okuma hatası: {}", e))?;
    let audio_data: Vec<f32> = reader.samples::<i16>()
        .map(|s| s.unwrap_or(0) as f32 / 32768.0)
        .collect();

    let _ = std::fs::remove_file(&temp_wav);

    let ctx_params = WhisperContextParameters::default();
    let ctx = WhisperContext::new_with_params(model_yolu.to_str().unwrap(), ctx_params)
        .map_err(|e| format!("Model yüklenemedi: {}", e))?;
        
    let mut state = ctx.create_state().map_err(|e| format!("State oluşturulamadı: {}", e))?;

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

    // ARTIK AUTO YOK, DİREKT KULLANICININ SEÇTİĞİ DİL KULLANILIYOR
    params.set_language(Some(dil)); 
    
    params.set_print_progress(false);
    params.set_print_special(false); 
    params.set_no_speech_thold(0.6); 
    params.set_logprob_thold(-1.0);
    params.set_suppress_nst(true);
    params.set_suppress_blank(true);
    params.set_temperature_inc(0.2);

    state.full(params, &audio_data).map_err(|e| format!("Whisper analizi çöktü: {}", e))?;

    let mut srt_file = File::create(hedef_srt_yolu).map_err(|e| format!("SRT dosyası oluşturulamadı: {}", e))?;

    for (index, segment) in state.as_iter().enumerate() {
        let text = segment.to_str().map_err(|e| e.to_string())?; 
        
        let temiz_metin = text.trim();
        if temiz_metin.is_empty() || (temiz_metin.starts_with('[') && temiz_metin.ends_with(']')) {
            continue;
        }

        let start_time = segment.start_timestamp(); 
        let end_time = segment.end_timestamp();

        let baslangic_sn = start_time / 100;
        let baslangic_ms = (start_time % 100) * 10;
        let bitis_sn = end_time / 100;
        let bitis_ms = (end_time % 100) * 10;

        let srt_metni = format!(
            "{}\n{} --> {}\n{}\n\n",
            index + 1,
            srt_zaman_damgasi(baslangic_sn, baslangic_ms),
            srt_zaman_damgasi(bitis_sn, bitis_ms),
            temiz_metin
        );

        srt_file.write_all(srt_metni.as_bytes()).unwrap();
    }

    Ok(())
}


pub fn run() {
    let drpc = DiscordClient::new(1483819416951984128);
    let discord_arc = Arc::new(Mutex::new(drpc));
    let discord_clone = Arc::clone(&discord_arc);

    std::thread::spawn(move || {
        if let Ok(mut client) = discord_clone.lock() {
            let _ = client.start();
        }
    });

    tauri::Builder::default()
        .setup(|app| {
            let db_conn = init_db(app.handle()).expect("Veritabanı başlatılamadı!");
            app.manage(DbState(Arc::new(Mutex::new(db_conn))));
            app.manage(DiscordState(discord_arc));

            let shortcuts = [
                Shortcut::new(None, Code::MediaPlayPause),
                Shortcut::new(None, Code::MediaTrackNext),
                Shortcut::new(None, Code::MediaTrackPrevious),
            ];

            for sc in shortcuts {
                let _ = app.global_shortcut().register(sc);
            }

            let tray_menu = Menu::with_items(
                app,
                &[
                    &MenuItem::with_id(app, "show", "Aç", true, None::<&str>)?,
                    &MenuItem::with_id(app, "exit", "Kapat", true, None::<&str>)?,
                ],
            )?;

            if let Some(icon) = app.default_window_icon().cloned() {
                let _tray = TrayIconBuilder::new()
                    .icon(icon)
                    .menu(&tray_menu)
                    .on_menu_event(|app_handle, event| match event.id.as_ref() {
                        "show" => {
                            if let Some(window) = app_handle.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "exit" => {
                            std::process::exit(0);
                        }
                        _ => {}
                    })
                    .on_tray_icon_event(|tray, event| {
                        if let TrayIconEvent::Click {
                            button: tauri::tray::MouseButton::Left,
                            ..
                        } = event
                        {
                            let app_handle = tray.app_handle();
                            if let Some(window) = app_handle.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    })
                    .build(app)?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app: &AppHandle, shortcut: &Shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        if shortcut.key == Code::MediaPlayPause {
                            let _ = app.emit("media-toggle", ());
                        } else if shortcut.key == Code::MediaTrackNext {
                            let _ = app.emit("media-next", ());
                        } else if shortcut.key == Code::MediaTrackPrevious {
                            let _ = app.emit("media-prev", ());
                        }
                    }
                })
                .build(),
        )
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            sarki_kaydet,
            sarkilari_getir,
            playlist_olustur,
            playlistleri_getir,
            playliste_sarki_ekle,
            favorileri_getir,
            playlistten_sarki_cikar,
            sarki_sirasi_guncelle,
            sarki_sil,
            favori_degistir,
            get_app_data_dir,
            sarki_metadata_oku,
            update_discord_status,
            clear_discord_status,
            dinlenme_sayisi_artir,
            open_data_folder,
            playlist_sil,
            youtube_arama,
            sarki_guncelle,
            youtube_indir,
            playlist_disa_aktar,
            ayarlari_getir,
            ayarlari_kaydet,
            youtube_playlist_getir,
            playlist_sirasi_guncelle,
            yedek_al,
            sarki_sozu_oku,
            mevcut_soz_dillerini_bul,
            yedekten_don
        ])
        .run(tauri::generate_context!())
        .expect("Lain Wave başlatılamadı");
}
