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
    son_dinlenme_tarihi?: number; // YENİ
    yil?: number;    // YENİ: Çıkış Yılı
    notlar?: string; // YENİ: Özel Etiketler/Notlar
};

export type Playlist = { 
    id: string; 
    isim: string; 
    sarkilar: string[]; 
};

// Merkezi durum yönetimi (Svelte 5 Runes)
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
    isAddMusicModalOpen: false,
    isLyricsOpen: false // Sırada gelecek olan Lyrics ekranı için hazırlık
});

// --- YENİ: Akıllı Discord Güncelleme Fonksiyonu ---
export async function discordGuncelle(durum: 'caliyor' | 'duraklatildi' | 'bosta' = 'caliyor') {
    try {
        if (durum === 'bosta' || !playerState.aktifSarki) {
            // 1. Hiçbir şey çalmıyorsa
            await invoke('update_discord_status', {
                detay: "Lain Wave Ağına Bağlı",
                durum: "Kütüphanede geziniyor...",
                toplamSaniye: 0 // Süre yok
            });
            return;
        }

        if (durum === 'duraklatildi') {
            // 2. Şarkı duraklatıldıysa
            await invoke('update_discord_status', {
                detay: playerState.aktifSarki.isim,
                durum: `⏸️ Duraklatıldı - ${playerState.aktifSarki.sarkici}`,
                toplamSaniye: 0 // Geri sayımı durdur
            });
        } else {
            // 3. Şarkı Çalıyorsa
            const kalanSure = playerState.audioRef ? Math.floor((playerState.aktifSarki.sure || 0) - playerState.audioRef.currentTime) : 0;
            await invoke('update_discord_status', {
                detay: playerState.aktifSarki.isim,
                durum: `▶️ Çalıyor - ${playerState.aktifSarki.sarkici}`,
                toplamSaniye: kalanSure > 0 ? kalanSure : 0
            });
        }
    } catch (e) {
        console.error("Discord güncellenemedi", e);
    }
}

// --- Şarkı Çalma ---
export async function sarkiCal(sarki: Sarki) {
    if (!playerState.audioRef) return;

    const simdi = Date.now();

    try {
        const [yeniSayi, yeniTarih]: any = await invoke('dinlenme_sayisi_artir', { 
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
        sarki.dinlenme_sayisi = (sarki.dinlenme_sayisi || 0) + 1;
    }

    localStorage.setItem('lainwave_son_sarki', sarki.id);

    playerState.audioRef.src = convertFileSrc(sarki.yol);
    playerState.aktifSarki = sarki;
    playerState.suAnOynuyorMu = true;
    
    playerState.audioRef.play()
        .then(() => {
            // Çalmaya başladığında durumu 'caliyor' olarak yolla
            discordGuncelle('caliyor');
        })
        .catch(err => console.warn("Oynatma hatası:", err));
}

// --- Oynat/Duraklat Geçişi ---
export async function oynatDuraklatToggle() {
    if (!playerState.audioRef || !playerState.aktifSarki) return;
    
    if (playerState.suAnOynuyorMu) {
        playerState.audioRef.pause();
        // YENİ: Temizlemek yerine 'duraklatildi' modunu gönder
        discordGuncelle('duraklatildi');
    } else {
        playerState.audioRef.play()
            .then(() => {
                // YENİ: Tekrar başladığında 'caliyor' moduna al ve süreyi yenile
                discordGuncelle('caliyor');
            })
            .catch(err => console.warn("Oynatma hatası:", err));
    }
    
    playerState.suAnOynuyorMu = !playerState.suAnOynuyorMu;
}
// Sonraki şarkı
export function sonrakiSarki() {
    const { aktifSarki, sarkiListesi } = playerState;
    if (!aktifSarki || sarkiListesi.length === 0) return;

    const index = sarkiListesi.findIndex(s => s.id === aktifSarki.id);
    const sonrakiIndex = (index + 1) % sarkiListesi.length;
    
    sarkiCal(sarkiListesi[sonrakiIndex]);
}

// Önceki şarkı
export function oncekiSarki() {
    const { aktifSarki, sarkiListesi } = playerState;
    if (!aktifSarki || sarkiListesi.length === 0) return;

    const index = sarkiListesi.findIndex(s => s.id === aktifSarki.id);
    const oncekiIndex = (index - 1 + sarkiListesi.length) % sarkiListesi.length;
    
    sarkiCal(sarkiListesi[oncekiIndex]);
}

// Uygulama açıldığında verileri Rust'tan çeken ve son şarkıyı hatırlayan fonksiyon
export async function initializePlayer() {
    try {
        playerState.sarkiListesi = await invoke('sarkilari_getir');
        playerState.playlistler = await invoke('playlistleri_getir');
        playerState.favoriler = await invoke('favorileri_getir');

        const sonSarkiId = localStorage.getItem('lainwave_son_sarki');
        if (sonSarkiId && playerState.sarkiListesi.length > 0) {
            const bulunanSarki = playerState.sarkiListesi.find(s => s.id === sonSarkiId);
            if (bulunanSarki && playerState.audioRef) {
                playerState.aktifSarki = bulunanSarki;
                playerState.audioRef.src = convertFileSrc(bulunanSarki.yol);
            }
        }

        // YENİ: Veriler yüklendikten hemen sonra Discord'a "boşta" durumunu bildir!
        if (!playerState.suAnOynuyorMu) {
            discordGuncelle('bosta');
        }

    } catch (e) {
        console.error("Veriler yüklenemedi:", e);
    }
}
// Yeni playlist oluşturma mantığını merkezi hale getirdik
export async function yeniPlaylistOlustur() {
    const isim = prompt("Yeni çalma listesinin adını girin (Örn: Gece Sürüşü):");
    if (isim && isim.trim() !== "") {
        try {
            const yeniListe: Playlist = await invoke('playlist_olustur', { isim: isim.trim() });
            playerState.playlistler = [...playerState.playlistler, yeniListe];
        } catch (e) {
            alert("Liste oluşturulurken hata oluştu!");
        }
    }
}

export async function sarkiPlaylisteEkle(sarkiId: string, playlistId: string) {
    if (!playlistId) return false;
    try {
        const guncelListe: Playlist = await invoke('playliste_sarki_ekle', { playlistId, sarkiId });
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
    // confirm() işlemi artık .svelte dosyasında yapılacak
    try {
        await invoke('sarki_sil', { sarkiId: sarki.id });
        playerState.sarkiListesi = playerState.sarkiListesi.filter(s => s.id !== sarki.id);
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
        // Hata durumunda listeyi eski haline getirme mantığı eklenebilir
    }
}

export async function playlistSil(id: string) {
    // Onay (confirm) işlemi artık arayüz (.svelte) tarafında yapıldığı için
    // burası sadece veriyi silmekle ilgileniyor.
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
    // confirm() işlemi artık .svelte dosyasında yapılacak
    try {
        const guncelListe: Playlist = await invoke('playlistten_sarki_cikar', { 
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

    // 1. ADIM: İyimser Güncelleme (Hız hissi için anında değiştiriyoruz)
    if (isFavorite) {
        playerState.favoriler = playerState.favoriler.filter(id => id !== sarkiId);
    } else {
        playerState.favoriler = [...playerState.favoriler, sarkiId];
    }

    try {
        // 2. ADIM: Arka planda Rust işlemini yap
        const guncelFavoriler: string[] = await invoke('favori_degistir', { sarkiId });
        playerState.favoriler = guncelFavoriler; 
    } catch (err) {
        // 3. ADIM: Hata olursa sessizce geri al
        console.error("Favori işlemi başarısız:", err);
        playerState.favoriler = eskiFavoriler;
    }
}