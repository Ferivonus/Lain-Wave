<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { type Playlist, type Sarki, playerState } from '../store.svelte';
    import { fade, fly } from 'svelte/transition';

    let { aktifPlaylist, sarkilar } = $props<{ aktifPlaylist: Playlist; sarkilar: Sarki[] }>();

    let islemDurumu = $state<'bekliyor' | 'isleniyor' | 'basarili' | 'hata'>('bekliyor');
    let mesaj = $state('');
    let kayitliDosyaYolu = $state('');

    async function disaAktar(e: Event) {
        e.stopPropagation();
        if (islemDurumu === 'isleniyor') return;

        islemDurumu = 'isleniyor';

        const paylasimVerisi = {
            paylasan_kisi: playerState.username || "Anonim",
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
            
            kayitliDosyaYolu = dosyaYolu;
            islemDurumu = 'basarili';
            mesaj = "Frekans paketi başarıyla oluşturuldu.";
            
            setTimeout(() => {
                if (islemDurumu === 'basarili') {
                    islemDurumu = 'bekliyor';
                    kayitliDosyaYolu = '';
                }
            }, 20000);

        } catch (hata) {
            islemDurumu = 'hata';
            mesaj = "Dışa aktarma başarısız oldu!";
            console.error(hata);
            
            setTimeout(() => { islemDurumu = 'bekliyor'; }, 4000);
        }
    }

    async function klasoruAc(e: Event) {
        e.stopPropagation();
        if (kayitliDosyaYolu) {
            await invoke('dosya_konumunu_ac', { yol: kayitliDosyaYolu });
            islemDurumu = 'bekliyor';
            kayitliDosyaYolu = '';
        }
    }
</script>

<div class="relative group">
    <button 
        type="button"
        onclick={disaAktar}
        disabled={islemDurumu === 'isleniyor'}
        class="flex items-center gap-2 bg-[var(--bg-surface)] text-[var(--text-dim)] hover:text-[var(--accent)] hover:border-[var(--accent)]/50 border border-[var(--border)] px-4 py-2.5 rounded-lg font-black transition-all active:scale-95 text-[10px] uppercase tracking-widest shrink-0 disabled:opacity-50"
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

    {#if islemDurumu === 'basarili'}
        <div class="absolute top-full mt-3 right-0 bg-[var(--bg-card)] border border-[var(--accent)]/30 p-4 rounded-xl shadow-2xl w-72 z-50 flex flex-col gap-3" in:fly={{ y: -5, duration: 300 }} out:fade>
            <div class="flex items-start gap-3 text-[var(--accent)]">
                <svg class="w-5 h-5 shrink-0 mt-0.5" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg>
                <div class="flex flex-col">
                    <span class="text-xs font-black uppercase tracking-widest">{mesaj}</span>
                    <span class="text-[9px] text-[var(--text-dim)] font-mono mt-1 break-all opacity-70 leading-tight">{kayitliDosyaYolu}</span>
                </div>
            </div>
            
            <button 
                type="button" 
                onclick={klasoruAc}
                class="w-full bg-[var(--accent)] text-white hover:shadow-[0_0_15px_var(--accent-glow)] py-2.5 rounded-lg text-[10px] font-black uppercase tracking-[0.2em] transition-all active:scale-95 flex items-center justify-center gap-2 mt-1"
            >
                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path></svg>
                Klasörü Aç
            </button>
        </div>
    {/if}
</div>