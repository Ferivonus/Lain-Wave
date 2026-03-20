<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { listen, type UnlistenFn } from '@tauri-apps/api/event';
    import { open } from '@tauri-apps/plugin-dialog';
    import { playerState, type Sarki } from '../store.svelte';
    import { fade, scale, fly, slide } from 'svelte/transition';
    import { cubicOut } from 'svelte/easing';

    let gorunum = $state<'secim' | 'detay' | 'youtube' | 'basarili'>('secim');
    let yukleniyor = $state(false);
    
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

    let downloadInfo = $state({
        pct: 0,
        speed: "0KiB/s",
        eta: "00:00"
    });

    let aramaSorgusu = $state("");
    let aramaYapiliyor = $state(false);
    let aramaSonuclari = $state<any[]>([]);
    let aramaMesaji = $state("");

    const tarzlar = ["Pop", "Rock", "Lofi", "Electronic", "Jazz", "Hip-Hop", "Classical", "Podcast"];

    $effect(() => {
        let unlistenFn: UnlistenFn;
        
        listen("download-progress", (event: any) => {
            downloadInfo.pct = event.payload.percentage;
            downloadInfo.speed = event.payload.speed;
            downloadInfo.eta = event.payload.eta;
        }).then((fn) => {
            unlistenFn = fn;
        });

        return () => {
            if (unlistenFn) unlistenFn();
        };
    });

    function kapat() {
        if (yukleniyor) return;
        playerState.isAddMusicModalOpen = false;
        setTimeout(baskaEkle, 300); 
    }

    function handleBackdropKeydown(e: KeyboardEvent) {
        if (e.key === 'Escape') {
            kapat();
        }
    }

    function baskaEkle() {
        gorunum = 'secim';
        secilenDosyaPath = "";
        formVerisi = { isim: "", sarkici: "", album: "", yil: null, notlar: "" };
        downloadInfo = { pct: 0, speed: "0KiB/s", eta: "00:00" };
        aramaSorgusu = "";
        aramaSonuclari = [];
        aramaMesaji = "";
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

    async function muzikAra() {
        if (!aramaSorgusu.trim()) return;

        if (aramaSorgusu.includes("http://") || aramaSorgusu.includes("https://")) {
            await kaydet(aramaSorgusu);
            return;
        }

        aramaYapiliyor = true;
        aramaSonuclari = [];
        aramaMesaji = "Ağ taranıyor...";

        try {
            const sonuclar = await invoke<any[]>('youtube_arama', { sorgu: aramaSorgusu });
            aramaSonuclari = sonuclar;
            if (sonuclar.length === 0) aramaMesaji = "Sinyal bulunamadı.";
            else aramaMesaji = "";
        } catch (e) {
            aramaMesaji = "Hata: " + e;
        } finally {
            aramaYapiliyor = false;
        }
    }

    async function kaydet(hedefUrl?: string) {
        downloadInfo.pct = 0;
        yukleniyor = true;
        try {
            let sarki: Sarki;
            if (gorunum === 'youtube' && hedefUrl) {
                sarki = await invoke('youtube_indir', { url: hedefUrl, tarz: secilenTarz });
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

{#if playerState.isAddMusicModalOpen}
    <div 
        class="fixed inset-0 z-[100] flex items-center justify-center bg-black/80 backdrop-blur-md p-4 transition-colors duration-500" 
        transition:fade={{ duration: 200 }} 
        onclick={kapat}
        onkeydown={handleBackdropKeydown}
        role="button"
        aria-label="Modalı Kapat"
        tabindex="-1"
    >
        <div 
            class="bg-[var(--bg-surface)] text-[var(--text-main)] w-full max-md:max-h-[90vh] max-w-md rounded-[var(--radius)] shadow-2xl overflow-hidden relative border border-[var(--border)] transition-all duration-500 flex flex-col" 
            transition:scale={{ start: 0.95, duration: 300, easing: cubicOut }} 
            onclick={(e) => e.stopPropagation()} 
            onkeydown={(e) => e.stopPropagation()}
            role="dialog"
            aria-modal="true"
            tabindex="-1"
        >
            <div class="flex justify-between items-center px-6 py-4 bg-[var(--bg-card)] border-b border-[var(--border)] shrink-0">
                <h2 class="text-sm font-black uppercase tracking-widest text-[var(--text-main)]/70">Sisteme Veri Aktar</h2>
                <button type="button" onclick={kapat} class="p-2 -mr-2 text-[var(--text-dim)] hover:text-white transition-colors" aria-label="Kapat">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
                </button>
            </div>

            {#if yukleniyor}
                <div class="absolute inset-0 bg-[var(--bg-surface)]/95 backdrop-blur-md z-50 flex flex-col items-center justify-center p-10 text-center" in:fade>
                    <div class="w-full max-w-[260px]">
                        <div class="flex justify-between text-[10px] mb-3 font-black uppercase tracking-widest">
                            <span class="text-[var(--accent)]">Veri İşleniyor</span>
                            <span>{Math.round(downloadInfo.pct)}%</span>
                        </div>
                        <div class="w-full h-1.5 bg-[var(--border)] rounded-full overflow-hidden">
                            <div class="h-full bg-[var(--accent)] transition-all duration-300" style="width: {downloadInfo.pct}%"></div>
                        </div>
                        <p class="text-[10px] text-[var(--text-dim)] mt-4 font-mono">
                            {#if gorunum === 'youtube'}{downloadInfo.speed} • ETA: {downloadInfo.eta}{:else}Bitstream doğrulanıyor...{/if}
                        </p>
                    </div>
                </div>
            {/if}

            <div class="p-6 overflow-y-auto custom-scrollbar flex-1">
                {#if gorunum === 'secim'}
                    <div class="grid gap-3" in:fly={{ y: 8, duration: 400 }}>
                        <button onclick={dosyaSec} class="flex items-center gap-5 p-5 bg-[var(--bg-card)] hover:bg-[var(--bg-card-hover)] border border-[var(--border)] rounded-2xl transition-all text-left group">
                            <div class="w-11 h-11 bg-[var(--accent)]/10 text-[var(--accent)] rounded-xl flex items-center justify-center shrink-0 group-hover:scale-110 transition-transform">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="17 8 12 3 7 8"></polyline><line x1="12" y1="3" x2="12" y2="15"></line></svg>
                            </div>
                            <div>
                                <span class="block font-bold">Yerel Diskten Aktar</span>
                                <span class="text-[11px] text-[var(--text-dim)] uppercase tracking-tight">Cihazdaki ses dosyalarını seç</span>
                            </div>
                        </button>

                        <button onclick={() => gorunum = 'youtube'} class="flex items-center gap-5 p-5 bg-[var(--bg-card)] hover:bg-[var(--bg-card-hover)] border border-[var(--border)] rounded-2xl transition-all text-left group">
                            <div class="w-11 h-11 bg-red-500/10 text-red-500 rounded-xl flex items-center justify-center shrink-0 group-hover:scale-110 transition-transform">
                                <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24"><path d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z"/></svg>
                            </div>
                            <div>
                                <span class="block font-bold">Ağdan Veri İndir</span>
                                <span class="text-[11px] text-[var(--text-dim)] uppercase tracking-tight">YouTube araması yap veya link gir</span>
                            </div>
                        </button>
                    </div>

                {:else if gorunum === 'detay' || gorunum === 'youtube'}
                    <div class="space-y-6" in:fly={{ x: 12, duration: 400 }}>
                        <button onclick={() => gorunum = 'secim'} class="text-[10px] font-black text-[var(--accent)] uppercase tracking-widest flex items-center gap-1 hover:opacity-80 transition-opacity">
                            <svg class="w-3 h-3" fill="none" stroke="currentColor" stroke-width="3" viewBox="0 0 24 24"><path d="M15 19l-7-7 7-7"></path></svg> Geri Dön
                        </button>
                        
                        <div class="space-y-5 pr-2 pb-2">
                            <div class="space-y-3">
                                <div class="flex justify-between items-center px-1">
                                    <span class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-widest">Kategori Belirle</span>
                                    {#if tespitEdilenTarz && gorunum !== 'youtube'}
                                        <span class="text-[9px] bg-[var(--accent)]/10 text-[var(--accent)] px-2 py-0.5 rounded font-bold uppercase tracking-tighter">{tespitEdilenTarz}</span>
                                    {/if}
                                </div>
                                <div class="grid grid-cols-4 gap-2">
                                    {#each tarzlar as tarz}
                                        <button onclick={() => secilenTarz = tarz} class="py-2 text-[9px] font-bold uppercase tracking-widest rounded-lg border transition-all {secilenTarz === tarz ? 'bg-[var(--accent)] border-[var(--accent)] text-white shadow-lg shadow-[var(--accent)]/20' : 'bg-[var(--bg-card)] border-transparent text-[var(--text-dim)] hover:bg-[var(--bg-card-hover)]'}">{tarz}</button>
                                    {/each}
                                </div>
                            </div>

                            {#if gorunum === 'youtube'}
                                <div class="space-y-2 pt-2 border-t border-[var(--border)]">
                                    <label for="yt-search-input" class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-widest">Arama veya URL</label>
                                    <div class="flex gap-2">
                                        <input id="yt-search-input" type="text" bind:value={aramaSorgusu} onkeydown={(e) => e.key === 'Enter' && !aramaYapiliyor && muzikAra()} placeholder="Tarkan..." class="flex-1 bg-[var(--bg-card)] border border-[var(--border)] rounded-xl p-3.5 text-sm focus:border-red-500/50 outline-none transition-all font-mono" />
                                        <button onclick={muzikAra} disabled={aramaYapiliyor || !aramaSorgusu.trim()} class="bg-red-500 hover:bg-red-600 text-white rounded-xl px-5 font-black uppercase tracking-widest text-[10px] transition-all disabled:opacity-50 min-w-[80px]">
                                            {#if aramaYapiliyor}...{:else}Tara{/if}
                                        </button>
                                    </div>
                                </div>

                                {#if aramaMesaji}<div class="text-[10px] font-mono text-[var(--accent)] uppercase tracking-widest" in:slide>{aramaMesaji}</div>{/if}

                                {#if aramaSonuclari.length > 0}
                                    <div class="flex flex-col gap-2 mt-2">
                                        {#each aramaSonuclari as sonuc}
                                            <button onclick={() => kaydet(sonuc.webpage_url)} class="flex items-center gap-3 p-2 bg-[var(--bg-card)] hover:bg-red-500/10 border border-[var(--border)] hover:border-red-500/30 rounded-xl transition-all text-left group">
                                                <div class="w-12 h-9 bg-black rounded-md overflow-hidden shrink-0 relative"><img src={sonuc.thumbnail} alt="" class="w-full h-full object-cover opacity-80 group-hover:opacity-100 transition-opacity" /></div>
                                                <div class="flex-1 min-w-0">
                                                    <p class="text-xs font-bold text-[var(--text-main)] truncate group-hover:text-red-400 transition-colors">{sonuc.title}</p>
                                                    <div class="flex items-center gap-2 mt-0.5"><span class="text-[8px] font-black text-[var(--text-dim)] uppercase truncate max-w-[100px]">{sonuc.channel}</span><span class="w-1 h-1 bg-[var(--border)] rounded-full"></span><span class="text-[8px] font-mono text-[var(--text-dim)]">{sonuc.duration_string}</span></div>
                                                </div>
                                            </button>
                                        {/each}
                                    </div>
                                {/if}
                            {:else}
                                <div class="space-y-4 pt-2 border-t border-[var(--border)]">
                                    <div class="space-y-2">
                                        <label for="local-isim" class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-widest">Şarkı Adı</label>
                                        <input id="local-isim" bind:value={formVerisi.isim} class="w-full bg-[var(--bg-card)] border border-[var(--border)] rounded-xl p-3.5 text-sm outline-none focus:border-[var(--accent)]/50" />
                                    </div>
                                    <div class="grid grid-cols-2 gap-4">
                                        <div class="space-y-2">
                                            <label for="local-artist" class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-widest">Sanatçı</label>
                                            <input id="local-artist" bind:value={formVerisi.sarkici} class="w-full bg-[var(--bg-card)] border border-[var(--border)] rounded-xl p-3.5 text-sm outline-none focus:border-[var(--accent)]/50" />
                                        </div>
                                        <div class="space-y-2">
                                            <label for="local-album" class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-widest">Albüm</label>
                                            <input id="local-album" bind:value={formVerisi.album} class="w-full bg-[var(--bg-card)] border border-[var(--border)] rounded-xl p-3.5 text-sm outline-none focus:border-[var(--accent)]/50" />
                                        </div>
                                    </div>
                                    <button onclick={() => kaydet()} disabled={!formVerisi.isim} class="w-full bg-[var(--accent)] text-white font-black py-4 rounded-2xl shadow-xl hover:opacity-90 transition-all disabled:opacity-20 uppercase tracking-[0.2em] text-[11px] mt-4">Kütüphaneye Ekle</button>
                                </div>
                            {/if}
                        </div>
                    </div>

                {:else if gorunum === 'basarili'}
                    <div class="text-center py-6" in:fly={{ y: 20, duration: 500 }}>
                        <div class="w-16 h-16 bg-emerald-500/10 text-emerald-500 rounded-full flex items-center justify-center mx-auto mb-6 border border-emerald-500/20">
                            <svg class="w-8 h-8" fill="none" stroke="currentColor" stroke-width="3" viewBox="0 0 24 24"><polyline points="20 6 9 17 4 12"></polyline></svg>
                        </div>
                        <h2 class="text-xl font-bold uppercase tracking-tighter mb-2 italic">Aktarım Başarılı</h2>
                        <p class="text-[var(--text-dim)] text-xs mb-10 px-6 leading-relaxed uppercase">"{formVerisi.isim}" kütüphane veritabanına işlendi.</p>
                        <div class="grid gap-3">
                            <button onclick={baskaEkle} class="w-full bg-[var(--accent)] text-white font-black py-4 rounded-2xl hover:opacity-90 transition-all uppercase text-[11px] tracking-widest">Yeni Giriş Yap</button>
                            <button onclick={kapat} class="w-full bg-[var(--bg-card)] text-[var(--text-dim)] font-bold py-3.5 rounded-2xl hover:bg-[var(--bg-card-hover)] transition-all uppercase text-[10px] tracking-widest">Kapat</button>
                        </div>
                    </div>
                {/if}
            </div>
        </div>
    </div>
{/if}

<style>
    div[role="dialog"]:focus { outline: none; }
    .custom-scrollbar::-webkit-scrollbar { width: 4px; }
    .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
</style>