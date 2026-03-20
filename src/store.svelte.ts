import { invoke, convertFileSrc } from '@tauri-apps/api/core';

export type Sarki = { 
    id: string; 
    isim: string; 
    sarkici: string; 
    album: string; 
    yol: string; 
    kapak_yolu?: string;
    tarz?: string;        
    kalite?: string;      
    sure?: number; 
    dinlenme_sayisi?: number; 
    son_dinlenme_tarihi?: number; 
    yil?: number;    
    notlar?: string; 
};

export type Playlist = { 
    id: string; 
    isim: string; 
    sarkilar: string[]; 
};

export type Ayarlar = {
    kullanici_adi: string;
    discord_aktif: boolean;
    medya_tuslari_aktif: boolean;
    tema: string;
};

export const playerState = $state({
    aktifSarki: null as Sarki | null,
    suAnOynuyorMu: false,
    sarkiListesi: [] as Sarki[],
    playlistler: [] as Playlist[],
    favoriler: [] as string[],
    audioRef: null as HTMLAudioElement | null,
    suAnkiZaman: 0,
    toplamZaman: 0,
    sesSeviyesi: 1,
    currentTheme: 'theme-modern',
    isAddMusicModalOpen: false,
    isLyricsOpen: false,
    isEditModalOpen: false,
    duzenlenecekSarki: null as Sarki | null,
    isCreatePlaylistModalOpen: false,
    username: "",
});

export async function discordGuncelle(durum: 'caliyor' | 'duraklatildi' | 'bosta' = 'caliyor') {
    try {
        if (durum === 'bosta' || !playerState.aktifSarki) {
            await invoke('update_discord_status', {
                detay: "Lain Wave Ağına Bağlı",
                durum: "Kütüphanede geziniyor...",
                toplamSaniye: 0 
            });
            return;
        }

        if (durum === 'duraklatildi') {
            await invoke('update_discord_status', {
                detay: playerState.aktifSarki.isim,
                durum: `⏸️ Duraklatıldı - ${playerState.aktifSarki.sarkici}`,
                toplamSaniye: 0 
            });
        } else {
            const kalanSure = playerState.audioRef ? Math.floor((playerState.aktifSarki.sure || 0) - playerState.audioRef.currentTime) : 0;
            await invoke('update_discord_status', {
                detay: playerState.aktifSarki.isim,
                durum: `▶️ Çalıyor - ${playerState.aktifSarki.sarkici}`,
                toplamSaniye: Math.max(0, kalanSure)
            });
        }
    } catch (e) {
        console.error("Discord güncellenemedi", e);
    }
}

export async function sarkiCal(sarki: Sarki) {
    if (!playerState.audioRef) return;

    const simdi = Date.now();

    try {
        const [yeniSayi, yeniTarih] = await invoke<[number, number]>('dinlenme_sayisi_artir', { 
            sarkiId: sarki.id, 
            tarih: simdi 
        });
        
        const index = playerState.sarkiListesi.findIndex(s => s.id === sarki.id);
        if (index !== -1) {
            playerState.sarkiListesi[index].dinlenme_sayisi = yeniSayi;
            playerState.sarkiListesi[index].son_dinlenme_tarihi = yeniTarih;
        }
        sarki.dinlenme_sayisi = yeniSayi;
        sarki.son_dinlenme_tarihi = yeniTarih;
    } catch (e) {
        const index = playerState.sarkiListesi.findIndex(s => s.id === sarki.id);
        if (index !== -1) {
            playerState.sarkiListesi[index].dinlenme_sayisi = (playerState.sarkiListesi[index].dinlenme_sayisi || 0) + 1;
            playerState.sarkiListesi[index].son_dinlenme_tarihi = simdi;
        }
        sarki.dinlenme_sayisi = (sarki.dinlenme_sayisi || 0) + 1;
        sarki.son_dinlenme_tarihi = simdi;
    }

    localStorage.setItem('lainwave_son_sarki', sarki.id);

    playerState.audioRef.src = convertFileSrc(sarki.yol);
    playerState.aktifSarki = sarki;
    playerState.suAnOynuyorMu = true;
    
    playerState.audioRef.play()
        .then(() => {
            discordGuncelle('caliyor');
        })
        .catch(err => console.warn("Oynatma hatası:", err));
}

export async function oynatDuraklatToggle() {
    if (!playerState.audioRef || !playerState.aktifSarki) return;
    
    if (playerState.suAnOynuyorMu) {
        playerState.audioRef.pause();
        discordGuncelle('duraklatildi');
    } else {
        playerState.audioRef.play()
            .then(() => {
                discordGuncelle('caliyor');
            })
            .catch(err => console.warn("Oynatma hatası:", err));
    }
    
    playerState.suAnOynuyorMu = !playerState.suAnOynuyorMu;
}

export function sonrakiSarki() {
    const { aktifSarki, sarkiListesi } = playerState;
    if (!aktifSarki || sarkiListesi.length === 0) return;

    const index = sarkiListesi.findIndex(s => s.id === aktifSarki.id);
    if (index === -1) return;
    
    const sonrakiIndex = (index + 1) % sarkiListesi.length;
    sarkiCal(sarkiListesi[sonrakiIndex]);
}

export function oncekiSarki() {
    const { aktifSarki, sarkiListesi } = playerState;
    if (!aktifSarki || sarkiListesi.length === 0) return;

    const index = sarkiListesi.findIndex(s => s.id === aktifSarki.id);
    if (index === -1) return;

    const oncekiIndex = (index - 1 + sarkiListesi.length) % sarkiListesi.length;
    sarkiCal(sarkiListesi[oncekiIndex]);
}

export async function initializePlayer() {
    try {
        playerState.sarkiListesi = await invoke<Sarki[]>('sarkilari_getir');
        playerState.playlistler = await invoke<Playlist[]>('playlistleri_getir');
        playerState.favoriler = await invoke<string[]>('favorileri_getir');

        try {
            const ayarlar = await invoke<Ayarlar>('ayarlari_getir');
            if (ayarlar) {
                playerState.username = ayarlar.kullanici_adi || "";
                if (ayarlar.tema) {
                    playerState.currentTheme = ayarlar.tema;
                }
            }
        } catch (ayarHata) {
            console.warn("Ayarlar yüklenemedi:", ayarHata);
        }

        const sonSarkiId = localStorage.getItem('lainwave_son_sarki');
        if (sonSarkiId && playerState.sarkiListesi.length > 0) {
            const bulunanSarki = playerState.sarkiListesi.find(s => s.id === sonSarkiId);
            if (bulunanSarki) {
                playerState.aktifSarki = bulunanSarki;
                if (playerState.audioRef) {
                    playerState.audioRef.src = convertFileSrc(bulunanSarki.yol);
                }
            }
        }

        if (!playerState.suAnOynuyorMu) {
            discordGuncelle('bosta');
        }

    } catch (e) {
        console.error("Veriler yüklenemedi:", e);
    }
}

export function yeniPlaylistOlustur() {
    playerState.isCreatePlaylistModalOpen = true;
}

export async function sarkiPlaylisteEkle(sarkiId: string, playlistId: string) {
    if (!playlistId) return false;
    try {
        const guncelListe = await invoke<Playlist>('playliste_sarki_ekle', { playlistId, sarkiId });
        const index = playerState.playlistler.findIndex(p => p.id === playlistId);
        if (index !== -1) {
            playerState.playlistler[index] = guncelListe;
        }
        return true;
    } catch (e) {
        console.error("Playliste eklenirken hata oluştu:", e);
        return false;
    }
}

export async function sarkiSil(sarki: Sarki) {
    try {
        await invoke('sarki_sil', { sarkiId: sarki.id });
        playerState.sarkiListesi = playerState.sarkiListesi.filter(s => s.id !== sarki.id);
        
        if (playerState.aktifSarki?.id === sarki.id) {
            playerState.aktifSarki = null;
            playerState.suAnOynuyorMu = false;
            if (playerState.audioRef) {
                playerState.audioRef.pause();
                playerState.audioRef.src = "";
            }
            discordGuncelle('bosta');
        }
        
        return true;
    } catch (err) {
        console.error("Silme hatası:", err);
        alert("Şarkı silinirken bir hata oluştu.");
        return false;
    }
}

export async function siraGuncelle(yeniListe: Sarki[]) {
    playerState.sarkiListesi = yeniListe;
    try {
        await invoke('sarki_sirasi_guncelle', { yeniListe });
    } catch (err) {
        console.error("Sıra güncellenemedi:", err);
    }
}

export async function playlistSil(id: string) {
    try {
        await invoke('playlist_sil', { playlistId: id });
        playerState.playlistler = playerState.playlistler.filter(p => p.id !== id);
        return true;
    } catch (err) {
        console.error("Liste silme hatası:", err);
        alert("Liste silinirken bir hata oluştu.");
        return false;
    }
}

export async function playlisttenSarkiCikar(playlistId: string, sarkiId: string) {
    try {
        const guncelListe = await invoke<Playlist>('playlistten_sarki_cikar', { 
            playlistId: playlistId, 
            sarkiId: sarkiId 
        });

        const index = playerState.playlistler.findIndex(p => p.id === playlistId);
        if (index !== -1) {
            playerState.playlistler[index] = guncelListe;
        }
        return true;
    } catch (e) {
        console.error("Listeden çıkarma hatası:", e);
        alert("Şarkı listeden çıkarılamadı.");
        return false;
    }
}

export async function toggleFavori(sarkiId: string) {
    const isFavorite = playerState.favoriler.includes(sarkiId);
    const eskiFavoriler = [...playerState.favoriler];

    if (isFavorite) {
        playerState.favoriler = playerState.favoriler.filter(id => id !== sarkiId);
    } else {
        playerState.favoriler = [...playerState.favoriler, sarkiId];
    }

    try {
        const guncelFavoriler = await invoke<string[]>('favori_degistir', { sarkiId });
        playerState.favoriler = guncelFavoriler; 
    } catch (err) {
        console.error("Favori işlemi başarısız:", err);
        playerState.favoriler = eskiFavoriler;
    }
}