<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { fly, fade } from 'svelte/transition';
  import { playerState, initializePlayer, sarkiCal } from '../../store.svelte';

  let yukleniyor = $state(playerState.sarkiListesi.length === 0);

  onMount(async () => {
    if (playerState.sarkiListesi.length === 0) {
      await initializePlayer();
    }
    yukleniyor = false;
  });

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

  function sanatciyiCal(isim: string, event: Event) {
    event.preventDefault();
    event.stopPropagation(); 
    
    const sanatcininSarkilari = playerState.sarkiListesi.filter(s => (s.sarkici || "Bilinmeyen Sanatçı") === isim);
    if (sanatcininSarkilari.length > 0) {
        sarkiCal(sanatcininSarkilari[0]);
    }
  }
</script>

<div class="p-8 lg:p-12 w-full min-h-full pb-32 flex flex-col relative min-w-0 bg-transparent text-(--text-main) transition-colors duration-500 overflow-y-auto custom-scrollbar">
  
  <header class="mb-12" in:fly={{ y: -20, duration: 600 }}>
    <h1 class="text-4xl lg:text-5xl font-black italic tracking-tighter uppercase leading-none orkun-favori">
      Sanatçılar
    </h1>
    <p class="text-(--text-dim) text-[10px] lg:text-xs mt-3 font-bold uppercase tracking-[0.3em]">
        Arşivinde {sanatciListesi.length} benzersiz sanatçı kimliği tanımlandı
    </p>
    <div class="h-1 w-16 bg-(--accent) mt-5 rounded-full"></div>
  </header>

  {#if yukleniyor}
    <div class="flex-1 flex flex-col items-center justify-center opacity-30 py-20" in:fade>
      <div class="w-20 h-20 border-2 border-t-(--accent) border-(--border) rounded-full animate-spin mb-6"></div>
      <p class="text-xl font-bold uppercase tracking-[0.2em]">Veri Tabanı taranıyor...</p>
    </div>
  {:else if sanatciListesi.length === 0}
    <div class="flex-1 flex flex-col items-center justify-center opacity-50 py-20 text-center" in:fade>
      <div class="text-6xl mb-6 grayscale">🎤</div>
      <p class="text-xl font-bold uppercase tracking-[0.2em] mb-2">Sinyal Yok</p>
      <p class="text-sm font-medium tracking-widest uppercase text-(--text-dim)">Kütüphanende henüz hiç sanatçı bulunmuyor.</p>
    </div>
  {:else}
    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-6 lg:gap-8">
      {#each sanatciListesi as sanatci, i}
        <a 
          href="/artist/{encodeURIComponent(sanatci.isim)}"
          class="group flex flex-col items-center text-center p-4 lg:p-6 rounded-(--radius) hover:bg-(--bg-card) border border-transparent hover:border-(--border) transition-all duration-300 relative"
          in:fly={{ y: 20, duration: 400, delay: i * 30 }}
        >
          <div class="w-full aspect-square rounded-full overflow-hidden mb-6 shadow-xl relative border-4 border-(--border) group-hover:border-(--accent)/30 transition-all duration-500 group-hover:scale-105 bg-(--bg-surface)">
            {#if sanatci.kapak}
              <img 
                src={convertFileSrc(sanatci.kapak)} 
                alt={sanatci.isim} 
                class="w-full h-full object-cover transition-transform duration-700 group-hover:rotate-2 group-hover:scale-110" 
              />
            {:else}
              <div class="w-full h-full flex items-center justify-center text-3xl font-black text-(--text-dim)/20 uppercase italic">
                {sanatci.isim[0]}
              </div>
            {/if}
            
            <div class="absolute inset-0 bg-linear-to-t from-black/40 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
            
            <button 
                type="button"
                onclick={(e) => sanatciyiCal(sanatci.isim, e)}
                class="absolute bottom-2 right-2 lg:bottom-4 lg:right-4 z-20 transform translate-y-4 opacity-0 group-hover:translate-y-0 group-hover:opacity-100 transition-all duration-300 ease-out border-none outline-none bg-transparent p-0"
                aria-label="{sanatci.isim} çal"
            >
                <div class="w-10 h-10 lg:w-12 lg:h-12 bg-(--accent) text-white rounded-full flex items-center justify-center shadow-[0_8px_20px_rgba(0,0,0,0.4)] hover:scale-110 active:scale-95 transition-transform">
                    <svg class="w-5 h-5 lg:w-6 lg:h-6 fill-current ml-0.5" viewBox="0 0 24 24">
                        <path d="M8 5v14l11-7z"/>
                    </svg>
                </div>
            </button>
          </div>

          <div class="min-w-0 w-full px-1">
            <h3 class="text-(--text-main) font-bold text-sm lg:text-base truncate mb-1 group-hover:text-(--accent) transition-colors uppercase tracking-tight">
              {sanatci.isim}
            </h3>
            <span class="text-[9px] font-black text-(--text-dim) uppercase tracking-widest opacity-60">
              {sanatci.sarkiSayisi} Parça Kayıtlı
            </span>
          </div>
        </a>
      {/each}
    </div>
  {/if}
</div>

<style>
  a:hover {
    box-shadow: 0 15px 35px -10px rgba(0,0,0,0.3);
  }

  a:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 4px;
    background: var(--bg-card);
  }

  .custom-scrollbar::-webkit-scrollbar { width: 4px; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }
</style>