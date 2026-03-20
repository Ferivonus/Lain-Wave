<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { type Playlist, type Sarki } from '../store.svelte';
    import { fade } from 'svelte/transition';

    let { aktifPlaylist, sarkilar } = $props<{ aktifPlaylist: Playlist; sarkilar: Sarki[] }>();

    let islemDurumu = $state<'bekliyor' | 'isleniyor' | 'basarili' | 'hata'>('bekliyor');
    let mesaj = $state('');

    async function disaAktar(e: Event) {
        e.stopPropagation();
        if (islemDurumu === 'isleniyor') return;

        islemDurumu = 'isleniyor';

        const paylasimVerisi = {
            playlist_adi: aktifPlaylist.isim,
            olusturulma_tarihi: new Date().toISOString(),
            sarki_sayisi: sarkilar.length,
            sarkilar: sarkilar.map((s: Sarki) => ({
                isim: s.isim,
                sarkici: s.sarkici,
                youtube_linki: (s.notlar && s.notlar.includes("http")) ? s.notlar : null,
                album: s.album,
                tarz: s.tarz
            }))
        };

        const jsonIcerik = JSON.stringify(paylasimVerisi, null, 2);

        try {
            const dosyaYolu = await invoke<string>('playlist_disa_aktar', { 
                playlistAdi: aktifPlaylist.isim, 
                icerik: jsonIcerik 
            });
            
            islemDurumu = 'basarili';
            mesaj = `Başarılı! Dosya konumu: ${dosyaYolu}`;
            
            setTimeout(() => {
                islemDurumu = 'bekliyor';
                mesaj = '';
            }, 4000);

        } catch (hata) {
            islemDurumu = 'hata';
            mesaj = "Dışa aktarma başarısız oldu!";
            console.error(hata);
            
            setTimeout(() => { islemDurumu = 'bekliyor'; }, 3000);
        }
    }
</script>

<div class="relative group">
    <button 
        type="button"
        onclick={disaAktar}
        disabled={islemDurumu === 'isleniyor'}
        class="flex items-center gap-2 bg-[var(--bg-surface)] text-[var(--text-dim)] hover:text-[var(--accent)] hover:border-[var(--accent)]/50 border border-[var(--border)] px-4 py-2 rounded-lg font-bold transition-all active:scale-95 text-[10px] uppercase tracking-widest shrink-0 disabled:opacity-50"
        aria-label="Playlisti Paylaş"
    >
        {#if islemDurumu === 'isleniyor'}
            <svg class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
            Paketleniyor...
        {:else if islemDurumu === 'basarili'}
            <svg class="w-4 h-4 text-emerald-500" fill="none" stroke="currentColor" stroke-width="3" viewBox="0 0 24 24"><polyline points="20 6 9 17 4 12"></polyline></svg>
            Aktarıldı
        {:else if islemDurumu === 'hata'}
            <svg class="w-4 h-4 text-red-500" fill="none" stroke="currentColor" stroke-width="3" viewBox="0 0 24 24"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            Hata
        {:else}
            <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"></path></svg>
            Dışa Aktar (JSON)
        {/if}
    </button>

    {#if mesaj && islemDurumu === 'basarili'}
        <div class="absolute top-full mt-2 right-0 bg-[var(--bg-card)] border border-[var(--accent)]/50 text-[var(--accent)] p-3 rounded-lg shadow-2xl text-[9px] w-64 z-50 font-mono" in:fade>
            {mesaj}
        </div>
    {/if}
</div>