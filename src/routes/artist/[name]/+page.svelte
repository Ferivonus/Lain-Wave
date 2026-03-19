<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { page } from '$app/state';
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import SongStats from '$lib/SongStats.svelte';
  // Merkezi store fonksiyonlarını alıyoruz
  import { playerState, sarkiCal, initializePlayer } from '../../../store.svelte';

  // Sayfa doğrudan linkle açıldığında kütüphanenin yüklü olduğundan emin oluyoruz
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

<div class="p-6 lg:p-10 w-full max-w-[1600px] mx-auto min-h-full pb-32 flex flex-col min-w-0 overflow-x-hidden">
  
  <div class="relative w-full min-h-[300px] lg:h-80 rounded-3xl overflow-hidden mb-10 shadow-2xl flex items-end p-6 lg:p-10 group border border-white/5">
    <div class="absolute inset-0 z-0">
      {#if kapakGorseli}
        <img src={convertFileSrc(kapakGorseli)} alt="" class="w-full h-full object-cover blur-2xl opacity-40 scale-110" />
      {/if}
      <div class="absolute inset-0 bg-gradient-to-t from-[#1a0b16] via-[#1a0b16]/40 to-transparent"></div>
    </div>

    <div class="relative z-10 flex flex-col md:flex-row items-center md:items-end gap-6 lg:gap-8 w-full">
      <div class="w-32 h-32 lg:w-48 lg:h-48 rounded-full overflow-hidden shadow-2xl border-4 border-white/10 shrink-0 bg-[#261825]">
        {#if kapakGorseli}
          <img src={convertFileSrc(kapakGorseli)} alt={sanatciAdi} class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-700" />
        {:else}
          <div class="w-full h-full bg-pink-600 flex items-center justify-center text-4xl lg:text-5xl font-black">{sanatciAdi[0]}</div>
        {/if}
      </div>

      <div class="flex flex-col flex-1 text-center md:text-left min-w-0">
        <div class="flex items-center justify-center md:justify-start gap-2 text-pink-400 mb-2">
          <svg class="w-4 h-4 shrink-0" fill="currentColor" viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 15h-2v-6h2v6zm4 0h-2V7h2v10z"/></svg>
          <span class="text-[10px] lg:text-xs font-black tracking-[0.3em] uppercase">Onaylanmış Sanatçı</span>
        </div>
        <h1 class="text-4xl lg:text-6xl xl:text-7xl font-black text-white italic tracking-tighter uppercase mb-4 drop-shadow-lg truncate">
          {sanatciAdi}
        </h1>
        
        <div class="flex flex-col sm:flex-row items-center gap-4 lg:gap-6">
          <p class="text-white/60 text-xs lg:text-sm font-bold uppercase tracking-widest whitespace-nowrap">
            {sanatciSarkilari.length} Parça • {toplamDinlenme.toLocaleString()} Dinlenme
          </p>
          {#if sanatciSarkilari.length > 0}
            <button 
              onclick={hepsiniCal}
              class="bg-white text-black hover:bg-pink-500 hover:text-white px-6 lg:px-8 py-2.5 lg:py-3 rounded-full font-black text-[10px] lg:text-xs uppercase tracking-widest transition-all active:scale-95 shadow-lg whitespace-nowrap"
            >
              Hepsini Oynat
            </button>
          {/if}
        </div>
      </div>
    </div>
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-3 gap-8 lg:gap-12">
    
    <div class="lg:col-span-2 min-w-0">
      <h2 class="text-xl lg:text-2xl font-black text-white mb-6 italic uppercase tracking-tight flex items-center gap-3">
        <span class="w-6 lg:w-8 h-1 bg-pink-500 rounded-full"></span>
        Popüler Parçalar
      </h2>

      <div class="flex flex-col gap-1.5">
        {#each sanatciSarkilari as sarki, i}
          <div 
            role="button" tabindex="0"
            onclick={() => sarkiCal(sarki)}
            onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)}
            class="flex items-center p-2 rounded-xl hover:bg-white/5 transition-all group cursor-pointer border border-transparent hover:border-white/5 min-w-0 {playerState.aktifSarki?.id === sarki.id ? 'bg-white/10' : ''}"
          >
            <span class="w-8 text-center text-white/20 font-mono text-xs shrink-0">{i + 1}</span>
            
            <div class="w-10 h-10 rounded bg-white/5 overflow-hidden shrink-0 mx-3">
              {#if sarki.kapak_yolu}
                <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover" />
              {/if}
            </div>

            <div class="flex-1 min-w-0 mr-4">
              <span class="block font-bold text-white truncate text-sm {playerState.aktifSarki?.id === sarki.id ? 'text-pink-400' : ''}">{sarki.isim}</span>
              <span class="block text-[10px] text-white/40 uppercase tracking-tighter truncate">{sarki.album}</span>
            </div>

            <div class="hidden sm:block shrink-0 mr-6">
              <SongStats {sarki} />
            </div>

            <div class="flex items-center gap-4 opacity-0 group-hover:opacity-100 transition-opacity shrink-0 px-2" 
                 onclick={(e) => e.stopPropagation()} 
                 onkeydown={(e) => e.stopPropagation()}
                 role="presentation">
              <FavoriteButton sarkiId={sarki.id} />
            </div>
          </div>
        {/each}
      </div>
    </div>

    <div class="min-w-0">
      <h2 class="text-xl lg:text-2xl font-black text-white mb-6 italic uppercase tracking-tight">Albümler</h2>
      <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-2 gap-4">
        {#each albumler as album}
          {@const albumKapak = sanatciSarkilari.find(s => s.album === album)?.kapak_yolu}
          <button class="flex flex-col gap-2 group text-left outline-none border-none bg-transparent p-0 w-full cursor-default">
            <div class="w-full aspect-square bg-white/5 rounded-2xl overflow-hidden relative border border-white/5 group-hover:border-pink-500/50 transition-all shadow-lg">
              {#if albumKapak}
                <img src={convertFileSrc(albumKapak)} alt={album} class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-500" />
              {:else}
                <div class="w-full h-full flex items-center justify-center text-2xl opacity-20">💿</div>
              {/if}
            </div>
            <span class="text-[11px] font-black text-white truncate w-full px-1 uppercase tracking-tighter">{album}</span>
            <span class="text-[9px] text-white/30 px-1 font-bold uppercase tracking-widest">
              {sanatciSarkilari.filter(s => s.album === album).length} Parça
            </span>
          </button>
        {/each}
      </div>
    </div>

  </div>
</div>

<style>
  h1 {
    text-shadow: 0 4px 12px rgba(0,0,0,0.5);
  }
</style>