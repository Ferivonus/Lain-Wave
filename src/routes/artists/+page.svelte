<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { fly, fade, scale } from 'svelte/transition';
  import { playerState, initializePlayer } from '../../store.svelte';

  onMount(async () => {
    if (playerState.sarkiListesi.length === 0) {
      await initializePlayer();
    }
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
</script>

<div class="p-8 lg:p-12 w-full min-h-full pb-32 flex flex-col relative min-w-0 bg-transparent text-[var(--text-main)] transition-colors duration-500">
  
  <header class="mb-12" in:fly={{ y: -20, duration: 600 }}>
    <h1 class="text-4xl lg:text-5xl font-black italic tracking-tighter uppercase leading-none">
      Sanatçılar
    </h1>
    <p class="text-[var(--text-dim)] text-[10px] lg:text-xs mt-3 font-bold uppercase tracking-[0.3em]">
        Arşivinde {sanatciListesi.length} benzersiz sanatçı kimliği tanımlandı
    </p>
    <div class="h-1 w-16 bg-[var(--accent)] mt-5 rounded-full"></div>
  </header>

  {#if sanatciListesi.length === 0}
    <div class="flex-1 flex flex-col items-center justify-center opacity-30 py-20" in:fade>
      <div class="w-20 h-20 border-2 border-t-[var(--accent)] border-[var(--border)] rounded-full animate-spin mb-6"></div>
      <p class="text-xl font-bold uppercase tracking-[0.2em]">Veri Tabanı taranıyor...</p>
    </div>
  {:else}
    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-6 lg:gap-8">
      {#each sanatciListesi as sanatci, i}
        <a 
          href="/artist/{encodeURIComponent(sanatci.isim)}"
          class="group flex flex-col items-center text-center p-4 lg:p-6 rounded-[var(--radius)] hover:bg-[var(--bg-card)] border border-transparent hover:border-[var(--border)] transition-all duration-300 relative"
          in:fly={{ y: 20, duration: 400, delay: i * 30 }}
        >
          <div class="w-full aspect-square rounded-full overflow-hidden mb-6 shadow-xl relative border-4 border-[var(--border)] group-hover:border-[var(--accent)]/30 transition-all duration-500 group-hover:scale-105 bg-[var(--bg-surface)]">
            {#if sanatci.kapak}
              <img 
                src={convertFileSrc(sanatci.kapak)} 
                alt={sanatci.isim} 
                class="w-full h-full object-cover transition-transform duration-700 group-hover:rotate-2 group-hover:scale-110" 
              />
            {:else}
              <div class="w-full h-full flex items-center justify-center text-3xl font-black text-[var(--text-dim)]/20 uppercase italic">
                {sanatci.isim[0]}
              </div>
            {/if}
            
            <div class="absolute inset-0 bg-[var(--accent)]/10 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center backdrop-blur-[2px]">
               <div class="w-12 h-12 bg-white text-black rounded-full flex items-center justify-center shadow-2xl transform scale-50 group-hover:scale-100 transition-all duration-300">
                  <svg class="w-6 h-6 fill-current" viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 15h-2v-6h2v6zm4 0h-2V7h2v10z"/></svg>
               </div>
            </div>
          </div>

          <div class="min-w-0 w-full px-1">
            <h3 class="text-[var(--text-main)] font-bold text-sm lg:text-base truncate mb-1 group-hover:text-[var(--accent)] transition-colors uppercase tracking-tight">
              {sanatci.isim}
            </h3>
            <span class="text-[9px] font-black text-[var(--text-dim)] uppercase tracking-widest opacity-60">
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
</style>