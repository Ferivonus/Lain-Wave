<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { fly, fade } from 'svelte/transition';
  // Merkezi store fonksiyonumuzu alıyoruz
  import { playerState, initializePlayer } from '../../store.svelte';

  // Sayfa açıldığında kütüphanenin boş olmadığından emin olalım
  onMount(async () => {
    if (playerState.sarkiListesi.length === 0) {
      await initializePlayer();
    }
  });

  // Kütüphanedeki şarkılardan benzersiz sanatçı listesini oluştur
  let sanatciListesi = $derived.by(() => {
    const artistMap = new Map();

    playerState.sarkiListesi.forEach(sarki => {
      const isim = sarki.sarkici || "Bilinmeyen Sanatçı";
      if (!artistMap.has(isim)) {
        artistMap.set(isim, {
          isim: isim,
          sarkiSayisi: 1,
          kapak: sarki.kapak_yolu
        });
      } else {
        const mevcut = artistMap.get(isim);
        mevcut.sarkiSayisi += 1;
      }
    });

    return Array.from(artistMap.values()).sort((a, b) => a.isim.localeCompare(b.isim));
  });
</script>

<div class="p-10 w-full min-h-full pb-32 flex flex-col relative min-w-0">
  
  <header class="mb-12" in:fly={{ y: -20, duration: 500 }}>
    <h1 class="text-5xl font-black text-white italic tracking-tighter uppercase drop-shadow-md">
      Sanatçılar
    </h1>
    <p class="text-white/40 text-sm mt-2 font-medium uppercase tracking-[0.2em]">
      Kütüphanende toplam {sanatciListesi.length} benzersiz sanatçı bulundu
    </p>
    <div class="h-1 w-20 bg-pink-500 mt-4 rounded-full shadow-[0_0_15px_rgba(236,72,153,0.5)]"></div>
  </header>

  {#if sanatciListesi.length === 0}
    <div class="flex-1 flex flex-col items-center justify-center opacity-20" in:fade>
      <svg class="w-32 h-32 mb-4" fill="currentColor" viewBox="0 0 24 24">
        <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 14.5c-2.49 0-4.5-2.01-4.5-4.5S9.51 7.5 12 7.5s4.5 2.01 4.5 4.5-2.01 4.5-4.5 4.5zm0-5.5c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1z"/>
      </svg>
      <p class="text-2xl font-black italic uppercase">Veri Analiz Ediliyor...</p>
    </div>
  {:else}
    <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-8">
      {#each sanatciListesi as sanatci, i}
        <a 
          href="/artist/{encodeURIComponent(sanatci.isim)}"
          class="group flex flex-col items-center text-center p-6 rounded-3xl hover:bg-white/5 transition-all duration-300 border border-transparent hover:border-white/10"
          in:fly={{ y: 20, duration: 400, delay: i * 20 }}
        >
          <div class="w-full aspect-square rounded-full overflow-hidden mb-6 shadow-2xl relative border-4 border-white/5 group-hover:border-pink-500/50 transition-all duration-500 group-hover:scale-105">
            {#if sanatci.kapak}
              <img src={convertFileSrc(sanatci.kapak)} alt={sanatci.isim} class="w-full h-full object-cover group-hover:rotate-3 transition-transform duration-700" />
            {:else}
              <div class="w-full h-full bg-gradient-to-br from-gray-700 to-gray-900 flex items-center justify-center text-4xl font-black text-white/20 uppercase tracking-tighter italic">
                {sanatci.isim[0]}
              </div>
            {/if}
            
            <div class="absolute inset-0 bg-pink-600/20 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
               <svg class="w-12 h-12 text-white drop-shadow-lg scale-50 group-hover:scale-100 transition-transform duration-300" fill="currentColor" viewBox="0 0 24 24">
                 <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 15h-2v-6h2v6zm4 0h-2V7h2v10z"/>
               </svg>
            </div>
          </div>

          <h3 class="text-white font-black text-lg truncate w-full mb-1 group-hover:text-pink-400 transition-colors uppercase italic tracking-tighter">
            {sanatci.isim}
          </h3>
          <span class="text-[10px] font-bold text-white/30 uppercase tracking-[0.2em]">
            {sanatci.sarkiSayisi} PARÇA
          </span>
        </a>
      {/each}
    </div>
  {/if}
</div>

<style>
  a:hover {
    box-shadow: 0 20px 40px -20px rgba(0,0,0,0.5);
  }
</style>