use serde::{Deserialize, Serialize};

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

#[derive(Serialize)]
pub struct AvailableLanguage {
    pub dil: String,
    pub yol: String,
}
