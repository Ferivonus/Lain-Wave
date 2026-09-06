mod commands;
mod db;
mod models;
mod utils;

use discord_presence::Client as DiscordClient;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager}; // AppHandle buraya eklendi
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::menu::{Menu, MenuItem};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Shortcut, ShortcutState};

// Veritabanı kurulum fonksiyonunu ve Durum (State) yapılarını db modülünden alıyoruz
use crate::db::{DbState, DiscordState, init_db};

// Komutlarımızı modüllerden içeri aktarıyoruz
use commands::music::*;
use commands::playlist::*;
use commands::system::*;
use commands::youtube::*;

pub fn run() {
    // Discord Entegrasyonu
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
            // Veritabanını başlat
            let db_conn = init_db(app.handle()).expect("Veritabanı başlatılamadı!");

            // State'leri uygulamaya kaydet
            app.manage(DbState(Arc::new(Mutex::new(db_conn))));
            app.manage(DiscordState(discord_arc));

            // Klavye kısayollarını ayarla
            let shortcuts = [
                Shortcut::new(None, Code::MediaPlayPause),
                Shortcut::new(None, Code::MediaTrackNext),
                Shortcut::new(None, Code::MediaTrackPrevious),
            ];

            for sc in shortcuts {
                let _ = app.global_shortcut().register(sc);
            }

            // Sistem Tepsisi (System Tray) menüsünü oluştur
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
                            if let Some(window) = tray.app_handle().get_webview_window("main") {
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
            // --- Müzik Komutları (9 Adet) ---
            sarki_kaydet,
            sarkilari_getir,
            sarki_sil,
            sarki_sirasi_guncelle,
            sarki_guncelle,
            dinlenme_sayisi_artir,
            sarki_metadata_oku,
            sarki_sozu_oku,
            mevcut_soz_dillerini_bul,
            
            // --- Çalma Listesi Komutları (9 Adet) ---
            playlist_olustur,
            playlistleri_getir,
            playliste_sarki_ekle,
            playlistten_sarki_cikar,
            playlist_sil,
            playlist_sirasi_guncelle,
            playlist_disa_aktar,
            favorileri_getir,
            favori_degistir,
            
            // --- Sistem ve Ayar Komutları (8 Adet) ---
            get_app_data_dir,
            open_data_folder,
            update_discord_status,
            clear_discord_status,
            ayarlari_getir,
            ayarlari_kaydet,
            yedek_al,
            yedekten_don,
            
            // --- YouTube Komutları (3 Adet) ---
            youtube_arama,
            youtube_indir,
            youtube_playlist_getir
        ])
        .run(tauri::generate_context!())
        .expect("Lain Wave başlatılamadı");
}