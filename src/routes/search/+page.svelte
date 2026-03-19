<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import SongStats from '$lib/SongStats.svelte';
  import { playerState, initializePlayer, sarkiPlaylisteEkle, sarkiCal, sarkiSil, type Sarki } from '../../store.svelte';
  import { fade, fly, scale } from 'svelte/transition';

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
            (sarki.album && sarki.album.toLowerCase().includes(aranan))
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

  async function handleSarkiSil(sarki: Sarki, event: MouseEvent | KeyboardEvent) {
    event.preventDefault();
    event.stopPropagation();
    
    const mesaj = `DİKKAT: "${sarki.isim}" adlı parçayı kütüphaneden ve diskten KALICI olarak silmek istediğinize emin misiniz?\n\nBu işlem geri alınamaz.`;
    
    if (confirm(mesaj)) {
        try {
            await sarkiSil(sarki);
        } catch (hata) {
            alert("Silme işlemi sırasında bir hata oluştu.");
        }
    }
  }

  const kategoriler = [
    { isim: "Pop", renk: "from-blue-600/40 to-blue-900/40 border-blue-500/30" },
    { isim: "Rock", renk: "from-red-600/40 to-red-900/40 border-red-500/30" },
    { isim: "Lofi", renk: "from-indigo-600/40 to-indigo-900/40 border-indigo-500/30" },
    { isim: "Electronic", renk: "from-emerald-600/40 to-emerald-900/40 border-emerald-500/30" },
    { isim: "Jazz", renk: "from-amber-600/40 to-amber-900/40 border-amber-500/30" },
    { isim: "Classical", renk: "from-zinc-600/40 to-zinc-900/40 border-zinc-500/30" },
  ];
</script>

<div class="w-full min-h-full pb-32 flex flex-col relative bg-transparent text-[var(--text-main)] transition-colors duration-500 custom-scrollbar overflow-y-auto">
  
  <div class="sticky top-0 z-30 px-8 lg:px-12 pt-10 pb-6 bg-gradient-to-b from-[var(--bg-main)] via-[var(--bg-main)]/90 to-transparent backdrop-blur-md">
    <div class="relative group max-w-4xl mx-auto">
      <div class="absolute inset-y-0 left-0 flex items-center pl-6 pointer-events-none text-[var(--text-dim)] group-focus-within:text-[var(--accent)] transition-colors">
        <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
          <circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line>
        </svg>
      </div>
      
      <input
        type="text"
        bind:value={aramaMetni}
        placeholder="Şarkı, sanatçı veya albüm ara..."
        class="w-full bg-[var(--bg-surface)] text-[var(--text-main)] placeholder-[var(--text-dim)]/50 text-lg lg:text-xl font-black rounded-2xl py-5 pl-16 pr-14 outline-none border border-[var(--border)] focus:border-[var(--accent)]/50 transition-all shadow-2xl focus:shadow-[0_10px_30px_var(--accent-glow)]"
      />

      {#if aramaMetni.length > 0}
        <button 
          type="button"
          onclick={() => aramaMetni = ""} 
          class="absolute inset-y-0 right-0 flex items-center pr-6 text-[var(--text-dim)] hover:text-[var(--accent)] transition-colors"
          aria-label="Temizle">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
        </button>
      {/if}
    </div>
  </div>

  <div class="px-8 lg:px-12 flex-1 max-w-7xl mx-auto w-full">
    {#if aramaMetni.trim() === ""}
      <div class="mt-4" in:fade>
        <h2 class="text-xs font-black text-[var(--text-dim)] mb-6 uppercase tracking-[0.3em] flex items-center gap-4">
          Frekans Kategorileri <div class="h-px flex-1 bg-[var(--border)]"></div>
        </h2>
        <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4 lg:gap-6">
          {#each kategoriler as kategori, i}
            <a href="/search?q={kategori.isim}" 
               class="relative aspect-square rounded-[var(--radius)] bg-gradient-to-br {kategori.renk} border p-5 cursor-pointer hover:scale-[1.03] transition-all duration-300 flex flex-col justify-end group shadow-lg overflow-hidden"
               in:scale={{ duration: 400, delay: i * 50 }}>
              <div class="absolute inset-0 bg-black/20 group-hover:bg-transparent transition-colors duration-500"></div>
              <span class="relative z-10 text-xl lg:text-2xl font-black tracking-tighter uppercase italic drop-shadow-md group-hover:translate-x-1 transition-transform">{kategori.isim}</span>
            </a>
          {/each}
        </div>
      </div>

    {:else if aramaSonuclari.length === 0}
      <div class="flex flex-col mt-10" in:fade>
        <div class="mb-12 text-center py-10 bg-[var(--bg-card)] border border-[var(--border)] rounded-[var(--radius)] border-dashed">
          <div class="text-5xl mb-4 opacity-30">🔍</div>
          <h3 class="text-2xl font-black text-[var(--text-main)] mb-2 tracking-tight">"{aramaMetni}" bulunamadı</h3>
          <p class="text-[var(--text-dim)] text-sm font-medium">Yazım hatası yapmış olabilir misin? Kütüphanende böyle bir veri yok.</p>
        </div>

        <div>
          <h4 class="text-[10px] font-black text-[var(--accent)] mb-6 uppercase tracking-[0.3em]">Alternatif Olarak Bunları Dinleyebilirsin</h4>
          
          <div class="flex flex-col gap-1.5">
            {#each enPopulerler as sarki, index}
              <div 
                role="button" tabindex="0" 
                onclick={() => sarkiCal(sarki)} 
                onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)} 
                aria-label="{sarki.isim} çal"
                class="flex items-center text-sm p-2.5 rounded-2xl hover:bg-[var(--bg-card-hover)] transition-all duration-300 cursor-pointer group 
                {playerState.aktifSarki?.id === sarki.id ? 'bg-[var(--accent)]/10 shadow-inner border border-[var(--accent)]/20' : 'border border-transparent'}"
              >
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
                  <div class="w-11 h-11 bg-[var(--bg-card)] rounded-lg overflow-hidden shrink-0 shadow-md border border-[var(--border)]">
                    {#if sarki.kapak_yolu}
                      <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover transition-transform group-hover:scale-110" />
                    {:else}
                      <div class="w-full h-full flex items-center justify-center text-[var(--text-dim)]/20 bg-[var(--bg-surface)]">🎵</div>
                    {/if}
                  </div>
                  <div class="flex flex-col min-w-0 pr-2">
                    <span class="font-bold text-sm truncate {playerState.aktifSarki?.id === sarki.id ? 'text-[var(--accent)]' : 'text-[var(--text-main)]'}">{sarki.isim}</span>
                    <a href="/artist/{encodeURIComponent(sarki.sarkici)}" onclick={(e) => e.stopPropagation()} class="text-[11px] text-[var(--text-dim)] truncate font-bold uppercase tracking-tight hover:text-[var(--accent)] transition-colors opacity-70 inline-block max-w-max">{sarki.sarkici}</a>
                  </div>
                </div>

                <div class="w-48 shrink-0 hidden md:flex items-center">
                  <SongStats {sarki} />
                </div>
                
                <span class="w-1/4 text-[var(--text-dim)] truncate font-black text-[10px] uppercase tracking-tighter pl-6 shrink-0 hidden lg:block opacity-60">
                  {sarki.album || "Single"}
                </span>

                <div class="w-48 flex items-center justify-end gap-3 shrink-0 pr-2" role="presentation" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
                  <FavoriteButton sarkiId={sarki.id} />
                  
                  <select 
                      aria-label="Listeye Ekle" 
                      onchange={(e) => handlePlaylistEkle(sarki.id, e)} 
                      class="bg-[var(--bg-surface)] text-[10px] text-[var(--text-dim)] rounded-lg px-2 py-1.5 outline-none border border-[var(--border)] hover:border-[var(--accent)]/50 cursor-pointer w-24 focus:border-[var(--accent)] transition-all font-bold uppercase opacity-0 group-hover:opacity-100 hidden sm:block"
                  >
                    <option value="">➕ EKLE</option>
                    {#each playerState.playlistler as pl}
                      {#if !pl.sarkilar.includes(sarki.id)}
                        <option value={pl.id}>{pl.isim.toUpperCase()}</option>
                      {/if}
                    {/each}
                  </select>

                  <button type="button" aria-label="Sil" title="Kalıcı Olarak Sil" onclick={(e) => handleSarkiSil(sarki, e)} class="text-[var(--text-dim)]/30 hover:text-red-500 transition-all p-1 opacity-0 group-hover:opacity-100">
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
                  </button>
                </div>
              </div>
            {/each}
          </div>
        </div>
      </div>

    {:else}
      <div class="mt-4" in:fade>
        <div class="flex text-[10px] font-black text-[var(--text-dim)] border-b border-[var(--border)] pb-3 mb-4 px-4 tracking-[0.2em] uppercase">
          <span class="w-10 text-center shrink-0">#</span>
          <span class="flex-1 min-w-0 ml-4">BAŞLIK</span>
          <span class="w-48 shrink-0 hidden md:block">İSTATİSTİK</span> 
          <span class="w-1/4 shrink-0 pl-6 hidden lg:block">ALBÜM</span>
          <span class="w-48 text-right pr-4 shrink-0">İŞLEMLER</span>
        </div>

        <div class="flex flex-col gap-1.5">
          {#each aramaSonuclari as sarki, index}
            <div 
              role="button" tabindex="0" 
              onclick={() => sarkiCal(sarki)} 
              onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)} 
              aria-label="{sarki.isim} çal"
              class="flex items-center text-sm p-2.5 rounded-2xl hover:bg-[var(--bg-card-hover)] transition-all duration-300 cursor-pointer group 
              {playerState.aktifSarki?.id === sarki.id ? 'bg-[var(--accent)]/10 shadow-inner border border-[var(--accent)]/20' : 'border border-transparent'}"
            >
              
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
                <div class="w-11 h-11 bg-[var(--bg-card)] rounded-lg overflow-hidden shrink-0 shadow-md border border-[var(--border)]">
                  {#if sarki.kapak_yolu}
                    <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover transition-transform group-hover:scale-110" />
                  {:else}
                    <div class="w-full h-full flex items-center justify-center text-[var(--text-dim)]/20 bg-[var(--bg-surface)]">🎵</div>
                  {/if}
                </div>
                <div class="flex flex-col min-w-0 pr-2">
                  <span class="font-bold text-sm truncate {playerState.aktifSarki?.id === sarki.id ? 'text-[var(--accent)]' : 'text-[var(--text-main)]'}">{sarki.isim}</span>
                  <a href="/artist/{encodeURIComponent(sarki.sarkici)}" onclick={(e) => e.stopPropagation()} class="text-[11px] text-[var(--text-dim)] truncate font-bold uppercase tracking-tight hover:text-[var(--accent)] transition-colors opacity-70 inline-block max-w-max">{sarki.sarkici}</a>
                </div>
              </div>

              <div class="w-48 shrink-0 hidden md:flex items-center">
                <SongStats {sarki} />
              </div>
              
              <span class="w-1/4 text-[var(--text-dim)] truncate font-black text-[10px] uppercase tracking-tighter pl-6 shrink-0 hidden lg:block opacity-60">
                {sarki.album || "Single"}
              </span>

              <div class="w-48 flex items-center justify-end gap-3 shrink-0 pr-2" role="presentation" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
                
                <FavoriteButton sarkiId={sarki.id} />
                
                <select 
                    aria-label="Listeye Ekle" 
                    onchange={(e) => handlePlaylistEkle(sarki.id, e)} 
                    class="bg-[var(--bg-surface)] text-[10px] text-[var(--text-dim)] rounded-lg px-2 py-1.5 outline-none border border-[var(--border)] hover:border-[var(--accent)]/50 cursor-pointer w-24 focus:border-[var(--accent)] transition-all font-bold uppercase opacity-0 group-hover:opacity-100 hidden sm:block"
                >
                  <option value="">➕ EKLE</option>
                  {#each playerState.playlistler as pl}
                    {#if !pl.sarkilar.includes(sarki.id)}
                      <option value={pl.id}>{pl.isim.toUpperCase()}</option>
                    {/if}
                  {/each}
                </select>

                <button type="button" aria-label="Sil" title="Kalıcı Olarak Sil" onclick={(e) => handleSarkiSil(sarki, e)} class="text-[var(--text-dim)]/30 hover:text-red-500 transition-all p-1 opacity-0 group-hover:opacity-100">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
                </button>
              </div>
              
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  @keyframes bounce {
    0%, 100% { height: 4px; }
    50% { height: 14px; }
  }

  input::selection {
    background: var(--accent);
    color: white;
  }

  .custom-scrollbar::-webkit-scrollbar { width: 4px; }
  .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }
</style>