use tauri::{AppHandle, Manager, Emitter};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use rusqlite::params;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

// Modeller ve veritabanı bağlantısı
use crate::models::{Sarki, YouTubeSonuc};
use crate::db::DbState;
use crate::utils::{app_data_dir, songs_klasoru_bul};

// Whisper için zaman damgası üreten yardımcı fonksiyon
fn srt_zaman_damgasi(saniye: i64, milisaniye: i64) -> String {
    let saat = saniye / 3600;
    let dakika = (saniye % 3600) / 60;
    let sn = saniye % 60;
    format!("{:02}:{:02}:{:02},{:03}",saat,dakika,sn,milisaniye)
}

// FFMPEG ile sesi işleyip Whisper ile altyazı üreten iç fonksiyon
fn whisper_altyazi_uret(
    orijinal_ses_yolu: &PathBuf, hedef_srt_yolu: &PathBuf, model_yolu: &PathBuf, ffmpeg_yolu: &PathBuf, dil: &str, 
) -> Result<(), String> {
    let temp_dir = std::env::temp_dir();
    let temp_wav = temp_dir.join(format!("temp_whisper_{}.wav", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros()));

    let mut cmd = std::process::Command::new(ffmpeg_yolu);
    cmd.args(["-i", orijinal_ses_yolu.to_str().unwrap(), "-ar", "16000", "-ac", "1", "-c:a", "pcm_s16le", "-y", temp_wav.to_str().unwrap()]);

    #[cfg(target_os = "windows")]
    { use std::os::windows::process::CommandExt; cmd.creation_flags(0x08000000); }

    let status = cmd.status().map_err(|e| format!("FFMPEG başlatılamadı: {}", e))?;
    if !status.success() { return Err("FFMPEG ses dönüştürme işlemi başarısız oldu.".to_string()); }

    let mut reader = hound::WavReader::open(&temp_wav).map_err(|e| format!("Wav okuma hatası: {}", e))?;
    let audio_data: Vec<f32> = reader.samples::<i16>().map(|s| s.unwrap_or(0) as f32 / 32768.0).collect();
    let _ = std::fs::remove_file(&temp_wav);

    let ctx_params = WhisperContextParameters::default();
    let ctx = WhisperContext::new_with_params(model_yolu.to_str().unwrap(), ctx_params).map_err(|e| format!("Model yüklenemedi: {}", e))?;
    let mut state = ctx.create_state().map_err(|e| format!("State oluşturulamadı: {}", e))?;

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
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
        let text = segment
            .to_str()
            .map_err(|_| "Whisper segment metni okunamadı.".to_string())?;
        let temiz_metin = text.trim();
        if temiz_metin.is_empty() || (temiz_metin.starts_with('[') && temiz_metin.ends_with(']')) { continue; }

        let start_time = segment.start_timestamp(); 
        let end_time = segment.end_timestamp();
        let baslangic_sn = start_time / 100;
        let baslangic_ms = (start_time % 100) * 10;
        let bitis_sn = end_time / 100;
        let bitis_ms = (end_time % 100) * 10;

        let srt_metni = format!("{}\n{} --> {}\n{}\n\n", index + 1, srt_zaman_damgasi(baslangic_sn, baslangic_ms), srt_zaman_damgasi(bitis_sn, bitis_ms), temiz_metin);
        srt_file.write_all(srt_metni.as_bytes()).unwrap();
    }
    Ok(())
}

#[tauri::command]
pub async fn youtube_indir(
    app: AppHandle, url: String, tarz: String, dil: String, youtube_cevirisi_kullan: bool, yapay_zeka_kullan: bool
) -> Result<Sarki, String> {
    let app_clone = app.clone();
    let db_arc = app.state::<DbState>().0.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let songs_klasoru = songs_klasoru_bul(&app_clone)?;
        let local_data = app_data_dir(&app_clone)?;
        let cache_dir = local_data.join("yt_cache");

        if !cache_dir.exists() { std::fs::create_dir_all(&cache_dir).map_err(|e| format!("Cache hatası: {}", e))?; }

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
        let hedef_kapak_yolu = songs_klasoru.join(format!("{}.webp", id)); 

        let mut cmd = std::process::Command::new(&yt_dlp_path);
        cmd.env("PYTHONIOENCODING", "utf-8").env("PATH", "C:\\Windows\\System32;C:\\Windows").current_dir(&songs_klasoru);
        cmd.arg("--no-warnings").arg("--no-playlist").arg("--newline").arg("--progress").arg("--no-simulate").arg("--ignore-errors")
           .arg("--cache-dir").arg(&cache_dir).arg("-f").arg("bestaudio/best").arg("-x").arg("--audio-format").arg("flac").arg("--audio-quality").arg("0");

        if youtube_cevirisi_kullan { cmd.arg("--write-sub").arg("--write-auto-sub").arg("--sub-langs").arg(&dil).arg("--convert-subs").arg("srt"); }

        cmd.arg("--write-thumbnail").arg("--ppa").arg("ThumbnailsConvertor:-q:v 2").arg("--convert-thumbnails").arg("webp").arg("--ffmpeg-location").arg(&ffmpeg_path);
        cmd.arg("--print").arg("%(title)s|*|%(uploader)s|*|%(duration)s").arg("-o").arg(&yt_dlp_hedef).arg("--").arg(&temiz_url)
           .stdin(std::process::Stdio::null()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped());

        #[cfg(target_os = "windows")]
        { use std::os::windows::process::CommandExt; cmd.creation_flags(0x08000000); }

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
        if !status.success() && !hedef_ses_yolu.exists() { return Err(format!("İndirme tamamlanamadı. Çıkış Kodu: {:?}", status.code())); }

        let mut final_sozler_yolu = None;
        if youtube_cevirisi_kullan || yapay_zeka_kullan {
            if youtube_cevirisi_kullan {
                let mut bulunan_srtler = Vec::new();
                if let Ok(entries) = std::fs::read_dir(&songs_klasoru) {
                    for entry in entries.flatten() {
                        let file_name = entry.file_name().to_string_lossy().to_string();
                        if file_name.starts_with(&id) && file_name.ends_with(".srt") { bulunan_srtler.push(entry.path()); }
                    }
                }
                let aranan_uzanti = format!(".{}.srt", dil); 
                if let Some(srt_yolu) = bulunan_srtler.iter().find(|p| p.to_string_lossy().contains(&aranan_uzanti)) {
                    final_sozler_yolu = Some(srt_yolu.to_string_lossy().to_string());
                } else if !bulunan_srtler.is_empty() {
                    final_sozler_yolu = Some(bulunan_srtler[0].to_string_lossy().to_string());
                }
            }

            if final_sozler_yolu.is_none() && yapay_zeka_kullan && hedef_ses_yolu.exists() {
                let _ = app_clone.emit("download-progress", serde_json::json!({ "percentage": 99.0, "speed": "AI PROCESSING", "eta": "Whisper" }));
                let yapay_zeka_srt_yolu = songs_klasoru.join(format!("{}.srt", id));
                if let Ok(_) = whisper_altyazi_uret(&hedef_ses_yolu, &yapay_zeka_srt_yolu, &model_yolu, &ffmpeg_path, &dil) {
                    final_sozler_yolu = Some(yapay_zeka_srt_yolu.to_string_lossy().to_string());
                }
            }
        }

        let parcalar: Vec<&str> = metadata_line.split("|*|").collect();
        let isim = parcalar.get(0).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| "Bilinmeyen Parça".to_string());
        let sarkici = parcalar.get(1).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| "YouTube".to_string());
        let sure: Option<u32> = parcalar.get(2).and_then(|s| s.trim().parse::<f64>().ok()).map(|v| v as u32);
        let mut kapak_yolu = None;
        if hedef_kapak_yolu.exists() { kapak_yolu = Some(hedef_kapak_yolu.to_string_lossy().to_string()); }

        let yeni_sarki = Sarki {
            id: id.clone(), isim, sarkici, album: "YouTube Arşivi".to_string(), yol: hedef_ses_yolu.to_string_lossy().to_string(), 
            kapak_yolu, sozler_yolu: final_sozler_yolu, tarz: Some(tarz), kalite: Some("FLAC (Kayıpsız)".to_string()), 
            sure, dinlenme_sayisi: Some(0), son_dinlenme_tarihi: None, yil: None, notlar: Some(temiz_url),
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
pub async fn youtube_arama(app: AppHandle, sorgu: String) -> Result<Vec<YouTubeSonuc>, String> {
    let app_clone = app.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let cache_dir = app_data_dir(&app_clone)?.join("yt_cache");
        if !cache_dir.exists() { std::fs::create_dir_all(&cache_dir).map_err(|e| e.to_string())?; }

        let arama_kodu = format!("ytsearch20:{}", sorgu);
        let (yt_dlp_path, binaries_dir) = {
            let exe_path = std::env::current_exe().map(|p| p.parent().map(|p| p.to_path_buf()).unwrap_or_default()).unwrap_or_default();
            let resource_dir = app_clone.path().resource_dir().unwrap_or_default();
            let current_dir = std::env::current_dir().unwrap_or_default();
            let olasi_yollar = vec![
                exe_path.join("binaries").join("yt-dlp.exe"), resource_dir.join("binaries").join("yt-dlp.exe"),
                current_dir.join("src-tauri").join("binaries").join("yt-dlp.exe"), current_dir.join("binaries").join("yt-dlp.exe"),
            ];
            let yt_path = olasi_yollar.into_iter().find(|p| p.exists()).ok_or_else(|| "Araç bulunamadı".to_string())?;
            let bin_dir = yt_path.parent().unwrap_or(Path::new("")).to_path_buf();
            (yt_path, bin_dir)
        };

        let mut cmd = std::process::Command::new(&yt_dlp_path);
        cmd.env("PYTHONIOENCODING", "utf-8").env("PATH", "C:\\Windows\\System32;C:\\Windows").current_dir(&cache_dir);
        cmd.args(["--no-warnings", "--cache-dir", cache_dir.to_str().unwrap_or(""), "--ffmpeg-location", binaries_dir.to_str().unwrap_or(""), "--dump-json", "--default-search", "ytsearch", "--no-playlist", "--", &arama_kodu])
           .stdin(std::process::Stdio::null()).stderr(std::process::Stdio::piped());

        #[cfg(target_os = "windows")]
        { use std::os::windows::process::CommandExt; cmd.creation_flags(0x08000000); }

        let output = cmd.output().map_err(|e| e.to_string())?;
        if !output.status.success() { return Err(format!("Arama başarısız: {}", String::from_utf8_lossy(&output.stderr))); }

        let mut sonuclar = Vec::new();
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                sonuclar.push(YouTubeSonuc {
                    title: json["title"].as_str().unwrap_or("Bilinmeyen").to_string(),
                    channel: json["uploader"].as_str().unwrap_or("Bilinmeyen").to_string(),
                    duration_string: json["duration_string"].as_str().unwrap_or("0:00").to_string(),
                    thumbnail: json["thumbnail"].as_str().unwrap_or("").to_string(),
                    webpage_url: json["webpage_url"].as_str().unwrap_or("").to_string(),
                });
            }
        }
        Ok(sonuclar)
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn youtube_playlist_getir(app: AppHandle, url: String) -> Result<Vec<YouTubeSonuc>, String> {
    let app_clone = app.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let cache_dir = app_data_dir(&app_clone)?.join("yt_cache");
        if !cache_dir.exists() { std::fs::create_dir_all(&cache_dir).map_err(|e| e.to_string())?; }

        let (yt_dlp_path, binaries_dir) = {
            let exe_path = std::env::current_exe().map(|p| p.parent().map(|p| p.to_path_buf()).unwrap_or_default()).unwrap_or_default();
            let resource_dir = app_clone.path().resource_dir().unwrap_or_default();
            let current_dir = std::env::current_dir().unwrap_or_default();
            let olasi_yollar = vec![
                exe_path.join("binaries").join("yt-dlp.exe"), resource_dir.join("binaries").join("yt-dlp.exe"),
                current_dir.join("src-tauri").join("binaries").join("yt-dlp.exe"), current_dir.join("binaries").join("yt-dlp.exe"),
            ];
            let yt_path = olasi_yollar.into_iter().find(|p| p.exists()).ok_or_else(|| "Araç bulunamadı".to_string())?;
            let bin_dir = yt_path.parent().unwrap_or(Path::new("")).to_path_buf();
            (yt_path, bin_dir)
        };

        let mut cmd = std::process::Command::new(&yt_dlp_path);
        cmd.env("PYTHONIOENCODING", "utf-8").env("PATH", "C:\\Windows\\System32;C:\\Windows").current_dir(&cache_dir);
        cmd.args(["--no-warnings", "--cache-dir", cache_dir.to_str().unwrap_or(""), "--ffmpeg-location", binaries_dir.to_str().unwrap_or(""), "--dump-json", "--flat-playlist", "--", &url])
           .stdin(std::process::Stdio::null()).stderr(std::process::Stdio::piped());

        #[cfg(target_os = "windows")]
        { use std::os::windows::process::CommandExt; cmd.creation_flags(0x08000000); }

        let output = cmd.output().map_err(|e| e.to_string())?;
        if !output.status.success() { return Err(format!("Arama başarısız: {}", String::from_utf8_lossy(&output.stderr))); }

        let mut sonuclar = Vec::new();
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                if json["url"].is_string() || json["id"].is_string() {
                    let video_url = json["url"].as_str().map(|s| s.to_string()).unwrap_or_else(|| format!("https://www.youtube.com/watch?v={}", json["id"].as_str().unwrap_or("")));
                    sonuclar.push(YouTubeSonuc {
                        title: json["title"].as_str().unwrap_or("Bilinmeyen").to_string(),
                        channel: json["uploader"].as_str().unwrap_or("Bilinmeyen").to_string(),
                        duration_string: json["duration_string"].as_str().map(|s| s.to_string()).unwrap_or_else(|| json["duration"].as_f64().map(|d| format!("{}:{:02.0}", (d / 60.0).floor(), d % 60.0)).unwrap_or_else(|| "0:00".to_string())),
                        thumbnail: json["thumbnails"].as_array().and_then(|t| t.last()).and_then(|t| t["url"].as_str()).unwrap_or("").to_string(),
                        webpage_url: video_url,
                    });
                }
            }
        }
        Ok(sonuclar)
    }).await.map_err(|e| e.to_string())?
}