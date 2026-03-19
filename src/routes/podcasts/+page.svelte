<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { fly, fade, scale } from 'svelte/transition';
  import SongStats from '$lib/SongStats.svelte';
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import { playerState, sarkiCal, initializePlayer, sarkiSil, sarkiPlaylisteEkle, type Sarki } from '../../store.svelte';

  onMount(async () => {
    if (playerState.sarkiListesi.length === 0) {
      await initializePlayer();
    }
  });

  let podcastListesi = $derived(
    playerState.sarkiListesi.filter(s => s.tarz?.toLowerCase() === 'podcast')
  );

  let sonPodcastler = $derived(
    [...podcastListesi].reverse().slice(0, 3)
  );

  function formatTarih() {
    return new Date().toLocaleDateString('tr-TR', { day: 'numeric', month: 'long', year: 'numeric' });
  }

  async function handleSarkiSil(sarki: Sarki, event: MouseEvent | KeyboardEvent) {
    event.preventDefault();
    event.stopPropagation();
    
    const mesaj = `DİKKAT: "${sarki.isim}" adlı yayını kütüphaneden ve diskten KALICI olarak silmek istediğinize emin misiniz?\n\nBu işlem geri alınamaz.`;
    
    if (confirm(mesaj)) {
        try {
            await sarkiSil(sarki);
        } catch (hata) {
            alert("Silme işlemi sırasında bir hata oluştu.");
        }
    }
  }

  async function handlePlaylistEkle(sarkiId: string, event: Event) {
    const selectElement = event.target as HTMLSelectElement;
    const playlistId = selectElement.value;
    if (!playlistId) return;

    const basarili = await sarkiPlaylisteEkle(sarkiId, playlistId);
    if(basarili) {
        selectElement.value = ""; 
    }
  }
</script>

<div class="p-8 lg:p-12 w-full min-h-full pb-32 flex flex-col relative min-w-0 bg-transparent text-[var(--text-main)] transition-colors duration-500 overflow-y-auto custom-scrollbar">
  
  <header class="mb-12 relative group" in:fly={{ y: -20, duration: 600 }}>
    <div class="absolute -inset-4 bg-gradient-to-r from-[var(--accent)]/10 to-transparent blur-2xl opacity-50 rounded-3xl -z-10"></div>
    
    <div class="flex items-center gap-6">
      <div class="w-16 h-16 lg:w-20 lg:h-20 bg-[var(--accent)] rounded-2xl flex items-center justify-center shadow-2xl rotate-3 group-hover:rotate-0 transition-all duration-500">
        <svg class="w-8 h-8 lg:w-10 lg:h-10 text-white" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true">
          <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"></path>
          <path d="M19 10v2a7 7 0 0 1-14 0v-2"></path>
          <line x1="12" y1="19" x2="12" y2="23"></line>
          <line x1="8" y1="23" x2="16" y2="23"></line>
        </svg>
      </div>
      
      <div class="min-w-0">
        <h1 class="text-4xl lg:text-5xl font-black italic tracking-tighter uppercase leading-none">Podcast Yayınları</h1>
        <p class="text-[var(--text-dim)] mt-2 font-bold uppercase tracking-[0.3em] text-[10px] lg:text-xs">
          {podcastListesi.length} Oturum Arşivlendi • {formatTarih()}
        </p>
      </div>
    </div>
  </header>

  {#if podcastListesi.length === 0}
    <div class="flex-1 flex flex-col items-center justify-center border-2 border-dashed border-[var(--border)] rounded-[var(--radius)] p-16 text-center" in:fade>
      <div class="w-20 h-20 bg-[var(--bg-card)] rounded-full flex items-center justify-center mb-6">
        <svg class="w-10 h-10 text-[var(--text-dim)]/20" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm-2-13.5l6 3.5-6 3.5v-7z"/></svg>
      </div>
      <h3 class="text-xl font-bold uppercase italic mb-2">Henüz Kayıt Bulunamadı</h3>
      <p class="text-[var(--text-dim)] max-w-sm mx-auto text-sm leading-relaxed font-medium">
        Kütüphanene eklediğin dosyalardan tarzı <span class="text-[var(--accent)] font-bold">"Podcast"</span> olarak belirlenenler burada otomatik olarak listelenir.
      </p>
    </div>
  {:else}
    
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-16">
      {#each sonPodcastler as pc, i}
        <button 
          type="button"
          onclick={() => sarkiCal(pc)}
          aria-label="{pc.isim} dinle"
          class="bg-[var(--bg-card)] border border-[var(--border)] p-6 rounded-[var(--radius)] flex flex-col gap-4 text-left hover:border-[var(--accent)]/40 transition-all group relative overflow-hidden shadow-lg active:scale-[0.98]"
          in:scale={{ duration: 400, start: 0.95, delay: i * 100 }}
        >
          <div class="w-full aspect-video rounded-[calc(var(--radius)-0.5rem)] bg-[var(--bg-surface)] overflow-hidden relative shadow-inner border border-[var(--border)] group-hover:border-[var(--accent)]/30 transition-colors">
            {#if pc.kapak_yolu}
              <img src={convertFileSrc(pc.kapak_yolu)} alt="" class="w-full h-full object-cover transition-transform duration-700 opacity-70 group-hover:scale-105 group-hover:opacity-100" />
            {:else}
              <div class="w-full h-full flex items-center justify-center opacity-10">
                 <svg class="w-12 h-12" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"></path></svg>
              </div>
            {/if}
            
            <div class="absolute inset-0 bg-black/20 group-hover:bg-transparent transition-colors duration-500"></div>

            <div class="absolute inset-0 flex items-center justify-center">
              <div class="w-14 h-14 bg-[var(--accent)] text-white rounded-full flex items-center justify-center opacity-0 group-hover:opacity-100 scale-50 group-hover:scale-110 transition-all duration-300 shadow-[0_8px_20px_rgba(0,0,0,0.4)]">
                <svg class="w-6 h-6 ml-1" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
              </div>
            </div>
          </div>

          <div class="min-w-0 w-full pr-2">
            <div class="flex items-center justify-between mb-1.5">
                <span class="text-[9px] font-black text-[var(--accent)] uppercase tracking-[0.2em]">Son Oturum</span>
                <div onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="presentation">
                    <FavoriteButton sarkiId={pc.id} />
                </div>
            </div>
            <h4 class="text-[var(--text-main)] font-black truncate text-lg leading-tight group-hover:text-[var(--accent)] transition-colors">{pc.isim}</h4>
            <p class="text-[var(--text-dim)] text-xs truncate mt-1.5 font-bold uppercase tracking-widest">{pc.sarkici}</p>
          </div>
        </button>
      {/each}
    </div>

    <div class="flex items-center justify-between mb-8 px-2">
        <h3 class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-[0.4em]">Arşivdeki Tüm Oturumlar</h3>
        <div class="h-px flex-1 bg-[var(--border)] ml-6 opacity-50"></div>
    </div>

    <div class="flex flex-col gap-1.5">
      {#each podcastListesi as pc, index}
        <div 
          role="button" tabindex="0"
          onclick={() => sarkiCal(pc)}
          onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(pc)}
          aria-label="{pc.isim} oturumunu başlat"
          class="flex items-center gap-5 p-3 rounded-2xl transition-all cursor-pointer group shadow-sm border {playerState.aktifSarki?.id === pc.id ? 'bg-[var(--accent)]/10 border-[var(--accent)]/30' : 'bg-[var(--bg-card)] border-[var(--border)] hover:bg-[var(--bg-card-hover)] hover:border-[var(--accent)]/30'}"
        >
          
          <div class="w-8 text-center shrink-0">
             {#if playerState.aktifSarki?.id === pc.id && playerState.suAnOynuyorMu}
                <div class="flex items-end justify-center gap-0.5 h-3.5 mb-1">
                   <div class="w-1 bg-[var(--accent)] animate-[bounce_1s_infinite]"></div>
                   <div class="w-1 bg-[var(--accent)] animate-[bounce_1.2s_infinite]"></div>
                   <div class="w-1 bg-[var(--accent)] animate-[bounce_0.8s_infinite]"></div>
                </div>
             {:else}
                <span class="text-xl font-black text-[var(--text-dim)]/20 group-hover:hidden transition-colors font-serif italic">
                    {index + 1}
                </span>
                <svg class="w-4 h-4 mx-auto hidden group-hover:block text-[var(--accent)]" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M8 5v14l11-7z"/></svg>
             {/if}
          </div>

          <div class="w-12 h-12 lg:w-14 lg:h-14 rounded-xl bg-[var(--bg-surface)] overflow-hidden shrink-0 shadow-md flex items-center justify-center border border-[var(--border)] relative">
            {#if pc.kapak_yolu}
              <img src={convertFileSrc(pc.kapak_yolu)} alt="" class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110" />
            {:else}
              <svg class="w-6 h-6 text-[var(--accent)] opacity-30" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24" aria-hidden="true"><path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"></path></svg>
            {/if}
          </div>

          <div class="flex-1 min-w-0 pr-2">
            <span class="font-bold text-[var(--text-main)] text-sm lg:text-base truncate block group-hover:text-[var(--accent)] transition-colors">{pc.isim}</span>
            <div class="flex items-center gap-2 mt-1">
                <span class="text-[10px] text-[var(--text-dim)] truncate font-bold uppercase tracking-wider">{pc.sarkici}</span>
                <span class="w-1 h-1 rounded-full bg-[var(--text-dim)] opacity-30"></span>
                <span class="text-[10px] text-[var(--text-dim)] truncate font-bold uppercase tracking-wider opacity-60">{pc.album}</span>
            </div>
          </div>

          <div class="shrink-0 hidden md:block">
            <SongStats sarki={pc} />
          </div>
          
          <div class="shrink-0 flex items-center gap-3 pr-2" role="presentation" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
             <select 
                 aria-label="Listeye Ekle" 
                 onchange={(e) => handlePlaylistEkle(pc.id, e)} 
                 class="bg-[var(--bg-surface)] text-[9px] text-[var(--text-dim)] rounded-lg px-1.5 py-1 outline-none border border-[var(--border)] hover:border-[var(--accent)]/50 cursor-pointer w-20 focus:border-[var(--accent)] transition-all font-bold uppercase opacity-0 group-hover:opacity-100 hidden sm:block"
             >
               <option value="">➕ EKLE</option>
               {#each playerState.playlistler as pl}
                 {#if !pl.sarkilar.includes(pc.id)}
                   <option value={pl.id}>{pl.isim.toUpperCase()}</option>
                 {/if}
               {/each}
             </select>

             <FavoriteButton sarkiId={pc.id} />
             
             <button 
                type="button" 
                aria-label="Sil" 
                title="Kalıcı Olarak Sil" 
                onclick={(e) => handleSarkiSil(pc, e)} 
                class="text-[var(--text-dim)]/30 hover:text-red-500 transition-all p-1 opacity-0 group-hover:opacity-100"
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

  h1 { text-shadow: 0 10px 30px rgba(0,0,0,0.4); }

  .custom-scrollbar::-webkit-scrollbar { width: 4px; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }
</style>