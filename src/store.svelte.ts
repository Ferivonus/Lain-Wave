import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export type Sarki = { 
    id: string; 
    isim: string; 
    sarkici: string; 
    album: string; 
    yol: string; 
    kapak_yolu?: string;
    sozler_yolu?: string;
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

export type YouTubeSonuc = {
    title: string;
    channel: string;
    duration_string: string;
    thumbnail: string;
    webpage_url: string;
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

    aramaSorgusu: "",
    aramaYapiliyor: false,
    aramaSonuclari: [] as YouTubeSonuc[],
    indirmeMesaji: "",
    gosterilenSayi: 5,
    aktifIndirmeler: new Set<string>(),
    topluIndirmeAktif: false,
    tekrarModu: 'liste' as 'kapali' | 'liste' | 'tek_sarki',
});

const appStartTime = Math.floor(Date.now() / 1000);

export async function discordGuncelle(durum: 'caliyor' | 'duraklatildi' | 'bosta' = 'caliyor') {
    try {
        if (durum === 'bosta' || !playerState.aktifSarki) {
            await invoke('update_discord_status', {
                detay: "Lain Wave Ağına Bağlı",
                durum: "Kütüphanede geziniyor...",
                startTimestamp: appStartTime,
                endTimestamp: null
            });
            return;
        }

        const formatSure = (saniye: number) => {
            const dk = Math.floor(saniye / 60);
            const sn = Math.floor(saniye % 60);
            return `${dk}:${sn.toString().padStart(2, '0')}`;
        };

        const totalSure = playerState.aktifSarki.sure || 0;
        const sureMetni = totalSure > 0 ? formatSure(totalSure) : "0:00";

        if (durum === 'duraklatildi') {
            await invoke('update_discord_status', {
                detay: playerState.aktifSarki.isim,
                durum: `⏸ ${sureMetni} - ${playerState.aktifSarki.sarkici}`,
                startTimestamp: null,
                endTimestamp: null
            });
        } else {
            const currentTime = playerState.audioRef ? playerState.audioRef.currentTime : 0;
            const simdi = Math.floor(Date.now() / 1000);
            
            const sarkiBaslamaZamani = simdi - Math.floor(currentTime);

            await invoke('update_discord_status', {
                detay: playerState.aktifSarki.isim,
                durum: `⏵ ${sureMetni} - ${playerState.aktifSarki.sarkici}`,
                startTimestamp: sarkiBaslamaZamani,
                endTimestamp: null 
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

    if ('mediaSession' in navigator) {
        navigator.mediaSession.metadata = new MediaMetadata({
            title: sarki.isim,
            artist: sarki.sarkici,
            album: sarki.album,
            artwork: sarki.kapak_yolu ? [{ src: convertFileSrc(sarki.kapak_yolu), sizes: '512x512' }] : []
        });

        navigator.mediaSession.setActionHandler('play', oynatDuraklatToggle);
        navigator.mediaSession.setActionHandler('pause', oynatDuraklatToggle);
        navigator.mediaSession.setActionHandler('previoustrack', oncekiSarki);
        navigator.mediaSession.setActionHandler('nexttrack', sonrakiSarki);
    }
    
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
    const { aktifSarki, sarkiListesi, tekrarModu } = playerState;
    if (!aktifSarki || sarkiListesi.length === 0) return;

    if (tekrarModu === 'tek_sarki') {
        if (playerState.audioRef) {
            playerState.audioRef.currentTime = 0;
            playerState.audioRef.play().catch(() => {});
            return;
        }
    }
    const index = sarkiListesi.findIndex(s => s.id === aktifSarki.id);
    if (index === -1) return;
    
    if (index === sarkiListesi.length - 1) {
        if (tekrarModu === 'kapali') {
            if (playerState.audioRef) playerState.audioRef.pause();
            playerState.suAnOynuyorMu = false;
            playerState.suAnkiZaman = 0;
            discordGuncelle('duraklatildi');
            return;
        }
        sarkiCal(sarkiListesi[0]);
    } else {
        sarkiCal(sarkiListesi[index + 1]);
    }
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
        } catch (ayarHata) {}

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

        listen('media-toggle', oynatDuraklatToggle);
        listen('media-next', sonrakiSarki);
        listen('media-prev', oncekiSarki);

        if (!playerState.suAnOynuyorMu) {
            discordGuncelle('bosta');
        }

    } catch (e) {
        console.error("Veriler yüklenemedi:", e);
    }
}

export function sarkiyiSardir(saniye: number) {
    if (playerState.audioRef) {
        playerState.audioRef.currentTime = saniye;
        if (playerState.suAnOynuyorMu) {
            discordGuncelle('caliyor');
        }
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

export async function youtubeAramaAPI(sorgu: string): Promise<YouTubeSonuc[]> {
    return await invoke<YouTubeSonuc[]>('youtube_arama', { sorgu });
}

export async function youtubePlaylistGetirAPI(url: string): Promise<YouTubeSonuc[]> {
    return await invoke<YouTubeSonuc[]>('youtube_playlist_getir', { url });
}

export async function youtubeIndirAPI(
    url: string, 
    tarz: string = "Pop", 
    dil: string = "auto", 
    youtubeCevirisiKullan: boolean = false, 
    yapayZekaKullan: boolean = false
): Promise<Sarki> {
    const sarki = await invoke<Sarki>('youtube_indir', { 
        url, 
        tarz, 
        dil, 
        youtubeCevirisiKullan, 
        yapayZekaKullan 
    });
    playerState.sarkiListesi = [...playerState.sarkiListesi, sarki];
    return sarki;
}

export async function muzikAra() {
    if (!playerState.aramaSorgusu.trim()) return;

    if (playerState.aramaSorgusu.includes("http://") || playerState.aramaSorgusu.includes("https://")) {
        if (playerState.aramaSorgusu.includes("list=")) {
            await playlistTarama(playerState.aramaSorgusu);
        } else {
            await youtubeIndir(playerState.aramaSorgusu);
        }
        return;
    }

    playerState.aramaYapiliyor = true;
    playerState.aramaSonuclari = [];
    playerState.gosterilenSayi = 5; 
    playerState.indirmeMesaji = "Ağda frekanslar taranıyor...";

    try {
        const sonuclar = await youtubeAramaAPI(playerState.aramaSorgusu);
        playerState.aramaSonuclari = sonuclar;
        playerState.indirmeMesaji = sonuclar.length > 0 ? `${sonuclar.length} sinyal tespit edildi.` : "Sinyal bulunamadı.";
    } catch (e) {
        playerState.indirmeMesaji = "Tarama başarısız: " + e;
    } finally {
        playerState.aramaYapiliyor = false;
    }
}

export async function playlistTarama(url: string) {
    playerState.aramaYapiliyor = true;
    playerState.aramaSonuclari = [];
    playerState.gosterilenSayi = 5;
    playerState.indirmeMesaji = "Playlist frekansları çözümleniyor...";

    try {
        const sonuclar = await youtubePlaylistGetirAPI(url);
        playerState.aramaSonuclari = sonuclar;
        playerState.indirmeMesaji = sonuclar.length > 0 ? `Liste çözümlendi: ${sonuclar.length} parça.` : "Liste boş veya okunamadı.";
    } catch (e) {
        playerState.indirmeMesaji = "Liste hatası: " + e;
    } finally {
        playerState.aramaYapiliyor = false;
    }
}

export async function youtubeIndir(
    hedefUrl: string, 
    tarz: string = "Pop", 
    dil: string = "auto", 
    youtubeCevirisiKullan: boolean = true, 
    yapayZekaKullan: boolean = true
) {
    if (!hedefUrl.trim() || playerState.aktifIndirmeler.has(hedefUrl)) return;

    playerState.aktifIndirmeler = new Set(playerState.aktifIndirmeler).add(hedefUrl);
    playerState.indirmeMesaji = "Veri akışı sağlanıyor...";

    try {
        await youtubeIndirAPI(hedefUrl, tarz, dil, youtubeCevirisiKullan, yapayZekaKullan);
        playerState.indirmeMesaji = "Veri başarıyla arşive eklendi.";
    } catch (e) {
        playerState.indirmeMesaji = "Bağlantı koptu: " + e;
    } finally {
        const yeniSet = new Set(playerState.aktifIndirmeler);
        yeniSet.delete(hedefUrl);
        playerState.aktifIndirmeler = yeniSet;

        setTimeout(() => { 
            if (playerState.aktifIndirmeler.size === 0 && !playerState.aramaYapiliyor && !playerState.topluIndirmeAktif) {
                playerState.indirmeMesaji = ""; 
            }
        }, 5000);
    }
}

export async function tumunuIndir() {
    if (playerState.aramaSonuclari.length === 0 || playerState.topluIndirmeAktif) return;
    
    playerState.topluIndirmeAktif = true;
    playerState.gosterilenSayi = playerState.aramaSonuclari.length;
    playerState.indirmeMesaji = "Toplu veri akışı başlatıldı...";

    for (const sonuc of playerState.aramaSonuclari) {
        if (!playerState.aktifIndirmeler.has(sonuc.webpage_url)) {
            await youtubeIndir(sonuc.webpage_url);
        }
    }
    
    playerState.topluIndirmeAktif = false;
    playerState.indirmeMesaji = "Toplu aktarım tamamlandı.";
}

export type MetadataBilgisi = {
    isim: string | null;
    sarkici: string | null;
    album: string | null;
    tarz: string | null;
};

export async function sarkiMetadataOkuAPI(yol: string): Promise<MetadataBilgisi> {
    return await invoke<MetadataBilgisi>('sarki_metadata_oku', { yol });
}

export async function sarkiKaydetAPI(veri: { 
    isim: string; 
    sarkici: string; 
    album: string; 
    yol: string; 
    manuel_tarz: string | null; 
    yil: number | null; 
    notlar: string;
}): Promise<Sarki> {
    const sarki = await invoke<Sarki>('sarki_kaydet', veri);
    playerState.sarkiListesi = [...playerState.sarkiListesi, sarki];
    return sarki;
}

export async function playlistOlusturAPI(isim: string): Promise<Playlist> {
    const yeniListe = await invoke<Playlist>('playlist_olustur', { isim });
    playerState.playlistler = [...playerState.playlistler, yeniListe];
    return yeniListe;
}

export async function playlistSirasiGuncelleAPI(playlistId: string, yeniSarkiSiralari: string[]) {
    try {
        await invoke('playlist_sirasi_guncelle', { playlistId, yeniSarkiSiralari });
    } catch (e) {
        console.error("Liste sırası kaydedilemedi:", e);
    }
}

 export async function handleSarkiSil(sarki: Sarki, event: MouseEvent | KeyboardEvent) {
    event.preventDefault();
    event.stopPropagation();
    
    const mesaj = `DİKKAT: "${sarki.isim}" adlı yayını kütüphaneden ve diskten KALICI olarak silmek istediğinize emin misiniz?\n\nBu işlem geri alınamaz.`;
    
    if (confirm(mesaj)) {
        try {
            await sarkiSil(sarki);
        } catch (hata) {
            alert("Silme işlemi sırasında bir hata oluştu.");
        }
    }
}
  
  export function editModaliAc(sarki: Sarki, event: Event) {
      event.stopPropagation();
      playerState.duzenlenecekSarki = sarki;
      playerState.isEditModalOpen = true;
  }

  export async function handlePlaylistEkle(sarkiId: string, event: Event) {
    const selectElement = event.target as HTMLSelectElement;
    const playlistId = selectElement.value;
    if (!playlistId) return;
    const basarili = await sarkiPlaylisteEkle(sarkiId, playlistId);
    if (basarili) {
        selectElement.value = ""; 
    }
}