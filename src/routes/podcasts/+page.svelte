<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { fly, fade } from 'svelte/transition';
  import SongStats from '$lib/SongStats.svelte';
  // YENİ: initializePlayer'ı store'dan çekiyoruz
  import { playerState, sarkiCal, initializePlayer } from '../../store.svelte';
  import FavoriteButton from '$lib/FavoriteButton.svelte';

  // Sayfa doğrudan açıldığında kütüphaneyi garantiye alalım
  onMount(async () => {
    if (playerState.sarkiListesi.length === 0) {
      await initializePlayer(); // Veri çekme işini tamamen store'a devrettik
    }
  });

  // Sadece "Podcast" tarzındaki dosyaları filtrele
  let podcastListesi = $derived(
    playerState.sarkiListesi.filter(s => s.tarz?.toLowerCase() === 'podcast')
  );

  // En son eklenen 3 podcast (Öne çıkanlar için)
  let sonPodcastler = $derived(
    [...podcastListesi].reverse().slice(0, 3)
  );

  function formatTarih() {
    return new Date().toLocaleDateString('tr-TR', { day: 'numeric', month: 'long', year: 'numeric' });
  }
</script>

<div class="p-10 w-full min-h-full pb-32 flex flex-col relative min-w-0">
  
  <header class="mb-12 relative group" in:fly={{ y: -20, duration: 600 }}>
    <div class="absolute -inset-4 bg-gradient-to-r from-blue-600/20 to-purple-600/20 blur-xl opacity-50 rounded-3xl -z-10"></div>
    
    <div class="flex items-center gap-6">
      <div class="w-20 h-20 bg-blue-600 rounded-2xl flex items-center justify-center shadow-2xl rotate-3 group-hover:rotate-0 transition-transform duration-500">
        <svg class="w-10 h-10 text-white" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
          <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"></path>
          <path d="M19 10v2a7 7 0 0 1-14 0v-2"></path>
          <line x1="12" y1="19" x2="12" y2="23"></line>
          <line x1="8" y1="23" x2="16" y2="23"></line>
        </svg>
      </div>
      
      <div>
        <h1 class="text-5xl font-black text-white italic tracking-tighter uppercase leading-none">Podcast Yayınları</h1>
        <p class="text-white/40 mt-2 font-bold uppercase tracking-[0.3em] text-xs">
          {podcastListesi.length} Oturum Arşivlendi • {formatTarih()}
        </p>
      </div>
    </div>
  </header>

  {#if podcastListesi.length === 0}
    <div class="flex-1 flex flex-col items-center justify-center border-2 border-dashed border-white/5 rounded-[3rem] p-20 text-center" in:fade>
      <div class="w-24 h-24 bg-white/5 rounded-full flex items-center justify-center mb-6">
        <svg class="w-10 h-10 text-white/20" fill="currentColor" viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm-2-13.5l6 3.5-6 3.5v-7z"/></svg>
      </div>
      <h3 class="text-2xl font-black text-white/50 uppercase italic mb-2">Henüz Kayıt Yok</h3>
      <p class="text-white/30 max-w-sm mx-auto text-sm leading-relaxed">
        "Müzik Ekle" kısmından tarzını **Podcast** olarak seçtiğin dosyalar otomatik olarak burada listelenir.
      </p>
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12">
      {#each sonPodcastler as pc, i}
        <button 
          onclick={() => sarkiCal(pc)}
          class="bg-gradient-to-br from-white/5 to-transparent border border-white/5 p-6 rounded-3xl flex flex-col gap-4 text-left hover:border-blue-500/40 transition-all group relative overflow-hidden"
          in:fly={{ y: 20, duration: 400, delay: i * 100 }}
        >
          <div class="w-full aspect-video rounded-xl bg-black/40 overflow-hidden shadow-inner relative">
            {#if pc.kapak_yolu}
              <img src={convertFileSrc(pc.kapak_yolu)} alt="" class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-700 opacity-60" />
            {/if}
            <div class="absolute inset-0 flex items-center justify-center">
              <div class="w-12 h-12 bg-blue-600 rounded-full flex items-center justify-center opacity-0 group-hover:opacity-100 scale-50 group-hover:scale-100 transition-all duration-300 shadow-xl">
                <svg class="w-6 h-6 text-white ml-1" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
              </div>
            </div>
          </div>
          <div class="min-w-0">
            <span class="text-[9px] font-black text-blue-400 uppercase tracking-[0.2em] mb-1 block">Son Eklenen</span>
            <h4 class="text-white font-bold truncate group-hover:text-blue-300 transition-colors">{pc.isim}</h4>
            <p class="text-white/40 text-xs truncate mt-1">{pc.sarkici}</p>
          </div>
        </button>
      {/each}
    </div>

    <h3 class="text-sm font-black text-white/30 uppercase tracking-[0.4em] mb-6 px-2">Tüm Oturumlar</h3>
    <div class="flex flex-col gap-2">
      {#each podcastListesi as pc}
        <div 
          role="button" tabindex="0"
          onclick={() => sarkiCal(pc)}
          onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(pc)}
          class="flex items-center gap-4 p-3 rounded-2xl bg-black/20 border border-white/5 hover:bg-white/5 hover:border-blue-500/30 transition-all group cursor-pointer"
        >
          <div class="w-14 h-14 rounded-lg bg-blue-900/20 overflow-hidden shrink-0 shadow-lg flex items-center justify-center">
            {#if pc.kapak_yolu}
              <img src={convertFileSrc(pc.kapak_yolu)} alt="" class="w-full h-full object-cover" />
            {:else}
              <svg class="w-6 h-6 text-blue-500/40" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"></path></svg>
            {/if}
          </div>

          <div class="flex-1 min-w-0">
            <span class="font-bold text-white text-base truncate block">{pc.isim}</span>
            <span class="text-xs text-white/40 truncate block mt-0.5 font-medium uppercase tracking-wider">{pc.sarkici} • {pc.album}</span>
          </div>

          <div class="shrink-0 flex items-center gap-6 pr-4">
            <SongStats sarki={pc} />
            <div class="h-8 w-px bg-white/5 hidden sm:block"></div>
            <div onclick={(e) => e.stopPropagation()} role="presentation">
              <FavoriteButton sarkiId={pc.id} />
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}

</div>

<style>
  header h1 {
    text-shadow: 0 10px 20px rgba(0,0,0,0.5);
  }
</style>