<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core'; 
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import SongStats from '$lib/SongStats.svelte';
  import { playerState, sarkiCal, initializePlayer } from '../../store.svelte';
  import { fade, fly } from 'svelte/transition';

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

<div class="p-8 lg:p-12 w-full min-h-full pb-32 flex flex-col relative min-w-0 bg-transparent text-[var(--text-main)] transition-colors duration-500 overflow-y-auto custom-scrollbar">
  
  <header class="flex flex-col md:flex-row gap-8 items-center md:items-end mb-12 mt-4" in:fade>
    <div class="w-48 h-48 lg:w-56 lg:h-56 bg-gradient-to-br from-[var(--accent)] via-[var(--accent-sec)] to-[var(--bg-surface)] rounded-[var(--radius)] shadow-2xl flex-shrink-0 border border-white/10 overflow-hidden relative group flex items-center justify-center">
        <div class="absolute inset-0 bg-black/20 group-hover:bg-transparent transition-colors duration-500"></div>
        <svg class="w-24 h-24 text-white drop-shadow-[0_0_15px_rgba(255,255,255,0.4)] group-hover:scale-110 transition-transform duration-700" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/>
        </svg>
    </div>
    
    <div class="flex flex-col text-center md:text-left pb-2 min-w-0">
      <span class="text-[10px] font-black mb-3 text-[var(--accent)] tracking-[0.4em] uppercase italic">Kişisel Arşiv</span>
      <h1 class="text-4xl lg:text-6xl font-black tracking-tighter leading-none mb-6 uppercase italic drop-shadow-md">
        Favori Parçalar
      </h1>
      
      <div class="flex flex-col sm:flex-row items-center gap-6">
        <p class="text-[var(--text-dim)] text-xs lg:text-sm font-bold uppercase tracking-widest whitespace-nowrap">
          {favoriSarkilar.length} Benzersiz Kayıt • Sisteme İşlendi
        </p>
        
        {#if favoriSarkilar.length > 0}
          <button 
            onclick={favorileriCal} 
            class="flex items-center gap-3 bg-[var(--text-main)] text-[var(--bg-main)] hover:bg-[var(--accent)] hover:text-white px-10 py-3.5 rounded-full font-black shadow-xl transition-all active:scale-95 uppercase text-[10px] lg:text-xs tracking-widest"
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
    <div class="flex flex-col items-center justify-center flex-1 text-center mt-10 p-16 bg-[var(--bg-card)] border border-[var(--border)] rounded-[var(--radius)] border-dashed" in:fade>
      <div class="text-6xl mb-8 opacity-20 filter grayscale">🖤</div>
      <h3 class="text-2xl font-bold mb-3 tracking-tight uppercase">Henüz Sinyal Yok</h3>
      <p class="text-[var(--text-dim)] font-medium text-sm max-w-sm leading-relaxed uppercase tracking-wider">
        Kütüphanendeki kalp ikonlarını kullanarak en sevdiğin frekansları burada toplayabilirsin.
      </p>
    </div>
  {:else}
    <div class="flex text-[10px] font-black text-[var(--text-dim)] border-b border-[var(--border)] pb-3 mb-4 px-4 tracking-[0.2em] uppercase">
      <span class="w-10 text-center shrink-0">#</span>
      <span class="flex-1 min-w-0 ml-4">BAŞLIK</span>
      <span class="w-48 shrink-0 text-right pr-4 hidden md:block">İSTATİSTİK</span> 
      <span class="w-1/4 shrink-0 pl-6 hidden lg:block">ALBÜM</span>
      <span class="w-16 text-center shrink-0">DURUM</span>
    </div>

    <div class="flex flex-col gap-1">
      {#each favoriSarkilar as sarki, index}
        <div 
            role="button" tabindex="0" 
            onclick={() => sarkiCal(sarki)} 
            onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)} 
            aria-label="{sarki.isim} çal"
            class="flex items-center text-sm p-2 rounded-2xl hover:bg-[var(--bg-card-hover)] transition-all duration-300 cursor-pointer group 
            {playerState.aktifSarki?.id === sarki.id ? 'bg-[var(--accent)]/5 border border-[var(--accent)]/20 shadow-lg' : 'border border-transparent'}">
          
          <div class="w-10 text-center shrink-0">
             {#if playerState.aktifSarki?.id === sarki.id && playerState.suAnOynuyorMu}
                <div class="flex items-end justify-center gap-0.5 h-3.5 mb-1">
                   <div class="w-1 bg-[var(--accent)] animate-[bounce_1s_infinite]"></div>
                   <div class="w-1 bg-[var(--accent)] animate-[bounce_1.2s_infinite]"></div>
                   <div class="w-1 bg-[var(--accent)] animate-[bounce_0.8s_infinite]"></div>
                </div>
             {:else}
                <span class="text-[var(--text-dim)]/40 group-hover:hidden font-mono text-xs">{index + 1}</span>
                <svg class="w-4 h-4 mx-auto hidden group-hover:block text-[var(--accent)]" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
             {/if}
          </div>
          
          <div class="flex-1 flex items-center gap-4 min-w-0 ml-4">
            <div class="w-12 h-12 bg-[var(--bg-card)] rounded-lg overflow-hidden shrink-0 shadow-md border border-[var(--border)]">
              {#if sarki.kapak_yolu}
                <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover transition-transform group-hover:scale-110" />
              {:else}
                <div class="w-full h-full flex items-center justify-center text-[var(--text-dim)]/20 bg-[var(--bg-surface)]">🎵</div>
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
          
          <div class="w-16 flex justify-center shrink-0" 
               onclick={(e) => e.stopPropagation()} 
               onkeydown={(e) => e.stopPropagation()} 
               role="presentation">
            <FavoriteButton sarkiId={sarki.id} />
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