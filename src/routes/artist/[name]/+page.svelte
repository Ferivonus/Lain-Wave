<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { page } from '$app/state';
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import SongStats from '$lib/SongStats.svelte';
  import { playerState, sarkiCal, initializePlayer } from '../../../store.svelte';
  import { fade, fly, scale } from 'svelte/transition';

  onMount(async () => {
    if (playerState.sarkiListesi.length === 0) {
      await initializePlayer();
    }
  });

  let sanatciAdi = $derived(decodeURIComponent(((page.params as any).name) ?? "Bilinmeyen Sanatçı"));

  let sanatciSarkilari = $derived(
    playerState.sarkiListesi.filter(s => s.sarkici === sanatciAdi)
  );

  let albumler = $derived(
    [...new Set(sanatciSarkilari.map(s => s.album))].filter(Boolean)
  );

  let toplamDinlenme = $derived(
    sanatciSarkilari.reduce((toplam, s) => toplam + (s.dinlenme_sayisi || 0), 0)
  );

  let kapakGorseli = $derived(
    sanatciSarkilari.find(s => s.kapak_yolu)?.kapak_yolu
  );

  function hepsiniCal() {
    if (sanatciSarkilari.length > 0) {
      sarkiCal(sanatciSarkilari[0]);
    }
  }
</script>

<div class="p-6 lg:p-10 w-full max-w-[1600px] mx-auto min-h-full pb-32 flex flex-col min-w-0 bg-transparent text-[var(--text-main)] transition-colors duration-500 overflow-y-auto custom-scrollbar">
  
  <section class="relative w-full min-h-[350px] lg:h-[400px] rounded-[var(--radius)] overflow-hidden mb-12 shadow-2xl flex items-end p-8 lg:p-12 border border-[var(--border)]" in:fade>
    <div class="absolute inset-0 z-0">
      {#if kapakGorseli}
        <img src={convertFileSrc(kapakGorseli)} alt="" class="w-full h-full object-cover blur-3xl opacity-30 scale-110 transition-transform duration-1000" />
      {/if}
      <div class="absolute inset-0 bg-gradient-to-t from-[var(--bg-main)] via-[var(--bg-main)]/40 to-transparent"></div>
    </div>

    <div class="relative z-10 flex flex-col md:flex-row items-center md:items-end gap-8 w-full">
      <div class="w-40 h-40 lg:w-56 lg:h-56 rounded-full overflow-hidden shadow-2xl border-4 border-[var(--border)] shrink-0 bg-[var(--bg-surface)] group">
        {#if kapakGorseli}
          <img src={convertFileSrc(kapakGorseli)} alt={sanatciAdi} class="w-full h-full object-cover transition-transform duration-700 group-hover:scale-110" />
        {:else}
          <div class="w-full h-full bg-[var(--accent)] flex items-center justify-center text-5xl font-black text-white">
            {sanatciAdi[0]}
          </div>
        {/if}
      </div>

      <div class="flex flex-col flex-1 text-center md:text-left min-w-0">
        <div class="flex items-center justify-center md:justify-start gap-2 text-[var(--accent)] mb-3">
          <svg class="w-5 h-5 shrink-0" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 15h-2v-6h2v6zm4 0h-2V7h2v10z"/></svg>
          <span class="text-[10px] lg:text-xs font-black tracking-[0.2em] uppercase">Onaylanmış Sanatçı</span>
        </div>
        
        <h1 class="text-5xl lg:text-7xl font-black italic tracking-tighter uppercase mb-6 leading-none">
          {sanatciAdi}
        </h1>
        
        <div class="flex flex-col sm:flex-row items-center gap-6">
          <p class="text-[var(--text-dim)] text-xs lg:text-sm font-bold uppercase tracking-widest">
            {sanatciSarkilari.length} Parça <span class="mx-2 opacity-30">•</span> {toplamDinlenme.toLocaleString()} Dinlenme
          </p>
          {#if sanatciSarkilari.length > 0}
            <button 
              type="button"
              onclick={hepsiniCal}
              aria-label="{sanatciAdi} tüm şarkılarını oynat"
              class="bg-[var(--text-main)] text-[var(--bg-main)] hover:bg-[var(--accent)] hover:text-white px-10 py-3.5 rounded-full font-black text-[10px] lg:text-xs uppercase tracking-widest transition-all active:scale-95 shadow-xl"
            >
              Hepsini Oynat
            </button>
          {/if}
        </div>
      </div>
    </div>
  </section>

  <div class="grid grid-cols-1 lg:grid-cols-3 gap-12 lg:gap-16">
    
    <div class="lg:col-span-2 min-w-0">
      <h2 class="text-xl font-bold mb-8 uppercase tracking-[0.2em] flex items-center gap-4">
        <span class="w-10 h-1 bg-[var(--accent)] rounded-full"></span>
        Popüler Parçalar
      </h2>

      <div class="flex flex-col gap-1">
        {#each sanatciSarkilari as sarki, i}
          <div 
            role="button" tabindex="0"
            onclick={() => sarkiCal(sarki)}
            onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)}
            aria-label="{sarki.isim} parçasını çal"
            class="flex items-center p-3 rounded-2xl hover:bg-[var(--bg-card-hover)] transition-all group cursor-pointer border border-transparent min-w-0 {playerState.aktifSarki?.id === sarki.id ? 'bg-[var(--accent)]/5 border-[var(--accent)]/20' : ''}"
          >
            <span class="w-8 text-center text-[var(--text-dim)]/40 font-mono text-xs shrink-0 group-hover:hidden">{i + 1}</span>
            <div class="w-8 text-center hidden group-hover:block shrink-0">
               <svg class="w-4 h-4 mx-auto text-[var(--accent)]" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M8 5v14l11-7z"/></svg>
            </div>
            
            <div class="w-12 h-12 rounded-lg bg-[var(--bg-card)] overflow-hidden shrink-0 mx-4 border border-[var(--border)]">
              {#if sarki.kapak_yolu}
                <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover" />
              {:else}
                <div class="w-full h-full flex items-center justify-center opacity-20">🎵</div>
              {/if}
            </div>

            <div class="flex-1 min-w-0 mr-4">
              <span class="block font-bold text-sm truncate {playerState.aktifSarki?.id === sarki.id ? 'text-[var(--accent)]' : 'text-[var(--text-main)]'}">
                {sarki.isim}
              </span>
              <span class="block text-[10px] text-[var(--text-dim)] uppercase tracking-tight truncate">{sarki.album || 'Tekli'}</span>
            </div>

            <div class="hidden sm:block shrink-0 mr-6">
              <SongStats {sarki} />
            </div>

            <div class="flex items-center px-2 shrink-0" 
                 role="presentation"
                 onclick={(e) => e.stopPropagation()} 
                 onkeydown={(e) => e.stopPropagation()}>
              <FavoriteButton sarkiId={sarki.id} />
            </div>
          </div>
        {/each}
      </div>
    </div>

    <div class="min-w-0">
      <h2 class="text-xl font-bold mb-8 uppercase tracking-[0.2em]">Diskografi</h2>
      <div class="grid grid-cols-2 lg:grid-cols-1 gap-6">
        {#each albumler as album}
          {@const albumKapak = sanatciSarkilari.find(s => s.album === album)?.kapak_yolu}
          <button 
            type="button"
            class="flex flex-col lg:flex-row items-center lg:items-start gap-4 group text-left outline-none border-none bg-transparent p-0 w-full"
            aria-label="{album} albüm detayları"
          >
            <div class="w-full lg:w-20 aspect-square bg-[var(--bg-card)] rounded-2xl overflow-hidden relative border border-[var(--border)] group-hover:border-[var(--accent)]/50 transition-all shadow-lg shrink-0">
              {#if albumKapak}
                <img src={convertFileSrc(albumKapak)} alt={album} class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110" />
              {:else}
                <div class="w-full h-full flex items-center justify-center text-2xl opacity-10">💿</div>
              {/if}
            </div>
            <div class="flex flex-col min-w-0 py-1">
                <span class="text-[11px] font-black text-[var(--text-main)] truncate w-full uppercase tracking-tighter mb-1 group-hover:text-[var(--accent)] transition-colors">{album}</span>
                <span class="text-[9px] text-[var(--text-dim)] font-bold uppercase tracking-widest">
                  {sanatciSarkilari.filter(s => s.album === album).length} Parça Kayıtlı
                </span>
            </div>
          </button>
        {/each}
      </div>
    </div>

  </div>
</div>

<style>
  h1 {
    text-shadow: 0 10px 30px rgba(0,0,0,0.5);
  }

  /* DÜZELTME: Artık 'custom-scrollbar' sınıfı HTML'de kullanıldığı için uyarı vermeyecek */
  .custom-scrollbar::-webkit-scrollbar { width: 4px; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }
</style>