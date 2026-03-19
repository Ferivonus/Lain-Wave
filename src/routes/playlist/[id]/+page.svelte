<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { page } from '$app/state'; 
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import SongStats from '$lib/SongStats.svelte';
  import { playerState, sarkiCal, initializePlayer, playlisttenSarkiCikar } from '../../../store.svelte';
  import { fade, fly } from 'svelte/transition';

  let playlistId = $derived(page.params.id);
  
  let aktifPlaylist = $derived(playerState.playlistler?.find(p => p.id === playlistId));

  let gosterilenSarkilar = $derived(
    playerState.sarkiListesi.filter(sarki => aktifPlaylist?.sarkilar?.includes(sarki.id))
  );

  onMount(async () => {
    if (playerState.sarkiListesi.length === 0 || playerState.playlistler.length === 0) {
      await initializePlayer();
    }
  });

  let playlistKapakGorseli = $derived(
    gosterilenSarkilar.length > 0 && gosterilenSarkilar[0].kapak_yolu 
      ? convertFileSrc(gosterilenSarkilar[0].kapak_yolu) 
      : null
  );

  function listeyiCal() {
    if (gosterilenSarkilar.length > 0) {
      sarkiCal(gosterilenSarkilar[0]);
    }
  }

  async function handleListedenCikar(sarkiId: string, isim: string, event: Event) {
    event.stopPropagation();
    if (!playlistId) return; 
    
    if (confirm(`"${isim}" adlı şarkıyı bu listeden çıkarmak istediğinize emin misiniz?`)) {
        await playlisttenSarkiCikar(playlistId, sarkiId);
    }
  }
</script>

<div class="p-8 lg:p-12 w-full min-h-full pb-32 flex flex-col relative min-w-0 bg-transparent text-[var(--text-main)] transition-colors duration-500 overflow-y-auto custom-scrollbar">
  
  <header class="flex flex-col md:flex-row gap-8 items-center md:items-end mb-12 mt-4" in:fade>
    <div class="w-52 h-52 lg:w-64 lg:h-64 bg-[var(--bg-card)] rounded-[var(--radius)] shadow-2xl flex-shrink-0 border border-white/10 overflow-hidden relative group flex items-center justify-center">
        {#if playlistKapakGorseli}
          <img src={playlistKapakGorseli} alt="Playlist Kapağı" class="w-full h-full object-cover transition-transform duration-700 group-hover:scale-110 opacity-80" />
          <div class="absolute inset-0 bg-black/20 group-hover:bg-transparent transition-colors duration-500"></div>
        {:else}
          <div class="absolute inset-0 bg-gradient-to-br from-[var(--accent)]/20 to-[var(--bg-surface)]"></div>
        {/if}

        <svg class="absolute w-24 h-24 text-white/30 drop-shadow-2xl group-hover:scale-110 transition-transform duration-500 z-10" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 14.5c-2.49 0-4.5-2.01-4.5-4.5S9.51 7.5 12 7.5s4.5 2.01 4.5 4.5-2.01 4.5-4.5 4.5zm0-5.5c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1z"/>
        </svg>
    </div>
    
    <div class="flex flex-col text-center md:text-left pb-2 min-w-0 flex-1">
      <span class="text-[10px] font-black mb-3 text-[var(--accent)] tracking-[0.4em] uppercase italic">User Playlist Collection</span>
      <h1 class="text-4xl lg:text-7xl font-black tracking-tighter leading-none mb-6 uppercase italic drop-shadow-md truncate">
        {aktifPlaylist?.isim || "Yükleniyor..."}
      </h1>
      
      <div class="flex flex-col sm:flex-row items-center gap-6">
        <p class="text-[var(--text-dim)] text-xs lg:text-sm font-bold uppercase tracking-widest whitespace-nowrap">
          {gosterilenSarkilar.length} Benzersiz Kayıt • Sıralı Liste
        </p>
        
        {#if gosterilenSarkilar.length > 0}
          <button 
            onclick={listeyiCal} 
            class="flex items-center gap-3 bg-[var(--text-main)] text-[var(--bg-main)] hover:bg-[var(--accent)] hover:text-white px-10 py-3.5 rounded-full font-black shadow-xl transition-all active:scale-95 uppercase text-[10px] lg:text-xs tracking-widest"
            aria-label="Listeyi oynat"
          >
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
            Listeyi Çal
          </button>
        {/if}
      </div>
    </div>
  </header>

  {#if gosterilenSarkilar.length === 0}
    <div class="flex flex-col items-center justify-center flex-1 text-center mt-10 p-16 bg-[var(--bg-card)] border border-[var(--border)] rounded-[var(--radius)] border-dashed" in:fade>
      <div class="text-6xl mb-8 opacity-20 filter grayscale">💽</div>
      <h3 class="text-2xl font-bold mb-3 tracking-tight uppercase">Sinyal Bulunamadı</h3>
      <p class="text-[var(--text-dim)] font-medium text-sm max-w-sm leading-relaxed uppercase tracking-wider">
        Bu çalma listesi henüz kütüphaneden veri almamış. "Arama" veya "Kütüphane" sekmelerinden şarkı ekleyebilirsin.
      </p>
    </div>
  {:else}
    <div class="flex text-[10px] font-black text-[var(--text-dim)] border-b border-[var(--border)] pb-3 mb-4 px-4 tracking-[0.2em] uppercase mt-6">
      <span class="w-10 text-center shrink-0">#</span>
      <span class="flex-1 min-w-0 ml-4">BAŞLIK</span>
      <span class="w-48 shrink-0 text-right pr-4 hidden md:block">İSTATİSTİK</span> 
      <span class="w-1/4 shrink-0 pl-6 hidden lg:block">ALBÜM</span>
      <span class="w-24 text-center shrink-0">İŞLEMLER</span>
    </div>

    <div class="flex flex-col gap-1">
      {#each gosterilenSarkilar as sarki, index}
        <div 
            role="button" tabindex="0" 
            onclick={() => sarkiCal(sarki)} 
            onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)} 
            aria-label="{sarki.isim} çal"
            class="flex items-center text-sm p-2 rounded-2xl hover:bg-[var(--bg-card-hover)] transition-all duration-300 cursor-pointer group 
            {playerState.aktifSarki?.id === sarki.id ? 'bg-[var(--accent)]/10 border border-[var(--accent)]/20 shadow-lg' : 'border border-transparent'}">
          
          <div class="w-10 text-center shrink-0">
             {#if playerState.aktifSarki?.id === sarki.id && playerState.suAnOynuyorMu}
                <div class="flex items-end justify-center gap-0.5 h-3.5 mb-1">
                   <div class="w-1 bg-[var(--accent)] animate-[bounce_1s_infinite]"></div>
                   <div class="w-1 bg-[var(--accent)] animate-[bounce_1.2s_infinite]"></div>
                   <div class="w-1 bg-[var(--accent)] animate-[bounce_0.8s_infinite]"></div>
                </div>
             {:else}
                <span class="text-[var(--text-dim)]/40 group-hover:hidden font-mono text-xs">{index + 1}</span>
                <svg class="w-4 h-4 mx-auto hidden group-hover:block text-[var(--accent)]" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M8 5v14l11-7z"/></svg>
             {/if}
          </div>
          
          <div class="flex-1 flex items-center gap-4 min-w-0 ml-4">
            <div class="w-11 h-11 bg-[var(--bg-card)] rounded-lg overflow-hidden shrink-0 shadow-md border border-[var(--border)]">
              {#if sarki.kapak_yolu}
                <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover transition-transform group-hover:scale-110" />
              {:else}
                <div class="w-full h-full flex items-center justify-center text-[var(--text-dim)]/20 bg-[var(--bg-surface)]">
                  <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/></svg>
                </div>
              {/if}
            </div>
            
            <div class="flex flex-col min-w-0 pr-2">
              <span class="font-bold text-sm truncate {playerState.aktifSarki?.id === sarki.id ? 'text-[var(--accent)]' : 'text-[var(--text-main)]'}">
                {sarki.isim}
              </span>
              <a href="/artist/{encodeURIComponent(sarki.sarkici)}" 
                 class="text-[11px] text-[var(--text-dim)] truncate font-bold uppercase tracking-tighter hover:text-[var(--accent)] transition-colors" 
                 onclick={(e) => e.stopPropagation()}>
                {sarki.sarkici}
              </a>
            </div>
          </div>

          <div class="w-48 shrink-0 hidden md:flex items-center justify-end pr-4">
            <SongStats {sarki} />
          </div>
          
          <span class="w-1/4 text-[var(--text-dim)] truncate font-black text-[10px] uppercase tracking-tighter pl-6 shrink-0 hidden lg:block opacity-60">
            {sarki.album || "Single"}
          </span>
          
          <div class="w-24 flex items-center justify-center gap-3 shrink-0" 
               role="presentation"
               onclick={(e) => e.stopPropagation()} 
               onkeydown={(e) => e.stopPropagation()}>
            
            <FavoriteButton sarkiId={sarki.id} />
            
            <button 
                type="button" 
                aria-label="Listeden Çıkar" 
                title="Listeden Kaldır" 
                onclick={(e) => handleListedenCikar(sarki.id, sarki.isim, e)} 
                class="text-[var(--text-dim)]/30 hover:text-red-500 transition-all p-1"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  @keyframes bounce {
    0%, 100% { height: 4px; }
    50% { height: 14px; }
  }

  .custom-scrollbar::-webkit-scrollbar { width: 4px; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }
</style>