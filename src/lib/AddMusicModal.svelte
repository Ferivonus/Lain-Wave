<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { listen, type UnlistenFn } from '@tauri-apps/api/event';
    import { open } from '@tauri-apps/plugin-dialog';
    import { playerState, type Sarki } from '../store.svelte';
    import { fade, scale, fly } from 'svelte/transition';
    import { cubicOut } from 'svelte/easing';

    // UI Durumları
    let gorunum = $state<'secim' | 'detay' | 'youtube' | 'basarili'>('secim');
    let youtubeLink = $state("");
    let yukleniyor = $state(false);
    
    // Şarkı Metadata Durumları
    let secilenDosyaPath = $state("");
    let formVerisi = $state({
        isim: "",
        sarkici: "",
        album: "",
        yil: null as number | null,
        notlar: ""
    });
    
    let secilenTarz = $state("Pop");
    let tespitEdilenTarz = $state<string | null>(null);

    // Canlı İndirme Verileri
    let downloadInfo = $state({
        pct: 0,
        speed: "0KiB/s",
        eta: "00:00"
    });

    const tarzlar = ["Pop", "Rock", "Lofi", "Electronic", "Jazz", "Hip-Hop", "Classical", "Podcast"];

    // Tauri Event Dinleyicisi
    $effect(() => {
        let unlistenFn: UnlistenFn;
        const init = async () => {
            unlistenFn = await listen("download-progress", (event: any) => {
                downloadInfo.pct = event.payload.percentage;
                downloadInfo.speed = event.payload.speed;
                downloadInfo.eta = event.payload.eta;
            });
        };
        init();
        return () => { if (unlistenFn) unlistenFn(); };
    });

    function kapat() {
        if (yukleniyor) return;
        playerState.isAddMusicModalOpen = false;
    }

    // Erişilebilirlik için Klavye Desteği
    function handleBackdropKeydown(e: KeyboardEvent) {
        if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault();
            kapat();
        }
    }

    function baskaEkle() {
        gorunum = 'secim';
        youtubeLink = "";
        secilenDosyaPath = "";
        formVerisi = { isim: "", sarkici: "", album: "", yil: null, notlar: "" };
        downloadInfo = { pct: 0, speed: "0KiB/s", eta: "00:00" };
    }

    async function dosyaSec() {
        const path = await open({
            multiple: false,
            filters: [{ name: 'Müzik', extensions: ['mp3', 'wav', 'flac'] }]
        });

        if (path && typeof path === 'string') {
            secilenDosyaPath = path;
            yukleniyor = true;
            try {
                const meta: any = await invoke('sarki_metadata_oku', { yol: path });
                formVerisi.isim = meta.isim || path.split(/[\\/]/).pop()?.replace(/\.[^/.]+$/, "") || "Yeni Şarkı";
                formVerisi.sarkici = meta.sarkici || "Bilinmeyen Sanatçı";
                formVerisi.album = meta.album || "Bilinmeyen Albüm";
                tespitEdilenTarz = meta.tarz;
                gorunum = 'detay';
            } catch (err) {
                gorunum = 'detay';
            } finally {
                yukleniyor = false;
            }
        }
    }

    async function kaydet() {
        downloadInfo.pct = 0;
        yukleniyor = true;
        try {
            let sarki: Sarki;
            if (gorunum === 'youtube') {
                sarki = await invoke('youtube_indir', { url: youtubeLink, tarz: secilenTarz });
            } else {
                sarki = await invoke('sarki_kaydet', {
                    isim: formVerisi.isim,
                    sarkici: formVerisi.sarkici,
                    album: formVerisi.album,
                    yol: secilenDosyaPath,
                    manuel_tarz: secilenTarz,
                    yil: formVerisi.yil,
                    notlar: formVerisi.notlar
                });
            }
            playerState.sarkiListesi = [...playerState.sarkiListesi, sarki];
            formVerisi.isim = sarki.isim;
            gorunum = 'basarili';
        } catch (hata) {
            alert(`İşlem sırasında hata: ${hata}`);
        } finally {
            yukleniyor = false;
        }
    }

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === 'Escape') kapat();
    }
</script>

<svelte:window onkeydown={handleKeydown} />

<div 
    class="fixed inset-0 z-[100] flex items-center justify-center bg-black/70 backdrop-blur-sm p-4" 
    transition:fade={{ duration: 200 }} 
    onclick={kapat}
    onkeydown={handleBackdropKeydown}
    role="button"
    aria-label="Kapat"
    tabindex="0"
>
    <div 
        class="bg-[#18181b] text-white w-full max-w-md rounded-2xl shadow-2xl overflow-hidden relative border border-white/5" 
        transition:scale={{ start: 0.95, duration: 300, easing: cubicOut }} 
        onclick={(e) => e.stopPropagation()} 
        onkeydown={(e) => e.stopPropagation()}
        role="dialog"
        aria-modal="true"
        tabindex="-1" 
    >
        
        <div class="flex justify-between items-center px-6 py-4 bg-white/[0.02] border-b border-white/5">
            <h2 class="text-sm font-bold uppercase tracking-widest text-white/70">Müzik Aktarımı</h2>
            <button onclick={kapat} class="p-2 -mr-2 text-white/30 hover:text-white transition-colors" aria-label="Kapat">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            </button>
        </div>

        {#if yukleniyor}
            <div class="absolute inset-0 bg-[#18181b]/95 backdrop-blur-md z-50 flex flex-col items-center justify-center p-10 text-center" in:fade>
                <div class="w-full max-w-[260px]">
                    <div class="flex justify-between text-[11px] mb-3 font-bold uppercase tracking-widest">
                        <span class="text-blue-400">Veri İşleniyor</span>
                        <span>{Math.round(downloadInfo.pct)}%</span>
                    </div>
                    <div class="w-full h-1.5 bg-white/10 rounded-full overflow-hidden">
                        <div class="h-full bg-blue-500 transition-all duration-300" style="width: {downloadInfo.pct}%"></div>
                    </div>
                    {#if downloadInfo.speed && gorunum === 'youtube'}
                        <p class="text-[10px] text-white/40 mt-4 font-mono">{downloadInfo.speed} • ETA: {downloadInfo.eta}</p>
                    {:else}
                        <p class="text-[10px] text-white/40 mt-4 italic uppercase tracking-tighter">Dosya blokları doğrulanıyor...</p>
                    {/if}
                </div>
            </div>
        {/if}

        <div class="p-6">
            {#if gorunum === 'secim'}
                <div class="grid gap-3" in:fly={{ y: 8, duration: 400 }}>
                    <button onclick={dosyaSec} class="flex items-center gap-5 p-5 bg-white/[0.03] hover:bg-white/[0.06] border border-white/5 rounded-xl transition-all text-left group">
                        <div class="w-11 h-11 bg-blue-500/10 text-blue-400 rounded-xl flex items-center justify-center shrink-0">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="17 8 12 3 7 8"></polyline><line x1="12" y1="3" x2="12" y2="15"></line></svg>
                        </div>
                        <div>
                            <span class="block font-bold">Dosya Gezgini</span>
                            <span class="text-[11px] text-white/30 uppercase tracking-tight">Yerel kütüphaneden aktar</span>
                        </div>
                    </button>

                    <button onclick={() => gorunum = 'youtube'} class="flex items-center gap-5 p-5 bg-white/[0.03] hover:bg-white/[0.06] border border-white/5 rounded-xl transition-all text-left group">
                        <div class="w-11 h-11 bg-red-500/10 text-red-500 rounded-xl flex items-center justify-center shrink-0">
                            <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24"><path d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z"/></svg>
                        </div>
                        <div>
                            <span class="block font-bold">YouTube Linki</span>
                            <span class="text-[11px] text-white/30 uppercase tracking-tight">URL ile kütüphaneye indir</span>
                        </div>
                    </button>
                </div>

            {:else if gorunum === 'detay' || gorunum === 'youtube'}
                <div class="space-y-6" in:fly={{ x: 12, duration: 400 }}>
                    <button onclick={() => gorunum = 'secim'} class="text-[10px] font-bold text-blue-400 uppercase tracking-widest flex items-center gap-1 hover:text-blue-300 transition-colors">
                        <svg class="w-3 h-3" fill="none" stroke="currentColor" stroke-width="3" viewBox="0 0 24 24"><path d="M15 19l-7-7 7-7"></path></svg> Geri Dön
                    </button>
                    
                    <div class="space-y-4 max-h-[400px] overflow-y-auto custom-scrollbar pr-2">
                        {#if gorunum === 'youtube'}
                            <div class="space-y-2">
                                <label for="yt-url" class="text-[10px] font-bold text-white/20 uppercase tracking-widest">Video URL</label>
                                <input id="yt-url" type="text" bind:value={youtubeLink} placeholder="https://..." class="w-full bg-white/[0.02] border border-white/10 rounded-lg p-3.5 text-sm focus:border-blue-500/50 outline-none transition-all" />
                            </div>
                        {:else}
                            <div class="space-y-2">
                                <label for="s-isim" class="text-[10px] font-bold text-white/20 uppercase tracking-widest">Şarkı Adı</label>
                                <input id="s-isim" bind:value={formVerisi.isim} class="w-full bg-white/[0.02] border border-white/10 rounded-lg p-3.5 text-sm outline-none focus:border-blue-500/50" />
                            </div>
                            <div class="grid grid-cols-2 gap-4">
                                <div class="space-y-2">
                                    <label for="s-artist" class="text-[10px] font-bold text-white/20 uppercase tracking-widest">Sanatçı</label>
                                    <input id="s-artist" bind:value={formVerisi.sarkici} class="w-full bg-white/[0.02] border border-white/10 rounded-lg p-3.5 text-sm outline-none focus:border-blue-500/50" />
                                </div>
                                <div class="space-y-2">
                                    <label for="s-album" class="text-[10px] font-bold text-white/20 uppercase tracking-widest">Albüm</label>
                                    <input id="s-album" bind:value={formVerisi.album} class="w-full bg-white/[0.02] border border-white/10 rounded-lg p-3.5 text-sm outline-none focus:border-blue-500/50" />
                                </div>
                            </div>
                        {/if}

                        <div class="space-y-3 pt-2">
                            <div class="flex justify-between items-center px-1">
                                <span class="text-[10px] font-bold text-white/20 uppercase tracking-widest">Kategori Seçimi</span>
                                {#if tespitEdilenTarz && gorunum !== 'youtube'}
                                    <span class="text-[9px] bg-blue-500/10 text-blue-400 px-2 py-0.5 rounded font-bold uppercase">{tespitEdilenTarz}</span>
                                {/if}
                            </div>
                            <div class="grid grid-cols-4 gap-2">
                                {#each tarzlar as tarz}
                                    <button 
                                        onclick={() => secilenTarz = tarz}
                                        class="py-2 text-[10px] font-bold rounded-lg border transition-all {secilenTarz === tarz ? 'bg-blue-600 border-blue-500 text-white' : 'bg-white/[0.02] border-transparent text-white/30 hover:bg-white/[0.05]'}"
                                    >
                                        {tarz}
                                    </button>
                                {/each}
                            </div>
                        </div>
                    </div>

                    <button 
                        onclick={kaydet}
                        disabled={(gorunum === 'youtube' && !youtubeLink) || (gorunum === 'detay' && !formVerisi.isim)}
                        class="w-full bg-white text-black font-black py-4 rounded-xl shadow-xl hover:bg-blue-600 hover:text-white transition-all disabled:opacity-10 active:scale-[0.98] mt-4 uppercase tracking-[0.2em] text-[11px]"
                    >
                        {gorunum === 'youtube' ? 'İndirmeyi Başlat' : 'Kütüphaneye Ekle'}
                    </button>
                </div>

            {:else if gorunum === 'basarili'}
                <div class="text-center py-6" in:fly={{ y: 20, duration: 500 }}>
                    <div class="w-16 h-16 bg-green-500/10 text-green-500 rounded-full flex items-center justify-center mx-auto mb-6 border border-green-500/20">
                        <svg class="w-8 h-8" fill="none" stroke="currentColor" stroke-width="3" viewBox="0 0 24 24"><polyline points="20 6 9 17 4 12"></polyline></svg>
                    </div>
                    <h2 class="text-xl font-bold uppercase tracking-tighter mb-2 italic">İşlem Başarılı</h2>
                    <p class="text-white/40 text-xs mb-10 px-6 leading-relaxed uppercase">"{formVerisi.isim}" kütüphaneye eklendi.</p>
                    <div class="grid gap-3">
                        <button onclick={baskaEkle} class="w-full bg-blue-600 text-white font-black py-4 rounded-xl hover:bg-blue-500 transition-all uppercase text-[11px]">Yeni Kayıt</button>
                        <button onclick={kapat} class="w-full bg-white/[0.03] text-white/40 font-bold py-3.5 rounded-xl hover:bg-white/[0.06] hover:text-white transition-all uppercase text-[10px]">Kapat</button>
                    </div>
                </div>
            {/if}
        </div>
    </div>
</div>

<style>
    div[role="dialog"]:focus {
        outline: none;
    }

    /* Scrollbar Tasarımı */
    .custom-scrollbar {
        scrollbar-width: thin;
        scrollbar-color: rgba(255, 255, 255, 0.1) transparent;
    }

    :global(.custom-scrollbar::-webkit-scrollbar) {
        width: 4px;
    }
    :global(.custom-scrollbar::-webkit-scrollbar-track) {
        background: transparent;
    }
    :global(.custom-scrollbar::-webkit-scrollbar-thumb) {
        background: rgba(255, 255, 255, 0.1);
        border-radius: 10px;
    }
</style>