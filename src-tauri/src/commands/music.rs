use id3::{Tag, TagLike};
use rusqlite::params;
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Manager};

// Projemizin diğer dosyalarındaki yapıları içeri aktarıyoruz
use crate::db::DbState;
use crate::models::{AvailableLanguage, MetadataBilgisi, Sarki};
use crate::utils::songs_klasoru_bul;

// Veritabanı satırını 'Sarki' modelimize dönüştüren yardımcı fonksiyon
pub fn row_to_sarki(row: &rusqlite::Row) -> rusqlite::Result<Sarki> {
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
pub async fn sarki_metadata_oku(yol: String) -> Result<MetadataBilgisi, String> {
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
pub async fn sarki_kaydet(
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

        let uzanti = orijinal_path.extension().and_then(|e| e.to_str()).unwrap_or("mp3");
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|_| "Zaman hesaplama hatası!")?.as_nanos();
        let id = format!("song_{}", timestamp);

        let yeni_dosya_adi = format!("{}.{}", id, uzanti);
        let mut hedef_yol = songs_klasoru.clone();
        hedef_yol.push(&yeni_dosya_adi);

        fs::copy(orijinal_path, &hedef_yol).map_err(|e| format!("Kopyalama hatası: {}", e))?;

        let mut kapak_yolu = None;
        let mut final_tarz = manuel_tarz;
        let mut sure = None;

        if let Ok(tag) = Tag::read_from_path(orijinal_path) {
            if isim.trim().is_empty() { if let Some(t) = tag.title() { isim = t.to_string(); } }
            if final_tarz.is_none() { final_tarz = tag.genre().map(|g| g.to_string()); }
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
            id: id.clone(), isim: isim.clone(), sarkici: sarkici.clone(), album: album.clone(),
            yol: hedef_yol.to_string_lossy().to_string(), kapak_yolu: kapak_yolu.clone(),
            sozler_yolu: None, tarz: final_tarz.clone(), kalite: Some(uzanti.to_uppercase()),
            sure, dinlenme_sayisi: Some(0), son_dinlenme_tarihi: None, yil, notlar: notlar.clone(),
        };

        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;
        let sira_sorgu: i32 = conn.query_row("SELECT COALESCE(MAX(sira), 0) + 1 FROM sarkilar", [], |r| r.get(0)).unwrap_or(0);

        conn.execute(
            "INSERT INTO sarkilar (id, isim, sarkici, album, yol, kapak_yolu, sozler_yolu, tarz, kalite, sure, dinlenme_sayisi, son_dinlenme_tarihi, yil, notlar, sira)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![yeni_sarki.id, yeni_sarki.isim, yeni_sarki.sarkici, yeni_sarki.album, yeni_sarki.yol, yeni_sarki.kapak_yolu, yeni_sarki.sozler_yolu, yeni_sarki.tarz, yeni_sarki.kalite, yeni_sarki.sure, yeni_sarki.dinlenme_sayisi, yeni_sarki.son_dinlenme_tarihi.map(|v| v as i64), yeni_sarki.yil, yeni_sarki.notlar, sira_sorgu],
        ).map_err(|e| format!("Veritabanına eklenemedi: {}", e))?;

        Ok(yeni_sarki)
    }).await.map_err(|e| format!("İşlem hatası: {}", e))?
}

#[tauri::command]
pub async fn sarkilari_getir(app: AppHandle) -> Result<Vec<Sarki>, String> {
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
pub async fn sarki_sil(app: AppHandle, sarki_id: String) -> Result<(), String> {
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
pub async fn sarki_sirasi_guncelle(app: AppHandle, yeni_liste: Vec<Sarki>) -> Result<(), String> {
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
pub async fn sarki_guncelle(
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
        conn.execute("UPDATE sarkilar SET isim = ?1, sarkici = ?2, album = ?3, tarz = ?4, yil = ?5 WHERE id = ?6", params![isim, sarkici, album, tarz, yil, id]).map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare("SELECT * FROM sarkilar WHERE id = ?").map_err(|e| e.to_string())?;
        let sarki = stmt.query_row(params![id], row_to_sarki).map_err(|_| "Güncellenen şarkı bulunamadı")?;
        Ok(sarki)
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn dinlenme_sayisi_artir(
    app: AppHandle,
    sarki_id: String,
    tarih: u64,
) -> Result<(u32, u64), String> {
    let db_arc = app.state::<DbState>().0.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let conn = db_arc.lock().map_err(|_| "Veritabanı kilitlendi!")?;
        conn.execute("UPDATE sarkilar SET dinlenme_sayisi = dinlenme_sayisi + 1, son_dinlenme_tarihi = ? WHERE id = ?", params![tarih as i64, sarki_id]).map_err(|e| e.to_string())?;
        let yeni_sayi: u32 = conn.query_row("SELECT dinlenme_sayisi FROM sarkilar WHERE id = ?", params![sarki_id], |row| row.get(0)).unwrap_or(1);
        Ok((yeni_sayi, tarih))
    }).await.map_err(|e| format!("İşlem hatası: {}", e))?
}

#[tauri::command]
pub async fn sarki_sozu_oku(yol: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        std::fs::read_to_string(&yol).map_err(|e| format!("Dosya okuma hatası: {}", e))
    })
    .await
    .map_err(|e| format!("Thread hatası: {}", e))?
}

#[tauri::command]
pub async fn mevcut_soz_dillerini_bul(yol: String) -> Result<Vec<AvailableLanguage>, String> {
    let path = Path::new(&yol);
    let parent = path.parent().ok_or("Klasör bulunamadı")?;
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .and_then(|s| s.split('.').next())
        .ok_or("Dosya adı geçersiz")?;

    let mut diller = Vec::new();
    if let Ok(entries) = std::fs::read_dir(parent) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with(file_stem) && name.ends_with(".srt") {
                let dil_kodu = name
                    .replace(file_stem, "")
                    .replace(".srt", "")
                    .replace(".", "")
                    .to_uppercase();
                let dil_adi = if dil_kodu.is_empty() {
                    "ORİJİNAL".to_string()
                } else {
                    dil_kodu
                };
                diller.push(AvailableLanguage {
                    dil: dil_adi,
                    yol: entry.path().to_string_lossy().to_string(),
                });
            }
        }
    }
    diller.sort_by(|a, b| a.dil.cmp(&b.dil));
    Ok(diller)
}
