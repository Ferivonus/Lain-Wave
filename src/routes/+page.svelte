<script lang="ts">
    import { onMount } from 'svelte';
    import { convertFileSrc } from '@tauri-apps/api/core';
    import { playerState, sarkiCal, initializePlayer } from '../store.svelte';
    import { fade, fly } from 'svelte/transition';
    import FavoriteButton from '$lib/FavoriteButton.svelte';

    let saat = new Date().getHours();
    
    let karsilama = $derived.by(() => {
        const mesaj = saat < 6 ? "İyi Geceler" : 
                      saat < 12 ? "Günaydın" : 
                      saat < 18 ? "İyi Günler" : 
                      "İyi Akşamlar";
        
        return playerState.username ? `${mesaj}, ${playerState.username}` : mesaj;
    });

    let toplamDinlenme = $derived(
        playerState.sarkiListesi.reduce((acc, sarki) => acc + (sarki.dinlenme_sayisi || 0), 0)
    );

    let enCokDinlenenler = $derived(
        [...playerState.sarkiListesi]
            .filter(s => (s.dinlenme_sayisi || 0) > 0)
            .sort((a, b) => (b.dinlenme_sayisi || 0) - (a.dinlenme_sayisi || 0))
    );
    
    let gununSarkisi = $derived(
        enCokDinlenenler.length > 0 ? enCokDinlenenler[0] : playerState.sarkiListesi[0]
    );

    let hizliErisim = $derived(
        [...playerState.sarkiListesi]
            .filter(s => s.son_dinlenme_tarihi) 
            .sort((a, b) => (b.son_dinlenme_tarihi || 0) - (a.son_dinlenme_tarihi || 0))
            .slice(0, 6)
    );

    let favoriSanatcilar = $derived.by(() => {
        const sanatciMap = playerState.sarkiListesi.reduce((acc, sarki) => {
            const sanatci = sarki.sarkici || "Bilinmeyen Sanatçı";
            if (!acc[sanatci]) {
                acc[sanatci] = { isim: sanatci, skor: 0, kapak: sarki.kapak_yolu };
            }
            acc[sanatci].skor += (sarki.dinlenme_sayisi || 0);
            if (!acc[sanatci].kapak && sarki.kapak_yolu) {
                acc[sanatci].kapak = sarki.kapak_yolu;
            }
            return acc;
        }, {} as Record<string, {isim: string, skor: number, kapak: string | undefined}>);

        return Object.values(sanatciMap)
            .sort((a, b) => b.skor - a.skor)
            .slice(0, 6);
    });

    let yeniEklenenler = $derived(
        [...playerState.sarkiListesi].reverse().slice(0, 5)
    );

    onMount(async () => {
        if (playerState.sarkiListesi.length === 0) {
            await initializePlayer();
        }
    });
</script>

<div class="p-8 lg:p-10 w-full min-h-full pb-32 flex flex-col relative min-w-0 overflow-y-auto custom-scrollbar bg-transparent text-(--text-main) transition-colors duration-500">
    
    {#if playerState.sarkiListesi.length === 0}
        <div class="flex flex-col items-center justify-center flex-1 mt-10 p-10 bg-(--bg-card) border border-(--border) rounded-(--radius) border-dashed" in:fade>
            <div class="mb-6 opacity-40">
                <svg class="w-20 h-20 text-(--text-main)" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24">
                    <path d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path>
                </svg>
            </div>
            <h3 class="text-2xl font-bold mb-2 uppercase tracking-tight">Sistem Kaydı Bulunamadı</h3>
            <p class="text-(--text-dim) mb-8 max-w-md text-center font-medium">Arşivin henüz boş. Bilgisayarından veya dış kaynaklardan yeni parçalar aktararak kütüphaneni oluşturmaya başla.</p>
            <button type="button" onclick={() => playerState.isAddMusicModalOpen = true} class="bg-(--accent) hover:opacity-90 text-white px-8 py-3 rounded-full font-black shadow-[0_10px_30px_var(--accent-glow)] transition-all hover:scale-105 active:scale-95 flex items-center gap-2 tracking-widest uppercase text-xs">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M12 4v16m8-8H4"></path></svg>
                Müzik Ekle
            </button>
        </div>
    {:else}
        
        <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 mb-8" in:fly={{ y: -20, duration: 600 }}>
            <h1 class="text-4xl lg:text-5xl font-black tracking-tighter drop-shadow-md italic uppercase leading-none truncate flex-1 pr-4" title={karsilama}>{karsilama}</h1>
            <div class="flex gap-3 shrink-0">
                <div class="bg-(--bg-surface) border border-(--border) px-4 py-2 rounded-xl flex items-center gap-2 text-[10px] font-black text-(--text-dim) tracking-[0.2em] uppercase shadow-sm">
                    <svg class="w-4 h-4 text-(--accent)" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="9" y1="3" x2="9" y2="21"></line></svg>
                    {playerState.sarkiListesi.length} Parça
                </div>
                <div class="bg-(--bg-surface) border border-(--border) px-4 py-2 rounded-xl flex items-center gap-2 text-[10px] font-black text-(--text-dim) tracking-[0.2em] uppercase shadow-sm">
                    <svg class="w-4 h-4 text-(--accent-sec)" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"></polygon></svg>
                    {toplamDinlenme} Dinlenme
                </div>
            </div>
        </div>

        {#if gununSarkisi}
            <div class="w-full relative rounded-(--radius) overflow-hidden mb-12 group shadow-2xl border border-(--border)" in:fade>
                <div class="absolute inset-0 bg-linear-to-r from-(--bg-main) via-(--bg-main)/80 to-transparent z-10"></div>
                {#if gununSarkisi.kapak_yolu}
                    <img src={convertFileSrc(gununSarkisi.kapak_yolu)} alt="" class="absolute inset-0 w-full h-full object-cover blur-md opacity-40 group-hover:scale-105 group-hover:opacity-50 transition-all duration-1000" />
                {:else}
                    <div class="absolute inset-0 bg-linear-to-br from-(--accent)/30 to-(--accent-sec)/30"></div>
                {/if}
                
                <div class="relative z-20 p-8 lg:p-10 flex flex-col md:flex-row items-center md:items-end gap-6 md:gap-10">
                    <div class="w-32 h-32 md:w-48 md:h-48 shrink-0 rounded-2xl overflow-hidden shadow-[0_20px_50px_rgba(0,0,0,0.5)] border border-white/10 relative group-hover:shadow-[0_20px_50px_var(--accent-glow)] transition-all duration-500">
                        {#if gununSarkisi.kapak_yolu}
                            <img src={convertFileSrc(gununSarkisi.kapak_yolu)} alt="" class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-700" />
                        {:else}
                            <div class="w-full h-full bg-(--bg-surface) flex items-center justify-center text-(--text-dim)">
                                <svg class="w-16 h-16 opacity-30" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"></path></svg>
                            </div>
                        {/if}
                    </div>
                    
                    <div class="flex flex-col flex-1 text-center md:text-left">
                        <span class="text-[10px] font-black tracking-[0.4em] text-(--accent) uppercase mb-3 drop-shadow-md">Günün Öne Çıkanı</span>
                        <h2 class="text-4xl md:text-5xl lg:text-6xl font-black mb-3 truncate leading-none uppercase italic tracking-tighter drop-shadow-xl">{gununSarkisi.isim}</h2>
                        <p class="text-lg text-(--text-dim) font-bold mb-8 uppercase tracking-widest">{gununSarkisi.sarkici}</p>
                        
                        <div class="flex items-center justify-center md:justify-start gap-4">
                            <button onclick={() => sarkiCal(gununSarkisi!)} class="bg-(--text-main) text-(--bg-main) hover:bg-(--accent) hover:text-white px-10 py-4 rounded-full font-black shadow-2xl transition-all hover:scale-105 active:scale-95 flex items-center gap-3 uppercase tracking-[0.2em] text-[10px]">
                                <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                                Şimdi Dinle
                            </button>
                            <div class="bg-(--bg-surface) p-2 rounded-full border border-(--border)">
                                <FavoriteButton sarkiId={gununSarkisi.id} />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        {/if}
        
        {#if hizliErisim.length > 0}
            <div class="mb-12">
                <h2 class="text-[11px] font-black text-(--text-dim) uppercase tracking-[0.3em] mb-5">Hızlı Erişim</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-4">
                    {#each hizliErisim as sarki}
                        <div 
                            role="button" 
                            tabindex="0" 
                            onclick={() => sarkiCal(sarki)}
                            onkeydown={(e) => e.key === 'Enter' && sarkiCal(sarki)}
                            class="flex items-center bg-(--bg-card) hover:bg-(--bg-card-hover) border border-(--border) hover:border-(--accent)/40 rounded-xl overflow-hidden cursor-pointer group transition-all duration-300 shadow-md active:scale-[0.98]"
                        >
                            <div class="w-16 h-16 bg-(--bg-surface) shrink-0 relative border-r border-(--border)">
                                {#if sarki.kapak_yolu}
                                    <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110" />
                                {:else}
                                    <div class="w-full h-full flex items-center justify-center opacity-20">
                                        <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"></path></svg>
                                    </div>
                                {/if}
                                <div class="absolute inset-0 bg-black/40 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity">
                                    <div class="w-8 h-8 bg-(--accent) rounded-full flex items-center justify-center shadow-lg transform scale-75 group-hover:scale-100 transition-all text-white">
                                        <svg class="w-4 h-4 ml-0.5" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                                    </div>
                                </div>
                            </div>
                            <div class="flex flex-col px-4 min-w-0 flex-1">
                                <span class="font-bold text-sm truncate group-hover:text-(--accent) transition-colors leading-tight">{sarki.isim}</span>
                                <span class="text-(--text-dim) font-bold text-[9px] uppercase tracking-widest truncate opacity-70 mt-0.5">{sarki.sarkici}</span>
                            </div>
                            <div class="px-4 opacity-0 group-hover:opacity-100 transition-opacity" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="presentation">
                                <FavoriteButton sarkiId={sarki.id} />
                            </div>
                        </div>
                    {/each}
                </div>
            </div>
        {/if}

        {#if favoriSanatcilar.length > 0}
            <div class="mb-14">
                <div class="flex items-center justify-between mb-6">
                    <h2 class="text-[11px] font-black text-(--text-dim) uppercase tracking-[0.3em]">Sık Dinlediğin Sanatçılar</h2>
                    <a href="/artists" class="text-[10px] font-black text-(--accent) hover:text-(--text-main) transition-colors uppercase tracking-widest bg-(--accent)/10 px-3 py-1.5 rounded-lg border border-(--accent)/20">Tümünü Gör</a>
                </div>
                <div class="flex gap-6 overflow-x-auto custom-scrollbar pb-4 -mx-2 px-2">
                    {#each favoriSanatcilar as sanatci}
                        <a href="/artist/{encodeURIComponent(sanatci.isim)}" class="flex flex-col items-center gap-4 group min-w-32.5 cursor-pointer">
                            <div class="w-32 h-32 rounded-full bg-(--bg-card) border-2 border-(--border) group-hover:border-(--accent-sec) shadow-lg flex items-center justify-center overflow-hidden transition-all duration-500 group-hover:-translate-y-2 group-hover:shadow-[0_15px_30px_var(--accent-glow)] relative">
                                {#if sanatci.kapak}
                                    <img src={convertFileSrc(sanatci.kapak)} alt={sanatci.isim} class="w-full h-full object-cover transition-transform duration-700 group-hover:scale-110" />
                                {:else}
                                    <svg class="w-12 h-12 text-(--text-dim)/30 group-hover:text-(--accent-sec) transition-colors relative z-10" fill="currentColor" viewBox="0 0 24 24"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/></svg>
                                {/if}
                                <div class="absolute inset-0 bg-linear-to-t from-black/50 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                            </div>
                            <span class="font-bold text-sm text-center truncate w-full group-hover:text-(--accent-sec) transition-colors tracking-tight">{sanatci.isim}</span>
                        </a>
                    {/each}
                </div>
            </div>
        {/if}

        {#if enCokDinlenenler.slice(0,5).length > 0}
            <div class="mb-14">
                <h2 class="text-[11px] font-black text-(--text-dim) uppercase tracking-[0.3em] mb-6">Senin İçin Zirvedekiler</h2>
                <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-6">
                    {#each enCokDinlenenler.slice(0,5) as sarki}
                        <div 
                            role="button" tabindex="0" 
                            onclick={() => sarkiCal(sarki)}
                            onkeydown={(e) => e.key === 'Enter' && sarkiCal(sarki)}
                            class="bg-(--bg-card) hover:bg-(--bg-card-hover) border border-(--border) hover:border-(--accent)/40 p-4 lg:p-5 rounded-2xl group transition-all duration-300 cursor-pointer shadow-lg flex flex-col active:scale-95 relative"
                        >
                            <div class="absolute top-6 right-6 z-20 opacity-0 group-hover:opacity-100 transition-opacity" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="presentation">
                                <FavoriteButton sarkiId={sarki.id} />
                            </div>

                            <div class="w-full aspect-square bg-(--bg-surface) rounded-xl mb-4 relative overflow-hidden shadow-inner border border-(--border) group-hover:border-(--accent)/30 transition-colors">
                                {#if sarki.kapak_yolu}
                                    <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-700 opacity-80 group-hover:opacity-100" />
                                {:else}
                                    <div class="w-full h-full flex items-center justify-center opacity-20 group-hover:scale-110 transition-transform duration-700">
                                        <svg class="w-12 h-12" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"></circle><circle cx="12" cy="12" r="3"></circle></svg>
                                    </div>
                                {/if}
                                <div class="absolute inset-0 bg-black/20 group-hover:bg-transparent transition-colors duration-500"></div>
                                <div class="absolute bottom-3 right-3 w-12 h-12 bg-(--accent) text-white rounded-full flex items-center justify-center opacity-0 translate-y-4 group-hover:opacity-100 group-hover:translate-y-0 transition-all duration-300 shadow-[0_10px_20px_rgba(0,0,0,0.5)]">
                                    <svg class="w-5 h-5 ml-1" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                                </div>
                            </div>
                            <div class="flex flex-col min-w-0 flex-1">
                                <span class="font-bold text-sm lg:text-base truncate mb-1 group-hover:text-(--accent) transition-colors">{sarki.isim}</span>
                                <span class="text-(--text-dim) font-bold text-[9px] uppercase tracking-widest truncate opacity-70 mb-3">{sarki.sarkici}</span>
                                <span class="text-[9px] text-(--accent) bg-(--accent)/10 border border-(--accent)/20 px-2 py-1 rounded-md font-black tracking-widest mt-auto w-fit uppercase">{sarki.dinlenme_sayisi} Dinlenme</span>
                            </div>
                        </div>
                    {/each}
                </div>
            </div>
        {/if}

        {#if yeniEklenenler.length > 0}
            <div class="mb-4">
                <div class="flex items-center justify-between mb-6">
                    <h2 class="text-[11px] font-black text-(--text-dim) uppercase tracking-[0.3em]">Yeni Eklenenler</h2>
                    <a href="/library" class="text-[10px] font-black text-(--accent-sec) hover:text-(--text-main) transition-colors uppercase tracking-widest bg-(--accent-sec)/10 px-3 py-1.5 rounded-lg border border-(--accent-sec)/20">Tümünü Gör</a>
                </div>
                <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-6">
                    {#each yeniEklenenler as sarki}
                        <div 
                            role="button" tabindex="0" 
                            onclick={() => sarkiCal(sarki)}
                            onkeydown={(e) => e.key === 'Enter' && sarkiCal(sarki)}
                            class="bg-(--bg-card) hover:bg-(--bg-card-hover) border border-(--border) hover:border-(--accent-sec)/40 p-4 lg:p-5 rounded-2xl group transition-all duration-300 cursor-pointer shadow-lg flex flex-col active:scale-95 relative"
                        >
                            <div class="absolute top-6 right-6 z-20 opacity-0 group-hover:opacity-100 transition-opacity" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="presentation">
                                <FavoriteButton sarkiId={sarki.id} />
                            </div>

                            <div class="w-full aspect-square bg-(--bg-surface) rounded-xl mb-4 relative overflow-hidden shadow-inner border border-(--border) group-hover:border-(--accent-sec)/30 transition-colors">
                                {#if sarki.kapak_yolu}
                                    <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-700 opacity-80 group-hover:opacity-100" />
                                {:else}
                                    <div class="w-full h-full flex items-center justify-center opacity-20 group-hover:scale-110 transition-transform duration-700">
                                        <svg class="w-12 h-12" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M12 2a10 10 0 100 20 10 10 0 000-20z"></path><path d="M12 6v6l4 2"></path></svg>
                                    </div>
                                {/if}
                                <div class="absolute inset-0 bg-black/20 group-hover:bg-transparent transition-colors duration-500"></div>
                                <div class="absolute bottom-3 right-3 w-12 h-12 bg-(--accent-sec) text-white rounded-full flex items-center justify-center opacity-0 translate-y-4 group-hover:opacity-100 group-hover:translate-y-0 transition-all duration-300 shadow-[0_10px_20px_rgba(0,0,0,0.5)]">
                                    <svg class="w-5 h-5 ml-1" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                                </div>
                            </div>
                            
                            <div class="flex flex-col min-w-0 flex-1">
                                <span class="font-bold text-sm lg:text-base truncate mb-1 group-hover:text-(--accent-sec) transition-colors">{sarki.isim}</span>
                                <span class="text-(--text-dim) font-bold text-[9px] uppercase tracking-widest truncate opacity-70 mb-3">{sarki.sarkici}</span>
                                
                                <div class="mt-auto flex gap-2 overflow-hidden">
                                    {#if sarki.kalite && sarki.kalite.trim() !== ""}
                                        <span class="text-[8px] px-1.5 py-0.5 rounded bg-(--bg-surface) text-(--text-dim) border border-(--border) font-black uppercase tracking-widest shrink-0">{sarki.kalite}</span>
                                    {/if}
                                    {#if sarki.tarz}
                                        <span class="text-[8px] px-1.5 py-0.5 rounded bg-(--accent-sec)/10 text-(--accent-sec) border border-(--accent-sec)/20 font-black uppercase tracking-widest truncate">{sarki.tarz}</span>
                                    {/if}
                                </div>
                            </div>
                        </div>
                    {/each}
                </div>
            </div>
        {/if}

    {/if}
</div>

<style>
    .custom-scrollbar::-webkit-scrollbar {
        width: 6px;
        height: 6px;
    }
    .custom-scrollbar::-webkit-scrollbar-track {
        background: transparent;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb {
        background: var(--border);
        border-radius: 10px;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover {
        background: var(--accent);
    }
    
    button {
        outline: none;
    }
</style>