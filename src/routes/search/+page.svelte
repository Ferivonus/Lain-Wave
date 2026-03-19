<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import SongStats from '$lib/SongStats.svelte';
  import { playerState, initializePlayer, sarkiPlaylisteEkle, sarkiCal } from '../../store.svelte';
  import { fade, fly } from 'svelte/transition';

  let aramaMetni = $state("");

  onMount(async () => {
    if (playerState.sarkiListesi.length === 0) {
      await initializePlayer();
    }
  });

  let aramaSonuclari = $derived(
    aramaMetni.trim() === ""
      ? [] 
      : playerState.sarkiListesi.filter((sarki) => {
          const aranan = aramaMetni.toLowerCase();
          return (
            sarki.isim.toLowerCase().includes(aranan) ||
            sarki.sarkici.toLowerCase().includes(aranan) ||
            sarki.album.toLowerCase().includes(aranan)
          );
        })
  );

  let enPopulerler = $derived(
    [...playerState.sarkiListesi]
      .sort((a, b) => (b.dinlenme_sayisi || 0) - (a.dinlenme_sayisi || 0))
      .slice(0, 5)
  );

  async function handlePlaylistEkle(sarkiId: string, event: Event) {
    const selectElement = event.target as HTMLSelectElement;
    const playlistId = selectElement.value;
    if (!playlistId) return;

    const basarili = await sarkiPlaylisteEkle(sarkiId, playlistId);
    if (basarili) {
      selectElement.value = "";
    }
  }

  const kategoriler = [
    { isim: "Pop", renk: "bg-blue-500/20 text-blue-400" },
    { isim: "Rock", renk: "bg-red-500/20 text-red-400" },
    { isim: "Lofi", renk: "bg-indigo-500/20 text-indigo-400" },
    { isim: "Electronic", renk: "bg-emerald-500/20 text-emerald-400" },
    { isim: "Jazz", renk: "bg-amber-500/20 text-amber-400" },
    { isim: "Classical", renk: "bg-zinc-500/20 text-zinc-400" },
  ];
</script>

<div class="w-full min-h-full pb-32 flex flex-col relative bg-transparent text-[var(--text-main)] transition-colors duration-500">
  
  <div class="sticky top-0 z-20 px-8 pt-10 pb-6 bg-transparent backdrop-blur-xl">
    <div class="relative group max-w-3xl">
      <div class="absolute inset-y-0 left-0 flex items-center pl-6 pointer-events-none text-[var(--text-dim)] group-focus-within:text-[var(--accent)] transition-colors">
        <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
          <circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line>
        </svg>
      </div>
      
      <input
        type="text"
        bind:value={aramaMetni}
        placeholder="Kütüphanende ara..."
        class="w-full bg-[var(--bg-surface)] text-[var(--text-main)] placeholder-[var(--text-dim)]/50 text-lg font-bold rounded-2xl py-4 pl-16 pr-14 outline-none border border-[var(--border)] focus:border-[var(--accent)]/50 transition-all shadow-xl"
      />

      {#if aramaMetni.length > 0}
        <button 
          onclick={() => aramaMetni = ""} 
          class="absolute inset-y-0 right-0 flex items-center pr-5 text-[var(--text-dim)] hover:text-[var(--text-main)]"
          aria-label="Temizle">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="3" viewBox="0 0 24 24"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
        </button>
      {/if}
    </div>
  </div>

  <div class="px-8 flex-1">
    {#if aramaMetni.trim() === ""}
      <div class="max-w-6xl mt-4" in:fade>
        <h2 class="text-sm font-black text-[var(--text-dim)] mb-6 uppercase tracking-[0.2em]">Türlere Göz At</h2>
        <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
          {#each kategoriler as kategori}
            <div class="relative aspect-square rounded-[var(--radius)] {kategori.renk} border border-white/5 p-5 cursor-pointer hover:scale-[1.02] transition-all flex flex-col justify-end">
              <span class="text-lg font-bold tracking-tight">{kategori.isim}</span>
            </div>
          {/each}
        </div>
      </div>

    {:else if aramaSonuclari.length === 0}
      <div class="flex flex-col mt-10" in:fade>
        <div class="mb-12">
          <h3 class="text-xl font-bold text-[var(--text-main)] mb-2">"{aramaMetni}" ile eşleşen bir sonuç yok.</h3>
          <p class="text-[var(--text-dim)] text-sm">Yazım hatası yapmış olabilir misiniz?</p>
        </div>

        <div>
          <h4 class="text-xs font-black text-[var(--accent)] mb-6 uppercase tracking-widest">Bunları mı aratmak istemiştiniz?</h4>
          <div class="flex flex-col gap-2 max-w-4xl">
            {#each enPopulerler as sarki}
               <div 
                role="button" tabindex="0" 
                onclick={() => sarkiCal(sarki)}
                onkeydown={(e) => e.key === 'Enter' && sarkiCal(sarki)}
                class="flex items-center gap-4 p-3 bg-[var(--bg-card)] border border-[var(--border)] rounded-xl hover:bg-[var(--bg-card-hover)] transition-all cursor-pointer group"
              >
                <div class="w-10 h-10 bg-[var(--bg-surface)] rounded-lg overflow-hidden shrink-0">
                  {#if sarki.kapak_yolu}
                    <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover" />
                  {:else}
                    <div class="w-full h-full flex items-center justify-center text-xs opacity-20">🎵</div>
                  {/if}
                </div>
                <div class="flex-1 min-w-0">
                  <p class="font-bold text-sm truncate">{sarki.isim}</p>
                  <p class="text-xs text-[var(--text-dim)] truncate">{sarki.sarkici}</p>
                </div>
                <span class="text-[10px] font-bold text-[var(--accent)] uppercase">{sarki.dinlenme_sayisi} Dinlenme</span>
              </div>
            {/each}
          </div>
        </div>
      </div>

    {:else}
      <div class="max-w-6xl" in:fade>
        <div class="flex text-[10px] font-black text-[var(--text-dim)] border-b border-[var(--border)] pb-3 mb-4 px-3 tracking-widest uppercase">
          <span class="w-10 text-center">#</span>
          <span class="flex-1">BAŞLIK</span>
          <span class="w-40 text-right pr-4">İSTATİSTİK</span> 
          <span class="w-1/4 pl-6 hidden md:block">ALBÜM</span>
          <span class="w-40 text-center">İŞLEMLER</span>
        </div>

        <div class="flex flex-col gap-1">
          {#each aramaSonuclari as sarki, index}
            <div 
              role="button" tabindex="0" 
              onclick={() => sarkiCal(sarki)} 
              onkeydown={(e) => e.key === 'Enter' && sarkiCal(sarki)} 
              class="flex items-center p-2 rounded-xl hover:bg-[var(--bg-card-hover)] transition-all cursor-pointer group {playerState.aktifSarki?.id === sarki.id ? 'bg-[var(--accent)]/5 border border-[var(--accent)]/20' : 'border border-transparent'}"
            >
              <span class="w-10 text-center shrink-0">
                <span class="text-[var(--text-dim)] group-hover:hidden font-mono text-xs">{index + 1}</span>
                <svg class="w-4 h-4 mx-auto hidden group-hover:block text-[var(--accent)]" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
              </span>
              
              <div class="flex-1 flex items-center gap-4 min-w-0">
                <div class="w-11 h-11 bg-[var(--bg-surface)] rounded-lg overflow-hidden shrink-0 shadow-sm border border-[var(--border)]">
                  {#if sarki.kapak_yolu}
                    <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover" />
                  {:else}
                    <div class="w-full h-full flex items-center justify-center text-white/10">🎵</div>
                  {/if}
                </div>
                
                <div class="flex flex-col min-w-0 pr-2">
                  <span class="font-bold text-sm truncate {playerState.aktifSarki?.id === sarki.id ? 'text-[var(--accent)]' : 'text-[var(--text-main)]'}">{sarki.isim}</span>
                  <span class="text-xs text-[var(--text-dim)] truncate font-medium">{sarki.sarkici}</span>
                </div>
              </div>

              <div class="w-40 shrink-0 flex items-center justify-end pr-4 text-[var(--text-dim)]">
                <SongStats {sarki} />
              </div>

              <span class="w-1/4 text-[var(--text-dim)] truncate font-bold text-[10px] uppercase pl-6 shrink-0 hidden md:block">{sarki.album}</span>

              <div class="w-40 shrink-0 flex items-center justify-end gap-3 pr-2" 
                   role="presentation"
                   onclick={(e) => e.stopPropagation()} 
                   onkeydown={(e) => e.stopPropagation()}>
                <FavoriteButton sarkiId={sarki.id} />
                
                <select 
                  aria-label="Playliste Ekle" 
                  onchange={(e) => handlePlaylistEkle(sarki.id, e)} 
                  class="bg-[var(--bg-surface)] text-[10px] text-[var(--text-dim)] rounded-lg px-2 py-1.5 outline-none border border-[var(--border)] hover:border-[var(--accent)]/50 cursor-pointer w-24 focus:border-[var(--accent)] transition-all font-bold uppercase"
                >
                  <option value="">➕ LİSTE</option>
                  {#each playerState.playlistler as pl}
                    {#if !pl.sarkilar.includes(sarki.id)}
                      <option value={pl.id}>{pl.isim.toUpperCase()}</option>
                    {/if}
                  {/each}
                </select>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  input::selection {
    background: var(--accent);
    color: white;
  }
</style>