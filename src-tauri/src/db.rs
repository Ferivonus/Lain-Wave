use crate::utils::app_data_dir;
use discord_presence::Client as DiscordClient; // Discord Client'ı içe aktarıyoruz
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use tauri::AppHandle;

// Uygulama durumlarımız (State) burada tanımlanıyor
pub struct DbState(pub Arc<Mutex<Connection>>);
pub struct DiscordState(pub Arc<Mutex<DiscordClient>>); // Discord durumunu buraya ekledik

/// Veritabanı bağlantısını başlatır ve gerekli tabloları oluşturur.
pub fn init_db(app: &AppHandle) -> Result<Connection, String> {
    let mut db_path = app_data_dir(app)?;
    db_path.push("lainwave.db");

    let conn = Connection::open(db_path).map_err(|e| format!("Veritabanı açılamadı: {}", e))?;

    // Gerekli tüm tabloları tek bir işlemde (batch) oluşturuyoruz
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

    // Varsayılan ayarları ekliyoruz (eğer daha önce eklenmediyse IGNORE ile atlanacak)
    conn.execute(
        "INSERT OR IGNORE INTO ayarlar (id, kullanici_adi, discord_aktif, medya_tuslari_aktif, tema)
         VALUES (1, '', 1, 1, 'theme-modern')",
        [],
    )
    .ok();

    Ok(conn)
}
