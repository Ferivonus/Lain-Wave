#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use discord_presence::Client as DiscordClient;
use id3::{Tag, TagLike};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use tauri::tray::TrayIconEvent;
use tauri::Emitter;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Shortcut, ShortcutState};

#[derive(Serialize, Deserialize, Clone)]
pub struct Sarki {
    pub id: String,
    pub isim: String,
    pub sarkici: String,
    pub album: String,
    pub yol: String,
    pub kapak_yolu: Option<String>,
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

struct DiscordState(Arc<Mutex<DiscordClient>>);

fn db_yolunu_bul(app: &AppHandle) -> PathBuf {
    let mut yol = app
        .path()
        .app_local_data_dir()
        .expect("Sistem klasörüne erişilemiyor!");
    if !yol.exists() {
        fs::create_dir_all(&yol).expect("Veri klasörü oluşturulamadı!");
    }
    yol.push("kutuphane.json");
    yol
}

fn songs_klasoru_bul(app: &AppHandle) -> PathBuf {
    let mut yol = app
        .path()
        .app_local_data_dir()
        .expect("Sistem klasörüne erişilemiyor!");
    yol.push("songs");
    if !yol.exists() {
        fs::create_dir_all(&yol).expect("Şarkı klasörü oluşturulamadı!");
    }
    yol
}

fn playlists_yolunu_bul(app: &AppHandle) -> PathBuf {
    let mut yol = app
        .path()
        .app_local_data_dir()
        .expect("Sistem klasörüne erişilemiyor!");
    yol.push("playlists.json");
    yol
}

fn favorites_yolunu_bul(app: &AppHandle) -> PathBuf {
    let mut yol = app
        .path()
        .app_local_data_dir()
        .expect("Sistem klasörüne erişilemiyor!");
    yol.push("favoriler.json");
    yol
}

#[tauri::command]
fn sarki_metadata_oku(yol: String) -> Result<MetadataBilgisi, String> {
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
    let db_yolu = db_yolunu_bul(&app);
    let songs_klasoru = songs_klasoru_bul(&app);
    let orijinal_path = Path::new(&yol);

    if !orijinal_path.exists() {
        return Err("Kaynak dosya bulunamadı!".into());
    }

    let uzanti = orijinal_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("mp3");

    let mut sarkilar: Vec<Sarki> = Vec::new();
    if db_yolu.exists() {
        let icerik = fs::read_to_string(&db_yolu).map_err(|e| e.to_string())?;
        sarkilar = serde_json::from_str(&icerik).unwrap_or_default();
    }

    let id = format!(
        "song_{}_{}",
        sarkilar.len() + 1,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    let yeni_dosya_adi = format!("{}.{}", id, uzanti);
    let mut hedef_yol = songs_klasoru.clone();
    hedef_yol.push(&yeni_dosya_adi);

    fs::copy(orijinal_path, &hedef_yol).map_err(|e| format!("Kopyalama hatası: {}", e))?;

    let mut kapak_yolu = None;
    let mut final_tarz = manuel_tarz;
    let mut sure = None;

    if let Ok(tag) = Tag::read_from_path(orijinal_path) {
        if isim.trim().is_empty() {
            if let Some(t) = tag.title() {
                isim = t.to_string();
            }
        }
        if final_tarz.is_none() {
            final_tarz = tag.genre().map(|g| g.to_string());
        }
        if let Some(tlen) = tag.get("TLEN").and_then(|f| f.content().text()) {
            if let Ok(ms) = tlen.parse::<u32>() {
                sure = Some(ms / 1000);
            }
        }
        if let Some(pic) = tag.pictures().next() {
            let pic_ext = if pic.mime_type == "image/png" {
                "png"
            } else {
                "jpg"
            };
            let kapak_adi = format!("{}_cover.{}", id, pic_ext);
            let mut kapak_hedef = songs_klasoru.clone();
            kapak_hedef.push(&kapak_adi);

            if fs::write(&kapak_hedef, &pic.data).is_ok() {
                kapak_yolu = Some(kapak_hedef.to_string_lossy().to_string());
            }
        }
    }

    let yeni_sarki = Sarki {
        id,
        isim,
        sarkici,
        album,
        yol: hedef_yol.to_string_lossy().to_string(),
        kapak_yolu,
        tarz: final_tarz,
        kalite: Some(uzanti.to_uppercase()),
        sure,
        dinlenme_sayisi: Some(0),
        son_dinlenme_tarihi: None,
        yil,
        notlar,
    };

    sarkilar.push(yeni_sarki.clone());
    let yeni_icerik = serde_json::to_string_pretty(&sarkilar).unwrap();
    fs::write(db_yolu, yeni_icerik).map_err(|e| e.to_string())?;

    Ok(yeni_sarki)
}

#[tauri::command]
fn sarkilari_getir(app: AppHandle) -> Result<Vec<Sarki>, String> {
    let db_yolu = db_yolunu_bul(&app);
    if !db_yolu.exists() {
        return Ok(Vec::new());
    }
    let icerik = fs::read_to_string(db_yolu).map_err(|e| e.to_string())?;
    Ok(serde_json::from_str(&icerik).unwrap_or_default())
}

#[tauri::command]
fn playlist_olustur(app: AppHandle, isim: String) -> Result<Playlist, String> {
    let db_yolu = playlists_yolunu_bul(&app);
    let mut listeler: Vec<Playlist> = Vec::new();
    if db_yolu.exists() {
        let icerik = fs::read_to_string(&db_yolu).unwrap_or_else(|_| "[]".to_string());
        listeler = serde_json::from_str(&icerik).unwrap_or_default();
    }
    let id = format!("pl_{}", listeler.len() + 1);
    let yeni_liste = Playlist {
        id,
        isim,
        sarkilar: Vec::new(),
    };

    listeler.push(yeni_liste.clone());
    fs::write(db_yolu, serde_json::to_string_pretty(&listeler).unwrap())
        .map_err(|e| e.to_string())?;

    Ok(yeni_liste)
}

#[tauri::command]
fn playlistleri_getir(app: AppHandle) -> Result<Vec<Playlist>, String> {
    let db_yolu = playlists_yolunu_bul(&app);
    if !db_yolu.exists() {
        return Ok(Vec::new());
    }
    let icerik = fs::read_to_string(&db_yolu).unwrap_or_else(|_| "[]".to_string());
    Ok(serde_json::from_str(&icerik).unwrap_or_default())
}

#[tauri::command]
fn playliste_sarki_ekle(
    app: AppHandle,
    playlist_id: String,
    sarki_id: String,
) -> Result<Playlist, String> {
    let db_yolu = playlists_yolunu_bul(&app);
    if !db_yolu.exists() {
        return Err("Veritabanı bulunamadı.".into());
    }

    let icerik = fs::read_to_string(&db_yolu).unwrap_or_else(|_| "[]".to_string());
    let mut listeler: Vec<Playlist> = serde_json::from_str(&icerik).unwrap_or_default();

    let mut guncellenmis_liste = None;
    if let Some(liste) = listeler.iter_mut().find(|p| p.id == playlist_id) {
        if !liste.sarkilar.contains(&sarki_id) {
            liste.sarkilar.push(sarki_id);
        }
        guncellenmis_liste = Some(liste.clone());
    }

    if let Some(liste) = guncellenmis_liste {
        fs::write(db_yolu, serde_json::to_string_pretty(&listeler).unwrap())
            .map_err(|e| e.to_string())?;
        Ok(liste)
    } else {
        Err("Belirtilen çalma listesi bulunamadı.".into())
    }
}

#[tauri::command]
fn favorileri_getir(app: AppHandle) -> Result<Vec<String>, String> {
    let db_yolu = favorites_yolunu_bul(&app);
    if !db_yolu.exists() {
        return Ok(Vec::new());
    }
    let icerik = fs::read_to_string(&db_yolu).unwrap_or_else(|_| "[]".to_string());
    Ok(serde_json::from_str(&icerik).unwrap_or_default())
}

#[tauri::command]
fn favori_degistir(app: AppHandle, sarki_id: String) -> Result<Vec<String>, String> {
    let db_yolu = favorites_yolunu_bul(&app);
    let mut favoriler: Vec<String> = Vec::new();

    if db_yolu.exists() {
        let icerik = fs::read_to_string(&db_yolu).unwrap_or_else(|_| "[]".to_string());
        favoriler = serde_json::from_str(&icerik).unwrap_or_default();
    }

    if favoriler.contains(&sarki_id) {
        favoriler.retain(|id| id != &sarki_id);
    } else {
        favoriler.push(sarki_id);
    }

    fs::write(db_yolu, serde_json::to_string_pretty(&favoriler).unwrap())
        .map_err(|e| e.to_string())?;
    Ok(favoriler)
}

#[tauri::command]
fn playlistten_sarki_cikar(
    app: AppHandle,
    playlist_id: String,
    sarki_id: String,
) -> Result<Playlist, String> {
    let db_yolu = playlists_yolunu_bul(&app);
    if !db_yolu.exists() {
        return Err("Veritabanı yok".into());
    }

    let icerik = fs::read_to_string(&db_yolu).unwrap_or_else(|_| "[]".to_string());
    let mut listeler: Vec<Playlist> = serde_json::from_str(&icerik).unwrap_or_default();

    let mut guncellenmis_liste = None;
    if let Some(liste) = listeler.iter_mut().find(|p| p.id == playlist_id) {
        liste.sarkilar.retain(|id| id != &sarki_id);
        guncellenmis_liste = Some(liste.clone());
    }

    if let Some(liste) = guncellenmis_liste {
        fs::write(db_yolu, serde_json::to_string_pretty(&listeler).unwrap())
            .map_err(|e| e.to_string())?;
        Ok(liste)
    } else {
        Err("Kaldırılacak çalma listesi bulunamadı.".into())
    }
}

#[tauri::command]
fn sarki_sil(app: AppHandle, sarki_id: String) -> Result<(), String> {
    let db_yolu = db_yolunu_bul(&app);
    if !db_yolu.exists() {
        return Err("Veritabanı yok".into());
    }

    let icerik = fs::read_to_string(&db_yolu).unwrap_or_else(|_| "[]".to_string());
    let mut sarkilar: Vec<Sarki> = serde_json::from_str(&icerik).unwrap_or_default();

    if let Some(silinecek_sarki) = sarkilar.iter().find(|s| s.id == sarki_id) {
        let _ = fs::remove_file(&silinecek_sarki.yol);
        if let Some(kapak) = &silinecek_sarki.kapak_yolu {
            let _ = fs::remove_file(kapak);
        }
    }

    sarkilar.retain(|s| s.id != sarki_id);
    fs::write(&db_yolu, serde_json::to_string_pretty(&sarkilar).unwrap())
        .map_err(|e| format!("Kütüphane güncellenemedi: {}", e))?;

    let favoriler_yolu = favorites_yolunu_bul(&app);
    if favoriler_yolu.exists() {
        let fav_icerik = fs::read_to_string(&favoriler_yolu).unwrap_or_else(|_| "[]".to_string());
        let mut favoriler: Vec<String> = serde_json::from_str(&fav_icerik).unwrap_or_default();
        if favoriler.contains(&sarki_id) {
            favoriler.retain(|id| id != &sarki_id);
            let _ = fs::write(
                favoriler_yolu,
                serde_json::to_string_pretty(&favoriler).unwrap(),
            );
        }
    }

    let playlists_yolu = playlists_yolunu_bul(&app);
    if playlists_yolu.exists() {
        let pl_icerik = fs::read_to_string(&playlists_yolu).unwrap_or_else(|_| "[]".to_string());
        let mut listeler: Vec<Playlist> = serde_json::from_str(&pl_icerik).unwrap_or_default();
        let mut degisiklik_oldu = false;
        for liste in listeler.iter_mut() {
            if liste.sarkilar.contains(&sarki_id) {
                liste.sarkilar.retain(|id| id != &sarki_id);
                degisiklik_oldu = true;
            }
        }
        if degisiklik_oldu {
            let _ = fs::write(
                playlists_yolu,
                serde_json::to_string_pretty(&listeler).unwrap(),
            );
        }
    }

    Ok(())
}

#[tauri::command]
fn sarki_sirasi_guncelle(app: AppHandle, yeni_liste: Vec<Sarki>) -> Result<(), String> {
    let db_yolu = db_yolunu_bul(&app);
    fs::write(db_yolu, serde_json::to_string_pretty(&yeni_liste).unwrap())
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_app_data_dir(app: AppHandle) -> String {
    app.path()
        .app_local_data_dir()
        .unwrap()
        .to_string_lossy()
        .to_string()
}

#[tauri::command]
fn open_data_folder(app: AppHandle) {
    let path = app.path().app_local_data_dir().unwrap();
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer").arg(path).spawn().unwrap();
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(path).spawn().unwrap();
    }
}

#[tauri::command]
fn update_discord_status(
    state: tauri::State<'_, DiscordState>,
    detay: String,
    durum: String,
    toplam_saniye: u64,
) -> Result<(), String> {
    let mut drpc = state.0.lock().unwrap();
    drpc.set_activity(|mut a| {
        a = a
            .details(detay)
            .state(durum)
            .assets(|ass| ass.large_image("icon"));
        if toplam_saniye > 0 {
            let start_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let end_time = start_time + toplam_saniye;
            a = a.timestamps(|t| t.start(start_time).end(end_time));
        }
        a
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn clear_discord_status(state: tauri::State<'_, DiscordState>) -> Result<(), String> {
    let mut drpc = state.0.lock().unwrap();
    drpc.clear_activity().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn dinlenme_sayisi_artir(
    app: AppHandle,
    sarki_id: String,
    tarih: u64,
) -> Result<(u32, u64), String> {
    let db_yolu = db_yolunu_bul(&app);
    if !db_yolu.exists() {
        return Err("Veritabanı yok".into());
    }

    let icerik = fs::read_to_string(&db_yolu).unwrap_or_else(|_| "[]".to_string());
    let mut sarkilar: Vec<Sarki> = serde_json::from_str(&icerik).unwrap_or_default();

    let yeni_sayi = if let Some(sarki) = sarkilar.iter_mut().find(|s| s.id == sarki_id) {
        let sayi = sarki.dinlenme_sayisi.unwrap_or(0) + 1;
        sarki.dinlenme_sayisi = Some(sayi);
        sarki.son_dinlenme_tarihi = Some(tarih);
        sayi
    } else {
        return Err("Şarkı bulunamadı".into());
    };

    fs::write(db_yolu, serde_json::to_string_pretty(&sarkilar).unwrap())
        .map_err(|e| format!("Kaydetme hatası: {}", e))?;
    Ok((yeni_sayi, tarih))
}

#[tauri::command]
fn playlist_sil(app: AppHandle, playlist_id: String) -> Result<(), String> {
    let db_yolu = playlists_yolunu_bul(&app);
    if !db_yolu.exists() {
        return Err("Veritabanı yok".into());
    }
    let icerik = fs::read_to_string(&db_yolu).unwrap_or_else(|_| "[]".to_string());
    let mut listeler: Vec<Playlist> = serde_json::from_str(&icerik).unwrap_or_default();
    listeler.retain(|p| p.id != playlist_id);
    fs::write(db_yolu, serde_json::to_string_pretty(&listeler).unwrap())
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn youtube_indir(app: tauri::AppHandle, url: String, tarz: String) -> Result<Sarki, String> {
    let db_yolu = db_yolunu_bul(&app);
    let songs_klasoru = songs_klasoru_bul(&app);

    let mut temiz_url = url.clone();

    if let Some(pos) = temiz_url.find("&list=") {
        temiz_url.truncate(pos);
    }
    if let Some(pos) = temiz_url.find("?list=") {
        temiz_url.truncate(pos);
    }
    if let Some(pos) = temiz_url.find("&index=") {
        temiz_url.truncate(pos);
    }

    let (yt_dlp_path, ffmpeg_path) = {
        let exe_path = std::env::current_exe()
            .map(|p| p.parent().map(|parent| parent.to_path_buf()))
            .ok()
            .flatten()
            .unwrap_or_default();
        let resource_dir = app.path().resource_dir().unwrap_or_default();
        let current_dir = std::env::current_dir().unwrap_or_default();

        let olasi_yollar = vec![
            exe_path.join("binaries").join("yt-dlp.exe"),
            resource_dir.join("binaries").join("yt-dlp.exe"),
            current_dir.join("binaries").join("yt-dlp.exe"),
            current_dir
                .join("src-tauri")
                .join("binaries")
                .join("yt-dlp.exe"),
        ];

        let found = olasi_yollar.into_iter().find(|p| p.exists());
        match found {
            Some(yt_path) => {
                let ff_path = yt_path.parent().unwrap().join("ffmpeg.exe");
                (yt_path, ff_path)
            }
            None => return Err("İndirme araçları bulunamadı!".into()),
        }
    };

    if !ffmpeg_path.exists() {
        return Err("ffmpeg.exe bulunamadı!".into());
    }

    let mut sarkilar: Vec<Sarki> = if db_yolu.exists() {
        let icerik = std::fs::read_to_string(&db_yolu).unwrap_or_else(|_| "[]".to_string());
        serde_json::from_str(&icerik).unwrap_or_default()
    } else {
        Vec::new()
    };

    let id = format!(
        "yt_{}_{}",
        sarkilar.len() + 1,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    let yt_dlp_hedef = songs_klasoru.join(format!("{}.%(ext)s", id));
    let hedef_ses_yolu = songs_klasoru.join(format!("{}.wav", id));
    let hedef_kapak_yolu = songs_klasoru.join(format!("{}.jpg", id));

    let mut cmd = std::process::Command::new(&yt_dlp_path);

    cmd.arg("--quiet")
        .arg("--no-warnings")
        .arg("--no-playlist")
        .arg("--newline")
        .arg("--progress")
        .arg("--no-simulate")
        .arg("-f")
        .arg("bestaudio/best")
        .arg("-x")
        .arg("--audio-format")
        .arg("wav")
        .arg("--audio-quality")
        .arg("0")
        .arg("--write-thumbnail")
        .arg("--ppa")
        .arg("ThumbnailsConvertor:-q:v 2")
        .arg("--convert-thumbnails")
        .arg("jpg")
        .arg("--ffmpeg-location")
        .arg(&ffmpeg_path)
        .arg("--print")
        .arg("%(title)s|*|%(uploader)s|*|%(duration)s")
        .arg("-o")
        .arg(&yt_dlp_hedef)
        .arg(&temiz_url)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    #[cfg(target_os = "windows")]
    use std::os::windows::process::CommandExt;

    #[cfg(target_os = "windows")]
    cmd.creation_flags(0x08000000);

    let mut child = cmd
        .spawn()
        .map_err(|e| format!("Süreç başlatılamadı: {}", e))?;

    let stdout = child.stdout.take().ok_or("Stdout alınamadı")?;
    let reader = std::io::BufReader::new(stdout);
    let mut metadata_line = String::new();

    use std::io::BufRead;
    for line in reader.lines() {
        let l = line.unwrap_or_default();
        if l.contains("|*|") {
            metadata_line = l.clone();
        }
        if l.contains("%") && l.contains("[download]") {
            let parts: Vec<&str> = l.split_whitespace().collect();
            if let Some(pct_str) = parts.get(1) {
                let clean_pct = pct_str.replace("%", "");
                if let Ok(pct) = clean_pct.parse::<f32>() {
                    let _ = app.emit(
                        "download-progress",
                        serde_json::json!({
                            "percentage": pct,
                            "speed": parts.get(7).unwrap_or(&"0KiB/s"),
                            "eta": parts.get(9).unwrap_or(&"00:00")
                        }),
                    );
                }
            }
        }
    }

    let _ = child.wait();

    let parcalar: Vec<&str> = metadata_line.split("|*|").collect();
    let isim = parcalar
        .get(0)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Bilinmeyen Parça".to_string());
    let sarkici = parcalar
        .get(1)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "YouTube".to_string());
    let sure: Option<u32> = parcalar
        .get(2)
        .and_then(|s| s.trim().parse::<f64>().ok())
        .map(|v| v as u32);

    let mut kapak_yolu = None;
    if hedef_kapak_yolu.exists() {
        kapak_yolu = Some(hedef_kapak_yolu.to_string_lossy().to_string());
    } else {
        for uzanti in vec!["webp", "png", "jpeg"] {
            let alternatif = songs_klasoru.join(format!("{}.{}", id, uzanti));
            if alternatif.exists() {
                kapak_yolu = Some(alternatif.to_string_lossy().to_string());
                break;
            }
        }
    }

    let yeni_sarki = Sarki {
        id,
        isim,
        sarkici,
        album: "YouTube Arşivi".to_string(),
        yol: hedef_ses_yolu.to_string_lossy().to_string(),
        kapak_yolu,
        tarz: Some(tarz),
        kalite: Some("WAV (Kayıpsız)".to_string()),
        sure,
        dinlenme_sayisi: Some(0),
        son_dinlenme_tarihi: None,
        yil: None,
        notlar: Some(temiz_url),
    };

    sarkilar.push(yeni_sarki.clone());
    std::fs::write(
        db_yolunu_bul(&app),
        serde_json::to_string_pretty(&sarkilar).unwrap(),
    )
    .map_err(|e| e.to_string())?;

    Ok(yeni_sarki)
}
#[tauri::command]
async fn youtube_arama(app: tauri::AppHandle, sorgu: String) -> Result<Vec<YouTubeSonuc>, String> {
    let arama_kodu = format!("ytsearch20:{}", sorgu);

    let yt_dlp_path = {
        let exe_path = std::env::current_exe()
            .map(|p| p.parent().map(|parent| parent.to_path_buf()))
            .ok()
            .flatten()
            .unwrap_or_default();
        let resource_dir = app.path().resource_dir().unwrap_or_default();
        let current_dir = std::env::current_dir().unwrap_or_default();

        let olasi_yollar = vec![
            exe_path.join("binaries").join("yt-dlp.exe"),
            resource_dir.join("binaries").join("yt-dlp.exe"),
            current_dir.join("binaries").join("yt-dlp.exe"),
            current_dir
                .join("src-tauri")
                .join("binaries")
                .join("yt-dlp.exe"),
        ];

        olasi_yollar
            .into_iter()
            .find(|p| p.exists())
            .ok_or_else(|| "Arama motoru (yt-dlp) bulunamadı!".to_string())?
    };

    let mut cmd = std::process::Command::new(&yt_dlp_path);
    cmd.args([
        "--dump-json",
        "--default-search",
        "ytsearch",
        "--no-playlist",
        &arama_kodu,
    ]);

    #[cfg(target_os = "windows")]
    use std::os::windows::process::CommandExt;

    #[cfg(target_os = "windows")]
    cmd.creation_flags(0x08000000);

    let output = cmd
        .output()
        .map_err(|e| format!("Arama başlatılamadı: {}", e))?;

    if !output.status.success() {
        return Err("Arama sırasında bir hata oluştu.".to_string());
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut sonuclar = Vec::new();

    for line in output_str.lines() {
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
}

#[tauri::command]
fn sarki_guncelle(
    app: AppHandle,
    id: String,
    isim: String,
    sarkici: String,
    album: String,
    tarz: Option<String>,
    yil: Option<u32>,
) -> Result<Sarki, String> {
    let db_yolu = db_yolunu_bul(&app);
    if !db_yolu.exists() {
        return Err("Veritabanı yok".into());
    }

    let icerik = std::fs::read_to_string(&db_yolu).unwrap_or_else(|_| "[]".to_string());
    let mut sarkilar: Vec<Sarki> = serde_json::from_str(&icerik).unwrap_or_default();

    if let Some(sarki) = sarkilar.iter_mut().find(|s| s.id == id) {
        sarki.isim = isim;
        sarki.sarkici = sarkici;
        sarki.album = album;
        sarki.tarz = tarz;
        sarki.yil = yil;

        let guncel_sarki = sarki.clone();

        std::fs::write(&db_yolu, serde_json::to_string_pretty(&sarkilar).unwrap())
            .map_err(|e| format!("Kütüphane güncellenemedi: {}", e))?;

        Ok(guncel_sarki)
    } else {
        Err("Şarkı bulunamadı".into())
    }
}

#[tauri::command]
async fn playlist_disa_aktar(
    app: tauri::AppHandle,
    playlist_adi: String,
    icerik: String,
) -> Result<String, String> {
    use std::path::PathBuf;

    let app_dir = app
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."));

    let settings_path = app
        .path()
        .app_local_data_dir()
        .unwrap_or_default()
        .join("settings.json");
    let kullanici_adi = if settings_path.exists() {
        let ayarlar_str =
            std::fs::read_to_string(&settings_path).unwrap_or_else(|_| "{}".to_string());
        if let Ok(ayarlar_json) = serde_json::from_str::<serde_json::Value>(&ayarlar_str) {
            let ad = ayarlar_json["kullanici_adi"]
                .as_str()
                .unwrap_or("Anonim")
                .trim();
            if ad.is_empty() {
                "Anonim".to_string()
            } else {
                ad.to_string()
            }
        } else {
            "Anonim".to_string()
        }
    } else {
        "Anonim".to_string()
    };

    let export_dir = app_dir.join("paylasima_uygun_playlistler");
    if !export_dir.exists() {
        std::fs::create_dir_all(&export_dir)
            .map_err(|e| format!("Klasör oluşturulamadı: {}", e))?;
    }

    let gecersiz_karakterler = &['/', '\\', ':', '*', '?', '"', '<', '>', '|'][..];
    let guvenli_isim = playlist_adi.replace(gecersiz_karakterler, "_");
    let guvenli_kullanici = kullanici_adi.replace(gecersiz_karakterler, "_");

    let dosya_yolu = export_dir.join(format!("{} - {}.json", guvenli_kullanici, guvenli_isim));

    std::fs::write(&dosya_yolu, icerik).map_err(|e| format!("Dosya yazılamadı: {}", e))?;

    Ok(dosya_yolu.to_string_lossy().to_string())
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

fn settings_yolunu_bul(app: &AppHandle) -> PathBuf {
    let mut yol = app.path().app_local_data_dir().unwrap();
    if !yol.exists() {
        let _ = fs::create_dir_all(&yol);
    }
    yol.push("settings.json");
    yol
}

#[tauri::command]
fn ayarlari_getir(app: AppHandle) -> Result<Ayarlar, String> {
    let db_yolu = settings_yolunu_bul(&app);
    if !db_yolu.exists() {
        return Ok(Ayarlar::default());
    }
    let icerik = fs::read_to_string(&db_yolu).unwrap_or_else(|_| "{}".to_string());
    Ok(serde_json::from_str(&icerik).unwrap_or_default())
}

#[tauri::command]
fn ayarlari_kaydet(app: AppHandle, ayarlar: Ayarlar) -> Result<(), String> {
    let db_yolu = settings_yolunu_bul(&app);
    let icerik = serde_json::to_string_pretty(&ayarlar).map_err(|e| e.to_string())?;
    fs::write(db_yolu, icerik).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn run() {
    let drpc = DiscordClient::new(1483819416951984128);
    let discord_arc = Arc::new(Mutex::new(drpc));
    let discord_clone = Arc::clone(&discord_arc);

    std::thread::spawn(move || {
        let mut client = discord_clone.lock().unwrap();
        let _ = client.start();
    });

    tauri::Builder::default()
        .manage(DiscordState(discord_arc))
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
        .setup(|app| {
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

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
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

            Ok(())
        })
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
            ayarlari_kaydet
        ])
        .run(tauri::generate_context!())
        .expect("Lain Wave başlatılamadı");
}
