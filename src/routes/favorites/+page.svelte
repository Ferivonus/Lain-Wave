<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core'; 
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import SongStats from '$lib/SongStats.svelte';
  import { 
      playerState, 
      sarkiCal, 
      initializePlayer, 
      handleSarkiSil,
      editModaliAc,
      handlePlaylistEkle
  } from '../../store.svelte';
  import { fade, fly, scale } from 'svelte/transition';

  let favoriSarkilar = $derived(
    playerState.sarkiListesi.filter(sarki => playerState.favoriler.includes(sarki.id))
  );

  onMount(async () => {
    if (playerState.sarkiListesi.length === 0) {
      await initializePlayer();
    }
  });

  function favorileriCal() {
    if (favoriSarkilar.length > 0) {
      sarkiCal(favoriSarkilar[0]);
    }
  }

</script>

<div class="p-8 lg:p-12 w-full min-h-full pb-32 flex flex-col relative min-w-0 bg-transparent text-(--text-main) transition-colors duration-500 overflow-y-auto custom-scrollbar">
  
  <header class="flex flex-col md:flex-row gap-8 items-center md:items-end mb-14 mt-4" in:fade>
    <div class="w-48 h-48 lg:w-56 lg:h-56 bg-linear-to-br from-red-500/80 via-pink-500/80 to-(--bg-surface) rounded-(--radius) shadow-2xl shrink-0 border border-white/10 overflow-hidden relative group flex items-center justify-center">
        <div class="absolute inset-0 bg-black/20 group-hover:bg-transparent transition-colors duration-500 z-10"></div>
        <div class="absolute inset-0 bg-[url('https://images.unsplash.com/photo-1518609878373-06d740f60d8b?q=80&w=2070')] bg-cover bg-center mix-blend-overlay group-hover:scale-110 transition-transform duration-1000"></div>
        <svg class="w-24 h-24 text-white drop-shadow-[0_0_20px_rgba(255,255,255,0.6)] group-hover:scale-110 transition-transform duration-700 z-20" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/>
        </svg>
    </div>
    
    <div class="flex flex-col text-center md:text-left pb-2 min-w-0">
      <span class="text-[10px] font-black mb-3 text-pink-500 tracking-[0.4em] uppercase italic">Kişisel Arşiv</span>
      <h1 class="text-5xl lg:text-7xl font-black tracking-tighter leading-none mb-6 uppercase orkun-favori drop-shadow-md">
        Favoriler
      </h1>
      
      <div class="flex flex-col sm:flex-row items-center gap-6">
        <p class="text-(--text-dim) text-xs lg:text-sm font-bold uppercase tracking-widest whitespace-nowrap">
          {favoriSarkilar.length} Kalp • Sisteme İşlendi
        </p>
        
        {#if favoriSarkilar.length > 0}
          <button 
            type="button"
            onclick={favorileriCal} 
            class="flex items-center gap-3 bg-(--text-main) text-(--bg-main) hover:bg-pink-500 hover:text-white px-10 py-3.5 rounded-full font-black shadow-xl transition-all active:scale-95 uppercase text-[10px] lg:text-xs tracking-widest"
            aria-label="Tüm favorileri oynat"
          >
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
            Hepsini Oynat
          </button>
        {/if}
      </div>
    </div>
  </header>

  {#if favoriSarkilar.length === 0}
    <div class="flex flex-col items-center justify-center flex-1 text-center mt-10 p-16 bg-(--bg-card) border border-(--border) rounded-(--radius) border-dashed" in:fade>
      <div class="text-6xl mb-8 opacity-20 filter grayscale">🤍</div>
      <h3 class="text-2xl font-bold mb-3 tracking-tight uppercase">Henüz Sinyal Yok</h3>
      <p class="text-(--text-dim) font-medium text-sm max-w-sm leading-relaxed uppercase tracking-wider">
        Kütüphanendeki kalp ikonlarını kullanarak en sevdiğin frekansları burada toplayabilirsin.
      </p>
    </div>
  {:else}
    <div class="flex items-center text-[10px] font-black text-(--text-dim) border-b border-(--border) pb-3 mb-4 px-4 tracking-[0.2em] uppercase shrink-0">
      <span class="w-10 text-center shrink-0">#</span>
      <span class="flex-1 min-w-0 ml-4">BAŞLIK</span>
      <span class="w-40 lg:w-56 xl:w-64 shrink-0 hidden md:block text-right pr-4">İSTATİSTİK</span> 
      <span class="w-32 xl:w-48 shrink-0 hidden lg:block pl-4">ALBÜM</span>
      <span class="w-48 sm:w-56 text-right pr-2 shrink-0">İŞLEMLER</span>
    </div>

    <div class="flex flex-col gap-2">
      {#each favoriSarkilar as sarki, index}
        <div 
            role="button" tabindex="0" 
            onclick={() => sarkiCal(sarki)} 
            onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)} 
            aria-label="{sarki.isim} çal"
            class="flex items-center text-sm p-3 rounded-2xl hover:bg-(--bg-card-hover) transition-all duration-300 cursor-pointer group border
            {playerState.aktifSarki?.id === sarki.id ? 'bg-(--accent)/10 shadow-inner border-(--accent)/20' : 'border-transparent'}"
        >
          
          <div class="w-10 text-center shrink-0">
             {#if playerState.aktifSarki?.id === sarki.id && playerState.suAnOynuyorMu}
                <div class="flex items-end justify-center gap-0.5 h-3.5 mb-1">
                   <div class="w-1 bg-pink-500 animate-bounce"></div>
                   <div class="w-1 bg-pink-500 animate-[bounce_1.2s_infinite]"></div>
                   <div class="w-1 bg-pink-500 animate-[bounce_0.8s_infinite]"></div>
                </div>
             {:else}
                <span class="text-(--text-dim)/40 group-hover:hidden font-mono text-xs">{index + 1}</span>
                <svg class="w-4 h-4 mx-auto hidden group-hover:block text-pink-500" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
             {/if}
          </div>
          
          <div class="flex-1 flex items-center gap-4 min-w-0 ml-4">
            <div class="w-12 h-12 bg-(--bg-card) rounded-xl overflow-hidden shrink-0 shadow-md border border-(--border)">
              {#if sarki.kapak_yolu}
                <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover transition-transform group-hover:scale-110" />
              {:else}
                <div class="w-full h-full flex items-center justify-center text-(--text-dim)/20 bg-(--bg-surface) font-black text-[10px] italic">LW</div>
              {/if}
            </div>
            
            <div class="flex flex-col min-w-0 flex-1 pr-4">
              <span class="font-bold text-sm lg:text-base truncate {playerState.aktifSarki?.id === sarki.id ? 'text-pink-500' : 'text-(--text-main)'}">
                {sarki.isim}
              </span>
              <a href="/artist/{encodeURIComponent(sarki.sarkici)}" 
                 class="text-[11px] text-(--text-dim) truncate font-bold uppercase tracking-tight hover:text-pink-500 transition-colors opacity-80 inline-block max-w-max mt-0.5" 
                 onclick={(e) => e.stopPropagation()}>
                {sarki.sarkici}
              </a>
            </div>
          </div>

          <div class="w-40 lg:w-56 xl:w-64 shrink-0 hidden md:flex items-center justify-end pr-4">
            <SongStats {sarki} />
          </div>
          
          <div class="w-32 xl:w-48 text-(--text-dim) truncate font-black text-[10px] uppercase tracking-tighter pl-4 shrink-0 hidden lg:block opacity-60">
            {sarki.album || "Single"}
          </div>
          
          <div class="w-48 sm:w-56 flex items-center justify-end gap-2 pr-2 shrink-0" 
               onclick={(e) => e.stopPropagation()} 
               onkeydown={(e) => e.stopPropagation()} 
               role="presentation">
            
            <select aria-label="Listeye Ekle" onchange={(e) => handlePlaylistEkle(sarki.id, e)} class="bg-(--bg-surface) text-[9px] text-(--text-dim) rounded-lg px-2 py-1.5 outline-none border border-(--border) w-20 focus:border-pink-500 transition-all cursor-pointer font-bold uppercase hidden sm:block">
              <option value="">➕ LİSTE</option>
              {#each playerState.playlistler as pl}
                {#if !pl.sarkilar.includes(sarki.id)}<option value={pl.id}>{pl.isim.toUpperCase()}</option>{/if}
              {/each}
            </select>

            <button 
                type="button" 
                aria-label="Düzenle" 
                title="Bilgileri Düzenle" 
                onclick={(e) => editModaliAc(sarki, e)} 
                class="p-2 text-(--text-dim)/60 hover:text-pink-500 hover:bg-pink-500/10 rounded-lg transition-all hidden sm:block"
            >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
            </button>

            <FavoriteButton sarkiId={sarki.id} />
            
            <button 
                type="button" 
                aria-label="Kütüphaneden Sil" 
                title="Kalıcı Olarak Sil" 
                onclick={(e) => handleSarkiSil(sarki, e)} 
                class="text-(--text-dim)/40 hover:text-red-500 hover:bg-red-500/10 rounded-lg transition-all p-2"
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