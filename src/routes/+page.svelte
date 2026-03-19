<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { playerState, sarkiCal, initializePlayer } from '../store.svelte';
  import { fade, fly } from 'svelte/transition';

  let saat = new Date().getHours();
  let karsilama = $derived(
    saat < 6 ? "İyi Geceler" : 
    saat < 12 ? "Günaydın" : 
    saat < 18 ? "İyi Günler" : 
    "İyi Akşamlar"
  );

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

  let favoriSanatcilar = $derived(
    Object.entries(
      playerState.sarkiListesi.reduce((acc, sarki) => {
        const sanatci = sarki.sarkici || "Bilinmeyen Sanatçı";
        acc[sanatci] = (acc[sanatci] || 0) + (sarki.dinlenme_sayisi || 0);
        return acc;
      }, {} as Record<string, number>)
    )
    .sort((a, b) => b[1] - a[1])
    .slice(0, 6)
    .map(entry => ({ isim: entry[0], skor: entry[1] }))
  );

  let yeniEklenenler = $derived(
    [...playerState.sarkiListesi].reverse().slice(0, 5)
  );

  onMount(async () => {
    if (playerState.sarkiListesi.length === 0) {
      await initializePlayer();
    }
  });
</script>

<div class="p-8 lg:p-10 w-full min-h-full pb-32 flex flex-col relative min-w-0 overflow-y-auto custom-scrollbar bg-transparent text-[var(--text-main)] transition-colors duration-500">
  
  {#if playerState.sarkiListesi.length === 0}
    <div class="flex flex-col items-center justify-center flex-1 mt-10 p-10 bg-[var(--bg-card)] border border-[var(--border)] rounded-[var(--radius)] border-dashed" in:fade>
      <div class="mb-6 opacity-40">
        <svg class="w-20 h-20 text-[var(--text-main)]" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24">
          <path d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path>
        </svg>
      </div>
      <h3 class="text-2xl font-bold mb-2">Sistem Kaydı Bulunamadı</h3>
      <p class="text-[var(--text-dim)] mb-8 max-w-md text-center">Arşivin henüz boş. Bilgisayarından veya dış kaynaklardan yeni parçalar aktararak kütüphaneni oluşturmaya başla.</p>
      <button type="button" onclick={() => playerState.isAddMusicModalOpen = true} class="bg-[var(--accent)] hover:opacity-90 text-white px-8 py-3 rounded-full font-bold shadow-lg transition-all hover:scale-105 flex items-center gap-2 tracking-widest uppercase text-xs">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M12 4v16m8-8H4"></path></svg>
        Müzik Ekle
      </button>
    </div>
  {:else}
    
    <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 mb-8">
      <h1 class="text-3xl lg:text-4xl font-black tracking-tight drop-shadow-md">{karsilama}</h1>
      <div class="flex gap-3">
        <div class="bg-[var(--bg-surface)] border border-[var(--border)] px-3 py-1.5 rounded-[var(--radius)] flex items-center gap-2 text-xs font-bold text-[var(--text-dim)] tracking-wider uppercase">
          <svg class="w-4 h-4 text-[var(--accent)]" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="9" y1="3" x2="9" y2="21"></line></svg>
          {playerState.sarkiListesi.length} Parça
        </div>
        <div class="bg-[var(--bg-surface)] border border-[var(--border)] px-3 py-1.5 rounded-[var(--radius)] flex items-center gap-2 text-xs font-bold text-[var(--text-dim)] tracking-wider uppercase">
          <svg class="w-4 h-4 text-[var(--accent-sec)]" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"></polygon></svg>
          {toplamDinlenme} Dinlenme
        </div>
      </div>
    </div>

    {#if gununSarkisi}
      <div class="w-full relative rounded-[var(--radius)] overflow-hidden mb-10 group shadow-2xl border border-[var(--border)]">
        <div class="absolute inset-0 bg-gradient-to-r from-[var(--bg-main)] via-[var(--bg-main)]/70 to-transparent z-10"></div>
        {#if gununSarkisi.kapak_yolu}
          <img src={convertFileSrc(gununSarkisi.kapak_yolu)} alt="" class="absolute inset-0 w-full h-full object-cover blur-sm opacity-50 group-hover:scale-105 group-hover:blur-md transition-all duration-700" />
        {:else}
          <div class="absolute inset-0 bg-gradient-to-br from-[var(--accent)] to-[var(--accent-sec)] opacity-20"></div>
        {/if}
        
        <div class="relative z-20 p-8 lg:p-10 flex flex-col md:flex-row items-center md:items-end gap-6 md:gap-8">
          <div class="w-32 h-32 md:w-48 md:h-48 shrink-0 rounded-[var(--radius)] overflow-hidden shadow-2xl border border-white/10 relative">
            {#if gununSarkisi.kapak_yolu}
              <img src={convertFileSrc(gununSarkisi.kapak_yolu)} alt="" class="w-full h-full object-cover" />
            {:else}
              <div class="w-full h-full bg-[var(--bg-surface)] flex items-center justify-center text-4xl text-[var(--text-dim)]">
                <svg class="w-12 h-12" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"></path></svg>
              </div>
            {/if}
          </div>
          
          <div class="flex flex-col flex-1 text-center md:text-left">
            <span class="text-xs font-bold tracking-[0.2em] text-[var(--accent)] uppercase mb-2">Günün Öne Çıkanı</span>
            <h2 class="text-4xl md:text-5xl font-black mb-2 truncate leading-tight">{gununSarkisi.isim}</h2>
            <p class="text-lg text-[var(--text-dim)] font-medium mb-6">{gununSarkisi.sarkici}</p>
            
            <button onclick={() => sarkiCal(gununSarkisi!)} class="bg-[var(--accent)] hover:opacity-90 text-white w-fit mx-auto md:mx-0 px-8 py-3 rounded-full font-bold shadow-xl transition-all hover:scale-105 flex items-center gap-3 uppercase tracking-widest text-xs">
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
              Şimdi Dinle
            </button>
          </div>
        </div>
      </div>
    {/if}
    
    {#if hizliErisim.length > 0}
      <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-4 mb-10">
        {#each hizliErisim as sarki}
          <div 
            role="button" 
            tabindex="0" 
            onclick={() => sarkiCal(sarki)}
            onkeydown={(e) => e.key === 'Enter' && sarkiCal(sarki)}
            class="flex items-center bg-[var(--bg-card)] hover:bg-[var(--bg-card-hover)] border border-[var(--border)] hover:border-[var(--accent)]/30 rounded-[var(--radius)] overflow-hidden cursor-pointer group transition-all duration-300 shadow-lg"
          >
            <div class="w-16 h-16 bg-black/40 shrink-0 relative">
              {#if sarki.kapak_yolu}
                <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover" />
              {:else}
                <div class="w-full h-full flex items-center justify-center opacity-30">
                  <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"></path></svg>
                </div>
              {/if}
              <div class="absolute inset-0 bg-black/40 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity">
                <div class="w-8 h-8 bg-[var(--accent)] rounded-full flex items-center justify-center shadow-lg transform scale-75 group-hover:scale-100 transition-all">
                  <svg class="w-4 h-4 text-white ml-1" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                </div>
              </div>
            </div>
            <div class="flex flex-col px-4 min-w-0">
              <span class="font-bold text-sm truncate group-hover:text-[var(--accent)] transition-colors">{sarki.isim}</span>
              <span class="text-[var(--text-dim)] font-medium text-xs truncate">{sarki.sarkici}</span>
            </div>
          </div>
        {/each}
      </div>
    {/if}

    {#if favoriSanatcilar.length > 0}
      <div class="mb-10">
        <div class="flex items-end justify-between mb-4">
          <h2 class="text-xl font-bold tracking-wide">Sık Dinlediğin Sanatçılar</h2>
          <a href="/artists" class="text-xs font-bold text-[var(--text-dim)] hover:text-[var(--text-main)] transition-colors uppercase tracking-widest">Tümünü Gör</a>
        </div>
        <div class="flex gap-6 overflow-x-auto custom-scrollbar pb-4">
          {#each favoriSanatcilar as sanatci}
            <a href="/artist/{encodeURIComponent(sanatci.isim)}" class="flex flex-col items-center gap-3 group min-w-[120px] cursor-pointer">
              <div class="w-28 h-28 rounded-full bg-[var(--bg-surface)] border-2 border-transparent group-hover:border-[var(--accent-sec)]/50 shadow-lg flex items-center justify-center overflow-hidden transition-all duration-300 group-hover:-translate-y-1">
                <svg class="w-10 h-10 text-[var(--text-dim)] group-hover:text-[var(--accent-sec)] transition-colors" fill="currentColor" viewBox="0 0 24 24"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/></svg>
              </div>
              <span class="font-bold text-sm text-center truncate w-full group-hover:text-[var(--accent-sec)] transition-colors">{sanatci.isim}</span>
            </a>
          {/each}
        </div>
      </div>
    {/if}

    {#if enCokDinlenenler.slice(0,5).length > 0}
      <div class="mb-10">
        <h2 class="text-xl font-bold tracking-wide mb-4">Senin İçin Zirvedekiler</h2>
        <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-6">
          {#each enCokDinlenenler.slice(0,5) as sarki}
            <div 
              role="button" tabindex="0" 
              onclick={() => sarkiCal(sarki)}
              onkeydown={(e) => e.key === 'Enter' && sarkiCal(sarki)}
              class="bg-[var(--bg-card)] hover:bg-[var(--bg-card-hover)] border border-[var(--border)] hover:border-[var(--accent)]/30 p-4 rounded-[var(--radius)] group transition-all duration-300 cursor-pointer shadow-lg flex flex-col"
            >
              <div class="w-full aspect-square bg-black/40 rounded-[var(--radius)] mb-4 relative overflow-hidden shadow-inner">
                {#if sarki.kapak_yolu}
                  <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500" />
                {:else}
                  <div class="w-full h-full flex items-center justify-center opacity-20 group-hover:scale-105 transition-transform duration-500">
                    <svg class="w-10 h-10" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"></circle><circle cx="12" cy="12" r="3"></circle></svg>
                  </div>
                {/if}
                <div class="absolute bottom-2 right-2 w-10 h-10 bg-[var(--accent)] rounded-full flex items-center justify-center opacity-0 translate-y-4 group-hover:opacity-100 group-hover:translate-y-0 transition-all duration-300 shadow-lg">
                  <svg class="w-4 h-4 text-white ml-1" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                </div>
              </div>
              <span class="font-bold text-sm truncate mb-1 group-hover:text-[var(--accent)] transition-colors">{sarki.isim}</span>
              <span class="text-[var(--text-dim)] font-medium text-xs truncate line-clamp-1">{sarki.sarkici}</span>
              <span class="text-[10px] text-[var(--accent)] font-black tracking-widest mt-auto pt-3 uppercase">{sarki.dinlenme_sayisi} Dinlenme</span>
            </div>
          {/each}
        </div>
      </div>
    {/if}

    {#if yeniEklenenler.length > 0}
      <div class="mb-4">
        <div class="flex items-end justify-between mb-4">
          <h2 class="text-xl font-bold tracking-wide">Yeni Eklenenler</h2>
          <a href="/library" class="text-xs font-bold text-[var(--text-dim)] hover:text-[var(--text-main)] transition-colors uppercase tracking-widest">Tümünü Gör</a>
        </div>
        <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-6">
          {#each yeniEklenenler as sarki}
            <div 
              role="button" tabindex="0" 
              onclick={() => sarkiCal(sarki)}
              onkeydown={(e) => e.key === 'Enter' && sarkiCal(sarki)}
              class="bg-[var(--bg-card)] hover:bg-[var(--bg-card-hover)] border border-[var(--border)] hover:border-[var(--accent-sec)]/30 p-4 rounded-[var(--radius)] group transition-all duration-300 cursor-pointer shadow-lg flex flex-col"
            >
              <div class="w-full aspect-square bg-black/40 rounded-[var(--radius)] mb-4 relative overflow-hidden shadow-inner">
                {#if sarki.kapak_yolu}
                  <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500" />
                {:else}
                  <div class="w-full h-full flex items-center justify-center opacity-20">
                     <svg class="w-10 h-10" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M12 2a10 10 0 100 20 10 10 0 000-20z"></path><path d="M12 6v6l4 2"></path></svg>
                  </div>
                {/if}
                <div class="absolute bottom-2 right-2 w-10 h-10 bg-[var(--accent-sec)] rounded-full flex items-center justify-center opacity-0 translate-y-4 group-hover:opacity-100 group-hover:translate-y-0 transition-all duration-300 shadow-lg">
                  <svg class="w-4 h-4 text-white ml-1" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                </div>
              </div>
              <span class="font-bold text-sm truncate mb-1 group-hover:text-[var(--accent-sec)] transition-colors">{sarki.isim}</span>
              <span class="text-[var(--text-dim)] font-medium text-xs truncate">{sarki.sarkici}</span>
              
              <div class="mt-auto pt-3 flex gap-2">
                 {#if sarki.kalite && sarki.kalite.trim() !== ""}
                   <span class="text-[9px] px-1.5 py-0.5 rounded bg-white/5 text-[var(--text-dim)] border border-[var(--border)] font-bold uppercase">{sarki.kalite}</span>
                 {/if}
                 {#if sarki.tarz}
                   <span class="text-[9px] px-1.5 py-0.5 rounded bg-[var(--accent-sec)]/10 text-[var(--accent-sec)] border border-[var(--accent-sec)]/20 font-bold uppercase truncate">{sarki.tarz}</span>
                 {/if}
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
</style>